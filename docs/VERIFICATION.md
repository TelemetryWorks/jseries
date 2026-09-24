# Verification and acceptance

## What has actually run

On the authoring container, `python tools/check.py` completed with 27 passing
Python unittest methods. Those checks validate all six schemas, generated-code
freshness, literal toy vectors, source hashes, source-register honesty, unknown
coverage denominators, and malformed-schema/interpretation cases. The log is
`evidence/python-checks.txt`.

Cargo and rustc were unavailable, and a compiler could not be retrieved in this
environment. Therefore Rust compilation, the included Rust tests, rustfmt,
clippy, performance measurements, Windows execution, and RHEL execution were
not performed. This is a real acceptance gap, not a passing or skipped Rust CI
result. Do not treat Python success as Rust execution evidence.

The workspace includes 38 Rust core integration-test functions and 9 CLI
integration-test functions. Some exercise multiple baselines, bit positions, or
boundary cases. Test counts are not message coverage or a security guarantee.
Before accepting this increment, run the entire suite on the provisioned runners
and amend the evidence report with actual results.

## Offline commands

From repository root, with an already installed compiler, Cargo, standard
library, platform linker, and Python:

```text
python tools/bundle_manifest.py --check
python tools/check.py --rust
```

The second command validates schemas and Python tests, records Rust/Cargo
versions to stdout, and executes:

```text
cargo test --workspace --locked --offline
cargo test --workspace --locked --offline --release
```

`--locked` refuses dependency-resolution changes; `--offline` disables Cargo
network access. `--frozen` combines both [S2]. Offline flags do not install
prerequisites or pin the compiler. This increment has no third-party crate
dependencies, but the toolchain and native build environment still matter.

To format handwritten Rust after installing rustfmt, run `cargo fmt --all`.
Generated Rust is marked to be skipped by rustfmt; the generator owns its format.
A formatting check and lints should become enforced gates once the baseline has
been compiled, formatted, and reviewed on the project's selected toolchain.
No clean formatting/lint result is claimed here.

## Test categories

| Area | Acceptance behavior |
|---|---|
| Identity | Exact six baseline identities; F never equals F-C1. |
| Qualification guard | Standards mode cannot use a synthetic bundle. |
| Isolation | No revision fallback and no global mutable selection. |
| Bounds | Reject high bits, empty ranges, out-of-bounds fields, and wrong lengths. |
| Arithmetic | Test full-width extraction, sign boundaries, and rational values. |
| Semantics | Specials precede conversion; undefined enum codes remain invalid. |
| Context | False predicates differ from invalid or special selectors. |
| Schema | Reject duplicate IDs/codes, conflicting values, cycles, and invalid scale. |
| Provenance | Preserve logical input, raw fields, source offset, selection evidence, and schema ID. |
| CLI | Reject missing/duplicate/unknown arguments and mark every demo result synthetic. |

## Qualification of a real message

Real-message acceptance requires authoritative source locators, reviewed
transcription, a declared scope of layouts/word forms/rules, independently
established expected outputs, ordinary and exceptional-value cases, applicable
conditional combinations, malformed/truncated input, and provenance back to
source bytes. The evidence must identify the exact baseline/profile snapshot.

An encoder/decoder round trip can reproduce a shared bug. It is useful but not
sufficient. Agreement with another implementation is also not a substitute for
understanding which baseline/profile and edge cases that implementation covers.

Broader production acceptance additionally requires fuzzing, sanitizing
inappropriate payload logs, resource-cap enforcement in actual assembly paths,
performance characterization with representative captures, deterministic replay,
actual cross-platform tests, supply-chain checks, and documented limitations.
These are planned gates, not completed tests in this increment.

## CI evidence and traceability

`.gitlab-ci.yml` uses two provisioned shell runners, with locally configured
tags. `tools/ci_check.py` creates fresh environment and verification logs under
`ci-evidence/`. It requires Rust and returns nonzero if any stage fails. Configure
artifact handling appropriately before adding controlled test data.

Requirement links are in `docs/requirements/requirements.json`. Status values
separate supplied implementation, supplied-but-unexecuted Rust tests, passing
Python evidence, and future work. No target-platform support claim should be
promoted on the basis of generic source portability alone.
