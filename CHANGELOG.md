# Changelog

This project has not published a release yet.

## Unreleased

### Breaking changes

- Accept only 70-bit information-word integers. Remove the Rust `transport`
  module, `InputFormat`, `NormalizedWord`, `NormalizeError`, and the
  `normalize_logical70`, `normalize_word75`, and `normalize_simple80` helpers.
  Use `InformationWord` and `normalize_word` (or `InformationWord::new`);
  transport unpacking and parity handling belong in caller-side adapters.
- Remove CLI `--input-format`, 75-bit and SIMPLE 80-bit input, and parity
  preservation. Schema vectors reject the removed `input_format` key.
- Use `Arc<str>` for Rust decode source IDs. Unify Python single-message and
  batch methods as `Decoder.decode(message_id, values, *, source_id=None,
  source_offset=0)`, choosing the result shape from the input shape.

### Error handling

- Python word integers outside `0..2^70 - 1`, including negative values and
  integers at or above `2^128`, raise `ValueError`. Invalid input shapes and
  bytes/bytearray/memoryview containers raise `TypeError`.
- Row-specific Rust batch failures use `DecodeError::BatchRow` with a zero-based
  index and the underlying error as `Error::source()`. Exhaustive enum matches
  need a new arm. Python batch exceptions include both row and cause, preserving
  their existing exception classes. Batch calls return no partial records;
  invalid shared source IDs and unknown messages remain global failures.

### Added

- Rust `Decoder::decode_many` resolves one message plan per homogeneous batch,
  preserves row order, and assigns consecutive source offsets. The Python
  binding owns converted inputs and releases the interpreter for native work.
- Locked Python quality tooling and install-tested Windows/Linux abi3 wheels.

See [migration guidance](docs/MIGRATION.md) for replacements and examples.
These changes contain no MIL-STD-6016 definitions or standards-conformance claim;
user schemas remain subject to the repository's public-source-only boundary.
