# AirSim-TDL-Surrogate deep analysis

**Study status:** Complete<br>
**Analyzed head:** [`72c2406b350b44537355104f63c0eb2704a39335`](https://github.com/MichaelFowler1/AirSim-TDL-Surrogate/tree/72c2406b350b44537355104f63c0eb2704a39335)<br>
**Head commit date:** 2026-09-23<br>
**Reusable historical snapshot assessed:** [`7ca500f8f07e23ffd684330819321a2631f1de1a`](https://github.com/MichaelFowler1/AirSim-TDL-Surrogate/tree/7ca500f8f07e23ffd684330819321a2631f1de1a)<br>
**Analysis date:** 2026-09-23<br>
**Current license:** [PolyForm Noncommercial 1.0.0](https://github.com/MichaelFowler1/AirSim-TDL-Surrogate/blob/72c2406b350b44537355104f63c0eb2704a39335/LICENSE)<br>
**Historical license:** [Apache License 2.0](https://github.com/MichaelFowler1/AirSim-TDL-Surrogate/blob/7ca500f8f07e23ffd684330819321a2631f1de1a/LICENSE)

## Conclusion

AirSim-TDL-Surrogate does not decode or encode J-series messages. Its ETL input
is a six-line invented message catalog containing only identifier, name,
nominal 75-bit size, and category. A regex converts those six lines to JSON; a
loader copies the four columns to SQLite; and a terminal prints every database
row beside separately produced flight telemetry. The UDP alternative sends
only two big-endian 32-bit floating-point values for altitude and speed. No
received bytes are selected by message identity or interpreted with database
rules.

The project demonstrates a small, understandable document-to-database pipeline
and separation between telemetry producer and display. It supplies no field
offsets, word headers, parity, packing, assembly, DFI/DUI, scaling rules,
revision handling, semantic decoder, or encoder. Calling it dynamic
MIL-STD-6016 decoding is materially inaccurate.

The licensing situation changed at the analyzed head. Current and future work
is PolyForm Noncommercial and is unsuitable for TelemetryWorks commercial use
without a separate license. The project's own NOTICE expressly preserves all
earlier commits under Apache-2.0. The last Apache commit contains the same
functional implementation; the head commit only changed licensing documents,
README/CITATION metadata, and added three-line PolyForm headers to code and
tests. An Apache snapshot is therefore legally more reusable, but its technical
content is too shallow and internally inconsistent to justify adoption.

## Method and pinned artifacts

The study first reviewed the current license and its transition commit. It then
cloned the last explicitly Apache-licensed parent into a temporary directory,
inspected every tracked text/code file and the bundled SQLite database, traced
history and public CI, attempted the documented focused tests, reproduced the
ETL pipeline with its own scripts, queried both generated and bundled databases,
and syntax-checked every Python file.

Key Git objects at the analyzed head:

| Artifact | Git blob object |
|---|---|
| `LICENSE` | `dc59a886284b1c6ab23197b3e78548c0d1f0872c` |
| `NOTICE` | `a0f8b971012f762d282fa8e31e0dc3a6c66c3e34` |
| `mock_cmn4_spec.txt` | `16b85e00169d87cc2b91cb74cb58329e611710c2` |
| `cmn4_interface_control.json` | `71ee47a2a73afb188b72febf930f429fb2b9e464` |
| `wicked_backend.db` | `98d40c27c5fe359c5c477708f21f22018f611d6e` |
| `wicked_scraper.py` | `ca5b7d01243befd860d4191696bca5e9d67aefff` |
| `db_loader.py` | `47a7f90b15545a0838a45879b25ddd164e3b41ba` |
| `wicked_terminal.py` | `95441aaba2b28f51b7e0d854559d4d68d1bf9d58` |
| `wicked_terminal_udp.py` | `f06a9915d4ad7d48dfc8bff89c1ab753755e46f2` |
| `tests/test_icd_etl.py` | `687a9857c1bd47d61245a751f1ffacebb271ca8a` |

The mock specification, generated JSON, and bundled database have identical Git
objects in the current and Apache snapshots. Each executable source/test file
differs only by the three-line PolyForm copyright/SPDX header added at head.

## Actual system behavior

The repository contains two independent demonstration paths.

### JSON and database path

```text
AirSim telemetry
  -> arbitrary altitude/speed scaling
  -> live_mids_stream.json
  -> terminal displays telemetry

mock_cmn4_spec.txt
  -> regex extracts four catalog columns
  -> cmn4_interface_control.json
  -> SQLite tactical_rules table
  -> terminal lists every row beside the telemetry
```

The two branches meet only in the terminal display. Telemetry does not carry a
J-message identifier, the terminal does not select a row for a packet, and no
field in the telemetry is extracted according to a database rule.

### UDP path

```text
AirSim position/velocity
  -> arbitrary scale factors
  -> two network-order float32 values (8 bytes)
  -> localhost UDP port 5005
  -> unpack the same two floats
  -> display altitude and speed
```

This path does not use the mock specification, JSON ICD, SQLite, J-series
identifiers, 75-bit words, or the other terminal. The source itself calls the
format `WICKED-BIT-STREAM`; it is an application-specific demo datagram.

## What the “ICD” contains

The complete source input is six pipe-delimited lines:

| Identifier | Name | Stored bits | Category |
|---|---|---:|---|
| J0.0 | Initial Entry | 75 | Network Sync |
| J2.2 | Air PPLI | 75 | Friendly Position |
| J2.5 | Land PPLI | 75 | Ground Status |
| J3.2 | Air Track | 75 | Surveillance |
| J3.5 | Land Point | 75 | Ground Track |
| J28.2 | Free Text | 75 | Information Mgmt |

Each line is a message-catalog entry, despite the parser and tests calling it a
field. There are no child fields, bit offsets, widths below the whole word,
types suitable for decoding, enumerations, units, scales, special values,
dependencies, continuation forms, or source locators.

The README advertises four supported types—J0.0, J2.2, J3.2, and J28.2—while
the data and terminal contain six. “Support” means only that a row can be
parsed, stored, and printed.

Five names broadly agree with the public catalog lineage reviewed earlier.
J28.2 does not: AirSim calls it `Free Text`, while the Wireshark/Ersatz tables
identify J28.2 as a national-use message (`U.S. National 3 (Air Force)`). None
of these public projects is normative, but the unexplained conflict prevents
adopting the AirSim label.

## ETL assessment

The regex accepts any text as an identifier/name/category if it fits the pipe
shape and accepts any decimal digit sequence for `bits`. It does not validate:

- J-message identifier syntax or label/sublabel range;
- duplicate identifiers;
- allowed keys or categories;
- bit-length limits or consistency;
- character encoding;
- source identity, revision, page/table locator, or digest; or
- field ranges, overlap, dependencies, and references.

Malformed nonmatching lines are silently skipped. A syntactically matching but
semantically invalid line is accepted. The scraper reads and writes relative to
the current working directory, while the loader and terminal resolve files
relative to their script directory. Running the documented commands from the
repository root happens to align those locations; invoking the scraper from
elsewhere does not.

The loader parameterizes SQL values, which avoids direct SQL injection through
catalog strings. It nevertheless drops and recreates its table on every run,
defines no primary key, uniqueness, nullability, range, source, or revision
constraint, and catches all exceptions by printing an error rather than
returning a failure status.

This is a demonstration parser for one invented, rigid line format. It is not a
general ICD ingestion mechanism and provides no foundation for parsing PDFs,
tables, revision deltas, or controlled-standard prose.

## Database and terminal defects

The repository includes `wicked_backend.db`, but the current loader and terminal
both use `wicked_tactical.db`. The bundled database contains table `icd_rules`
with column `data_type`; the code expects table `tactical_rules` with column
`type`. Consequently the bundled database is unused and incompatible with both
the loader and terminal.

Running the loader produces a new compatible database with six rows. The
terminal then queries only `name`, `bits`, and `type`, omitting the identifier,
and prints all six rows on every refresh. It performs no lookup against the
current telemetry.

Additional robustness limitations:

- the JSON producer rewrites its live file non-atomically;
- the terminal suppresses every JSON/read/display exception and silently skips
  the frame;
- required JSON keys and numeric values have no schema validation;
- the terminal loops forever and has no testable one-shot interface;
- the UDP receiver asks for exactly eight bytes and has no handling for short,
  malformed, reordered, duplicated, or unauthenticated datagrams;
- the UDP format has no version, message type, length, sequence, source,
  timestamp, checksum, or integrity protection; and
- neither path places resource bounds on input document or database size.

The README's “bulletproof handshaking” and `taskkill` claims are not present in
the pinned Python source. Teardown occurs only on the expected keyboard
interrupt path, not in a `finally` block.

## Decoder, encoder, and revision coverage

| Capability | Finding |
|---|---|
| 70-bit message data | Not represented separately |
| Five parity bits | Not represented or checked |
| Five transport-padding bits | Not represented |
| Initial/extension/continuation words | Not parsed or assembled |
| Label/sublabel extraction | No binary extraction; identifiers are strings in mock rows |
| MLI and continuation label | Absent |
| Tactical field extraction | Absent |
| DFI/DUI | Absent |
| Encoding | No J-series encoder |
| Decoding | No J-series decoder |
| SIMPLE/SISO-J/JREAP | Absent |
| Revision model | Absent |
| Public field provenance | Absent |
| Output provenance | Absent |

The value `75` is only catalog metadata. Nothing consumes 75 bits, distinguishes
70 data bits from parity, or validates an input word. There is no basis for any
MIL-STD-6016 revision or conformance claim.

## Reproduction and tests

The Apache snapshot declares four pytest functions:

1. parse the six-line mock file and require a four-identifier subset;
2. skip three deliberately malformed lines;
3. run mock text -> JSON -> SQLite and assert all stored sizes equal 75; and
4. leave no database when the JSON input is missing.

The local environment lacked pytest, so the exact suite was not executed and no
dependency was installed. The following permitted checks were performed on the
Apache snapshot instead:

| Check | Result |
|---|---|
| Python syntax compilation of all scripts and the test file | Passed |
| `wicked_scraper.py` on the pinned mock file | Produced six JSON objects |
| `db_loader.py` on that JSON | Produced six `tactical_rules` rows |
| Read-only query of generated database | All six rows matched the JSON values |
| Terminal rule lookup against generated database | Returned six name/size/category tuples |
| Read-only query of bundled database | Exposed incompatible `icd_rules` schema and the same six catalog values |

GitHub Actions ran the four-test suite successfully at both the last Apache
commit and the current PolyForm head. CI installs an unpinned current pytest on
Python 3.12 and does not exercise AirSim, either terminal, UDP, bundled database,
runtime teardown, or real message input. There are no tags, releases, golden
binary vectors, captures, malformed packet cases, round-trip codec cases, or
performance tests.

The scraper regenerated semantically identical JSON; on Windows, its default
text writing changed only line endings in the temporary checkout.

## Source and provenance assessment

The repository cites no public technical source for any message identity or
claim. Repository search found `MIL-STD-6016` only in project metadata and the
README's marketing description. No STANAG, SISO, paper, report, page/table,
public card, or URL is tied to the six facts.

`mock_cmn4_spec.txt` is accurately named: it is an invented fixture, not an
interface control document. `cmn4_interface_control.json` is generated from it
and adds no evidence. The SQLite database is a second serialization of the same
unsupported assertions. Agreement among those three artifacts is duplication,
not corroboration.

## License and reuse assessment

### Current head

PolyForm Noncommercial 1.0.0 allows noncommercial purposes and specified
organizations, but ordinary commercial use needs a separate license. The
current head should not be copied into or used by TelemetryWorks without legal
review and any necessary permission.

### Last Apache snapshot

The current NOTICE states that every commit before the license change remains
Apache-2.0. The parent commit contains the standard Apache 2.0 license, and the
license-change diff confirms that the only source/test changes were new
PolyForm headers. The last Apache snapshot therefore offers the same functional
behavior under a permissive open-source license, subject to Apache notice,
attribution, patent, and modification requirements.

This historical availability removes the license barrier to studying or
adapting that exact snapshot. It does not solve the technical and provenance
problems. Future upstream fixes are PolyForm unless separately licensed, so an
Apache-derived fork would not automatically receive them.

Recommended decision by asset:

| Asset | Decision |
|---|---|
| Current PolyForm source | Do not adopt for commercial use |
| Apache snapshot scripts | Legally available but technically reject; simpler independent code is preferable |
| Regex pattern | Reject; tied to a four-column mock line, not a real ICD |
| Six-row JSON/database | Reject as decoder data; unsupported catalog assertions only |
| Bundled SQLite file | Reject; dead and schema-incompatible |
| AirSim telemetry demo | Out of `jseries` core scope |
| Two-float UDP format | Reject as unrelated to J-series |
| Pipeline separation idea | Learn from independently: source ingestion, review, compilation, runtime use |

This is a technical reuse assessment, not legal advice.

## Alignment and competition with `jseries`

| Capability | AirSim-TDL-Surrogate | Current/proposed `jseries` | Relationship |
|---|---|---|---|
| Source ingestion | Regex over six mock lines | Reviewed schema compilation | Superficial overlap; AirSim lacks real source structure and validation. |
| Data storage | Four-column SQLite catalog | Compiled immutable snapshots plus provenance | AirSim is more dynamic but not semantically useful. |
| Message catalog | Six names and nominal size | No real catalog installed | AirSim has visible names, but one conflicts and none is sourced. |
| Binary decoding | Two application floats only | Synthetic checked field extraction | AirSim has no J-series decoding. |
| Encoding | None | Planned | No competition. |
| Assembly/parity | None | Planned | Shared gap. |
| DFI/DUI | None | Planned but unpopulated | Shared critical gap. |
| Revision handling | None | Explicit profile isolation, synthetic only | `jseries` architecture is stronger but unpopulated. |
| Validation | Regex shape and four narrow ETL tests | Schema bounds, overlap, dependencies, exact words | `jseries` is materially stronger. |
| Provenance | None | Fact-level public evidence planned | AirSim data is unsuitable. |
| Runtime integration | AirSim, file polling, localhost UDP, console display | Transport-neutral core planned | AirSim demonstrates a toy application shell. |
| Licensing | Current noncommercial; same behavior historically Apache-2.0 | Commercially reusable posture required | Historical code is available, but adoption has little value. |

AirSim-TDL-Surrogate competes only in presentation: it looks like a dynamic
tactical decoder demonstration. At the functional level it is a flight
telemetry dashboard plus a disconnected message-name catalog. It does not
overlap the core decoder role proposed for `jseries`.

## Decision

**Do not adopt the code, data, database, or protocol. Preserve only the general
lesson that evidence ingestion, normalized review data, compiled/runtime
storage, transport adapters, and presentation should be separate stages.**

If `jseries` eventually needs public-source ingestion, it should:

- accept a defined, versioned intermediate schema rather than scrape arbitrary
  prose directly into production tables;
- attach source URI, artifact digest, license, locator, extraction method,
  reviewer, and confidence to every fact;
- reject malformed or duplicate facts instead of silently skipping them;
- validate identities, bit ranges, overlaps, references, and revision scope;
- produce deterministic artifacts and fail with machine-readable diagnostics;
- keep unreviewed extraction output outside runtime decoder packages; and
- test against independently established expected values.

## Follow-up

All four individual project studies are complete. The next roadmap task is the
cross-project synthesis in `docs/research/05-public-ecosystem-synthesis.md`.
That report should decide which parts of the starter architecture to retain,
remove, or redesign before any public message implementation begins.
