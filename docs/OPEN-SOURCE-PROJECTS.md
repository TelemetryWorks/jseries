# Public J-series project landscape

This catalog tracks public projects relevant to `jseries` and gives each one a
short, qualified summary. A public repository is not necessarily open source:
the license classification below is part of the technical evaluation because
it controls whether code, schemas, and tests may be reused.

Deep analyses are performed sequentially according to `docs/ROADMAP.md`. Their
results live under `docs/research/`; this catalog remains the concise index.

| Project | License posture | Primary contribution | Study status |
|---|---|---|---|
| [Ersatz-MIL-STD-6016](https://github.com/liotier/Ersatz-MIL-STD-6016) | Unlicense; open source | Public reconstruction and browsable J-message catalog | [Analyzed](research/01-ersatz-mil-std-6016.md) |
| [Wireshark](https://github.com/wireshark/wireshark) | GPL-2.0-or-later for relevant dissectors; open source | Link 16 word identification plus SIMPLE capture framing | [Analyzed](research/02-wireshark-link16.md) |
| [SENTINEL](https://github.com/bwiemz/sentinel) | Custom view/study-only terms; not open source | Toy Python J-series encode/decode inside a sensor simulation | [Analyzed](research/03-sentinel-datalink.md) |
| [AirSim-TDL-Surrogate](https://github.com/MichaelFowler1/AirSim-TDL-Surrogate) | PolyForm Noncommercial 1.0.0; source-available | Mock-ICD ETL, SQLite lookup, and simulated tactical display | Planned |

License descriptions are planning summaries. Every deep analysis pins and
reviews the license text at the exact upstream commit before recommending use.

## Ersatz-MIL-STD-6016

Ersatz is the closest public project to the data/schema side of `jseries`. It
collects public Link 16 material into a JSON file and an interactive HTML
reference covering physical and link-layer facts, NPGs, and 78 J-message
identifiers. It explicitly says that it is not a substitute for MIL-STD-6016 or
STANAG 5516.

Its message catalog is much broader than its decodable detail. Most messages
are name-and-description stubs, field positions are largely absent, and the
project has no DFI/DUI dictionary, revision model, decoder, encoder, or tests.
Its best use for `jseries` is as a pinned public evidence lead and catalog to
corroborate fact by fact—not as an authoritative schema imported wholesale.
See the [completed deep analysis](research/01-ersatz-mil-std-6016.md).

## Wireshark

Wireshark's compact Link 16 dissector identifies initial, extension, and
continuation words and extracts five structural fields. Its SIMPLE dissector
validates a simulation envelope, exposes transport metadata, splits fixed-format
payloads into 10-byte words, and carries label/sublabel state between words.
Its 85 message identities are names only: it has no tactical field layouts,
DFI/DUI semantics, message assembly, encoder, or revision model.

This overlaps with `jseries` at structural recognition and message-name lookup,
while Wireshark's capture integration is far more mature. The recommended use
is as a pinned external structural oracle and an adapter-design reference. Do
not copy its GPL implementation or treat its secondary-source tables as
normative data. See the [completed deep analysis](research/02-wireshark-link16.md).

## SENTINEL

SENTINEL is a large Python educational/research sensor and tracking simulation.
Its data-link package implements immutable data classes, bit packing,
bidirectional codecs and adapters, numeric validation, track-number allocation,
an in-memory gateway, diagnostics, and 205 focused test functions.

Those are toy packets, not J-series words. All four advertised identifiers have
meanings that conflict with the earlier public project mappings, packet sizes
are bespoke, one declared size contradicts its encoder, and no field has source
or revision provenance. The restrictive root license also conflicts with an
`MIT` string in package metadata; direct reuse is excluded. The useful output
is an independently described end-to-end architecture lesson, not code or
message data. See the [completed deep analysis](research/03-sentinel-datalink.md).

## AirSim-TDL-Surrogate

AirSim-TDL-Surrogate combines simulated flight telemetry with a regex-driven
mock interface-control-document parser, SQLite rule storage, network scripts,
and a terminal display. It claims J0.0, J2.2, J3.2, and J28.2 support, but the
deep study must determine whether that means binary message decoding, telemetry
mapping, database lookup, or presentation behavior.

The repository uses the PolyForm Noncommercial 1.0.0 license, so it is
source-available rather than open source for a commercial organization. Its
potential value lies in ETL and integration ideas, subject to provenance,
technical, and license review—not direct adoption.

## Adding candidates

Add a project here only after recording its canonical repository, current
license posture, claimed capability, and reason for relevance. Add its deep
study to the sequential roadmap before relying on its output. Do not describe a
project as open source merely because its source can be viewed online.
