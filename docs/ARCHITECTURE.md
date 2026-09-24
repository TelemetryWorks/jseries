# Architecture

## Implemented boundary

```text
Six synthetic JSON sources
          |
          v
Python schema compiler / validator -- source SHA-256
          |
          v
Generated immutable Rust bundles
          |
          v
Explicit baseline + synthetic mode + evidence
          |
          v
Checked logical Word -> validated bit ranges -> raw field extraction
          |
          v
Dependency-ordered applicability and interpretation
          |
          v
DecodedRecord with raw bits, value/status, and provenance -> text CLI
```

The block above describes implementation intent reflected in supplied code.
Python checks ran; Rust runtime behavior still requires compilation and tests.
The input is an integer supplied by the operator. There is no capture reader,
transport parser, real word normalizer, or J-series structural parser in this
increment.

`l16-core` has no filesystem, database, network, or CLI dependency. It binds a
static bundle to an immutable decoder, validates all message fields once, builds
bit ranges and selector indices, and reuses those structures for each record.
This first implementation allocates per-record result vectors; zero-allocation
or throughput claims are not made. A bounded message count of 128 and field
count of 128 are toy-framework limits, not standard limits or final sizing.

`l16-cli` parses explicit operator arguments and provides `baselines`, `demo`,
and a deliberately blocked `decode` command. It prints bounded input-derived
identifiers using escaped/debug formatting where appropriate. No record payload
is submitted to external services. No bulk file reading is implemented.

The Python compiler is an authoring/build utility. It is not in the runtime hot
path. It rejects unknown keys, duplicate JSON keys, invalid field ranges,
overlaps, conflicting codes, invalid scale rules, missing selectors, and cycles.
It accepts synthetic sources only and emits source digests with generated code.
The core performs its own structural checks when instantiated as a second
validation boundary, not as a substitute for source qualification.

## Target capture architecture — not implemented

```text
Recording reader
  -> interface/transport adapter
  -> explicit packing normalizer
  -> baseline/profile-aware message assembler
  -> field and dictionary interpretation
  -> decoded event output
```

Every arrow should carry provenance. The recording reader owns source file
identity, byte offsets, and capture timestamps. The adapter owns transport
headers and source/session identity. The normalizer owns packing, padding,
byte order, and conversion between source bit numbering and the internal
least-significant-bit convention. The assembler owns word association and
structural completeness. The field engine owns extraction and interpretation.
Output adapters own schema evolution for JSON, Arrow/Parquet, and downstream
systems. None should cause an opaque change to the selected baseline.

A future normalized word should retain both the logical value and references
to original bytes and framing. The current `Word` is only the value component.
A `u128` is chosen as an internal checked bit container; its capacity is not a
claim about an external transport or a guessed byte stride.

Assembly state must be keyed by the correct source/channel/session identity,
not merely by message label or participant identifier. The correct key and
assembly rules are to be determined from the applicable interface and standard.
Set explicit bounds on words per message, active assemblies, elapsed time,
source count, and queued bytes. Do not borrow unrelated following words to make
an incomplete message appear complete. Those safeguards remain unimplemented.

## Dictionary and revision model

The target schema layer separates layout occurrences, definition references,
DFI/DUI dictionary entries, interpretation dependencies, and validation rules.
The complete reference key includes the resolved baseline/profile snapshot.
Normalization into application-wide concepts is downstream and must preserve
source definition identity; it must not erase real revision differences.

The toy format embeds a few interpretation rules directly in layouts. This is
an intentionally limited proof of isolation and interpretation order, not the
final authoritative schema representation.

## Track state, storage, and deployment

Track reconstruction is a separate consumer of decoded events. It may carry
forward prior values or align events in time, but must mark those transformations
rather than modify what the decoder says was present in an individual message.

PostgreSQL can later hold job/source/schema metadata; Parquet or ClickHouse can
receive decoded observations. None is required for decoding. Keep bulk capture
and decoded-data movement in reader/worker/output processes rather than passing
it through a control-plane service. A local library plus CLI is the first unit
of deployment; additional services require demonstrated operational need.

Only after representative real captures exist should performance tuning target
batching, allocation reuse, dictionary lookup, parallelism, and output I/O.
Benchmark extraction separately from capture reading and storage writes.
