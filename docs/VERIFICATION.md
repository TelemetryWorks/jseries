# Verification and acceptance

## Current local result

On 2026-09-24 the complete local gate passed on Windows with `rustc 1.98.0` and `cargo 1.98.0`: requirement traceability, rustfmt check, Clippy with warnings denied, and all seven Rust tests in both debug and release profiles. This is local development evidence, not GitHub-hosted or cross-platform qualification.

Run the complete local gate with:

```text
python tools/check.py
```

That command validates requirement/test traceability, checks rustfmt, denies Clippy warnings, and runs locked debug and release workspace tests. It fails if Rust is absent. Once dependencies are cached, `CARGO_NET_OFFLINE=true cargo test --workspace --locked` provides an offline execution option; offline mode cannot install prerequisites.

Acceptance layers include core correctness, bounded package safety, exact run evidence, GitHub-hosted platform results, measured performance, and fact-level semantic qualification. Encoder/decoder round trips and agreement with another decoder are useful comparison evidence but can share defects and do not establish standards conformance.

The evidence directory is being redesigned according to Priority 0 in the roadmap. Until that work lands, current truth comes from a fresh local command or an exact GitHub Actions run, not a checked-in historical “current status” file.
