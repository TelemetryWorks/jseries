# J-series public-source decoder research foundation

**Increment 0.1 — D, E, F, F Change 1, G, and H identities; synthetic decoding only.**

This repository is the initial engineering package for a future J-series decoder
with a Rust core and planned Python bindings. It does not include authoritative
MIL-STD-6016 layouts, DFI/DUI dictionaries, real message assembly, capture
packing rules, or a claim of standards conformance. All included field
definitions and all differences between the six demo profiles are deliberately
invented software-test data. They are NOT descriptions of actual changes to
MIL-STD-6016.

## Public-source-only boundary

TelemetryWorks does not have lawful access to the controlled MIL-STD-6016
revisions through a contract, government customer, the controlling office, or
an authorized document service. This project therefore uses only material that
is lawfully available to the unrestricted public on the Internet. Authorized
portal access, leaked or reposted controlled documents, and information supplied
under access restrictions are outside the project's source set.

The public DLA catalog identifies MIL-STD-6016 revisions and marks the selected
documents Distribution Statement C. That metadata lets this project identify
and cite a revision; it does not provide the normative message layouts,
DFI/DUI dictionary, processing rules, or revision deltas needed to claim a
complete implementation. The D, E, F, F Change 1, G, and H names in this
repository are consequently document identities and synthetic isolation tests,
not implemented standards support.

Open literature, public code, and public captures may support a useful partial
decoder. Every such capability must identify its public sources, tested scope,
known conflicts, and confidence. Other implementations may be used as
comparison evidence when their licenses permit, but they are not automatically
authoritative and agreement between implementations is not standards
conformance. Unsupported or uncertain semantics must remain explicit rather
than being filled from inference.

Under this source constraint, the project must not advertise itself as a
reference implementation of MIL-STD-6016, a conformant Link 16 implementation,
or a complete D-through-H decoder. The forward plan is to determine the maximum
honest capability available from public evidence and then narrow or redesign
the product around that result. See `docs/ROADMAP.md`.

**Verification here:** 27 Python schema/fixture tests passed; generated Rust
matches its six JSON sources. **Rust was not compiled or executed in the authoring
environment because cargo/rustc were unavailable.** No Windows or RHEL execution
is claimed. The included Rust tests must pass on your runners before accepting
this increment. See `evidence/VERIFICATION-STATUS.md`.

## Start with the specification

`docs/REVISION-SUPPORT-SPEC.md` is the main engineering document. It defines the
product boundaries, baseline identities, qualification workflow, coverage model,
and next implementation gates. The supporting documents are:

| File | Purpose |
|---|---|
| `docs/ARCHITECTURE.md` | Current core/CLI design and future adapter/assembly boundaries. |
| `docs/SCHEMA-CONTRACT.md` | Exact synthetic format and proposed qualified-package contract. |
| `docs/SYNTHETIC-FORMAT.md` | Hand-checkable toy vectors and explicitly invented revision differences. |
| `docs/requirements/requirements.json` | L1/L2/L3 requirements with implementation and evidence status. |
| `docs/adr/` | Recorded design choices and consequences. |
| `docs/VERIFICATION.md` | Tests, evidence, acceptance gates, and offline execution. |
| `docs/SECURITY.md` | Trust boundaries, controls, and unfinished security work. |
| `docs/ROADMAP.md` | Ordered milestones and completion criteria. |
| `docs/OPEN-SOURCE-PROJECTS.md` | Public project catalog and links to sequential deep analyses. |
| `docs/images/link16-message-data-picture.svg` | Diagram of the 70-bit data, parity, padding, word sequence, and SIMPLE transport layers. |
| `schemas/authoritative/` | Public source register and honest zero-coverage status. |

## Requirements to run locally

Use an already provisioned Rust toolchain and native linker, and Python 3.10+.
The Cargo manifests declare Rust 1.85 as the intended minimum; this minimum has
not been validated here. Use an organization-approved, pinned toolchain on your
runners and record its exact version. This repository does not auto-install or
auto-update a toolchain.

The Rust workspace has no third-party crate dependencies. The Python tools use
only the standard library. The Rust standard library, compiler, Cargo, Python,
and platform linker still have to be available locally. Offline Cargo flags do
not install those prerequisites; see the official Cargo reference in
`docs/SOURCES.md`.

From the extracted repository root, on Linux or Windows with `python` on PATH:

```text
python tools/bundle_manifest.py --check
python tools/check.py --rust
cargo run --locked --offline -p l16-cli -- baselines
cargo run --locked --offline -p l16-cli -- demo --baseline D --word 0x0765 --evidence "manual synthetic test"
cargo run --locked --offline -p l16-cli -- demo --baseline H --word 0x0765 --evidence "manual synthetic test"
```

Use `python3` on Linux or `py -3` on Windows where that is your local Python
invocation. `tools/check.py --rust` runs Python checks and both debug and release
Rust test suites; it fails rather than silently skipping Rust when cargo/rustc
are unavailable. To run ONLY the Python/schema checks:

```text
python tools/check.py
```

The toy input `0x0765` has raw `level=25`. Its invented D profile scales it to
25 test units; its invented H profile scales it to 250 test units. This tests
schema selection, not actual D/H differences. The CLI currently emits human-readable
text; a stable JSON/Parquet output adapter is not implemented.

This command is expected to **refuse**, with process exit code 3:

```text
cargo run --locked --offline -p l16-cli -- decode --baseline H
```

It cannot substitute the toy schema for an authoritative one.

## Editing synthetic schemas

The source of truth for bundled demo definitions is `schemas/synthetic/*.json`.

```text
python tools/schema_compile.py
python tools/check.py --rust
```

The compiler generates `crates/l16-core/src/generated.rs` and embeds a SHA-256
of each exact source file. The generated module is excluded from rustfmt so a
formatting pass does not make it diverge from the generator. Handwritten Rust
may be formatted with `cargo fmt --all` once your toolchain is available.

The runtime uses generated, immutable Rust tables. **Dynamic JSON schema loading
is deliberately deferred.** Source review and regeneration happen before a
build, never in the per-field hot path. After intentional changes, reissue the
file inventory with `python tools/bundle_manifest.py`; do not claim an old
archive digest describes modified files.

## GitLab and handling

`.gitlab-ci.yml` provides Linux and Windows shell-runner jobs. Configure the two
runner tags locally and provision Python, Rust, and native linkers beforehand.
No public image is selected by this pipeline. The jobs have not been executed
on your GitLab instance; generated `ci-evidence/` files contain each runner's
actual results rather than copying this package's evidence.

Do not add controlled standards, unauthorized reproductions, or schemas derived
from them to this repository. Public availability of a third-party repository
also does not by itself grant reuse rights; review its license before copying,
adapting, or redistributing anything. The official public catalog lists the six
selected baseline variants separately, but catalog metadata is not full-text
access [S1].

`publish = false` prevents accidental Cargo publication of these starter
crates while the project scope and licensing posture are being evaluated.
