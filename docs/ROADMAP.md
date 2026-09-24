# Forward roadmap

This roadmap contains future work only. Completed investigations and their
evidence will live in dated reports under `docs/research/`, not in this file.
Do not mark a study complete here or turn preliminary observations into product
claims.

## Governing constraint

The project will use only lawfully available, unrestricted public Internet
sources. TelemetryWorks cannot obtain MIL-STD-6016 through an authorized
government or contractor channel. The roadmap must therefore optimize for a
traceable public-evidence implementation, not standards conformance.

No public project, capture, patent, paper, presentation, or agreement between
implementations will be treated as a substitute for the controlled standard.
The work may establish a useful, precisely scoped decoder, but it must preserve
unknown and conflicting semantics and must not claim complete D-through-H
support.

## Cross-project synthesis

Only after all four individual reports are reviewed, produce
`docs/research/05-public-ecosystem-synthesis.md`. It will:

- normalize the four capability matrices into one comparison;
- identify duplicated functionality and gaps that remain unserved;
- distinguish data/schema assets from decoder engines, transport adapters,
  simulation tooling, and test oracles;
- compare source quality and confidence at field granularity;
- recommend which parts of the current `jseries` starter to retain, refactor,
  replace, or remove;
- define the narrowest defensible product scope and wording for public claims;
  and
- propose an implementation sequence with explicit evidence and licensing
  gates.

## Engineering gates after research

### Gate A — establish an executable baseline

Compile and run the current Rust debug and release suites on supported Windows
and Linux toolchains. Record versions, correct build failures, apply formatting,
and establish lint policy. This proves only the synthetic foundation.

### Gate B — revise the product contract

Use the synthesis report to replace the current D-through-H aspiration with an
evidence-bounded scope. Reconcile the README, architecture, requirements,
schema contract, command names, crate names, and coverage model. Decide whether
the current implementation is worth evolving or should be replaced before
adding public reconstructions.

### Gate C — define a public-evidence schema contract

Represent per-fact source locators, confidence, conflicts, inference status,
license provenance, and unsupported semantics. Prevent a reconstructed or toy
schema from being reported as authoritative. Define a reproducible snapshot and
review process for third-party public data.

### Gate D — prove one public input path and message subset

Select one openly documented packing format and the smallest message subset
supported by traceable public evidence. Implement framing, normalization,
assembly, decoding, and diagnostics without guessing missing semantics. Verify
against independently derived public vectors or comparison tools where
available.

### Gate E — expand evidence-driven coverage

Prioritize new fields and messages by public-source quality and user value.
Publish coverage as a denominator of the explicitly declared public-evidence
scope, never as a percentage of MIL-STD-6016. Preserve unresolved conflicts and
source-specific variants.

### Gate F — production hardening

Add stable Rust and Python APIs, fuzzing, bounded resource use, deterministic
replay, sanitized logging, performance measurements, cross-platform CI,
supply-chain evidence, and a documented compatibility policy. Production
quality will describe software robustness, not standards conformance.
