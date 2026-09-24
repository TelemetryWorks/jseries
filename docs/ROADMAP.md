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

## Research method

Study the remaining candidate below before beginning the cross-project
synthesis. Completed studies and their status are recorded in
`docs/OPEN-SOURCE-PROJECTS.md`, with evidence under `docs/research/`; they are
removed from this forward-looking document.

The remaining individual study is source-available rather than open source:

| Project | Current license posture to verify at pinned commit |
|---|---|
| AirSim-TDL-Surrogate | PolyForm Noncommercial 1.0.0; source-available, not open source |

The pinned license text, not this planning summary, will govern each study.

Each study will:

1. Pin the upstream repository, branch, commit SHA, retrieval date, and license
   text. Distinguish open-source software from code that is merely publicly
   viewable.
2. Reproduce the documented build and tests in an isolated environment when
   practical, recording commands, versions, failures, and skipped checks.
3. Inventory implemented functionality from code and tests rather than from
   README claims alone: accepted inputs, packing, word recognition, assembly,
   message coverage, field extraction, DFI/DUI semantics, encoding, validation,
   state handling, outputs, and transport adapters.
4. Trace each J-series fact used by the project to its stated public source.
   Classify it as directly sourced, inferred, synthetic, unexplained, or in
   conflict. Record whether citations exist per field or only at project level.
5. Identify revision behavior: explicit MIL-STD-6016 baseline selection,
   unversioned assumptions, schema inheritance, and any claimed cross-revision
   differences.
6. Exercise ordinary, boundary, special, malformed, and round-trip cases.
   Treat self-consistency as implementation evidence, not proof of correctness.
7. Compare the project with `jseries` across capability, architecture,
   provenance, offline use, testability, performance, language/API, maintenance,
   and licensing dimensions.
8. End with a decision for each asset or idea: reuse, adapt with attribution,
   use only as an independent oracle, learn from without copying, or reject.
   State the evidence and license basis for that decision.

Each report will include a capability matrix, source/provenance matrix,
license/reuse assessment, reproducible test notes, defects and uncertainties,
alignment/competition analysis, and recommendations. Use neutral descriptions:
"competes" means overlapping user-visible capability, not that either project
is authoritative or commercially viable.

## Study 4 — AirSim-TDL-Surrogate

Repository: <https://github.com/MichaelFowler1/AirSim-TDL-Surrogate>

The repository currently uses the PolyForm Noncommercial 1.0.0 license. It is
source-available rather than open source and must not be adopted for a
commercial use without a separate license.

The analysis will:

- pin and review the effective license before running, copying, or adapting
  anything, and keep reuse out of scope unless the intended use is permitted;
- separate AirSim flight generation, UDP/RPC transport, ETL, SQLite storage,
  terminal presentation, and claimed J-series decoding capabilities;
- inspect the mock interface-control input and establish whether the scraper is
  a general document-ingestion technique or is coupled to invented fixtures;
- trace the claimed J0.0, J2.2, J3.2, and J28.2 definitions to public sources and
  determine whether the terminal performs binary field decoding, database
  lookup, telemetry mapping, or presentation only;
- run the ETL-focused tests without requiring the full simulator where
  possible, then assess determinism, validation, malformed-input behavior, and
  database schema quality;
- compare its dynamic regex/SQLite approach with `jseries` compiled schema
  snapshots and offline constraints; and
- decide whether any ETL patterns, fixtures, or integration ideas are suitable
  for reuse, independent comparison, or rejection.

Planned report: `docs/research/04-airsim-tdl-surrogate.md`.

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
