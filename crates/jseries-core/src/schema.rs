use crate::word::{BitRange, mask};
use std::{collections::HashMap, fmt, sync::Arc};

pub const MAX_MESSAGES: usize = 4096;
pub const MAX_FIELDS_PER_MESSAGE: usize = 4096;
pub const MAX_WORDS_PER_MESSAGE: u8 = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeLabel {
    pub raw: u128,
    pub label: Arc<str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Condition {
    pub selector: Arc<str>,
    pub equals: u128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Interpretation {
    Unsigned,
    TwosComplement,
    ScaledUnsigned {
        numerator: i64,
        denominator: u64,
        unit: Arc<str>,
    },
    Enumeration(Arc<[CodeLabel]>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldSpec {
    pub id: Arc<str>,
    pub definition_ref: Arc<str>,
    pub word: u8,
    pub lsb: u8,
    pub width: u8,
    pub interpretation: Interpretation,
    pub specials: Arc<[CodeLabel]>,
    pub condition: Option<Condition>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageSpec {
    pub id: Arc<str>,
    pub word_count: u8,
    pub fields: Arc<[FieldSpec]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageMetadata {
    pub format_version: u16,
    pub id: Arc<str>,
    pub version: Arc<str>,
    pub baseline: Arc<str>,
    pub profile: Arc<str>,
    pub qualification: Arc<str>,
    pub source: Arc<str>,
}

#[derive(Clone, Debug)]
pub struct SchemaPackage {
    pub metadata: PackageMetadata,
    pub messages: Arc<[MessageSpec]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaError(pub String);

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SchemaError {}

impl SchemaPackage {
    pub fn new(metadata: PackageMetadata, messages: Vec<MessageSpec>) -> Result<Self, SchemaError> {
        validate_text("package id", &metadata.id)?;
        validate_text("package version", &metadata.version)?;
        validate_text("baseline", &metadata.baseline)?;
        validate_text("profile", &metadata.profile)?;
        validate_text("qualification", &metadata.qualification)?;
        validate_text("source", &metadata.source)?;
        if metadata.format_version != 1 {
            return Err(SchemaError("unsupported package format version".into()));
        }
        if messages.is_empty() || messages.len() > MAX_MESSAGES {
            return Err(SchemaError(format!(
                "message count must be 1..={MAX_MESSAGES}"
            )));
        }

        let mut ids = HashMap::with_capacity(messages.len());
        for message in &messages {
            if ids.insert(message.id.as_ref(), ()).is_some() {
                return Err(SchemaError(format!(
                    "duplicate message identifier {}",
                    message.id
                )));
            }
            validate_message(message)?;
        }

        Ok(Self {
            metadata,
            messages: messages.into(),
        })
    }
}

pub fn validate_message(message: &MessageSpec) -> Result<Vec<usize>, SchemaError> {
    let fail = |text: String| SchemaError(format!("{}: {text}", message.id));
    validate_text("message id", &message.id).map_err(|error| fail(error.0))?;
    if message.word_count == 0 || message.word_count > MAX_WORDS_PER_MESSAGE {
        return Err(fail(format!(
            "word count must be 1..={MAX_WORDS_PER_MESSAGE}"
        )));
    }
    if message.fields.is_empty() || message.fields.len() > MAX_FIELDS_PER_MESSAGE {
        return Err(fail(format!(
            "field count must be 1..={MAX_FIELDS_PER_MESSAGE}"
        )));
    }

    let mut occupied = vec![0u128; usize::from(message.word_count)];
    let mut ids = HashMap::with_capacity(message.fields.len());
    for (index, field) in message.fields.iter().enumerate() {
        validate_text("field id", &field.id).map_err(|error| fail(error.0))?;
        validate_text("definition reference", &field.definition_ref)
            .map_err(|error| fail(error.0))?;
        if field.word >= message.word_count {
            return Err(fail(format!(
                "field {} references an absent word",
                field.id
            )));
        }
        if ids.insert(field.id.as_ref(), index).is_some() {
            return Err(fail(format!("duplicate field {}", field.id)));
        }
        let range = BitRange::new(field.lsb, field.width)
            .map_err(|error| fail(format!("{}: {error}", field.id)))?;
        let bits = mask(range.width()) << range.lsb();
        let word = usize::from(field.word);
        if occupied[word] & bits != 0 {
            return Err(fail(format!("overlap at field {}", field.id)));
        }
        occupied[word] |= bits;
        validate_codes(&field.specials, field.width, "special", &field.id)
            .map_err(|error| fail(error.0))?;
        match &field.interpretation {
            Interpretation::ScaledUnsigned {
                denominator, unit, ..
            } => {
                if *denominator == 0 || unit.is_empty() || field.width > 63 {
                    return Err(fail(format!("invalid scaling in {}", field.id)));
                }
            }
            Interpretation::Enumeration(entries) => {
                if entries.is_empty() {
                    return Err(fail(format!("empty enumeration in {}", field.id)));
                }
                validate_codes(entries, field.width, "enumeration", &field.id)
                    .map_err(|error| fail(error.0))?;
                if entries
                    .iter()
                    .any(|entry| find_code(&field.specials, entry.raw).is_some())
                {
                    return Err(fail(format!(
                        "normal/special code conflict in {}",
                        field.id
                    )));
                }
            }
            Interpretation::Unsigned | Interpretation::TwosComplement => {}
        }
    }

    for field in &*message.fields {
        if let Some(condition) = &field.condition {
            let selector = ids
                .get(condition.selector.as_ref())
                .ok_or_else(|| fail(format!("missing selector {}", condition.selector)))?;
            if condition.equals > mask(message.fields[*selector].width) {
                return Err(fail(format!(
                    "selector equality is outside {} width",
                    condition.selector
                )));
            }
        }
    }

    let mut done = vec![false; message.fields.len()];
    let mut order = Vec::with_capacity(message.fields.len());
    while order.len() < message.fields.len() {
        let before = order.len();
        for (index, field) in message.fields.iter().enumerate() {
            if done[index] {
                continue;
            }
            let ready = field.condition.as_ref().is_none_or(|condition| {
                ids.get(condition.selector.as_ref())
                    .is_some_and(|selector| done[*selector])
            });
            if ready {
                done[index] = true;
                order.push(index);
            }
        }
        if order.len() == before {
            return Err(fail("conditional dependency cycle".into()));
        }
    }
    Ok(order)
}

pub fn find_code(entries: &[CodeLabel], raw: u128) -> Option<&CodeLabel> {
    entries
        .binary_search_by_key(&raw, |entry| entry.raw)
        .ok()
        .map(|index| &entries[index])
}

fn validate_codes(
    entries: &[CodeLabel],
    width: u8,
    kind: &str,
    field: &str,
) -> Result<(), SchemaError> {
    let mut previous = None;
    for entry in entries {
        if entry.raw > mask(width)
            || entry.label.is_empty()
            || previous.is_some_and(|value| value >= entry.raw)
        {
            return Err(SchemaError(format!(
                "invalid, duplicate, or unsorted {kind} code in {field}"
            )));
        }
        previous = Some(entry.raw);
    }
    Ok(())
}

fn validate_text(name: &str, value: &str) -> Result<(), SchemaError> {
    if value.trim().is_empty() || value.len() > 4096 {
        return Err(SchemaError(format!("{name} must contain 1..=4096 bytes")));
    }
    Ok(())
}
