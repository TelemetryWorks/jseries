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

Study the candidate projects below strictly one at a time and in the listed
order. Finish and review one report before beginning the next so that findings
from a later project do not silently alter the evidence recorded for an earlier
one.

The inventory currently contains two open-source projects and two
source-available comparison projects:

| Project | Current license posture to verify at pinned commit |
|---|---|
| Ersatz-MIL-STD-6016 | Unlicense; open source |
| Wireshark | GPL-2.0-or-later for the relevant dissector; open source |
| SENTINEL | View/study-only custom terms; not open source |
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

## Study 1 — Ersatz-MIL-STD-6016

Repository: <https://github.com/liotier/Ersatz-MIL-STD-6016>

This is the first study because its machine-readable public reconstruction is
the closest apparent overlap with the schema role proposed for `jseries`.

The analysis will:

- audit the structure and completeness of `link16-schema.json`, including the
  meaning of its confidence values and whether layouts contain enough detail
  for bit-level decoding rather than message-name lookup;
- measure actual coverage by message, word form, field, interpretation, and
  source citation instead of relying on the headline message count;
- determine whether DFI/DUI identities, units, scaling, exceptional values,
  conditional meanings, validation rules, and revision identities are present;
- trace reconstructed facts through the files in `sources/` and flag facts that
  cannot be tied to a specific public locator;
- validate internal ranges, overlaps, identifiers, references, and example
  values with independent tooling;
- assess whether its Unlicense grant covers every bundled artifact and source
  document relevant to possible reuse; and
- compare direct schema reuse or conversion with maintaining a separate
  `jseries` evidence model and treating Ersatz as an external data provider.

Planned report: `docs/research/01-ersatz-mil-std-6016.md`.

## Study 2 — Wireshark Link 16 and SIMPLE dissectors

Repository: <https://github.com/wireshark/wireshark>

The analysis will cover `packet-link16.c`, its state interface,
`packet-simple.c`, any other verified callers that supply normalized message
data, relevant tests, and public sample captures.

The analysis will:

- document exact byte order, bit numbering, word-format extraction, label and
  sublabel handling, message-length information, extension sequencing, and
  continuation-label state;
- follow the complete call path from supported encapsulations into the Link 16
  dissector so capture framing is not confused with J-series semantics;
- identify the public references cited by the code and the age or revision
  assumptions behind its tables;
- run selected captures through a pinned Wireshark/TShark build and preserve
  expected structural output as external comparison evidence;
- establish precisely what Wireshark does not decode, especially field-level
  layouts and DFI/DUI semantics;
- assess GPL-2.0-or-later implications separately for copying code, porting an
  algorithm, invoking TShark as a test oracle, and recording factual test
  results; and
- compare its mature capture integration with the intentionally transport-free
  `jseries` core boundary.

Planned report: `docs/research/02-wireshark-link16.md`.

## Study 3 — SENTINEL data-link module

Repository: <https://github.com/bwiemz/sentinel>

SENTINEL is publicly viewable, but its current repository terms restrict
copying, reuse, modification, redistribution, and commercial use. Unless those
terms change or written permission is obtained, this study will inspect it only
as a non-reusable comparison project and will not copy code, schemas, fixtures,
or tests.

The analysis will:

- isolate the data-link package from the larger sensor, tracking, and targeting
  simulation and inventory its claimed J2.2, J3.2, J3.5, and J7.0 behavior;
- determine which layouts and validations are explicitly synthetic or toy and
  whether any claimed STANAG or J-series facts have public source locators;
- inspect its bit reader/writer, message dataclasses, encoder/decoder symmetry,
  validators, gateway, transport abstraction, and track-number mapping;
- run only those tests and demonstrations permitted by the repository terms,
  without importing their artifacts into `jseries`;
- compare its Python API and bidirectional simulation workflow with the planned
  Rust core and Python bindings; and
- record architectural lessons independently, with clean-room notes that do
  not reproduce protected implementation expression.

Planned report: `docs/research/03-sentinel-datalink.md`.

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
