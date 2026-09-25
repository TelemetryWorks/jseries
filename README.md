# jseries

`jseries` is a high-performance Rust foundation for decoding normalized J-series information words with user-supplied TOML schema packages. It currently implements strict package loading, 70-bit information-word normalization, named 75-bit and SIMPLE 80-bit representations, bounded message assembly, and schema-driven field interpretation.

It does **not** contain MIL-STD-6016 message definitions and does not claim standards conformance.

## Source boundary

TelemetryWorks cannot lawfully obtain the applicable MIL-STD-6016 revisions through a contract, government customer, controlling office, authorized document service, or other authorized access. The project may use only material lawfully available to the unrestricted public on the Internet. Leaked, reposted, access-controlled, or contract-furnished material is out of scope.

Public catalog metadata can identify a revision but does not supply the normative layouts, DFI/DUI catalogs, processing rules, or revision deltas needed for a complete implementation. Public reconstructions must retain their sources, uncertainty, conflicts, and qualification; they must never be presented as authoritative standards content.

## Workspace

- `jseries-core`: transport normalization, assembly, validated schema model, and decode hot path; no TOML or filesystem dependency.
- `jseries-schema`: bounded, strict TOML package loading and cold-path reference resolution.
- `jseries-cli`: package validation, inspection, and explicit decoding commands.
- `schemas/`: runnable project-authored starter package showing how users provide message layouts and catalogs.

Schema TOML is parsed once. The decoder holds immutable validated definitions, a single message index, precomputed bit ranges, resolved conditional selectors, and sorted code tables. File I/O, TOML parsing, and catalog-name resolution do not occur in the decode hot path.

## Try it

```text
cargo run --locked -p jseries-cli -- --version
cargo run --locked -p jseries-cli -- schema validate schemas
cargo run --locked -p jseries-cli -- schema inspect schemas
cargo run --locked -p jseries-cli -- decode --schema schemas --message EXAMPLE-70 --input-format logical70 --word 0x1c94
```

The example is invented test data, not a real Link 16 message. See [schemas/README.md](schemas/README.md) for the package layout and [docs/SCHEMA-CONTRACT.md](docs/SCHEMA-CONTRACT.md) for the contract.

## Development

Rust edition 2024 is used with a minimum supported Rust version of 1.86. The current workspace was compiled and tested locally with Rust 1.98.0 on Windows. Run:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --locked
cargo test --workspace --locked --release
cargo bench --workspace --no-run --locked
taplo fmt --check
taplo check
cargo llvm-cov --workspace --all-features --locked --lcov --output-path lcov.info
python tools/check.py
```

GitHub Actions checks Taplo and Rust formatting, Clippy, traceability, debug and release tests, Windows/Linux behavior, the declared MSRV, CodeQL findings, LCOV coverage, and the SonarCloud quality gate. See [docs/CI.md](docs/CI.md) for workflow and secret configuration.

## License

Licensed under the [Apache License 2.0](LICENSE).
