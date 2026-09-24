# Ersatz-MIL-STD-6016 deep analysis

**Study status:** Complete<br>
**Analyzed commit:** [`8a55abe09097a2889f515a977c5f0873069907e5`](https://github.com/liotier/Ersatz-MIL-STD-6016/tree/8a55abe09097a2889f515a977c5f0873069907e5)<br>
**Commit date:** 2026-03-31<br>
**Analysis date:** 2026-09-23<br>
**Upstream license:** [Unlicense](https://github.com/liotier/Ersatz-MIL-STD-6016/blob/8a55abe09097a2889f515a977c5f0873069907e5/LICENSE)

## Conclusion

Ersatz is a useful public Link 16 reference catalog and a promising source lead,
but it is not a field-level J-series schema that `jseries` can compile into a
general decoder. It provides broad message-name coverage and useful structural
and simulation metadata. It provides almost no complete message layouts, no
DFI/DUI dictionary, no MIL-STD-6016 revision model, no executable codec, and no
test suite.

Do not import `link16-schema.json` wholesale or describe its 78 identifiers as
78 supported messages. Treat the pinned repository as an external,
non-authoritative public reconstruction. Individual facts may be adopted only
after source-level review, normalization into an explicitly public-evidence
profile, and independent tests.

## Method

The study cloned the upstream repository, pinned the commit above, inspected
every tracked file, parsed the JSON independently, checked duplicate keys and
message identities, measured field completeness and bit ranges, inspected the
published HTML limitations, reviewed the GitHub workflow and license, and read
the opening and scope portions of all four bundled PDFs.

No upstream build or test command exists. The only workflow publishes the
repository as static GitHub Pages content. Independent checks were therefore
limited to the data and documentation rather than executable behavior.

Analyzed artifact digests:

| Artifact | SHA-256 |
|---|---|
| `link16-schema.json` | `85F9CC7B40E182102F3750ED4DFD909715EFE3B90B83AEC5C40B53E5D430A060` |
| `sources/SISO-STD-002-2021.pdf` | `6B7136DA82F78978E52A8C496BF50860B3A674760DA5CCFAF17E06B3E71D2165` |
| `sources/DSTO-TN-1257.pdf` | `C8C015869565A74E61ACB0A227A73C3F0432522AE5864D25E8266B6EA4D51BCD` |
| `sources/AFIT-IP-Over-Link16.pdf` | `6AD879C768F9B0B83DCDE987333A9AF060A7F86D8D26E3D41670AAD362F3CF3C` |
| `sources/SimTecT2010-64.pdf` | `D1FB12C8B2FD4ED73765E661FE270239F2B600AE99D175C75E79211D16DABAA6` |

## What the project provides

The repository consists of a machine-readable JSON document, a large static
HTML rendering, four source PDFs, a small landing page, and a Pages deployment
workflow. There is no library or command-line program.

The JSON covers several different layers:

- physical-layer and TDMA reference facts;
- link header and generic J-word structure;
- SISO-STD-002 simulation-network header fields;
- 23 NPG entries and associations;
- 32 label categories;
- 78 label/sublabel message identifiers; and
- five terminal descriptions.

This makes it useful for browsing, terminology, message identification, NPG
context, and locating public evidence. The physical/RF material is largely
outside the transport-independent `jseries` core boundary.

## Measured schema coverage

The following values come from independent parsing of the pinned JSON, not from
the upstream summary:

| Measure | Result |
|---|---:|
| Message entries | 78 |
| Entries with no fields | 74 |
| Entries with any fields | 4 |
| Entries with every listed field located and sized | 1 (`J31.7`) |
| Total listed message fields | 39 |
| Fields with both bit offset and width | 16 |
| Fields missing offset or width | 23 |
| Fields without their own `source` value | 25 |
| Duplicate label/sublabel pairs | 0 |
| Located fields outside a 75-bit word | 0 |
| Duplicate JSON object keys | 0 |

The four entries with fields are:

| Message | Listed fields | Located fields | Assessment |
|---|---:|---:|---|
| `J2.2` Air PPLI | 13 | 4 | Only the generic initial-word header is located; tactical fields are not decodable. |
| `J2.3` Surface PPLI | 8 | 4 | Only the generic initial-word header is located; tactical fields are not decodable. |
| `J3.2` Air Track | 13 | 3 | Word format, label, and sublabel are located; tactical fields are not decodable. |
| `J31.7` No Statement | 5 | 5 | Covers bits 0–69 without gaps or overlap; this is the only complete listed layout. |

The 16 located fields have valid positive widths, stay within their declared
word, and do not overlap inside their message. That structural consistency is
useful, but it does not independently establish semantic correctness.

### Stale internal statistics

The README's headline count of 78 agrees with the actual message array and the
HTML contains 78 message entries. The JSON's own `statistics` object does not:

| Statistic | Actual array | Stored statistics |
|---|---:|---:|
| Total/known messages | 78 | 69 |
| `HIGH` | 1 | 3 |
| `MEDIUM` | 6 | 5 |
| `STUB` | 71 | 61 |

The stored confidence counts sum to 69, showing that the statistics block is
stale rather than a reliable generated invariant. The repository contains no
generator or test that would catch this drift.

## Missing decoder semantics

The schema cannot currently drive the proposed `jseries` semantic decoder:

- no DFI or DUI appears in the JSON;
- no baseline or revision is attached to a message or field;
- 75 of 78 messages have unknown word counts;
- packing mode is absent for 71 entries;
- most message fields have no offset and many have no width;
- units, scale factors, signed encodings, special values, reserved values, and
  complete enumerations are generally absent;
- extension and continuation occurrences are not modeled per message;
- conditional interpretations and cross-field dependencies are absent; and
- issuance, receipt, stateful validation, and malformed-message rules are
  absent.

The upstream HTML states these limitations candidly. It says that DFI/DUI
mapping is entirely absent, most bit offsets require the controlled standards,
packing assignments are undocumented, and detailed issuance/receipt rules and
key enumerations are unavailable.

## Source and provenance assessment

### Strongest sources

`SISO-STD-002-2021` is the strongest bundled source for simulation framing,
headers, bit-stream carriage, and generic Link 16 simulation behavior. It is
not MIL-STD-6016 and does not supply the missing tactical field dictionary.

`DSTO-TN-1257` and the SimTecT paper document the Wireshark work and public
structural dissector. The SimTecT paper explicitly says detailed J-series
content analysis was not required. These sources support framing and message
identification, not complete semantic decoding.

The AFIT thesis is a public-release study useful for throughput, networking,
and Link 16 context. It is not a normative J-message dictionary.

### Provenance weaknesses

- Citations are free-text labels rather than stable source records with URL,
  digest, page/table locator, and fact-level relationship.
- Twenty-five of 39 message fields lack their own source value and rely, at
  best, on a message-level citation.
- Some citations are vague (`Multiple sources`, `web sources`, `Web search
  inference`, `Inferred`) and cannot be independently resolved.
- Other facts cite Wikipedia, a forum, Course Hero, and a defunct Viasat card.
- The HTML bibliography lists sources that are not bundled, while the JSON does
  not contain a normalized bibliography or source identifiers.
- Confidence is asserted rather than derived from documented criteria. A
  message-level `MEDIUM` can coexist with mostly unlocated `LOW` fields.

These issues do not make the catalog useless. They mean `jseries` needs its own
fact-level provenance and conflict model rather than inheriting Ersatz's
confidence values.

## Revision support

Ersatz is unversioned with respect to MIL-STD-6016. It has one schema version
(`1.0.0`) but no D, E, F, F Change 1, G, or H profiles, no effective-date model,
and no source-to-revision mapping. A `Link 16 Version` field in the SISO header
describes carried simulation metadata; it is not a set of revision-specific
message layouts.

Ersatz therefore cannot validate the current starter's multi-baseline design or
populate any D-through-H schema. At most, it can populate a separately named
public reconstruction profile whose facts remain tied to their public sources.

## Verification and maintenance assessment

| Area | Finding |
|---|---|
| JSON syntax | Valid UTF-8 JSON; no duplicate object keys found. |
| Identity uniqueness | All 78 label/sublabel pairs are unique. |
| Located ranges | In bounds and non-overlapping for the four partial layouts. |
| Automated tests | None. |
| Schema validation | None. No JSON Schema or equivalent contract is included. |
| Build | None; repository is static content. |
| CI | GitHub Pages deployment only. |
| Examples/vectors | No executable decoder vectors or expected decoded outputs. |
| Releases/tags | No tags were present at the analyzed checkout. |
| Generated-data discipline | No generator links JSON, HTML, README counts, and statistics. |

Round-trip, malformed-input, boundary, special-value, and performance testing
are not applicable because Ersatz supplies no codec. Its data could be tested
after conversion, but such tests would validate the conversion and internal
consistency, not conformity to MIL-STD-6016.

## License and reuse assessment

The repository's software/data files are offered under the Unlicense. The four
bundled PDFs retain their own notices and must not be assumed to be covered by
that repository-level dedication. Public release or free download is not the
same as public-domain status.

In particular, the bundled SISO standard carries a SISO copyright notice and
specific no-charge distribution and formatting permissions. The DSTO report
and AFIT thesis are approved for public release but also contain attribution or
copyright notices. `jseries` should link to official sources and record their
terms rather than vendor the PDFs based only on Ersatz's README description of
them as public-domain sources.

Recommended license posture:

- the JSON may be studied and transformed under the stated Unlicense;
- retain the upstream commit and artifact digest for any derived facts;
- independently confirm the license and public locator of every underlying
  source used for a fact;
- do not copy bundled PDFs into `jseries`; and
- do not let a permissive data license elevate weak provenance into a technical
  correctness claim.

This is a technical reuse assessment, not legal advice.

## Alignment and competition with `jseries`

| Capability | Ersatz | Current/proposed `jseries` | Relationship |
|---|---|---|---|
| Public message catalog | 78 identifiers and descriptions | No real identifiers installed | Direct overlap; Ersatz is broader today. |
| Complete field layouts | One complete listed layout | Synthetic layouts only | Neither supplies a general real layout catalog. |
| DFI/DUI semantics | Absent | Engine planned; real data absent | Shared critical gap. |
| Revision handling | No MIL-STD revision profiles | Explicit D–H isolation, synthetic only | `jseries` architecture is stronger but unpopulated. |
| Decoder/encoder | None | Checked synthetic decoder; encoder absent | Complementary rather than competitive. |
| Validation | No schema validator | Bounds, overlap, dependency, and interpretation validation | `jseries` is stronger mechanically. |
| Provenance | Free-text sources and confidence | Digests and selection evidence; fact-level public provenance planned | Ersatz supplies leads; `jseries` needs a stricter model. |
| Transport/simulation | Describes SISO header and link facts | Core intentionally transport-independent | Useful adapter reference, not core functionality. |
| Human reference UI | Self-contained searchable HTML | None | Ersatz provides a useful feature `jseries` may not need to duplicate. |
| Tests and vectors | None | Synthetic Python/Rust tests | `jseries` has stronger verification infrastructure. |

Ersatz competes with a possible `jseries` schema/reference catalog, but not with
the decoder engine, Rust/Python API, validation model, provenance-bearing
results, or revision selection machinery. The projects could be complementary
if `jseries` consumes only reviewed, source-qualified public facts.

## Decision

**Use as an external evidence lead and comparison dataset. Do not import it as
an authoritative or decoder-ready schema.**

Potentially reusable after independent review:

- message label/sublabel names;
- generic initial, extension, and continuation word structure;
- SISO-STD-002 simulation header definitions;
- NPG names and associations; and
- `J31.7` as a candidate first public-evidence layout.

Reject or quarantine until stronger evidence exists:

- fields with null offset or width;
- inferred latitude/longitude resolutions and other web-search inferences;
- partial enumerations;
- facts cited only to vague or inaccessible sources;
- all implied D-through-H applicability; and
- the stale aggregate statistics.

If Ersatz data is later integrated, use a distinct identity such as
`public-reconstruction/ersatz/<commit>` rather than any `MIL-STD-6016H`-style
identity. The decoder output must retain the upstream commit, normalized fact
sources, confidence rationale, and unsupported fields.

## Follow-up after the sequential studies

Do not implement the Ersatz adapter yet. The next roadmap study is Wireshark,
which is one of Ersatz's main structural sources. Its analysis may confirm the
word-level subset and provide executable comparison captures. Integration
decisions belong in the cross-project synthesis after all four studies.
