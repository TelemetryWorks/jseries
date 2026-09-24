# SENTINEL data-link deep analysis

**Study status:** Complete<br>
**Analyzed commit:** [`6f3c21d3ceebd491234aebc44086c6086fff6867`](https://github.com/bwiemz/sentinel/tree/6f3c21d3ceebd491234aebc44086c6086fff6867)<br>
**Commit date:** 2026-04-08<br>
**Analysis date:** 2026-09-23<br>
**Upstream terms:** [Custom view-and-study-only license](https://github.com/bwiemz/sentinel/blob/6f3c21d3ceebd491234aebc44086c6086fff6867/License)

## Conclusion

SENTINEL has the most complete bidirectional workflow encountered so far: data
classes, a bit reader/writer, four handwritten codecs, validators, application
adapters, track-number allocation, a gateway, an in-memory transport, metrics,
and 205 focused test functions. That architecture is useful as a comparison
for the eventual `jseries` API.

It is not a Link 16 or J-series implementation that `jseries` can interoperate
with or use as message evidence. The repository itself calls the data-link a
toy protocol, but source comments also make stronger and unsupported STANAG and
external-C2 claims. Its four message names conflict with the earlier public
project data, its packets do not use 70-bit J-word structure, and it has no word
format, label/sublabel header, parity, extension/continuation assembly, DFI/DUI,
or revision model. One declared layout is internally inconsistent: the J3.2
data class says 72 bits and 9 bytes while the codec writes 73 meaningful bits
and returns 10 bytes.

The license permits personal viewing and study only and expressly withholds
copying, modification, distribution, and other use without written permission.
No SENTINEL code, schema, fixture, test, or constants should enter `jseries`.
Only high-level architectural lessons described independently in this report
should proceed to the final synthesis.

## Method and license-bounded scope

The study pinned the commit above and used read-only GitHub API views to inspect
the root license, README, build metadata, full `src/sentinel/datalink` package,
related core enums, default configuration, ten focused test files, Git history,
workflow definition, and public Actions status. It did not clone, install,
import, execute, modify, or redistribute SENTINEL.

That restriction is deliberate. The effective root `License` says the content
is for educational and informational viewing, permits personal study, and
requires prior written permission for any copying, modification, distribution,
publication, adaptation, or other use. GitHub consequently reports the license
as `NOASSERTION`.

There is conflicting package metadata: `pyproject.toml` says `license = {text =
"MIT"}`, but the repository contains no MIT license grant and its root license
states the opposite. The explicit root terms and README must be treated as
controlling unless the owner clarifies the conflict in writing.

Principal inspected Git objects:

| Artifact | Git blob object |
|---|---|
| `License` | `82cdd83ad80d535753c3e077dd23bc1cce4b0721` |
| `src/sentinel/datalink/j_series.py` | `a59534cee82585d0a66a4618f84ff38d23159388` |
| `src/sentinel/datalink/codec.py` | `c34d34463ec73f3afe96ab09126875f15563b457` |
| `src/sentinel/datalink/encoding.py` | `32cdc631bfe78fb32d05c443ac7310411bbbb219` |
| `src/sentinel/datalink/validator.py` | `ea752289a77bfbeffe543d345784ee9296ae9aaf` |
| `src/sentinel/datalink/adapter.py` | `c286d152657d37b60f0dbebbf2200c4d2dd9f4f4` |
| `src/sentinel/datalink/gateway.py` | `8336a27df7edb4ac0d79500d09cdebcf159ebfd3` |
| `tests/integration/test_datalink_e2e.py` | `8b88be64cbc213590d47d2ee1bc7f44e1206fabe` |

This report paraphrases observed behavior and does not reproduce protected
implementation expression. This is a technical licensing assessment, not
legal advice.

## What the data-link package provides

| Layer | Implemented behavior |
|---|---|
| Message model | Four immutable Python data classes with application metadata excluded from the wire form |
| Bit codec | MSB-first, arbitrary-width unsigned, two's-complement signed, Boolean, and byte padding operations |
| Message codecs | Handwritten encode/decode functions and high-nibble dispatch for four toy packet types |
| Validation | Per-data-class numeric range checks |
| Outbound adapter | Converts simulated tracks, IFF results, engagement state, and drops into toy messages |
| Inbound adapter | Converts decoded toy messages into dictionaries used by SENTINEL subsystems |
| Identity mapping | Maps SENTINEL IFF states to an eight-value three-bit identity enum |
| Track mapping | Thread-safe, bidirectional 13-bit number allocation with LRU eviction |
| Gateway | Rate-limited publication, inbound dispatch, buffering, counters, and configurable validation |
| Transport | Protocol abstraction plus bidirectionally connected in-memory queues |
| Configuration | Disabled by default; only `in_memory` is implemented and UDP is described as future work |
| Diagnostics | Sent, received, invalid, error, and per-content-type counters |

This is meaningful simulation functionality. It demonstrates a complete
application-facing encode/send/receive/decode workflow rather than only a
schema catalog or packet viewer.

## The wire format is bespoke

SENTINEL dispatches on a private four-bit tag in the high nibble of the first
byte. It packs message-specific fields directly after that tag and pads only to
the next byte. It does not encode the two-bit J-word format, five-bit label,
three-bit sublabel, MLI, extension or continuation words, five parity bits, or
the five-bit simulation padding described by the public sources reviewed in the
previous studies.

Measured layouts from the encoder are:

| SENTINEL class | Claimed meaning | Meaningful bits written | Encoded bytes | Assessment |
|---|---|---:|---:|---|
| `J2_2AirTrack` | Air track report | 128 | 16 | Bespoke single packet; includes explicit SENTINEL threat/environment extensions |
| `J3_2TrackManagement` | Track management | 73 | 10 | Documentation says 72 bits/9 bytes; codec and test use 73 bits padded to 80 |
| `J3_5EngagementStatus` | Engagement status | 32 | 4 | Bespoke single packet |
| `J7_0IFF` | IFF/SIF result | 96 | 12 | Bespoke single packet |

Only the 10-byte J3.2 result happens to equal the size of one SIMPLE-carried
J-word. Its first 73 bits are toy fields and its remaining seven bits are byte
padding; it is not the public 70 data + 5 parity + 5 transport-padding model.

The decoder does not require an exact buffer length. It reads the fields it
expects and leaves extra bytes or padding unexamined. Reserved fields are read
and discarded without checking that they are zero. Short data fails through
the generic bit reader; wrong private tags are rejected.

Unsigned writes silently retain only the low requested bits, while signed
writes saturate to the representable range. Message-specific encoders also
clamp or mask several values. This behavior can turn invalid application data
into a different valid-looking value if validation is disabled or bypassed.

## Identifier conflicts

All four SENTINEL meanings conflict with the message names recorded by both
earlier public projects. Ersatz largely derives these names from the same
public card and Wireshark lineage, so agreement is corroboration rather than
independent normative proof. It is nevertheless enough to show that SENTINEL
is not implementing the same public naming model.

| Identifier | SENTINEL meaning | Earlier public-project mapping | Public identifier matching SENTINEL's words |
|---|---|---|---|
| J2.2 | Air Track | Air PPLI | Air Track is J3.2 |
| J3.2 | Track Management | Air Track | Track Management is J7.0 |
| J3.5 | Engagement Status | Land Point or Track | Engagement Status is J10.2 |
| J7.0 | IFF/SIF Management | Track Management | IFF/SIF Management is J7.5 |

These are identity errors, not merely field-level differences. A SENTINEL
packet labeled J2.2 cannot safely be presented as a partial public J2.2
implementation.

## Field semantics and validation

The toy messages contain useful application concepts such as track number,
identity, latitude/longitude, altitude, speed, course, quality, IFF codes,
engagement state, and source-system extensions. The codecs include coordinate
quantization and several all-ones `no code` values.

No field has a source locator, document revision, page/table reference, or
confidence classification. Repository-wide search found only three generic
`STANAG` mentions in the data-link source, no MIL-STD or SISO citation, and no
URL. Constants described as STANAG ranges are therefore unexplained assertions.

Important internal issues include:

- J3.2's declared 72-bit/9-byte size conflicts with its 73-bit/10-byte codec.
- J7.0's Mode C field is described as 13-bit 100-foot increments, but the
  encoder writes the supplied `mode_c_alt_ft` value directly and the validator
  permits values up to 126,000; values above 13 bits are silently masked.
- J3.2 documents four action meanings but its validator accepts all eight
  three-bit values without defining meanings for four through seven.
- course validation permits 360 degrees, while encoding normalizes it to zero.
- outbound validation is applied to published tracks but not consistently to
  IFF, engagement, or track-drop publication paths.
- validation is optional for both inbound and outbound processing.
- decoding validates numeric ranges after extraction, but extraction already
  constrains most unsigned values to those ranges.
- source identifier and timestamp are not encoded, so decoded messages receive
  data-class defaults rather than wire provenance or capture time.

The project supplies no DFI/DUI dictionary, special-value provenance,
conditional field logic, issuance/receipt rules, message packing rules, or
cross-field semantic validation.

## Revision and framing support

There is no MIL-STD-6016 or STANAG 5516 revision identifier in the data model,
configuration, codecs, tests, or output. The same unversioned handwritten
layout is always used.

There is also no real Link 16 or simulation framing layer:

- no initial/extension/continuation message assembly;
- no parity calculation or checking;
- no SIMPLE, SISO-J, JREAP, pcap, radio, or terminal input;
- no byte-order or packing profile selection;
- no NPG, network, time-slot, or source-track transport metadata; and
- no UDP implementation despite the gateway's interoperability wording.

The only transport is an in-memory fan-out queue. This makes the gateway a
well-factored simulation component, not an external Link 16 interface.

## Test and maintenance assessment

The data-link change was introduced in one commit on 2026-02-10. Its commit
message claims 205 new tests and external C2 interoperability. The focused
test inventory does contain exactly 205 test functions:

| Test area | Files | Functions |
|---|---:|---:|
| Data-link unit tests | 9 | 190 |
| Data-link integration tests | 1 | 15 |
| Total | 10 | 205 |

The tests cover bit operations, round trips, quantization tolerances, declared
encoded sizes, dispatch, numeric validation, adapter mapping, track allocation,
in-memory transport, gateway rate limiting, counters, and bidirectional
loopback. They are valuable software tests for the toy protocol.

They do not contain an independently sourced known-answer bit string, external
capture, TShark comparison, revision vector, parity case, word-sequence case,
or interoperability result. The only literal malformed message found in the
integration suite is a two-zero-byte decode-error case. Encoder/decoder
agreement therefore establishes internal symmetry, not J-series correctness.

The repository's workflow defines Python 3.10-3.12 unit/integration jobs,
Linux and Windows C++ jobs, lint, formatting, and benchmarks. However, the
public Actions run for the data-link commit and the run for the pinned commit
both concluded `failure` with zero jobs exposed by the API. No successful
upstream execution of the 205 tests was available to cite. The repository has
no tags or releases.

Because the license does not grant execution or other use, this study did not
run the test suite locally. No statement in this report relies on the tests
passing.

## License and reuse decision

The repository is source-visible but not open source. Its root terms prohibit
the activities that would be needed to reuse any implementation asset. The
contradictory `MIT` package metadata is not a sufficient grant on which to base
commercial or open-source reuse.

Do not copy, port, translate, vendor, import, or derive from:

- the bit reader/writer implementation;
- message classes, field widths, constants, or enumerations;
- encoding, validation, adapter, gateway, or allocator code;
- tests, fixtures, demonstrations, or configuration; or
- documentation wording and diagrams.

General software patterns such as separating codec, validation, application
mapping, transport, and diagnostics are common architectural ideas. They may
inform an independently designed `jseries` architecture, but the implementation
must be written from this project's own requirements and public evidence, not
as a translation of SENTINEL.

## Alignment and competition with `jseries`

| Capability | SENTINEL | Current/proposed `jseries` | Relationship |
|---|---|---|---|
| Bidirectional API | Four handwritten toy encoders/decoders | Synthetic decoder only; encoder planned | SENTINEL demonstrates the fuller workflow. |
| Bit cursor | MSB-first Python reader/writer | Checked logical Rust word/range extraction | Overlapping utility, but no reuse is permitted. |
| Message structure | Arbitrary 32/73/96/128-bit packets | Public 70/75/80 distinction now documented | Incompatible models. |
| Message identifiers | Four incorrectly named public identifiers | No real catalog installed | SENTINEL data must not populate the catalog. |
| Assembly/parity | Absent | Planned | Shared functional gap. |
| Semantic fields | Numerous toy application fields | Synthetic fields only | SENTINEL appears richer but has no usable provenance. |
| DFI/DUI | Absent | Planned but unpopulated | Shared critical gap. |
| Revisions | None | Explicit profile-isolation framework, synthetic only | `jseries` architecture is stronger but unpopulated. |
| Validation | Numeric ranges with silent encoder coercion | Bounds, overlap, dependencies, and explicit checked words | `jseries` is more fail-closed. |
| Application adapter | Track/IFF/engagement conversion | Not implemented | Useful high-level product lesson. |
| Transport | In-memory simulation only | Core intentionally transport-neutral | SENTINEL illustrates a port, not real interoperability. |
| Provenance | None at field level | Fact-level public provenance planned | `jseries` requirement is materially stronger. |
| Tests | 205 focused self-consistency tests, not verified passing upstream | Synthetic Python/Rust tests | SENTINEL shows breadth; neither proves real semantics. |
| License | View/study only; metadata conflict | Must remain independently reusable | Direct adoption is excluded. |

SENTINEL competes superficially through its advertised four-message binary API
and simulated C2 workflow. Once identity, framing, provenance, and license are
considered, it is not a competing real decoder. Its real comparison value is
product architecture: converting domain objects to messages, transporting
them, receiving them, and mapping them back into application state.

## Decision

**Learn from the end-to-end separation of concerns without copying. Reject all
message definitions and interoperability claims as inputs to `jseries`.**

Ideas to reconsider independently during synthesis:

- distinct domain, wire, adapter, transport, and gateway types;
- immutable decoded message values;
- explicit outbound and inbound validation policies;
- bounded bidirectional identity/track-number mapping;
- diagnostic counters for invalid, skipped, encode-error, and decode-error
  paths; and
- end-to-end tests that exercise application conversion as well as the codec.

Improvements required in a `jseries` design:

- never label a synthetic format with a real J-message identifier;
- fail rather than silently truncate, mask, or saturate invalid inputs;
- require exact input length and validate reserved/padding bits;
- keep capture time and source provenance outside message payload semantics;
- distinguish logical word, parity, and transport padding;
- base every installed field on a public source locator; and
- use independent known-answer evidence, not only round-trip tests.

## Follow-up after the sequential studies

Do not implement a SENTINEL-derived codec or gateway. The next roadmap study is
AirSim-TDL-Surrogate, which must likewise be treated as source-available rather
than open source. Cross-project architectural decisions remain deferred until
that final individual study is complete.
