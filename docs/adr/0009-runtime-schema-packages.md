# ADR-0009: TOML runtime schema packages

Status: Accepted

## Context

Users need to add layouts, DFI/DUI catalogs, and vectors without changing or rebuilding the decoder. JSON is concise for machines but awkward for maintaining reviewed field tables, while XML is unnecessarily verbose. Values may exceed TOML's signed 64-bit integer range.

## Decision

Use strict, versioned TOML packages loaded by the separate `jseries-schema` crate. Quote all raw patterns and codes as canonical decimal or lowercase hexadecimal strings. Split packages into a manifest, message files, catalog files, and vector files.

Load and validate a package once. Resolve references and construct immutable owned `jseries-core` data before creating a decoder. The hot path must not perform filesystem access, TOML parsing, or symbolic reference resolution.

Package metadata includes source and qualification statements but is not proof of authority. Standards content is not bundled, and public reconstruction is labeled as such.

## Consequences

Packages are reviewable and independently distributable. Strict parsing and bounded I/O add startup work but no per-field parse cost. Quoted bit patterns require explicit conversion yet safely cover 70-bit words. Schema format evolution requires versioning and migration policy.
