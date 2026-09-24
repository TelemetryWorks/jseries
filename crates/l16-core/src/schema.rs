use crate::{baseline::Baseline, word::{mask, BitRange}};
use std::{collections::{HashMap, HashSet}, fmt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchemaKind { Synthetic }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunMode { SyntheticTest, StandardsDecode }
#[derive(Clone, Copy, Debug)]
pub struct Condition { pub selector: &'static str, pub equals: u128 }
#[derive(Clone, Copy, Debug)]
pub enum Interpretation {
    Unsigned,
    TwosComplement,
    ScaledUnsigned { numerator: i64, denominator: u64, unit: &'static str },
    Enumeration(&'static [(u128, &'static str)]),
}
#[derive(Clone, Copy, Debug)]
pub struct FieldSpec {
    pub id: &'static str,
    /// Synthetic references use a SYNTH: prefix, never pretend to be DFI/DUI.
    pub definition_ref: &'static str,
    pub lsb: u8,
    pub width: u8,
    pub interpretation: Interpretation,
    pub specials: &'static [(u128, &'static str)],
    pub condition: Option<Condition>,
}
#[derive(Clone, Copy, Debug)]
pub struct MessageSpec {
    pub id: &'static str,
    pub word_bits: u8,
    pub fields: &'static [FieldSpec],
}
#[derive(Clone, Copy, Debug)]
pub struct Bundle {
    pub baseline: Baseline,
    pub schema_id: &'static str,
    pub schema_version: &'static str,
    pub profile_id: &'static str,
    pub digest_sha256: &'static str,
    pub kind: SchemaKind,
    pub messages: &'static [MessageSpec],
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaError(pub String);
impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
}
impl std::error::Error for SchemaError {}

/// Exact lookup only. No nearest-revision, newest-revision, or parent fallback.
pub fn bundled_schema(baseline: Baseline) -> &'static Bundle {
    match baseline {
        Baseline::D => &crate::generated::BUNDLE_D,
        Baseline::E => &crate::generated::BUNDLE_E,
        Baseline::F => &crate::generated::BUNDLE_F,
        Baseline::FChange1 => &crate::generated::BUNDLE_F_C1,
        Baseline::G => &crate::generated::BUNDLE_G,
        Baseline::H => &crate::generated::BUNDLE_H,
    }
}

/// Check a message once and return an interpretation dependency order.
/// This increment rejects all overlap; conditional overlays are not modeled.
pub fn validate_message(message: &MessageSpec) -> Result<Vec<usize>, SchemaError> {
    let fail = |s: String| SchemaError(format!("{}: {}", message.id, s));
    if message.id.is_empty() || message.word_bits == 0 || message.word_bits > 128 {
        return Err(fail("invalid message identifier or word length".into()));
    }
    if message.fields.is_empty() || message.fields.len() > 128 {
        return Err(fail("field count must be 1..=128".into()));
    }
    let mut occupied = 0u128;
    let mut ids = HashMap::new();
    for (index, field) in message.fields.iter().enumerate() {
        if field.id.is_empty() || field.definition_ref.is_empty() {
            return Err(fail("empty field or definition identifier".into()));
        }
        if ids.insert(field.id, index).is_some() {
            return Err(fail(format!("duplicate field {}", field.id)));
        }
        let range = BitRange::new(field.lsb, field.width, message.word_bits)
            .map_err(|e| fail(format!("{}: {}", field.id, e)))?;
        let bits = mask(range.width()) << range.lsb();
        if occupied & bits != 0 { return Err(fail(format!("overlap at {}", field.id))); }
        occupied |= bits;
        let mut codes = HashSet::new();
        for &(raw, label) in field.specials {
            if raw > mask(field.width) || label.is_empty() || !codes.insert(raw) {
                return Err(fail(format!("invalid/duplicate special in {}", field.id)));
            }
        }
        match field.interpretation {
            Interpretation::ScaledUnsigned { denominator, unit, .. } => {
                if denominator == 0 || unit.is_empty() || field.width > 63 {
                    return Err(fail(format!("invalid scaling in {}", field.id)));
                }
            }
            Interpretation::Enumeration(items) => {
                if items.is_empty() { return Err(fail("empty enumeration".into())); }
                for &(raw, label) in items {
                    if raw > mask(field.width) || label.is_empty() || !codes.insert(raw) {
                        return Err(fail(format!("invalid/conflicting enumeration in {}", field.id)));
                    }
                }
            }
            _ => {}
        }
    }
    for field in message.fields {
        if let Some(condition) = field.condition {
            let selector = ids.get(condition.selector)
                .ok_or_else(|| fail(format!("missing selector {}", condition.selector)))?;
            let source = &message.fields[*selector];
            if condition.equals > mask(source.width) {
                return Err(fail("selector equality is outside its bit width".into()));
            }
        }
    }
    // A bounded topological pass; do not recurse on untrusted dependency graphs.
    let mut done = vec![false; message.fields.len()];
    let mut order = Vec::with_capacity(message.fields.len());
    while order.len() < message.fields.len() {
        let before = order.len();
        for (index, field) in message.fields.iter().enumerate() {
            if done[index] { continue; }
            let ready = match field.condition {
                None => true,
                Some(c) => ids.get(c.selector).map(|i| done[*i]).unwrap_or(false),
            };
            if ready { done[index] = true; order.push(index); }
        }
        if order.len() == before { return Err(fail("conditional dependency cycle".into())); }
    }
    Ok(order)
}
