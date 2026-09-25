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

## Priority 1 — GitHub Actions baseline

- Make the initial cross-platform CI workflow required for protected branches.
- Run format, Clippy with warnings denied, requirement trace validation, debug tests, release tests, and a Rust 1.86 MSRV job.
- Enable Dependabot and dependency review after repository policy is agreed.
- Add CodeQL once the base workflow is stable; add scheduled fuzzing and benchmark monitoring after harnesses exist.
- Pin action revisions according to organization policy and document secret/permission minimization.
- Do not reproduce the reference repository's Sonar workflow unless its service, secrets, and value are explicitly approved.

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

Add stable library APIs, capture readers, structured output, Python bindings, fuzzing, deterministic replay, and observability as independently testable adapters. Stateful track reconstruction remains downstream from received decoded events.
