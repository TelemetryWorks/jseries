# ADR-0002: Synthetic development cannot imply standards support

Status: Proposed; reflected in increment 0.1

## Context

The complete authoritative sources are not available in this workspace. Generic software can still be developed without inventing message definitions.

## Decision

Ship only SYNTH-prefixed fixtures. The schema compiler accepts synthetic sources only; standards mode refuses all bundled profiles. Keep real coverage at zero with unknown inventory totals.

## Consequences

The prototype proves framework behavior only. Installing real packages requires a separate reviewed qualification mechanism, not changing a flag in the demo data.
