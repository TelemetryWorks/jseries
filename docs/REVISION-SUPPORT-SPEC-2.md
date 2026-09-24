# Revision-support specification

**Document:** L16-SPEC-001  
**Version:** 0.1.0  
**Status:** Proposed architecture; synthetic foundation supplied  
**Date:** September 21, 2026

## 1. Product purpose and present scope

Build a reusable Rust library and offline CLI that decode recorded message data
against an explicitly selected and independently qualified MIL-STD-6016 baseline.
The production objective is support across D through H, including F Change 1 as
a separately identifiable baseline. This specification does not claim that those
messages are already implemented.

Increment 0.1 establishes baseline isolation, validated field extraction,
interpretation status, schema-generation discipline, provenance, and test
infrastructure using an intentionally unrelated toy format. There are zero
implemented authoritative J-messages. No RF demodulation, terminal decryption,
transmission, tactical command generation, or network participation is in scope.
The project is a recorded-data software decoder, not a terminal implementation.

The accepted input to the current core is one bounded **normalized logical
integer** together with an explicit synthetic message identity. A caller cannot
feed arbitrary capture bytes and expect real message recognition. Real message
assembly, word-format identification, and byte/bit packing remain unimplemented.

## 2. Baseline identities

| Internal identity | Requested document baseline | Catalog date |
|---|---|---|
| D | MIL-STD-6016D | 2008-12-12 |
| E | MIL-STD-6016E | 2012-07-20 |
| F | MIL-STD-6016F | 2017-01-31 |
| F-C1 | MIL-STD-6016F, Change 1 incorporated | 2017-08-31 |
| G | MIL-STD-6016G | 2020-07-08 |
| H | MIL-STD-6016H | 2024-04-26 |

These dates and the separate F/F-C1 catalog entries are from [S1]. A baseline
constant identifies a requested source baseline; it is not proof of access,
implementation, verification, or conformance. No newer revision is silently
substituted. `F` means the recorded unmodified F baseline; `F-C1` must be selected
explicitly for the incorporated-change variant.

The source register records availability **in this package** as `not_acquired`.
Availability inside the user's approved work environment remains `unknown`.
Document digests, local locators, completeness reviews, and reviewers are null,
not guessed. Public catalog information must never be cited as evidence for a
specific bit location, enumeration, or conversion rule.

## 3. Version identities to keep separate

A qualified release will resolve the following identity tuple:

```text
source document baseline and incorporated changes
+ explicitly approved additional change set
+ platform implementation profile
+ resolved schema snapshot and source inventory
+ schema compiler identity
+ decoder build identity
+ capture adapter identity and packing profile
+ output-contract version
```

The baseline is a source-document fact. The schema version is our maintained
transcription and interpretation of that source. The implementation profile
constrains applicability for a recorded system. The output version governs
consumers of decoded results. None is a substitute for another.

Current output stores the selected baseline, synthetic schema identity/version,
source-JSON digest, profile identity, decoder crate version, output version,
selection-evidence text, logical word, and caller-supplied source reference and
offset. It does **not yet** identify an actual capture adapter, full binary build
hash, compiler build digest, or standard source inventory. Those are release
requirements for real capture processing, not completed capabilities.

## 4. Three independently reported support dimensions

**Structural support:** applicable message/word identification, assembly rules,
bit ranges, and structural validity for a declared scope.

**Semantic support:** applicable definitions, normal values, special encodings,
conversions, and conditional interpretations for that scope.

**Stateful validation:** checks requiring context beyond the current assembled
message. This belongs in a separate layer, with declared ordering and missing-
history behavior. It must not turn inferred or carried-forward values into
values allegedly received in the current message.

A real message must have separate structural, semantic, and stateful status.
Recognizing a name does not establish that all of its forms are understood.
A useful partial decoder is permitted, but its exact coverage and limitations
must be visible to the caller and release report.

## 5. Source-to-schema qualification workflow

For every baseline, establish an approved local source register. Record the
exact document identity, incorporated changes, document digest, completeness
assessment, and access/handling owner. The present starter provides the register
format but does not ingest actual standards.

Within the approved environment, create a reviewed inventory of applicable
messages, word layouts, dictionary definitions, conditional rules, and relevant
processing requirements. Assign stable source locators that a reviewer can
resolve to the actual local document. A locator is evidence linkage; it is not
itself a full text excerpt or a permission to redistribute one.

Use an evidence progression rather than a single `supported=true` flag:

```text
source_registered -> transcribed -> peer_reviewed -> schema_validated
                  -> independent_vectors_passed -> qualified_for_declared_scope
```

These states should be tracked per definition/rule and rolled up to message
coverage. Their transition requirements must be explicit. A transcription that
parses successfully is not necessarily correct; a correct ordinary-value test
is not proof of every special or conditional case.

Before a real-schema release, a reviewer who did not simply reproduce the same
implementation logic must establish expected results from approved sources and
vectors. Acceptable evidence may include authorized reference vectors or
independently reviewed calculations tied to the governing source. Comparisons
with another decoder can add evidence but are not automatically normative.

Automated extraction from a document may accelerate transcription, but must
not produce an approval state automatically. No actual standard extraction is
implemented here. Especially review bit numbering, signs, units, exceptional
codes, applicability notes, and cross-references; none should be guessed.

## 6. Revision comparison and reuse

Maintain an independent released snapshot for each baseline. During authoring,
shared definitions are allowed only after their equivalence is reviewed.
Shared source content must not create runtime inheritance from a later revision.
The effective resolved snapshot must stand alone.

Perform adjacent comparisons D→E, E→F, F→F-C1, F-C1→G, and G→H. The comparison
report records additions, removals, layout changes, semantic changes,
conditional/validation changes, reviewed editorial-only changes, and unresolved
questions. These are comparison categories, not claims that each kind of change
actually occurred.

The authoritative model should identify a field occurrence by baseline/profile,
message/word context, and field identity. Its dictionary lookup should be scoped
by effective schema plus DFI/DUI and interpretation context. The same numeric
DFI/DUI in two baselines is a reuse candidate, not evidence of identical meaning.
The toy compiler intentionally uses `SYNTH:` references and no real DFI/DUI codes.

Production schema authoring should separate layout occurrences, dictionary
definitions, and interpretation/validation rules. This increment embeds its
small toy interpretation alongside each field for simplicity; it does not yet
implement the production dictionary store or a complete standard-schema model.

## 7. Revision selection and uncertainty

The decoder requires explicit selection evidence. The current CLI has no
default baseline and accepts neither `latest` nor fuzzy aliases. One immutable
decoder instance binds to one resolved bundle. Batch pipelines may construct
several instances, but there is no mutable process-wide current revision.

For future recordings, define a revision assignment by source/session and,
when necessary, by non-overlapping time interval. A recording must not be
assumed homogeneous merely because it is one file. Capture timestamps alone
must not be treated as proof of revision.

Configuration conflict or missing evidence must prevent a qualified semantic
decode. An optional future comparison mode may show multiple candidate
interpretations, but must not promote a successful parse into identification of
the true baseline. Current `SelectionEvidence` records a caller assertion; it
does not authenticate an operator or verify interface metadata.

For unknown messages, preserve the input in a rejected-record result and report
`unsupported for selected baseline`. Never try the same payload under H as a
hidden recovery strategy for D. The core's current unsupported-message error
retains the logical word; a future adapter must also preserve captured bytes
and framing context because the logical value alone cannot reconstruct them.

## 8. Interpretation and diagnostics

First extract raw fields. Then evaluate applicability and interpretation in a
validated dependency order. A field whose condition is false is semantically
not applicable, even though its bit positions still exist. An invalid or
special selector must not be reduced to false or guessed as a normal value.

When applicable, check special encodings before ordinary numeric conversion.
Preserve each special meaning independently. Unknown enumeration codes produce
an explicit invalid-encoding status; they are not silently coerced to zero.

The current status model includes value, special, not applicable, unresolved
context, and invalid encoding. It retains raw values for all of those states.
Whole-message length errors are currently rejected rather than partly decoded;
field-level truncation and missing-word statuses are future assembly work.

Current numeric rules are unsigned, explicit two's-complement signed,
scaled-unsigned rational, and enumeration. Rational output retains an exact
numerator/denominator. This is a chosen prototype subset, not a list of every
encoding that the standard uses. Scaling is presently limited to widths at most
63 bits; unsupported arithmetic is rejected rather than silently rounded.

## 9. Coverage report and acceptance

The inventory denominator is unknown until the authoritative inventory is
established. `null` is not zero. Do not compute 100% coverage from an empty
inventory or report a tiny declared subset as the whole standard.

The release report must separate full baseline inventory, explicitly declared
scope, implemented scope, peer-reviewed scope, and independently verified scope.
It must also identify excluded requirements and unresolved issues. A future
report can show, for example, verification of a declared subset while retaining
its smaller proportion of the complete inventory, without conflating them.

The current coverage file lists zero implemented and zero verified standard
messages for all six baselines. Synthetic test counts live in a separate
verification report and never contribute to these totals.

A real-message qualification gate requires exact source identity, reviewed
layout and semantics, all declared word forms, special-value tests, conditional
cases, malformed/truncated-input behavior, source-to-output traceability, and
known limitations. A broad release additionally needs successful Windows and
RHEL execution, bounded resource behavior, and an approved software/schema
supply-chain process.

## 10. Immediate next gate

The next engineering gate is to compile and run this supplied framework on the
approved Linux and Windows runners, record its results, and correct any build
or test failures. Python validation here is not evidence that the Rust compiles.

In parallel, populate the approved local source register and obtain the actual
recorder/interface packing description. Then select one representative message
with accessible authoritative definitions and independent vectors, implement
its adapter/assembly/semantics for a declared scope, and compare its applicable
forms across all six baselines before expanding the catalog.

This order proves revision support early, without postponing architecture
problems until after one entire revision has been hand-coded.
