# Migrating the unreleased decoder APIs

These changes are unreleased and break earlier repository APIs. They do not
add standards definitions or establish standards conformance. The
[public-source-only boundary](../README.md#source-boundary) still applies to
schemas, examples, and any external transport adapter. Examples below use
project-authored synthetic data.

## Rust words and transport

| Removed surface | Replacement or caller responsibility |
| --- | --- |
| `jseries_core::transport`, `InputFormat` | Supply an already unpacked 70-bit information integer. |
| `NormalizedWord` and its information/parity fields | Pass `InformationWord` directly; use `.value()` for its integer. Retain any transport/parity metadata in your adapter. |
| `NormalizeError` | Word construction returns `WordError`; assembly still reports `AssemblyError`. |
| `normalize_logical70(value)` | `normalize_word(value)` or `InformationWord::new(value)`. |
| `normalize_word75(value)`, `normalize_simple80(bytes)` | Unpack and validate your documented transport outside this library before constructing an information word. |

Valid information values range from zero through `2^70 - 1`, carried in a
Rust `u128`; higher set bits are rejected. Do not silently mask or truncate a
packed word to make it pass validation. Packed 75-bit words, SIMPLE 80-bit
bytes, padding and parity are no longer accepted or preserved by the decoder.
Transport interpretation and any parity verification require their own
interface evidence and caller-side diagnostics.

```rust
use jseries_core::{AssembledMessage, DecodeContext, InformationWord, normalize_word};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let word: InformationWord = normalize_word(0x1c94)?;
    let message = AssembledMessage::new(vec![word])?;
    let context = DecodeContext {
        source_id: Arc::from("capture-a"),
        source_offset: 42,
    };
    assert_eq!(message.words()[0].value(), 0x1c94);
    assert_eq!(context.source_id.as_ref(), "capture-a");
    Ok(())
}
```

`DecodeContext.source_id` and decoded source IDs use `Arc<str>` rather than
`String`. Use `Arc::from("capture-a")`, convert an owned string with `.into()`,
and use `Arc::clone` when sharing an ID. `Decoder::decode` still decodes one
assembled message. `Decoder::decode_many(message_id, &messages, source_id,
start_offset)` decodes a homogeneous slice with one plan lookup, preserves
input order, and assigns offsets `start_offset + row_index`. Those offsets are
consecutive caller-defined positions, not inferred byte offsets.

## CLI and schema vectors

Remove `--input-format` entirely, including the former `logical70` value:

```text
jseries decode --schema schemas --message EXAMPLE-70 --word 0x1c94
```

Each `--word` is a canonical decimal or lowercase hexadecimal 70-bit information
integer. The CLI no longer accepts 75-bit/SIMPLE 80-bit input or returns parity
metadata. Unsupported options fail instead of selecting a representation.

Remove `input_format` from each vector TOML case, including cases previously
marked `logical70`; strict parsing rejects that unknown key. Vector `words`
contain canonical integer strings, never packed byte strings. Convert external
capture formats before creating vectors rather than simply relabeling bytes.

```toml
[[cases]]
name = "active"
message = "EXAMPLE-70"
words = ["0x1c94"]
```

## Python single-message and batch shape

Replace `decode_logical70(message_id, words, ...)` with
`decode(message_id, words, ...)`. Replace
`decode_many_logical70(message_id, rows, start_offset=N, ...)` with
`decode(message_id, rows, source_offset=N, ...)`.

```python
import jseries

decoder = jseries.Decoder("schemas")
record = decoder.decode("EXAMPLE-70", [0x1C94], source_offset=42)
records = decoder.decode("EXAMPLE-70", [[0x1C94], [0x1494]], source_offset=42)
```

A sequence of integer words returns one `DecodedRecord`; a sequence of rows
returns a list of records in input order. A batch reuses one compiled message
plan and assigns consecutive offsets. Empty top-level input is ambiguous and
rejected; it does not mean an empty successful batch. Transport bytes are not
information-word integers and must be unpacked outside this API.

`record.fields` constructs a fresh list and fresh field wrappers on every
access. Store the returned list if accessing it repeatedly. Fields are read-only;
there is no cached field collection or new caching API.

## Error handling and batch atomicity

Python word values outside `0..2^70 - 1` raise `ValueError`, including negative
integers and integers at or above `2^128`. Noninteger or mixed input shapes
raise `TypeError`. `bytes`, `bytearray`, and `memoryview` are explicitly rejected
at the top level and as batch rows; this API does not interpret their packing.
Bad word assembly also raises `ValueError`; native decoding failures retain
`jseries.DecodeError` (a `JSeriesError`).

Both Rust and Python batch decoding are all-or-nothing: an exception/error
returns no partial record collection. Row-specific failures include the
zero-based input row, including later-row word-count mismatches and source-offset
overflow. Global unknown-message and invalid shared-source errors remain global.
A field's diagnostic status is still part of a returned record; it is not an
exception and does not by itself abort a batch.

Rust `DecodeError` adds `BatchRow { index: usize, error: Box<DecodeError> }`.
Update exhaustive matches to handle this arm. Its `Display` is only
`batch row {index}`; inspect `error` or `std::error::Error::source()` for the
original word-count/offset/decode cause. The decoder wraps row errors once,
never nests row wrappers, and leaves single-message errors unchanged. Python
renders both the row and the underlying cause in its existing exception message.
No Python exception attributes or new exception hierarchy are introduced.

The wrapper allocates only on failure. A successful batch retains one message
plan lookup, owned input/output buffers, and interpreter release during native
assembly and decoding; no per-row call to the single-message decoder is added.
