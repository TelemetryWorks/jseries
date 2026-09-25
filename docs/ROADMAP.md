# Forward-looking roadmap

This file contains future work only. Completed research belongs in dated reports under `docs/research/`.

## Priority 0 — evidence consolidation

Sort out `evidence/` before expanding semantic coverage:

1. inventory every evidence file, its producer, freshness, scope, and whether it is source-controlled or generated;
2. designate human-maintained verification policy in `docs/VERIFICATION.md` and runner-generated evidence under `evidence/runs/<date>-<commit>/`;
3. define a small machine-readable run manifest containing commit, dirty state, toolchain, platform, commands, results, and artifact hashes;
4. remove duplicate status narratives and never copy a prior run forward as current evidence;
5. have GitHub Actions publish run evidence as immutable workflow artifacts before deciding which summaries belong in Git;
6. establish retention, sanitization, provenance, and review rules; and
7. link requirements and releases to exact workflow runs and public-source fact records.

Completion means every retained artifact has one owner/source, an explicit freshness rule, and no conflicting current-status document.

## Priority 1 — CI/CD and supply-chain expansion

- Make the existing cross-platform CI jobs required in the `main` branch protection rules.
- Establish reviewed coverage thresholds after representative test coverage and a stable baseline exist.
- Add dependency review for pull requests, Dependabot updates, `cargo audit`, and a `cargo deny` policy covering advisories, licenses, bans, and sources.
- Add scheduled Criterion regression reporting after representative benchmarks and stable runner methodology exist; compile every benchmark in ordinary CI meanwhile.
- Add scheduled fuzzing for transport normalization, assembly, and TOML loading with bounded corpora and artifact retention.
- Add documentation checks, `cargo semver-checks` once a public API is released, and release workflows that produce an SBOM, attestations, signatures, and checksums for release artifacts.
- Automate reviewed updates for immutable action pins, define retention for every produced artifact, and connect those artifacts to the evidence plan.

## Priority 2 — package contract completion

- Add expected field/status assertions to vector TOML and execute every vector during validation.
- Add fact-level provenance records, source locators, confidence, conflicts, license notes, and inference flags.
- Define package compatibility, integrity/signing, deterministic snapshot, and migration policies.
- Add safe symlink/reparse-point policy and fuzz the loader.

## Priority 3 — transport and assembly evidence

- Validate each named 70/75/80 representation against public interface documentation and independent vectors.
- Preserve original source bytes, timestamps, offsets, channel/session identity, parity results, and normalization diagnostics.
- Implement only publicly supported assembly rules; keep incomplete or ambiguous sequences explicit.

## Priority 4 — publicly supportable semantics

- Convert public facts only after license and provenance review.
- Start with the smallest useful message subset whose every field can cite public evidence.
- Record conflicts rather than selecting a convenient interpretation.
- Publish coverage against the declared public-evidence scope, never against the controlled standard.

## Priority 5 — keep hot paths hot

- Add single-word, multiword, catalog, conditional, and batch benchmarks with representative distributions.
- Separate normalization, assembly, field extraction, and output measurements.
- Add reusable output buffers and batch APIs; measure allocation count before making allocation claims.
- Establish reviewed regression thresholds and store benchmark metadata/artifacts in CI.
- Profile before adding caching, SIMD, parallelism, or unsafe code; unsafe remains forbidden until a separate reviewed decision.

## Priority 6 — product adapters

Extend the persistent Python decoder beyond logical-70 with explicit word75 and simple80 inputs. Measure and add compact columnar batch output, reusable buffers, chunked streaming, and optional parallel execution for very large datasets. Add capture readers, structured serialization, fuzzing, deterministic replay, and observability as independently testable adapters. Stateful track reconstruction remains downstream from received decoded events.
