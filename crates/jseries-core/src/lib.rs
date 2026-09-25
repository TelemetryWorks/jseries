//! Transport normalization, message assembly, validation, and high-performance decode primitives.
//!
//! This crate contains no restricted standards content. User-supplied schema packages retain
//! explicit source and qualification metadata.
#![forbid(unsafe_code)]

pub mod assembly;
pub mod decode;
pub mod schema;
pub mod transport;
pub mod word;

pub use assembly::{AssembledMessage, AssemblyError, WordFormat};
pub use decode::{
    DecodeContext, DecodeError, DecodedField, DecodedRecord, Decoder, FieldStatus, Value,
};
pub use schema::{
    CodeLabel, Condition, FieldSpec, Interpretation, MessageSpec, PackageMetadata, SchemaError,
    SchemaPackage,
};
pub use transport::{
    InputFormat, NormalizeError, NormalizedWord, normalize_logical70, normalize_simple80,
    normalize_word75,
};
pub use word::{BitRange, INFORMATION_BITS, InformationWord, WordError};
