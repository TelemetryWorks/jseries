# Verification and acceptance

## Current local result

On 2026-09-24 the expanded local gate passed on Windows with `rustc 1.98.0` and `cargo 1.98.0`: TOML and runtime-schema validation, requirement traceability, rustfmt, Clippy with warnings denied, all Rust tests in debug and release profiles, and compilation of every benchmark target. Direct compiler configuration checks also reported `panic="unwind"` for development and `panic="abort"` for release.

Run the complete local gate with:

```text
python tools/check.py
```

That command checks TOML formatting and validity with Taplo, validates every
TOML document and the runnable schema package, checks requirement/test
traceability and rustfmt, denies Clippy warnings, runs locked debug and release
workspace tests, and compiles benchmark targets. It fails if a required tool
is absent. Once dependencies are cached,
`CARGO_NET_OFFLINE=true cargo test --workspace --locked` provides an offline
execution option; offline mode cannot install prerequisites.

GitHub Actions additionally runs Taplo formatting and validation, CodeQL Rust
analysis, and SonarCloud analysis backed by an LCOV report from
`cargo-llvm-cov`. See [CI.md](CI.md) for the workflow boundaries, required
secrets, and matching local commands. PR #2 passed the hosted Windows/Linux,
MSRV, CodeQL, LCOV, and SonarCloud quality-gate checks before merge. CI pins
cargo-llvm-cov 0.8.7; a local coverage run also passed with 0.6.21.

The Python workflow builds and install-tests CPython stable-ABI wheels on
Windows x86-64 and manylinux x86-64 and tests locally built packages with
Python 3.10 and 3.14. [GitHub Actions run 36088008490](https://github.com/TelemetryWorks/jseries/actions/runs/36088008490)
passed all six jobs on commit `c08bdf7` and retained both wheel artifacts.

A local Windows CPython 3.12 test built and installed
`jseries-0.1.0-cp310-abi3-win_amd64.whl`; the installed native module reported
`0.1.0`, matching its distribution metadata and the Cargo workspace version.

On 2026-09-24, the release-mode Rust benchmark decoded a synthetic 1,024-row,
single-word, two-field batch at a median 2.29 million messages/second on the
local Windows development machine. The installed-wheel benchmark decoded
10,000 rows using the four-field repository example at a median 676 thousand
messages/second in one batch, versus 437 thousand messages/second using
individual Python calls. These are initial local baselines, not portable
performance guarantees; the schemas differ and the Python measurement includes
input conversion and result-object construction.

Acceptance layers include core correctness, bounded package safety, exact run evidence, GitHub-hosted platform results, measured performance, and fact-level semantic qualification. Encoder/decoder round trips and agreement with another decoder are useful comparison evidence but can share defects and do not establish standards conformance.

The evidence directory is being redesigned according to Priority 0 in the roadmap. Until that work lands, current truth comes from a fresh local command or an exact GitHub Actions run, not a checked-in historical “current status” file.
