# Architecture

```text
user TOML package                      input representation
package + messages + catalogs          logical70 | word75 | simple80
          |                                      |
          v                                      v
jseries-schema (cold path)              explicit normalization
strict parse, bounds, references                  |
          |                                      v
          +----> immutable SchemaPackage   bounded word assembly
                           |                       |
                           v                       v
                    Decoder construction ----> decode hot path
                    ranges/selectors/index     extraction + interpretation
                                                   |
                                                   v
                                             DecodedRecord
```

`jseries-core` has no filesystem, network, CLI, Python, Serde, or TOML dependency. `jseries-schema` performs bounded reads, rejects unknown keys and unsafe relative paths, resolves catalogs, sorts code tables, and constructs the validated owned model. The CLI and PyO3 extension are adapters, not part of decoding semantics. The Python decoder loads a package once, retains the immutable Rust plan, offers homogeneous batch calls, and releases the interpreter during native work. Calls must remain coarse-grained so the binding does not compromise the Rust hot path.

## Representation boundary

The internal unit is exactly 70 information bits in a `u128`. A `word75` carries those bits plus five preserved parity bits. The implemented `simple80` adapter consumes ten bytes in capture order, preserves the same five parity bits, and requires its five padding bits to be zero. These named representations are not interchangeable byte layouts.

Assembly requires an initial word followed by continuation or extension words and enforces a 32-word bound. It does not infer undocumented sequence rules.

## Hot-path rules

- Never parse TOML, read files, resolve names, or allocate lookup tables while decoding.
- Validate once and use immutable shared schema data.
- Use a single message hash lookup, indexed fields/selectors, precomputed bit ranges, and binary search over sorted codes.
- Preserve integer/rational values; do not force floating point into field interpretation.
- Benchmark extraction separately from ingestion and output. Regressions require evidence, not intuition.

Decoded output currently allocates result vectors and strings are shared through `Arc<str>`. Future throughput work should add caller-owned output buffers, batch APIs, and representative benchmarks without weakening validation.

Capture readers, full transport framing, schema signatures, stateful track reconstruction, and stable JSON/Arrow/Parquet outputs are future adapters. They must preserve source identity and remain outside the field engine.
