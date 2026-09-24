# Conversation  

This is a ChatGPT conversation I have had and it is the bases for this project.

In link 16 do you know what dfi/dui means?

Yes. In Link 16, DFI/DUI means Data Field Identifier / Data Use Identifier. They work together to identify a data element in the message data dictionary.  

| Identifier | Meaning                   | Purpose                                                                                                                                            |
| ---------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| **DFI**    | **Data Field Identifier** | Identifies a general data category or concept. A DFI can contain multiple DUIs representing more specific definitions within that category.        |
| **DUI**    | **Data Use Identifier**   | Identifies a specific data-element definition within that DFI, including its name and permitted contents—the data items and associated bit codes.  |

The pair is important: the DUI is interpreted within its parent DFI. Together, the two numbers identify the particular dictionary entry. Think of (DFI, DUI) as a composite key into a schema dictionary.

You will also encounter DI — Data Item. That is a defined value or item of information permitted under the data element, rather than the identifier of the field definition. Depending on the definition, there can also be special items such as No Statement, Unknown, or reserved values; those are not necessarily interchangeable.

What this means when decoding

There are two separate pieces:

The message layout specifies which fields occur in a particular J-message/J-word and where their bits are located.
The DFI/DUI dictionary entry specifies how to interpret a field, including its size, resolution, or enumerated values.

For example, a published Link 16 modeling reference identifies 270/007 as “Voice Call Sign.” That pair references the field’s definition; it is not the call-sign value itself. The message layout connects that definition to a particular location in a J-word.

Do you know the J messages?

Yes—both the Link 16 J-series message types and how their words and fields relate to DFI/DUI definitions. The U.S. message standard is MIL-STD-6016, Tactical Data Link (TDL) 16 Message Standard.

Some familiar J-messages

These are examples—not the complete catalog—from ViaSat’s public Link 16 introduction.

| Message   | Name / purpose                                                     |
| --------- | ------------------------------------------------------------------ |
| **J2.2**  | Air Precise Participant Location and Identification (**Air PPLI**) |
| **J2.3**  | Surface PPLI                                                       |
| **J3.0**  | Reference Point                                                    |
| **J3.2**  | Air Surveillance Track                                             |
| **J3.3**  | Surface Track                                                      |
| **J3.5**  | Land Point/Track                                                   |
| **J7.0**  | Track Management                                                   |
| **J7.2**  | Correlation                                                        |
| **J12.0** | Mission Assignment                                                 |
| **J12.6** | Target Sorting                                                     |
| **J13.2** | Air Platform Status                                                |
| **J14.0** | Electronic Warfare Parametric                                      |

An important distinction is J2.2 versus J3.2: Air PPLI reports an airborne participant’s location and identification, whereas an Air Surveillance Track reports an airborne track. Those are different information categories, even though both concern airborne objects.

How this connects to your DFI/DUI question

There are several levels to keep separate:

| Level                          | What it identifies or defines                                                       |
| ------------------------------ | ----------------------------------------------------------------------------------- |
| **Message label and sublabel** | The message type. In **J3.2**, the label is **3** and the sublabel is **2**.        |
| **J-word**                     | A constituent word of the message: **Initial**, **Extension**, or **Continuation**. |
| **Field within the word**      | A defined bit location and width, associated with a data-element definition.        |
| **DFI/DUI**                    | The dictionary reference used to interpret that field.                              |
| **Data Item (DI)**             | A defined item/value associated with that data element.                             |

This message → word → field → dictionary relationship is described explicitly in a published Link 16 interoperability-modeling patent. The 3.2 in J3.2 is not a DFI/DUI pair.

A J-message has one initial word, with extension and/or continuation words as specified by its format. Decoding therefore requires selecting the correct message and word layout, extracting its fields, and applying the corresponding DFI/DUI interpretation—including any context-dependent definitions.

For bit-accurate decoding or validation, I would work against the applicable standard revision and platform interface specification rather than assume that remembered field offsets, scaling, or enumerations are authoritative.

Are you interested in a particular J-message, or in building a message dictionary and decoder?

Decoder

Then I would build a Rust J-series decoder with two separate pieces: a reusable decoding engine and a versioned message/DFI–DUI schema. For your recorded-data work, the target is the message bytes delivered by a recorder or terminal interface—not RF demodulation.

The important distinction is between recognizing a message as J2.2 and correctly decoding all its supported words and fields. The first is a small structural parser; the second requires verified layouts, dictionary definitions, and interpretation rules.

1. How DFI/DUI fits into the decoder

The decoder does not generally read a DFI number, a DUI number, and then a value from the payload. Instead, the message layout tells it which bit field corresponds to which DFI/DUI definition. J-series messages are organized into words, whose field definitions connect bit locations to dictionary entries.

The decoding relationship is therefore:

```
Selected standard revision and implementation profile
    → Message label/sublabel
    → Particular initial, extension, or continuation word
    → Field location and width
    → DFI/DUI definition
    → Interpretation of the extracted bits
```

That is why I would keep message layouts separate from data-element definitions. The layout answers “Where are the bits?” The dictionary answers “What do those bits mean?”

There is also an important refinement to my earlier explanation: a DFI/DUI pair is not always sufficient by itself to choose the final interpretation. Some definitions are context-dependent, with another field’s value selecting the meaning. The decoder needs to represent those dependencies explicitly.

2. Keep capture handling outside the J-message decoder

For your existing recorded-data tooling, I would use these boundaries:

| Component                    | Responsibility                                                                                                                               |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| **Recording reader**         | Read CH.10, PCAP, or another recording format; preserve timestamps and source offsets.                                                       |
| **Source/interface adapter** | Extract the message payload from the actual transport or application wrapper. Handle DDS/RTPS only when that is what the recording contains. |
| **Word normalizer**          | Convert the interface’s packing and byte order into one documented internal J-word representation.                                           |
| **Message assembler**        | Identify initial, extension, and continuation words and associate them according to the selected message format.                             |
| **Field decoder**            | Extract fields, resolve DFI/DUI definitions and dependencies, and produce values with diagnostics.                                           |
| **Output adapter**           | Present human-readable output or write JSON/Parquet without changing decoding behavior.                                                      |

The CH.10 reader should not know what J2.2 means, and the J2.2 decoder should not know how to read a CH.10 file.

The public message model describes logical J-words as 70 bits. I would represent that logical value internally with a checked u128, while retaining the original captured bytes separately.

But 70 logical bits does not mean “take every nine bytes from the recording.” Packing must follow the interface format. For example, Wireshark’s SISO-J implementation unpacks message data into an intermediate word representation before calling its separate Link 16 dissector. That is a concrete example of why the adapter and message parser should be separate.

Byte order, bit numbering, padding, and word boundaries should all be explicit—not inferred by trying combinations until a plausible coordinate appears.

3. Make the schema a first-class part of the project

I would define three related configuration objects.

Message definitions would describe each supported message and word: identifiers, field bit ranges, permitted word combinations, and references to dictionary definitions.

Data-element definitions would hold the DFI/DUI key and the supported interpretation rule: integer encoding, enumeration, numeric conversion, text handling, or a context-dependent choice. Special encodings would be represented explicitly rather than buried in conversion code.

A schema manifest would identify the standard revision, platform/interface profile, schema version, source references, and a digest of the exact schema content.

Conceptually:

```
schema-package/
    manifest.json
    messages/
        ...
    data-elements/
        ...
    profiles/
        ...
    test-vectors/
        ...
```

The file format is secondary; JSON would be a reasonable starting point. The important part is validating the package before decoding any recording.

For example, my schema loader would reject an out-of-bounds field, an unresolved dictionary reference, an unexplained overlap, or a cyclic interpretation dependency.

I would also load and validate the schema once, then compile it into in-memory extraction and interpretation operations. There should be no database query or JSON parsing for every decoded field.

Pin the applicable revision—not simply the newest one

DLA currently lists MIL-STD-6016H, dated April 26, 2024, and identifies the standard as a controlled-distribution document. That catalog entry establishes the revision, but it does not provide us with the complete field definitions for this implementation.

For your decoder, the correct schema is the one applicable to the recorded system. I would keep any controlled definitions and operational test data in your approved environment and use synthetic or releasable fixtures for public development.

4. Preserve the difference between raw bits and interpreted values

For flight-test analysis, I would make every decoded value traceable to its source.

A field result should retain its message and word identity, bit location, DFI/DUI reference, raw encoded value, interpreted value, units where applicable, and interpretation status. The enclosing message should retain the source record location, capture timestamp, available source/stream identity, decoder version, and schema digest.

I would distinguish at least these result categories:

| Result category               | Decoder behavior                                                                                              |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------- |
| **Valid value**               | Return the interpreted value and its raw encoding.                                                            |
| **Defined special value**     | Preserve the specific meaning assigned by the schema, rather than converting it into an ordinary measurement. |
| **Absent or incomplete**      | Report that the field or required word was not available.                                                     |
| **Unsupported or unresolved** | Preserve the raw data and explain which definition or dependency was unavailable.                             |
| **Invalid encoding**          | Report the violated rule without silently substituting another value.                                         |

These are proposed decoder result categories, not a claim that every Link 16 field supports every category.

Two implementation rules matter particularly:

Check special encodings before numeric conversion. Otherwise, a code that represents “No Statement” could become a believable but false measurement.

Resolve context dependencies before interpreting dependent fields. A practical implementation can extract raw fields first and then evaluate interpretations in dependency order. Missing context should produce an unresolved result, not a guessed meaning.

I would also keep decoded messages separate from reconstructed track state. Carrying forward a previously reported value may be useful in a later analysis layer, but it must not make the decoder appear to have received that value in the current message.

5. Start with a small Rust workspace

I would begin with a library and a CLI, not a collection of services:

link16/
├── crates/
│   ├── link16-core/
│   │   └── src/
│   │       ├── word.rs
│   │       ├── message.rs
│   │       ├── schema.rs
│   │       ├── decode.rs
│   │       └── diagnostic.rs
│   └── link16-cli/
├── schemas/
└── tests/
    ├── fixtures/
    └── expected/

Your existing recording pipeline would feed extracted payloads through a source adapter into link16-core.

The core should have no dependency on CH.10, a network socket, PostgreSQL, or ClickHouse. That makes it straightforward to test the same message bytes through a CLI, a batch-processing job, or your eventual data-ingestion pipeline.

6. Build and prove one increment at a time

First, prove the word representation. Establish the input framing, byte order, padding, and bit numbering. Use synthetic single-bit patterns and boundary cases so normalization mistakes are obvious.

Second, prove structural decoding. Decode word format, label, sublabel, message-length information, and continuation labels. Wireshark exposes these structural fields, and its source shows how it maintains message context for extension and continuation words. It is a useful comparison point for this stage.

Third, decode one message completely within a declared scope. Select a message present in your test data and verify its supported words and fields against independently established expected results. “Complete” should mean an explicit coverage statement—not merely that the initial word works.

Fourth, prove failure behavior. Test truncation, unexpected words, unknown definitions, invalid special values, missing context, and mixed source streams. Impose bounds on message assembly and emit diagnostics rather than consuming unrelated words to make an incomplete message appear complete.

Wireshark publishes SISO-STD-002 and SIMPLE example captures that could support an initial public test harness. Those would test those specific encapsulations; they would not establish that your recorder uses the same packing.

My recommended first deliverable is a word/header inspector with traceable source offsets, followed by one verified message decoder. A synthetic or releasable example record, its packing/interface description, and the applicable standard revision would let us replace the remaining input assumptions with a concrete implementation.

Can we make a plan to support revisions D through H. Do you have access to the full range of revisions?

Yes—we can plan for MIL-STD-6016 revisions D through H. But I do not have verified full-text access to that entire set. I can access the official revision catalog and public supporting material; I have not inspected the complete message layouts, dictionaries, and processing requirements for those revisions.

To clarify my earlier answers: familiarity with J-series message structure is not the same as possessing every revision’s authoritative field definitions. We should build the decoder around verified source documents, not fill missing definitions from memory.

My recommendation is one Rust decoding engine, separately validated revision schemas, and explicit evidence for every supported message and field.

1. The baselines we should support

The official ASSIST catalog lists the following documents within your requested range.

| Baseline                                 | Official document date |
| ---------------------------------------- | ---------------------- |
| **MIL-STD-6016D**                        | December 12, 2008      |
| **MIL-STD-6016E**                        | July 20, 2012          |
| **MIL-STD-6016F**                        | January 31, 2017       |
| **MIL-STD-6016F, Change 1 incorporated** | August 31, 2017        |
| **MIL-STD-6016G**                        | July 8, 2020           |
| **MIL-STD-6016H**                        | April 26, 2024         |

F and F with Change 1 should be distinct baseline identities. The catalog lists them separately, so I would not collapse them into a single F configuration before reviewing the change. That gives us six catalogued baseline variants, not merely five revision letters.

Access to the complete documents

All six catalog entries carry Distribution Statement C. DLA describes that statement as restricting distribution to U.S. Government agencies and their contractors, with other requests referred to the controlling office. A distribution statement is distinct from a security-classification marking; “controlled distribution” does not, by itself, tell us that a document is classified.

The official ASSIST FAQ directs contractors to their government liaison or government supervisor for controlled documents, or to the document’s Preparing Activity for release approval. A commercial ASSIST account does not provide access to controlled-distribution documents.

For this project, I would request the complete historical baseline set, including applicable changes and supporting interface documentation, through your organization’s approved channels. Keep those documents—and any potentially controlled schemas derived from them—in an environment approved for their handling.

We can develop the generic framework and synthetic tests without bringing the controlled definitions into this conversation.

2. Define what “supports D through H” actually means

I would make our initial product scope:

Decode recorded J-series messages according to an explicitly selected, versioned baseline; preserve raw evidence; and report unsupported, ambiguous, or invalid content without guessing.

That is different from claiming complete MIL-STD-6016 implementation or interoperability certification.

I would track three levels separately:

| Support level           | What we would demonstrate                                                                                           |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------- |
| **Structural decoding** | Correct message identification, word handling, field extraction, and structural diagnostics for the declared scope. |
| **Semantic decoding**   | Correct dictionary interpretation, enumerations, conversions, special encodings, and applicable field dependencies. |
| **Stateful validation** | Separately implemented checks that require prior messages, elapsed time, or maintained participant/track context.   |

For your recorded flight-data work, I would prioritize structural and semantic decoding, with stateful validation as a separate capability.

This separation matters: a field can be extracted and interpreted correctly even when evaluating the surrounding exchange requires additional history. Conversely, a parser that recognizes the message name is not yet a complete semantic decoder.

3. Make the effective baseline more precise than a revision letter

The decoder configuration should identify:

```
Document baseline
    + incorporated change level
    + explicitly selected additional changes
    + platform implementation profile
    + schema package version and digest
```

The capture/interface format should be identified separately. How the recorder packages the bytes and which message definitions apply are two different configuration decisions.

There is a documented reason to accommodate changes beyond revision letters: Curtiss-Wright’s June 2023 announcement described five change proposals approved for inclusion in H that were available as add-ons to its G configuration. That is an example of a vendor configuration—not a complete G-to-H difference list—but it demonstrates why “G” alone may not fully describe an implementation.

For our design, any additional change would need an identified source, applicability rules, and tests. We should reject contradictory or unsupported combinations rather than allow arbitrary feature switches.

Also, keep these versions distinct:

Standard revision identifies the governing document. Schema package version identifies our transcription and interpretation of that document. Decoder version identifies our software. Output schema version identifies the format presented to downstream consumers.

A correction to our G schema would not turn it into a different MIL-STD revision.

4. Use one engine, but version more than the dictionary

I would retain the separation proposed earlier:

```
Capture adapter
    → Normalized input
    → Revision-aware message assembly
    → Field extraction
    → Semantic interpretation
    → Diagnostics and decoded output
```

Message assembly must also be revision-aware. We should not assume that multi-revision support consists only of switching enumeration tables.

During the standards review, we would compare the following dimensions. These are items to investigate, not claims that each one changed between D and H:

| Definition area            | What the review must establish                                                             |
| -------------------------- | ------------------------------------------------------------------------------------------ |
| Message and word inventory | Which messages and word forms exist in each baseline.                                      |
| Layout                     | Field positions, widths, and applicable word layouts.                                      |
| Dictionary definitions     | DFI/DUI references, permitted values, conversions, units, and special encodings.           |
| Conditional interpretation | Which other fields or contexts select a meaning.                                           |
| Assembly and validation    | Required, optional, repeated, or conditionally applicable content and related constraints. |
| Processing requirements    | Which checks are message-local and which require maintained state.                         |

Do not make DFI/DUI globally revision-independent

For dictionary lookup, I would use a conceptual key such as:
```
(effective schema identity, DFI, DUI)
```
The selected message layout and interpretation context would still be available to the decoder.

Matching DFI/DUI identifiers across two revisions would be a candidate for reuse—not proof that their definitions are identical. We would establish equivalence from the sources and record that determination.

Independent release snapshots, with controlled reuse

During authoring, shared definitions and explicit changes can reduce duplication. However, every released baseline should compile into a complete, independently identifiable schema snapshot.

At runtime, selecting D should load the released D schema—not load H and search backward when a definition is missing.

My preferred sequence is:

Represent each baseline explicitly first. Compare them. Share definitions only after equivalence is verified.

That is less compact initially, but it makes accidental cross-revision inheritance easier to prevent and audit.

5. Require explicit revision selection

Until we verify an authoritative revision indication in your actual input interface, I would require the baseline to come from configuration or trusted recording metadata.

The assignment could apply to an entire capture, a particular source stream, or a source over a defined time interval. It should not be a mutable global setting that silently changes the interpretation of unrelated records.

For every decode, retain both:

The selected effective baseline, and the evidence for selecting it—for example, the operator’s configuration, a recording manifest, or a verified interface metadata field.

When that evidence is missing, the decoder should not silently assume H.

An optional diagnostic mode could compare a record against several schemas and show disagreements. But a record decoding successfully under a schema would not be treated as proof that the schema is the correct one. Multiple interpretations may remain possible.

Similarly, an apparent mismatch should produce a diagnostic, not an automatic switch to another revision.

6. Implementation plan

I would organize the work into six phases with concrete completion gates.

| Phase                                          | Work                                                                                                                                                                                                            | Completion gate                                                                                              |
| ---------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| **1. Source and scope register**               | Inventory D, E, F, F Change 1, G, H; record document identities, digests, completeness, applicable changes, interface specifications, and permitted handling. Define decoding versus stateful-validation scope. | Every baseline has an explicit source status. Missing documents or sections remain visible blockers.         |
| **2. Generic Rust framework**                  | Implement schema loading, validation, bounded extraction, diagnostics, source tracing, and output serialization. Use synthetic schemas with intentionally different layouts and meanings.                       | Tests prove that the selected schema controls behavior and that no cross-revision fallback occurs.           |
| **3. First message across the revision range** | Choose a representative message from your recordings. Implement it first against the best-supported baseline, then verify its applicable forms in the remaining baselines.                                      | An end-to-end demonstration exercises the multi-revision design with independently checked expected results. |
| **4. Verified revision comparison**            | Compare adjacent baselines and produce an impact report linking changed definitions to affected layouts, interpretations, and tests. Record verified unchanged definitions too.                                 | Every identified change has a disposition: implemented, outside the declared scope, or unresolved.           |
| **5. Expand message coverage**                 | Add messages in recording-driven priority order while maintaining the complete source-derived inventory for eventual coverage.                                                                                  | Each supported message has declared word, field, and interpretation coverage for each applicable baseline.   |
| **6. Release and regression**                  | Package immutable schemas, run cross-platform and offline tests, validate representative recordings, and publish coverage and limitations.                                                                      | Each released baseline is reproducible and supported by its own evidence and regression results.             |

Prove the revision architecture early

I would not implement all of H first and then attempt to retrofit D–G.

Instead, implement one meaningful vertical slice across the requested range early. That will expose weaknesses in the schema model before we populate thousands of definitions.

The first message should come from your actual data and available reference material. We do not need to choose it solely because it is familiar.

Produce a semantic difference report

A text comparison between documents is useful for locating edits, but our review output should answer implementation questions:

Did the encoded representation change? Did an interpretation change? Was a constraint added? Is this only an editorial correction? Which messages reference the affected definition?

Each determination should link back to the relevant source location and reviewer decision.

I would not publish a D-to-H field-level change list until those comparisons have actually been performed.

7. Verification must be independent of the implementation

The central testing rule should be:

A schema generating its own test messages and then successfully decoding them does not independently prove that the schema matches the standard.

That kind of round-trip test is useful, but it can reproduce the same mistake on both sides.

For acceptance, we should use independently reviewed expected results from approved reference material, trusted test vectors, or captures whose expected interpretation has been established separately.

The test program should cover three complementary concerns:

Revision correctness. Verified unchanged definitions should behave equivalently. Verified changed definitions should show the expected difference. A definition absent from a selected baseline must not appear through fallback.

Interpretation correctness. Exercise special encodings, numeric boundaries, enumeration cases, and each supported conditional interpretation—not just ordinary values.

Robustness. Exercise truncation, unexpected content, incomplete messages, unknown definitions, and conflicting configuration. Malformed input should produce bounded failures and useful diagnostics.

For your Windows and disconnected Linux environments, I would make reproducible offline execution part of the project’s acceptance criteria, including the schema validation and test tooling—not just the final CLI.

8. Preserve enough provenance to re-decode later

Each decoded record should retain, directly or through a referenced run manifest:

```
Source recording identity and record location
Capture timestamp and source identity
Original message bytes or a durable reference to them
Effective baseline and selection evidence
Schema package identity and digest
Decoder version
Decoded values and interpretation status
Diagnostics
```

This lets you answer:

> “Was this value decoded using F or F Change 1, and was it produced before or after our schema correction?”

It also allows corrected decoding without pretending that an earlier result never existed.

For downstream analysis, I would preserve the distinction between a valid measurement, a defined special value, an absent field, and an unresolved interpretation. None should silently become a plausible numeric value.

9. The first deliverable

I would start with a revision-support specification and testable framework, rather than a large collection of unverified message definitions.

That first increment should establish the six baseline identities, the source register, the schema format, revision-selection rules, a coverage-report format, and the Rust workspace with synthetic multi-revision tests. The controlled standards-population work can then proceed within your approved environment.

We can begin the architecture now. Claiming verified D–H coverage depends on obtaining the authoritative sources and validating each declared capability against them. The public catalog establishes the document history and access restrictions; it does not supply the complete definitions needed to certify our transcription.

Do you already have D, E, F—including Change 1—G, and H available in your approved work environment?

Please continue

I’ve created the first engineering package for the D–H decoder: a revision-support specification, traceable requirements, architecture decisions, and a Rust implementation foundation.

Download the starter repository · Read the revision-support specification

This is a framework milestone—not yet a real Link 16 decoder. All supplied message definitions are explicitly synthetic. The package does not contain authoritative J-message layouts, DFI/DUI dictionaries, or actual recorder-packing rules.

| Component                                 | What I created                                                                                                                                                                             |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Revision-support specification**        | Scope, baseline identities, source qualification, revision comparison, coverage reporting, and release gates.                                                                              |
| **59 traceable requirements**             | Five L1 requirements, ten L2 requirements, and 44 L3 requirements, with implementation and verification status.                                                                            |
| **Eight architecture decision records**   | Decisions covering baseline isolation, synthetic-data separation, schema generation, interpretation status, component boundaries, qualification, state reconstruction, and offline builds. |
| **Rust core library**                     | Explicit baseline selection, checked bit extraction, schema validation, conditional interpretation, special-value handling, and provenance-bearing results.                                |
| **Rust CLI**                              | Baseline inspection, synthetic decoding, argument validation, and a standards-decoding path that refuses unqualified schemas.                                                              |
| **Six synthetic schema bundles**          | Separate identities for D, E, F, F-C1, G, and H, generated from validated JSON sources.                                                                                                    |
| **Source and coverage registers**         | Public catalog metadata, unset authoritative-document hashes, and zero claimed real-message coverage.                                                                                      |
| **Verification and GitLab configuration** | Python checks, Rust tests, requirement-to-test mapping, and Linux/Windows shell-runner jobs.                                                                                               |

The separate F-C1 identity represents F with Change 1 incorporated. The official catalog lists that baseline separately from the original F, so the implementation preserves the distinction.

2. The architectural decisions now have concrete implementations
Explicit selection, with no revision fallback

The Rust baseline type distinguishes:

```
D
E
F
F-C1
G
H
```

There is no default to H, no latest alias, and no automatic attempt to reinterpret an unsupported D message using another baseline.

Each decoder instance binds to one immutable schema bundle. The caller must supply selection evidence, which is retained in the output. That evidence is currently a caller-provided description—not authenticated proof that the selected baseline matches a recording.

For the eventual recording pipeline, we will need to associate the selected baseline and profile with the correct source, session, and potentially time interval. That assignment belongs outside the field-decoding engine.

Schema authoring is separate from runtime decoding

The supplied implementation uses this path:

```
Synthetic JSON definitions
        ↓
Python validation and schema compilation
        ↓
Generated immutable Rust tables
        ↓
Baseline-bound Rust decoder
        ↓
Raw fields + interpreted values/status + provenance
```

The Python tooling validates definitions and generates Rust tables before building. It is not involved in the per-field decoding path.

This also makes the initial offline dependency footprint small: the Rust workspace has no third-party crate dependencies, and the Python tools use the standard library.

Dynamic JSON schema loading is not implemented in this increment. Changing a bundled definition requires regenerating the tables and rebuilding. A future dynamically loaded authoritative package will need its own integrity, trust, qualification, and compatibility checks.

Raw values and interpreted results remain distinct

The result model does not reduce every field to a number. It distinguishes:

| Result                 | Meaning in the starter                                                                     |
| ---------------------- | ------------------------------------------------------------------------------------------ |
| **Value**              | A supported interpretation produced a normal value.                                        |
| **Special**            | The encoding represents a defined special item in the selected synthetic schema.           |
| **Not applicable**     | A valid selector made the field’s applicability condition false.                           |
| **Unresolved context** | The selector was invalid, special, or otherwise unsuitable for choosing an interpretation. |
| **Invalid encoding**   | The bits do not represent a permitted ordinary value under the implemented rule.           |

The raw field value and bit range remain available in each case.

This distinction is particularly important for conditional fields. “The condition is false” and “we cannot determine the condition” are different outcomes. Likewise, a special encoding must be recognized before applying a numerical conversion.

The current numeric subset includes unsigned values, explicit two’s-complement signed values, enumerations, and scaled unsigned values represented as exact rational numbers. That is a prototype capability set, not an assertion that it covers every encoding needed by the standard.

Synthetic schemas cannot enter standards-decoding mode

The schema compiler accepts only explicitly synthetic definitions. The Rust standards-decoding mode refuses all bundled profiles.

The package therefore cannot turn a synthetic example into an apparently authoritative decode just because its baseline label says H.

The source register is equally explicit: authoritative documents are not acquired in this workspace, while their availability in your approved environment remains unknown. Document hashes and local source locations are left unset rather than invented.

3. Coverage is an evidence model, not a percentage attached to a revision

The requirements and specification distinguish:

```
Source registered
    → Transcribed
    → Peer reviewed
    → Schema validated
    → Independent vectors passed
    → Qualified for a declared scope
```

Those states are intended to apply to definitions and rules, then roll up into message-level coverage.

For all six baselines, the current coverage register reports:

```
Authoritative inventory total: unknown
Implemented standard messages: 0
Verified standard messages:    0
Conformance claim:             false
```

Unknown inventory is represented as null, not zero. That prevents an empty inventory or a small implemented subset from producing a misleading “100% supported” result.

When we populate the actual standards, the report will need to distinguish the complete baseline inventory from the subset we have deliberately chosen to implement. Structural decoding, semantic interpretation, and stateful validation will also remain separately reported.

4. What I verified—and what remains unverified

| Check                                                | Actual result                                                       |
| ---------------------------------------------------- | ------------------------------------------------------------------- |
| **Python schema and fixture tests**                  | **27 passed.**                                                      |
| **Synthetic schema validation**                      | **All six bundles passed.**                                         |
| **Generated Rust freshness**                         | **Passed:** the generated tables match their JSON sources.          |
| **Requirements and test mapping**                    | **Passed:** 59 requirements; all 47 Rust test functions are mapped. |
| **Python, JSON, and Cargo manifest/lockfile syntax** | **Passed.**                                                         |
| **GitLab YAML syntax**                               | Parsed successfully; the jobs were not executed.                    |
| **Archive integrity**                                | Checked against the included file-hash inventory.                   |
| **Rust compilation and execution**                   | **Not performed.**                                                  |
| **Windows and RHEL execution**                       | **Not performed.**                                                  |

Neither Rust nor Cargo was installed in this environment, so I could not compile the Rust implementation or run its tests. The package includes 38 core integration tests and nine CLI integration tests, but their execution remains an acceptance gate.

The Python checks validate the schema tooling, fixture arithmetic, and project metadata. They do not establish that the Rust compiles or that any real military-standard message has been decoded correctly.

Read the verification report

5. Running the first increment

After extracting the archive, use a locally provisioned Rust toolchain, native linker, and Python installation:

```
cd link16-revision-starter

python tools/bundle_manifest.py --check
python tools/check.py --rust

cargo run --locked --offline -p l16-cli -- baselines
```

Use python3 on Linux or py -3 on Windows where that is your local Python invocation.

The --rust check deliberately fails when Cargo or Rust is unavailable; it does not silently skip the Rust acceptance step. On a provisioned runner, it runs the schema checks, Python tests, and both debug and release Rust test suites.

Cargo’s --locked and --offline options constrain dependency resolution and network access. They do not install missing compilers, standard libraries, or linkers, so those prerequisites must already be present.

Demonstrating revision isolation

The supplied toy fixture uses a 16-bit logical integer—not a Link 16 word format:

```
cargo run --locked --offline -p l16-cli -- demo --baseline D --word 0x0765 --evidence "manual synthetic test"

cargo run --locked --offline -p l16-cli -- demo --baseline H --word 0x0765 --evidence "manual synthetic test"
```
The independently specified fixture expectations are:

| Selected synthetic profile | Raw `level` | Expected interpretation |
| -------------------------- | ----------: | ----------------------: |
| D                          |          25 |           25 test units |
| H                          |          25 |          250 test units |

Those multipliers are deliberately invented. They are not actual D-to-H changes. Their purpose is to demonstrate that the selected schema—not a global default or an accidental fallback—controls interpretation.

A separate fixture exists only in the synthetic H bundle. Tests specify that attempting to decode it under D must fail rather than search another revision.

The following command is written to refuse standards decoding because no qualified schema is installed:
```
cargo run --locked --offline -p l16-cli -- decode --baseline H
```
6. What comes next

The immediate acceptance gate is to compile and run this foundation on the approved Linux and Windows runners, record the exact toolchain versions, and resolve any build or test failures. The supplied GitLab configuration uses pre-provisioned shell runners rather than assuming access to a public container image.

The next substantive implementation increment should then establish the authoritative schema contract and actual capture/interface contract. That requires the approved source documents, the recorder’s packing description, and independently reviewed vectors. The current core accepts an already normalized integer; it does not yet read CH.10, PCAP, terminal wrappers, or assemble real J-messages.

After those inputs are established, we should implement one representative real message across its applicable D–H forms, including its declared word forms, exceptional values, and conditional interpretations, before expanding the catalog.

The governing acceptance rule is: a baseline label identifies a source version; verified support belongs to a specific, evidenced set of messages, words, fields, and rules.

I need to make a GitHub repo for this under my telemetryworks organization. What should I call the project?

Since this is intended to become a serious multi-revision Link 16/J-series decoding library, rather than just a one-off decoder, I’d give it a name that can survive expansion.

My strongest candidates are:

jseries — my favorite. Short, technically meaningful, and doesn't unnecessarily tie the project name to a particular transport or capture format.
jseries-rs — same idea, while making the Rust implementation obvious.
jseries-codec — good if we expect encoding as well as decoding eventually.
jseries-decoder — very clear, but potentially restrictive if we later add encoding, validation, message construction, and state tracking.
tdl16 — broader and concise, but less immediately recognizable than Link 16/J-series.
link16-rs — extremely obvious, although I prefer avoiding a name that might imply a complete Link 16 implementation including terminal/network/RF functionality.
jword — memorable and appropriate for the low-level decoding work, but too narrow once the project handles complete messages.
I recommend jseries

That gives you:
```
telemetryworks/jseries
```
and leaves room for the repository to grow into:

jseries
├── jseries-core
├── jseries-schema
├── jseries-codec
├── jseries-capture
├── jseries-cli
├── schemas/
│   ├── 6016d/
│   ├── 6016e/
│   ├── 6016f/
│   ├── 6016f-c1/
│   ├── 6016g/
│   └── 6016h/
└── tools/

I'd describe it on GitHub as

> Revision-aware Rust toolkit for decoding, validating, and analyzing J-series tactical data messages.

That wording is deliberately narrower than claiming to be a complete Link 16 implementation.

I also like jseries under TelemetryWorks because the organization supplies the broader context: telemetryworks/jseries reads like a reusable telemetry protocol project rather than a throwaway utility.

I would choose jseries over link16-decoder. It gives us room to add encoding, schema inspection, revision comparison, validation, capture adapters, provenance, and eventually message-state reconstruction without having to rename the repository.

Can you tell me if there are any open source projects which already exists?

Yes. I found several projects worth studying, and one of them is particularly relevant to what you're building. The key finding is that I still don't see an open-source project that already provides the thing we want: a revision-aware, field-level MIL-STD-6016 D–H decoder with DFI/DUI semantics.

There are, however, some useful projects we should treat as references rather than reinventing everything ourselves.

| Project                                                                                                                                  | What it provides                          | Value to `jseries`                    |
| ---------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- | ------------------------------------- |
| [Wireshark Link 16 dissector](https://github.com/wireshark/wireshark/blob/master/epan/dissectors/packet-link16.c?utm_source=chatgpt.com) | Real Link 16 structural dissector         | **Very high**                         |
| [Ersatz-MIL-STD-6016](https://github.com/liotier/Ersatz-MIL-STD-6016?utm_source=chatgpt.com)                                             | Open reconstruction/schema of J-series    | **Very high**                         |
| [SENTINEL](https://github.com/bwiemz/sentinel?utm_source=chatgpt.com)                                                                    | Python J-series encode/decode examples    | Medium/high                           |
| [AirSim-TDL-Surrogate](https://github.com/MichaelFowler1/AirSim-TDL-Surrogate?utm_source=chatgpt.com)                                    | Experimental J-series decoding/simulation | Medium                                |
| [SISO Link 16 Simulation resources](https://www.sisostandards.org/page/DataFiles?utm_source=chatgpt.com)                                 | Public simulation standard/data files     | **Very high for test infrastructure** |

Wireshark is more useful to us than it might first appear

Wireshark has a genuine packet-link16.c dissector licensed GPL-2.0-or-later. It recognizes the Link 16 word format and extracts fields including:

```
Word Format
Label
Sublabel
Message Length Indicator
Continuation Word Label
```
It also maintains state so extension and continuation words can be associated with the current J-message.

It knows, for example, that the label/sublabel combination identifies messages such as:
```
J2.2  Air PPLI
J3.0  Reference Point
J3.2  Air Track
J3.3  Surface Track
...
```
The important limitation is that Wireshark doesn't perform the complete field-level J-message decode we're trying to build. The current dissector is only about 306 lines and primarily handles J-word structural information; it doesn't contain the comprehensive DFI/DUI semantic definitions we'd need.

That's actually useful confirmation of the architecture we've been discussing:
```
transport/packing
       ↓
J-word framing
       ↓
J-message identification
       ↓
message schema
       ↓
DFI/DUI semantics
```

I would use Wireshark as an independent reference implementation for those first few layers, rather than copying its architecture wholesale.

Ersatz-MIL-STD-6016 is the project I'd examine first

This was the most relevant find:

Ersatz-MIL-STD-6016 GitHub repository

It's explicitly an open-source reconstruction of the Link 16/TADIL-J message schema built from public information, and—very useful for us—it contains a machine-readable:

```
link16-schema.json
```
along with an HTML reference.

Its current inventory identifies 78 J-series message types across J0–J31 and 23 NPGs. But there's an important catch: "78 messages" definitely does not mean 78 fully decoded messages.

Their own coverage report says:
```
78 known message types

 1 HIGH confidence
 6 MEDIUM confidence
71 STUB — name only
```

So Ersatz gives us an excellent public message inventory and schema starting point, but it doesn't eliminate our need for authoritative, revision-specific schemas.

I'd actually make this distinction explicit in our repository:

```
schemas/
    public/
        ersatz/
    authoritative/
        6016d/
        6016e/
        6016f/
        6016f-c1/
        6016g/
        6016h/
```
And our qualification system should prevent this from ever happening:

```
public reconstruction
        ↓
silently becomes
        ↓
authoritative MIL-STD decode
```
The provenance work we've already started fits that requirement very well.

SENTINEL has actual encode/decode code worth studying

SENTINEL GitHub repository

This one is useful more from an implementation perspective. Its repository includes:

```
datalink/
    codec.py
    j_series.py
    encoding.py
    validator.py
    adapter.py
    gateway.py
    transport.py
    track_mapping.py
```
and describes implementations for:
```
J2.2
J3.2
J3.5
J7.0
```
including binary encoding/decoding and field-range validation.

I'd absolutely study the implementation, but I wouldn't treat its output as authoritative. Instead, it gives us another implementation to compare:

```
their interpretation
        vs.
our schema
        vs.
Wireshark
        vs.
MIL-STD-6016
```
Disagreement between those sources becomes something we investigate rather than something we silently resolve in favor of one implementation.

AirSim-TDL-Surrogate is interesting for a different reason

AirSim-TDL-Surrogate

This is an experimental tactical-data-link simulation/decoding pipeline supporting messages including:

```
J0.0
J2.2
J3.2
J28.2
```
Its architecture parses specification-like input into SQLite and then performs J-series interpretation using database-driven rules.

That's surprisingly close conceptually to one part of what we're proposing:

```
Their approach

specification
     ↓
regex parser
     ↓
SQLite
     ↓
decoder


Our proposed approach

MIL-STD source
     ↓
validated schema compiler
     ↓
immutable revision schema
     ↓
Rust decoder
```
For what you want—revision traceability, offline deployment, reproducibility, schema hashes, and strong verification—I'd stick with our compiled/validated schema approach. But this project could still give us useful ideas and test cases.

SISO can help solve the testing problem

SISO publishes resources for Link 16 simulation, including SISO-STD-002-2021 and Link 16 FOM data.

That's potentially quite important because the difficult part isn't writing something like:

```
let value = extract_bits(word, 12, 7);
```
The hard part is demonstrating that the whole chain is correct:

```
test capture
     ↓
simulation transport
     ↓
packing
     ↓
J-word
     ↓
our decoder
```

Public SISO material could give jseries a legitimate foundation for its public CI and interoperability tests, while the authoritative D–H field definitions remain in the appropriate controlled environment.

What this means for telemetryworks/jseries

After searching specifically around Link 16, TADIL-J, J-series, MIL-STD-6016, DFI/DUI, JREAP, SISO, Rust, parsers, and decoders, I still don't see an existing project equivalent to what we're proposing:

```
                  jseries
                     │
        ┌────────────┼─────────────┐
        │            │             │
      6016D        6016E         6016F
        │            │             │
      schema        schema        schema
                                     │
                                  F Change 1

        ┌────────────┴─────────────┐
        │                          │
      6016G                      6016H
        │                          │
      schema                     schema

                     │
                DFI / DUI
                     │
              semantic decoder
                     │
           validation + provenance
                     │
             reconstructed state
```

I especially didn't find that combination implemented in Rust.

So I think telemetryworks/jseries remains a good, distinct project. Seeing the existing work actually sharpens what its purpose should be:

jseries — A revision-aware Rust framework for decoding, validating, and analyzing J-series tactical data messages using independently versioned message schemas.

I'd also make the public/private schema boundary a deliberate architectural feature from day one. The public GitHub repository could operate entirely with Ersatz, SISO, and synthetic schemas, while an internal build can add qualified D–H schemas without placing controlled material on GitHub.

One other interesting data point: I found an unrelated project feasibility analysis that reached essentially the same conclusion—public Link 16 word framing and message identification can be checked against Wireshark, while field-level decoding lacks a comparable public oracle. It's not authoritative evidence, but it reinforces why the provenance and qualification system we're building matters.

My next move would be to do a code-level review of Wireshark, Ersatz, and SENTINEL against our current jseries starter and produce a concrete “reuse / adapt / don't copy / missing capability” matrix. That would tell us exactly what should go into the first real telemetryworks/jseries commit instead of rebuilding pieces that already have good public implementations.

I don understand why Jseries is a good name versus something related to link 16

That's a fair challenge. jseries is technically defensible, but I don't think it's the best name if your project's identity is specifically Link 16. I was weighting extensibility too heavily.

The key distinction is:

Link 16 is the tactical data link/system context. J-series messages are the message formats carried by Link 16 and defined by MIL-STD-6016. So our decoder is not a generic "J-series" project in some broader sense—we are building tooling specifically around Link 16 J-series messages, their DFI/DUI definitions, revision differences, validation, and eventually capture integration.

That makes a Link 16-oriented name clearer.

| Repository            | What it communicates                  | Concern                                       |
| --------------------- | ------------------------------------- | --------------------------------------------- |
| **`link16`**          | Exactly the domain                    | Could imply a complete Link 16 implementation |
| **`link16-rs`**       | Link 16 implemented in Rust           | Same breadth issue, but manageable            |
| **`link16-codec`**    | Encode/decode Link 16 messages        | Excellent if codec remains central            |
| **`link16-decoder`**  | Exactly what we're doing today        | Too narrow if we add encoding                 |
| **`link16-toolkit`**  | Broader analysis/validation tooling   | Slightly generic                              |
| **`link16-messages`** | Focuses specifically on message layer | Very accurate                                 |
| **`link16-jseries`**  | Makes both concepts explicit          | Longer, but unambiguous                       |
| **`tdl16`**           | Short technical name                  | Less recognizable                             |

I now particularly like link16-jseries for your project.

```
telemetryworks/link16-jseries
```
It says precisely where our boundary is:
```
                    LINK 16
                       │
          ┌────────────┼────────────┐
          │            │            │
       Terminal       Network     Messages
       / Radio        behavior       │
                                    │
                              link16-jseries
                                    │
                         ┌──────────┼──────────┐
                         │          │          │
                       J2.2       J3.2       ...
                         │
                    DFI / DUI
                         │
                   6016 revisions
                   D E F F-C1 G H
```
It doesn't imply that we're implementing the RF waveform, JTIDS/MIDS terminal, time-slot allocation, frequency hopping, cryptography, or the entire Link 16 network stack.

That's the concern I had with simply calling it link16.

There's another reason I like link16-jseries

It gives TelemetryWorks room for adjacent repositories later without awkward naming:

```
telemetryworks/
│
├── link16-jseries
│     J-message encode/decode/validation
│
├── link16-jreap
│     JREAP transport/adapters
│
├── link16-capture
│     CH.10 / PCAP / recorder ingestion
│
├── link16-analysis
│     message/track analysis
│
└── link16-test
      simulation/interoperability tooling
```
We don't necessarily need those separate repositories—the functionality could remain crates inside one workspace—but the naming remains coherent either way.

Alternatively, if you think this is eventually going to become the TelemetryWorks Link 16 library, encompassing transport adapters, message processing, state reconstruction, analysis, encoding and decoding, then I'd actually take the simpler name:

telemetryworks/link16

and make jseries one crate:

```
link16/
├── crates/
│   ├── link16-core
│   ├── link16-jseries
│   ├── link16-schema
│   ├── link16-codec
│   ├── link16-jreap
│   ├── link16-capture
│   └── link16-cli
└── ...
```

Given where you've been taking this project, I prefer link16 if you expect it to grow into a broader toolkit; otherwise link16-jseries is the most precise name for the decoder we're designing today.

Are jseries messages used anywhere else besides link16?

Yes. And this changes my view of the repository name somewhat.

J-series is closely associated with Link 16, but it isn't exclusively synonymous with the Link 16 radio/network. There are two important cases.

1. Link 22 uses the J-series family

Link 22 uses F-series messages, but those are considered part of the broader J-series message family. It deliberately reuses Link 16 field definitions for interoperability. Canadian DND documentation states this explicitly: Link 16 uses J-series messages, while Link 22's F-series belongs to the J-series family and uses the same Link 16 field definitions.

So conceptually:

```
J-series message family
│
├── Link 16
│    └── J-series messages
│         J2.2
│         J3.2
│         J7.x
│         ...
│
└── Link 22
     └── F-series messages
          using common/related field definitions
```

I would not, however, interpret this to mean that a MIL-STD-6016 J-message decoder automatically decodes Link 22 F-series messages. That's a separate protocol implementation despite the common lineage.

2. Actual Link 16 J-messages can travel without the Link 16 RF waveform

This distinction is even more relevant to our project.

JREAP exists specifically to extend Link 16 and other TDL information over communications media other than the normal line-of-sight JTIDS/MIDS Link 16 radio network. Marine Corps documentation describes JREAP as carrying tactical data over digital media and networks not originally designed for tactical data exchange.

Similarly, SIMPLE (STANAG 5602) can carry J-series/TADIL-J information over IP-oriented simulation/interoperability environments.

So you can have:

```
                     J-series
                        │
              ┌─────────┼──────────┐
              │         │          │
          Link 16     JREAP      SIMPLE
         JTIDS/MIDS      │          │
              │          │          │
             RF       WAN/IP     simulation/
                                  integration
```

The message semantics can remain J-series even though the bearer isn't the Link 16 RF waveform.

That distinction is actually important for our decoder architecture.

Therefore, jseries makes more sense than I gave it credit for

If our library's job is:

"Take a J-series message and tell me exactly what it means according to the applicable MIL-STD-6016 revision."

then jseries is arguably the technically cleaner name.

The library shouldn't care whether those bytes came from:

```
MIDS terminal
      │
      ├──── Link 16 RF
      │
JREAP-C/IP
      │
SIMPLE
      │
CH.10 recording
      │
PCAP
      │
DDS/RTPS wrapper
      │
test fixture
      ▼
 ┌─────────────┐
 │   jseries   │
 │   decoder   │
 └─────────────┘
      │
      ▼
 J3.2 / DFI/DUI / fields
```

That's an excellent separation of concerns.

But there's a naming tradeoff

jseries describes the layer we're implementing. link16 describes the ecosystem people will recognize.

I'd now narrow the choices to:

telemetryworks/jseries

Revision-aware Rust codec and schema framework for Link 16 J-series tactical data messages.

or:

telemetryworks/link16-jseries

Rust toolkit for decoding and validating Link 16 J-series messages across MIL-STD-6016 revisions.

I actually lean back toward jseries now that we've drawn the boundary precisely. It gives us a nice architecture where transport-specific functionality doesn't contaminate the message library:

```
telemetryworks/jseries
        ↑
        │
   ┌────┼────────────┐
   │    │            │
 JREAP SIMPLE   Link16 capture
   │    │            │
   └────┴────────────┘
```

And if we eventually implement Link 22 F-series, I would not force it into jseries just because the standards share field definitions. I'd probably make fseries a sibling that can reuse a common tactical-data-element crate.

So your original question exposes the important naming principle: we aren't building "a Link 16 radio implementation." We're building the message-language decoder that is most famously used by Link 16 but can be transported independently of it.

What is "JREAP", "SIMPLE", "LINK16 Capture"

Those are three different ways/contexts in which Link 16 J-series data might reach our decoder. I used “Link 16 capture” a little too casually before; it isn't the name of a protocol.

The easiest way to think about it is:

```
                   J-series message
                       J3.2
                  "Air Track", etc.
                         │
        ┌────────────────┼────────────────┐
        │                │                │
   Native Link 16       JREAP           SIMPLE
    environment       transport       test/simulation
        │                │                │
   recording/capture   IP/SATCOM           IP
        │                │                │
        └────────────────┼────────────────┘
                         ▼
                  J-series decoder
```

JREAP

JREAP = Joint Range Extension Applications Protocol.

Its purpose is essentially:

Carry already-formatted tactical data messages across networks/media other than the communications system for which those messages were originally designed.

That's the formal purpose given by MIL-STD-3011. It allows tactical data to travel over terrestrial links, satellite links, and other digital networks, extending tactical networks beyond their normal line-of-sight limitations.

So imagine two geographically separated locations:

```
Aircraft / Link 16
       │
       ▼
   Link 16
     J3.2
       │
       ▼
  JREAP gateway
       │
       │   WAN / SATCOM / IP
       │=======================│
                               ▼
                         JREAP gateway
                               │
                               ▼
                         J3.2 message
                               │
                               ▼
                         C2 application
```

The important thing is:

JREAP isn't replacing the J3.2 message definition. It's providing another mechanism for transporting that formatted tactical information. MIL-STD-3011 explicitly describes formatted tactical messages being embedded inside JREAP messages.

There are multiple JREAP variants. For our eventual software, JREAP-C is particularly interesting because it operates over IP networks.

That suggests a clean architecture:

```
JREAP-C packet
      │
      ▼
jreap decoder
      │
      ▼
J-series payload
      │
      ▼
jseries decoder
      │
      ▼
J3.2
├── field A
├── field B
└── ...
```

JREAP is its own standard, MIL-STD-3011. NATO also has STANAG 5518 for JREAP.

SIMPLE

SIMPLE = Standard Interface for Multiple Platform Link Evaluation.

This is quite different.

SIMPLE was designed primarily to let geographically separated tactical-data-link test, simulation, C4ISR and integration facilities communicate with each other.

It's standardized by STANAG 5602.

Think:

```
Simulator A                         Simulator B

F-16 model                          AWACS model
    │                                    │
    │                                    │
Link-16/J-series                    Link-16/J-series
simulation                              simulation
    │                                    │
    └────────── SIMPLE / IP ──────────────┘
```

Instead of needing real MIDS terminals and RF communication just to perform integration testing, facilities can exchange the relevant tactical-data-link information over an IP network.

SIMPLE supports transmission of J-series and M-series messages, so it's broader than just Link 16.

This is especially interesting for us because there are public Wireshark SIMPLE packet captures. Wireshark's sample-capture collection includes a stanag-5602-simple-example.pcap.

That could eventually give us a public integration test:

```
stanag-5602-simple-example.pcap
             │
             ▼
       PCAP reader
             │
             ▼
      SIMPLE decoder
             │
             ▼
      J-series message
             │
             ▼
       jseries decoder
```

No controlled flight-test data required for that test.

"Link 16 capture"

This one isn't a protocol. I meant a recording containing Link 16-related data.

For example, you might receive data from some interface associated with a terminal, gateway, instrumentation system, or test system and record it into:

```
PCAP
Chapter 10
vendor recording format
test instrumentation
network capture
```

Exactly what's in that recording depends on where you tapped the system.

That's important.

A capture might contain something close to:

```
J3.2 words
```
Or it might contain:
```
Ethernet
  ↓
IP
  ↓
UDP/TCP
  ↓
JREAP-C
  ↓
J-series
```
Or:
```
Chapter 10
   ↓
particular data channel
   ↓
vendor/interface framing
   ↓
J-series words
```
So I would rename the component I previously called link16-capture.

Something like jseries-input or separate adapters would be clearer.

How this affects our project

This actually reinforces the architecture we've been developing.

The J-series decoder should know nothing about JREAP, SIMPLE, PCAP, Chapter 10, Ethernet, UDP, etc.

It receives something normalized:
```
            Transport / Recording Layer

        ┌─────────┬──────────┬──────────┐
        │         │          │          │
      JREAP     SIMPLE      CH.10      PCAP
        │         │          │          │
        ▼         ▼          ▼          ▼
     adapters / framing / normalization
        │         │          │          │
        └──────────────┬────────────────┘
                       │
                       ▼
                J-series words
                       │
             ┌─────────▼─────────┐
             │      jseries      │
             │                   │
             │ MIL-STD-6016 D-H  │
             │ J-message decoder │
             │ DFI/DUI semantics │
             └─────────┬─────────┘
                       │
                       ▼
                 Decoded J3.2
```
That's a strong separation.

For example, if tomorrow someone hands you a JREAP-C PCAP:

```
PCAP
 ↓
Ethernet
 ↓
IPv4
 ↓
JREAP-C
 ↓
J3.2
 ↓
jseries
```

If instead someone hands you a Chapter 10 recording containing J-series data:

```
CH.10
 ↓
channel parser
 ↓
interface framing
 ↓
J3.2
 ↓
jseries
```
The same jseries decoder handles both.

And that's probably the strongest technical argument for calling the repository telemetryworks/jseries rather than telemetryworks/link16: our software boundary is the message representation, not the RF/network transport system. The input adapters can grow independently around it.

This does not sound like a correct description for the jseries repo
Revision-aware Rust codec and schema framework for Link 16 J-series tactical data messages.

Agreed. “codec and schema framework” describes our implementation mechanics rather than what the project actually does, and “Link 16 J-series tactical data messages” can misleadingly make Link 16 sound like the transport boundary.

I would describe telemetryworks/jseries as:

A Rust library and toolkit for decoding, encoding, validating, and analyzing J-series tactical data messages across MIL-STD-6016 revisions.

That is much closer to the intended scope.

If you want a shorter GitHub description, my preference is:

Rust toolkit for J-series tactical data message processing across MIL-STD-6016 revisions.

And the README can establish the larger boundary more precisely:

jseries provides revision-aware processing of J-series tactical data messages defined by MIL-STD-6016. It supports decoding, encoding, validation, field interpretation, and message analysis independently of the mechanism used to transport or record those messages. Transport and capture protocols such as JREAP, SIMPLE, PCAP, and Chapter 10 are handled by separate adapters and are outside the core J-series message model.

One thing I'd avoid claiming right now is:

“Supports MIL-STD-6016 revisions D through H.”

until we've populated and independently verified those schemas. Instead:

Designed to support MIL-STD-6016 revisions D through H through explicitly versioned message definitions.

That accurately distinguishes the architecture we've built from the coverage we haven't established yet.

For the GitHub repository's one-line description, “Rust toolkit for J-series tactical data message processing across MIL-STD-6016 revisions.” is the one I'd use.

I am going to leave out the word rust because we will have a python library which uses PyO3.

Yes, that makes sense. The project identity should describe the capability, not the implementation language—especially if Python is a first-class interface through PyO3.

I'd use:

Toolkit for J-series tactical data message processing across MIL-STD-6016 revisions.

Or, a little more descriptive:

Toolkit for decoding, encoding, validating, and analyzing J-series tactical data messages across MIL-STD-6016 revisions.

I prefer the second for the GitHub description because someone finding telemetryworks/jseries immediately understands what it does.

Then the README can explain the implementation:

The core implementation is written in Rust, with first-class Rust and Python APIs. Python bindings are provided using PyO3.

That also gives us a natural package structure:
```
jseries/
├── crates/
│   ├── jseries-core
│   ├── jseries-schema
│   ├── jseries-codec
│   └── jseries-cli
│
├── python/
│   └── jseries
│       └── PyO3 bindings
│
├── schemas/
│   ├── 6016d
│   ├── 6016e
│   ├── 6016f
│   ├── 6016f-c1
│   ├── 6016g
│   └── 6016h
│
├── tests/
├── docs/
└── Cargo.toml
```

And eventually users get equally natural interfaces:

```Rust
use jseries::Decoder;
```
and:
```Python
import jseries

decoder = jseries.Decoder(...)
```
So I would make the official GitHub description:

Toolkit for decoding, encoding, validating, and analyzing J-series tactical data messages across MIL-STD-6016 revisions.

That's language-neutral and leaves plenty of room for where we're taking the project.

