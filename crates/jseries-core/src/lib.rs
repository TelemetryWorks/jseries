//! 70-bit word validation, message assembly, and high-performance decode primitives.
//!
//! This crate contains no restricted standards content. User-supplied schema packages retain
//! explicit source and qualification metadata.
#![forbid(unsafe_code)]

/// Package version inherited from the workspace's central Cargo manifest.
pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod assembly;
pub mod decode;
pub mod schema;
pub mod word;

pub use assembly::{AssembledMessage, AssemblyError, WordFormat};
pub use decode::{
    DecodeContext, DecodeError, DecodedField, DecodedRecord, Decoder, FieldStatus, Value,
};
pub use schema::{
    CodeLabel, Condition, FieldSpec, Interpretation, MessageSpec, PackageMetadata, SchemaError,
    SchemaPackage,
};
pub use word::{BitRange, INFORMATION_BITS, InformationWord, WordError, normalize_word};
