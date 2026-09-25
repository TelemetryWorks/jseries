use crate::{
    assembly::AssembledMessage,
    schema::{Interpretation, SchemaError, SchemaPackage, find_code, validate_message},
    word::{BitRange, mask},
};
use std::{collections::HashMap, fmt, sync::Arc};

#[derive(Clone, Debug)]
pub struct DecodeContext {
    pub source_id: Arc<str>,
    pub source_offset: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Unsigned(u128),
    Signed(i128),
    Rational {
        numerator: i128,
        denominator: u64,
        unit: Arc<str>,
    },
    Enumeration {
        raw: u128,
        label: Arc<str>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldStatus {
    Value(Value),
    Special(Arc<str>),
    NotApplicable { selector: Arc<str> },
    UnresolvedContext { selector: Arc<str> },
    InvalidEncoding(&'static str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodedField {
    pub id: Arc<str>,
    pub definition_ref: Arc<str>,
    pub word: u8,
    pub lsb: u8,
    pub width: u8,
    pub raw: u128,
    pub status: FieldStatus,
}

#[derive(Clone, Debug)]
pub struct DecodedRecord {
    pub package_id: Arc<str>,
    pub package_version: Arc<str>,
    pub baseline: Arc<str>,
    pub profile: Arc<str>,
    pub qualification: Arc<str>,
    pub source: Arc<str>,
    pub decoder_version: &'static str,
    pub source_id: Arc<str>,
    pub source_offset: u64,
    pub message_id: Arc<str>,
    pub fields: Vec<DecodedField>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecodeError {
    Schema(SchemaError),
    UnsupportedMessage(String),
    WordCountMismatch { expected: u8, actual: usize },
    InvalidContext(&'static str),
    InternalInvariant(&'static str),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Schema(error) => write!(f, "schema: {error}"),
            Self::UnsupportedMessage(id) => {
                write!(f, "message {id:?} is not in the selected package")
            }
            Self::WordCountMismatch { expected, actual } => write!(
                f,
                "word count mismatch: expected {expected}, received {actual}"
            ),
            Self::InvalidContext(message) | Self::InternalInvariant(message) => {
                f.write_str(message)
            }
        }
    }
}
impl std::error::Error for DecodeError {}
impl From<SchemaError> for DecodeError {
    fn from(error: SchemaError) -> Self {
        Self::Schema(error)
    }
}

struct CompiledMessage {
    spec_index: usize,
    order: Vec<usize>,
    ranges: Vec<BitRange>,
    selectors: Vec<Option<(usize, u128, Arc<str>)>>,
}

/// An immutable decode plan. Parsing and name resolution happen at construction;
/// the hot path performs one message lookup followed by indexed operations.
pub struct Decoder {
    package: Arc<SchemaPackage>,
    messages: Vec<CompiledMessage>,
    message_index: HashMap<Arc<str>, usize>,
}

impl Decoder {
    pub fn new(package: Arc<SchemaPackage>) -> Result<Self, DecodeError> {
        let mut messages = Vec::with_capacity(package.messages.len());
        let mut message_index = HashMap::with_capacity(package.messages.len());
        for (spec_index, spec) in package.messages.iter().enumerate() {
            let order = validate_message(spec)?;
            let mut ranges = Vec::with_capacity(spec.fields.len());
            let mut selectors = Vec::with_capacity(spec.fields.len());
            for field in &*spec.fields {
                ranges.push(
                    BitRange::new(field.lsb, field.width)
                        .map_err(|e| SchemaError(e.to_string()))?,
                );
                selectors.push(field.condition.as_ref().map(|condition| {
                    let index = spec
                        .fields
                        .iter()
                        .position(|candidate| candidate.id == condition.selector)
                        .expect("validated selector must exist");
                    (index, condition.equals, Arc::clone(&condition.selector))
                }));
            }
            let index = messages.len();
            message_index.insert(Arc::clone(&spec.id), index);
            messages.push(CompiledMessage {
                spec_index,
                order,
                ranges,
                selectors,
            });
        }
        Ok(Self {
            package,
            messages,
            message_index,
        })
    }

    pub fn package(&self) -> &SchemaPackage {
        &self.package
    }

    #[inline]
    pub fn decode(
        &self,
        message_id: &str,
        message: &AssembledMessage,
        context: &DecodeContext,
    ) -> Result<DecodedRecord, DecodeError> {
        validate_context(context)?;
        let compiled = self.compiled_message(message_id)?;
        let spec = &self.package.messages[compiled.spec_index];
        self.decode_compiled(spec, compiled, message, context)
    }

    /// Decode a homogeneous batch while resolving the immutable message plan once.
    ///
    /// Offsets are assigned consecutively beginning at `start_offset`. The returned
    /// records preserve input order.
    pub fn decode_many(
        &self,
        message_id: &str,
        messages: &[AssembledMessage],
        source_id: Arc<str>,
        start_offset: u64,
    ) -> Result<Vec<DecodedRecord>, DecodeError> {
        let context = DecodeContext {
            source_id: Arc::clone(&source_id),
            source_offset: start_offset,
        };
        validate_context(&context)?;
        let compiled = self.compiled_message(message_id)?;
        let spec = &self.package.messages[compiled.spec_index];
        let mut records = Vec::with_capacity(messages.len());
        for (index, message) in messages.iter().enumerate() {
            let index = u64::try_from(index)
                .map_err(|_| DecodeError::InvalidContext("source offset overflow"))?;
            let source_offset = start_offset
                .checked_add(index)
                .ok_or(DecodeError::InvalidContext("source offset overflow"))?;
            records.push(self.decode_compiled(
                spec,
                compiled,
                message,
                &DecodeContext {
                    source_id: Arc::clone(&source_id),
                    source_offset,
                },
            )?);
        }
        Ok(records)
    }

    #[inline]
    fn compiled_message(&self, message_id: &str) -> Result<&CompiledMessage, DecodeError> {
        self.message_index
            .get(message_id)
            .and_then(|index| self.messages.get(*index))
            .ok_or_else(|| DecodeError::UnsupportedMessage(message_id.into()))
    }

    #[inline]
    fn decode_compiled(
        &self,
        spec: &crate::schema::MessageSpec,
        compiled: &CompiledMessage,
        message: &AssembledMessage,
        context: &DecodeContext,
    ) -> Result<DecodedRecord, DecodeError> {
        if message.words().len() != usize::from(spec.word_count) {
            return Err(DecodeError::WordCountMismatch {
                expected: spec.word_count,
                actual: message.words().len(),
            });
        }
        let mut raw = Vec::with_capacity(spec.fields.len());
        for (field, range) in spec.fields.iter().zip(&compiled.ranges) {
            raw.push(
                message.words()[usize::from(field.word)]
                    .information
                    .extract(*range),
            );
        }
        let mut statuses = vec![None; raw.len()];
        for &index in &compiled.order {
            let field = &spec.fields[index];
            let status = match &compiled.selectors[index] {
                Some((selector, expected, name)) => match &statuses[*selector] {
                    Some(FieldStatus::Value(_)) if raw[*selector] == *expected => {
                        interpret(field, raw[index])
                    }
                    Some(FieldStatus::Value(_)) => FieldStatus::NotApplicable {
                        selector: Arc::clone(name),
                    },
                    Some(_) => FieldStatus::UnresolvedContext {
                        selector: Arc::clone(name),
                    },
                    None => {
                        return Err(DecodeError::InternalInvariant(
                            "selector evaluated out of dependency order",
                        ));
                    }
                },
                None => interpret(field, raw[index]),
            };
            statuses[index] = Some(status);
        }
        let fields = spec
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| DecodedField {
                id: Arc::clone(&field.id),
                definition_ref: Arc::clone(&field.definition_ref),
                word: field.word,
                lsb: field.lsb,
                width: field.width,
                raw: raw[index],
                status: statuses[index]
                    .take()
                    .expect("all validated fields are evaluated"),
            })
            .collect();
        let metadata = &self.package.metadata;
        Ok(DecodedRecord {
            package_id: Arc::clone(&metadata.id),
            package_version: Arc::clone(&metadata.version),
            baseline: Arc::clone(&metadata.baseline),
            profile: Arc::clone(&metadata.profile),
            qualification: Arc::clone(&metadata.qualification),
            source: Arc::clone(&metadata.source),
            decoder_version: env!("CARGO_PKG_VERSION"),
            source_id: Arc::clone(&context.source_id),
            source_offset: context.source_offset,
            message_id: Arc::clone(&spec.id),
            fields,
        })
    }
}

fn validate_context(context: &DecodeContext) -> Result<(), DecodeError> {
    if context.source_id.trim().is_empty() || context.source_id.len() > 4096 {
        return Err(DecodeError::InvalidContext(
            "source identifier must contain 1..=4096 bytes",
        ));
    }
    Ok(())
}

#[inline]
fn interpret(field: &crate::schema::FieldSpec, raw: u128) -> FieldStatus {
    if let Some(entry) = find_code(&field.specials, raw) {
        return FieldStatus::Special(Arc::clone(&entry.label));
    }
    let value = match &field.interpretation {
        Interpretation::Unsigned => Value::Unsigned(raw),
        Interpretation::TwosComplement => {
            let value = if raw & (1u128 << (field.width - 1)) != 0 {
                (raw | !mask(field.width)) as i128
            } else {
                raw as i128
            };
            Value::Signed(value)
        }
        Interpretation::ScaledUnsigned {
            numerator,
            denominator,
            unit,
        } => {
            let Ok(raw) = i128::try_from(raw) else {
                return FieldStatus::InvalidEncoding("numeric overflow");
            };
            let Some(numerator) = raw.checked_mul(i128::from(*numerator)) else {
                return FieldStatus::InvalidEncoding("numeric overflow");
            };
            Value::Rational {
                numerator,
                denominator: *denominator,
                unit: Arc::clone(unit),
            }
        }
        Interpretation::Enumeration(entries) => match find_code(entries, raw) {
            Some(entry) => Value::Enumeration {
                raw,
                label: Arc::clone(&entry.label),
            },
            None => return FieldStatus::InvalidEncoding("undefined enumeration code"),
        },
    };
    FieldStatus::Value(value)
}
