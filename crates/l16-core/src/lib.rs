//! Revision-isolated decoder foundation; bundled formats are SYNTHETIC ONLY.
//!
//! This crate contains no authoritative MIL-STD-6016 message definitions and
//! makes no conformance claim. It accepts a normalized integer, not wire bytes.
#![forbid(unsafe_code)]

pub mod baseline;
pub mod decode;
pub mod schema;
pub mod word;
#[rustfmt::skip]
mod generated;

pub use baseline::Baseline;
pub use decode::{DecodeContext, DecodeError, DecodedField, DecodedRecord, Decoder,
    FieldStatus, SelectionEvidence, Value};
pub use schema::{bundled_schema, Bundle, RunMode, SchemaError, SchemaKind};
pub use word::{BitRange, Word, WordError};
