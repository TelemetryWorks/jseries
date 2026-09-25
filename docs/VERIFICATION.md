# Verification and acceptance

## Current local result

On 2026-09-24 the expanded local gate passed on Windows with `rustc 1.98.0` and `cargo 1.98.0`: TOML and runtime-schema validation, requirement traceability, rustfmt, Clippy with warnings denied, all eleven Rust tests in debug and release profiles, and compilation of every benchmark target. Direct compiler configuration checks also reported `panic="unwind"` for development and `panic="abort"` for release. This is local development evidence; the new branch still needs GitHub-hosted MSRV and cross-platform results.

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
secrets, and matching local commands. These newly supplied workflows are not
accepted as passing evidence until they run successfully on GitHub.
The LCOV command also completed locally with cargo-llvm-cov 0.6.21; CI pins
0.8.7, whose hosted result remains pending.

The Python workflow builds and install-tests CPython stable-ABI wheels on
Windows x86-64 and manylinux x86-64 and tests locally built packages with
Python 3.10 and 3.14. [GitHub Actions run 36088008490](https://github.com/TelemetryWorks/jseries/actions/runs/36088008490)
passed all six jobs on commit `c08bdf7` and retained both wheel artifacts.

A local Windows CPython 3.12 test built and installed
`jseries-0.1.0-cp310-abi3-win_amd64.whl`; the installed native module reported
`0.1.0`, matching its distribution metadata and the Cargo workspace version.

Acceptance layers include core correctness, bounded package safety, exact run evidence, GitHub-hosted platform results, measured performance, and fact-level semantic qualification. Encoder/decoder round trips and agreement with another decoder are useful comparison evidence but can share defects and do not establish standards conformance.

The evidence directory is being redesigned according to Priority 0 in the roadmap. Until that work lands, current truth comes from a fresh local command or an exact GitHub Actions run, not a checked-in historical “current status” file.
