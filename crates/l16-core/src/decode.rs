use crate::{baseline::Baseline, schema::{bundled_schema, validate_message, Bundle,
    Interpretation, MessageSpec, RunMode, SchemaError}, word::{mask, BitRange, Word, WordError}};
use std::{collections::HashSet, fmt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionEvidence { description: String }
impl SelectionEvidence {
    /// Caller-provided provenance, not authenticated proof of a baseline.
    pub fn new(description: impl Into<String>) -> Result<Self, DecodeError> {
        let description = description.into();
        if description.trim().is_empty() || description.len() > 4096 {
            return Err(DecodeError::InvalidContext("selection evidence must contain 1..=4096 bytes"));
        }
        Ok(Self { description })
    }
    pub fn description(&self) -> &str { &self.description }
}
#[derive(Clone, Debug)]
pub struct DecodeContext {
    pub source_id: String,
    /// Supplied by the adapter/caller; this library does not open the source.
    pub source_offset: u64,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Unsigned(u128),
    Signed(i128),
    Rational { numerator: i128, denominator: u64, unit: &'static str },
    Enumeration { raw: u128, label: &'static str },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldStatus {
    Value(Value),
    Special(&'static str),
    NotApplicable { selector: &'static str },
    UnresolvedContext { selector: &'static str },
    InvalidEncoding(&'static str),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodedField {
    pub id: &'static str,
    pub definition_ref: &'static str,
    pub lsb: u8,
    pub width: u8,
    pub raw: u128,
    pub status: FieldStatus,
}
#[derive(Clone, Debug)]
pub struct DecodedRecord {
    pub baseline: Baseline,
    pub schema_id: &'static str,
    pub schema_version: &'static str,
    pub schema_digest_sha256: &'static str,
    pub profile_id: &'static str,
    pub decoder_version: &'static str,
    pub output_version: &'static str,
    pub synthetic: bool,
    pub selection_evidence: String,
    pub source_id: String,
    pub source_offset: u64,
    pub message_id: &'static str,
    /// A normalized logical value, NOT a reconstruction of captured bytes.
    pub original_word: Word,
    pub fields: Vec<DecodedField>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecodeError {
    NoVerifiedSchema(Baseline),
    BaselineMismatch,
    Schema(SchemaError),
    Word(WordError),
    UnsupportedMessage { baseline: Baseline, message_id: String, word: Word },
    LengthMismatch { expected: u8, actual: u8 },
    InvalidContext(&'static str),
    InternalInvariant(&'static str),
}
impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoVerifiedSchema(b) => write!(f, "no verified MIL-STD-6016 schema is installed for {b}; demo schemas cannot be used for standards decoding"),
            Self::BaselineMismatch => f.write_str("selected baseline does not match the schema bundle"),
            Self::Schema(e) => write!(f, "schema: {e}"),
            Self::Word(e) => write!(f, "word: {e}"),
            Self::UnsupportedMessage { baseline, message_id, .. } => write!(f, "message {message_id:?} is not available in selected baseline {baseline}; no fallback attempted"),
            Self::LengthMismatch { expected, actual } => write!(f, "word length mismatch: expected {expected}, received {actual}"),
            Self::InvalidContext(s) | Self::InternalInvariant(s) => f.write_str(s),
        }
    }
}
impl std::error::Error for DecodeError {}
impl From<SchemaError> for DecodeError { fn from(e: SchemaError) -> Self { Self::Schema(e) } }
impl From<WordError> for DecodeError { fn from(e: WordError) -> Self { Self::Word(e) } }

struct CompiledMessage {
    spec: &'static MessageSpec,
    order: Vec<usize>,
    ranges: Vec<BitRange>,
    selectors: Vec<Option<(usize, u128, &'static str)>>,
}
/// Immutable after construction; independent instances cannot change each
/// other's revision selection. Schema work happens at construction, not per field.
pub struct Decoder {
    bundle: &'static Bundle,
    messages: Vec<CompiledMessage>,
}
impl Decoder {
    pub fn new(baseline: Baseline, mode: RunMode) -> Result<Self, DecodeError> {
        Self::from_bundle(bundled_schema(baseline), baseline, mode)
    }
    /// Public for integration tests/custom SYNTHETIC bundles. There is no API
    /// to mark a package authoritative in this increment.
    pub fn from_bundle(bundle: &'static Bundle, expected: Baseline, mode: RunMode) -> Result<Self, DecodeError> {
        if bundle.baseline != expected { return Err(DecodeError::BaselineMismatch); }
        if mode == RunMode::StandardsDecode { return Err(DecodeError::NoVerifiedSchema(expected)); }
        if bundle.digest_sha256.len() != 64
            || !bundle.digest_sha256.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)) {
            return Err(SchemaError("invalid SHA-256 identity string".into()).into());
        }
        if [bundle.schema_id, bundle.schema_version, bundle.profile_id].iter()
            .any(|s| s.is_empty() || s.len() > 512) {
            return Err(SchemaError("invalid schema metadata".into()).into());
        }
        if bundle.messages.is_empty() || bundle.messages.len() > 128 {
            return Err(SchemaError("message count must be 1..=128".into()).into());
        }
        let mut ids = HashSet::new();
        let mut messages = Vec::with_capacity(bundle.messages.len());
        for spec in bundle.messages {
            if !ids.insert(spec.id) { return Err(SchemaError("duplicate message identifier".into()).into()); }
            let order = validate_message(spec)?;
            let mut ranges = Vec::with_capacity(spec.fields.len());
            let mut selectors = Vec::with_capacity(spec.fields.len());
            for field in spec.fields {
                ranges.push(BitRange::new(field.lsb, field.width, spec.word_bits)?);
                let selector = match field.condition {
                    None => None,
                    Some(c) => {
                        let index = spec.fields.iter().position(|s| s.id == c.selector)
                            .ok_or(DecodeError::InternalInvariant("validated selector not found"))?;
                        Some((index, c.equals, c.selector))
                    }
                };
                selectors.push(selector);
            }
            messages.push(CompiledMessage { spec, order, ranges, selectors });
        }
        Ok(Self { bundle, messages })
    }
    pub fn bundle(&self) -> &'static Bundle { self.bundle }
    pub fn message_word_bits(&self, id: &str) -> Option<u8> {
        self.messages.iter().find(|m| m.spec.id == id).map(|m| m.spec.word_bits)
    }
    pub fn decode(&self, message_id: &str, word: Word, evidence: &SelectionEvidence,
                  context: &DecodeContext) -> Result<DecodedRecord, DecodeError> {
        if context.source_id.trim().is_empty() || context.source_id.len() > 4096 {
            return Err(DecodeError::InvalidContext("source identifier must contain 1..=4096 bytes"));
        }
        let message = self.messages.iter().find(|m| m.spec.id == message_id)
            .ok_or_else(|| DecodeError::UnsupportedMessage {
                baseline: self.bundle.baseline, message_id: message_id.into(), word })?;
        if word.bit_len() != message.spec.word_bits {
            return Err(DecodeError::LengthMismatch { expected: message.spec.word_bits, actual: word.bit_len() });
        }
        let raw: Vec<u128> = message.ranges.iter().map(|r| word.extract(*r))
            .collect::<Result<_, _>>()?;
        let mut statuses: Vec<Option<FieldStatus>> = vec![None; raw.len()];
        for &index in &message.order {
            let spec = &message.spec.fields[index];
            let status = match message.selectors[index] {
                Some((selector, equal, name)) => match &statuses[selector] {
                    Some(FieldStatus::Value(_)) if raw[selector] == equal => interpret(spec, raw[index]),
                    Some(FieldStatus::Value(_)) => FieldStatus::NotApplicable { selector: name },
                    Some(_) => FieldStatus::UnresolvedContext { selector: name },
                    None => return Err(DecodeError::InternalInvariant("selector evaluated out of dependency order")),
                },
                None => interpret(spec, raw[index]),
            };
            statuses[index] = Some(status);
        }
        let mut fields = Vec::with_capacity(raw.len());
        for (index, spec) in message.spec.fields.iter().enumerate() {
            let status = statuses[index].take()
                .ok_or(DecodeError::InternalInvariant("field not interpreted"))?;
            fields.push(DecodedField { id: spec.id, definition_ref: spec.definition_ref,
                lsb: spec.lsb, width: spec.width, raw: raw[index], status });
        }
        Ok(DecodedRecord {
            baseline: self.bundle.baseline, schema_id: self.bundle.schema_id,
            schema_version: self.bundle.schema_version,
            schema_digest_sha256: self.bundle.digest_sha256, profile_id: self.bundle.profile_id,
            decoder_version: env!("CARGO_PKG_VERSION"), output_version: "synthetic-record-v1",
            synthetic: true, selection_evidence: evidence.description().into(),
            source_id: context.source_id.clone(), source_offset: context.source_offset,
            message_id: message.spec.id, original_word: word, fields,
        })
    }
}

fn interpret(spec: &crate::schema::FieldSpec, raw: u128) -> FieldStatus {
    // Applicability has already been checked; special encodings precede scaling.
    if let Some((_, label)) = spec.specials.iter().find(|(code, _)| *code == raw) {
        return FieldStatus::Special(label);
    }
    let value = match spec.interpretation {
        Interpretation::Unsigned => Value::Unsigned(raw),
        Interpretation::TwosComplement => {
            let signed = if spec.width == 128 { raw as i128 }
                else if raw & (1u128 << (spec.width - 1)) != 0 { (raw | !mask(spec.width)) as i128 }
                else { raw as i128 };
            Value::Signed(signed)
        }
        Interpretation::ScaledUnsigned { numerator, denominator, unit } => {
            let Ok(integer) = i128::try_from(raw) else { return FieldStatus::InvalidEncoding("numeric overflow"); };
            let Some(product) = integer.checked_mul(i128::from(numerator)) else { return FieldStatus::InvalidEncoding("numeric overflow"); };
            Value::Rational { numerator: product, denominator, unit }
        }
        Interpretation::Enumeration(items) => match items.iter().find(|(code, _)| *code == raw) {
            Some((_, label)) => Value::Enumeration { raw, label },
            None => return FieldStatus::InvalidEncoding("undefined enumeration code"),
        },
    };
    FieldStatus::Value(value)
}
