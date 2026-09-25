# CLAUDE.md

This file provides guidance to coding agents working in this repository. It is
tool-neutral: keep project knowledge here rather than maintaining separate,
divergent instructions for individual assistants.

## Project overview

`jseries` is a schema-driven J-series decoding foundation with a Rust core.
Its sole supported input representation is an integer containing exactly the
70 information bits. It accepts runtime TOML packages, contains no MIL-STD-6016
message definitions, and is not a standards-conformance implementation.

TelemetryWorks cannot lawfully obtain the controlled MIL-STD-6016 revisions
through an authorized channel. The project is permanently bounded to lawfully
available, unrestricted public Internet sources unless the owner explicitly
changes that constraint. Read the **Public-source-only boundary** in `README.md`
before making product, schema, or capability decisions. Never convert a public
reconstruction or agreement between implementations into a standards-
conformance claim.

Preserve explicit source and qualification metadata. Never infer authority
from a package path or promote public reconstruction to standards content.

## Sources of truth

Use this order when repository information conflicts:

1. The user's current request and explicitly recorded project decisions.
2. Executable behavior and tests.
3. Project configuration and automation.
4. Maintainer documentation.
5. Comments and historical notes.

Raise consequential contradictions instead of silently choosing one. Do not
change product behavior, public interfaces, data formats, compatibility
targets, security posture, or dependency policy merely to make a check pass.

`CONVERSATION.md` records the historical discussion that led to the starter.
It contains hypotheses and earlier recommendations, not settled requirements.
Current explicit decisions and maintained repository documents supersede it.

## Before making changes

- Read this file and any more specific `AGENTS.md` or `CLAUDE.md` in the area
  being changed.
- Inspect the repository status and preserve unrelated user work.
- Read the relevant implementation, tests, configuration, and documentation;
  do not reason from a diff alone.
- Identify the smallest coherent change that satisfies the request.
- Ask before making a choice that establishes a hard-to-reverse public API,
  data model, platform commitment, or external service dependency when the
  requirement does not already determine it.

## Development commands

Use an already provisioned Python 3.10+ and Rust toolchain. The project must not
download or update toolchains implicitly.

```text
python tools/check.py                    # complete local gate
cargo fmt --all                          # format Rust
cargo test --workspace --locked          # focused Rust verification
taplo fmt --check                        # check TOML formatting
taplo check                              # validate TOML syntax and structure
cargo llvm-cov --workspace --all-features --exclude jseries-python --locked --lcov --output-path lcov.info
python -m maturin develop --manifest-path python/Cargo.toml --locked
python -m unittest discover --start-directory python/tests --verbose
python python/benchmarks/decode.py --rows 10000 --iterations 7 --include-single
```

`tools/check.py` checks TOML with Taplo, validates traceability, checks rustfmt,
denies Clippy warnings, runs locked debug and release Rust tests, and compiles
benchmark targets. It fails if a required tool is absent.

GitHub-hosted analysis also includes CodeQL and SonarCloud. See `docs/CI.md`
for the workflow boundaries and required repository secrets.

## Architecture and repository layout

The current proposal separates the decoder core, CLI, synthetic schemas,
verification fixtures, and engineering documentation:

- `crates/jseries-core/` — normalization, assembly, schema model, and decode hot path.
- `crates/jseries-schema/` — strict TOML loading and cold-path resolution.
- `crates/jseries-cli/` — schema validation, inspection, and decoding CLI.
- `python/` — thin PyO3 adapter, Python facade, package tests, and maturin metadata.
- `schemas/` — runnable project-authored TOML starter package, schema guidance,
  and public source/coverage metadata; it contains no authoritative message
  definitions.
- `docs/` — architecture, contracts, requirements, decisions, and roadmap.
- `tools/` — repository verification and traceability checks.

TOML/filesystem work belongs outside `jseries-core`. Parsing, reference
resolution, and lookup-table construction must never move into the decode hot
path. Python bindings must expose coarse-grained Rust operations rather than
crossing the FFI boundary per field. Batch APIs resolve a homogeneous message
plan once and release the Python interpreter during native work. Prefer clear
boundaries and straightforward code over abstractions for hypothetical
requirements.

## Implementation standards

- Match conventions already established by the repository. If none exist,
  favor idiomatic, readable code for the selected language.
- Keep changes focused. Do not combine feature work with unrelated cleanup.
- Make invalid states and error paths explicit. Preserve useful diagnostic
  context without exposing secrets or sensitive data.
- Treat persisted data, serialized output, command-line behavior, and public
  APIs as compatibility surfaces once released or consumed.
- Prefer deterministic behavior in builds, tests, generated artifacts, and
  output intended for comparison or automation.
- Add dependencies deliberately. Record why a non-obvious dependency is
  needed, keep its scope narrow, and commit the ecosystem's lockfile when one
  is used.
- Never commit credentials, tokens, private keys, production data, or local
  environment files. Use documented examples with inert placeholder values.
- Do not suppress lint, type, security, or test findings without a narrow,
  documented reason.

## Testing and verification

Every behavior change should have proportionate automated coverage when the
project has a test framework. A fix should normally include a regression test
that fails for the original defect. Test observable behavior and boundary
conditions rather than implementation details.

Before handing off work:

1. Run the narrowest relevant checks while iterating.
2. Run the repository's full verification command when one exists.
3. Inspect the final diff and repository status for accidental files, secrets,
   generated output, or unrelated edits.
4. Report exactly what was verified and any check that could not be run.

Do not weaken assertions, refresh expected output, reduce coverage, or disable
a check merely to obtain a green result. If expected behavior intentionally
changes, make the resulting test or fixture diff part of the reviewable
change.

## Documentation and decisions

Documentation is part of the change when setup, behavior, an interface, or an
operator workflow changes. Use a lightweight decision record for choices with
long-lived tradeoffs, especially architecture, storage, interoperability,
security, and dependency decisions.

Keep this file concrete. Remove bootstrap language as facts become known, and
record the commands and invariants most likely to surprise a future
contributor. Avoid turning it into a generic style guide that merely repeats
formatter or linter configuration.

## Git conventions

Do **not** add `Co-Authored-By: Claude ...` trailers to commit messages on this
repo, even if the harness's default instructions suggest it. Commit messages
are the human-authored record of intent; tool attribution belongs in tool logs,
not history. This overrides the default trailer behavior.

- Do not discard, overwrite, stage, or commit unrelated user changes.
- Do not rewrite history or use destructive Git commands unless explicitly
  requested.
- Keep commits cohesive and use imperative commit subjects that explain the
  change.
- Do not create a commit unless the user asks for one.
