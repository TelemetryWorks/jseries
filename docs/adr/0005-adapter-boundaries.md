# ADR-0005: Keep capture packing and message semantics separate

Status: Proposed; reflected in increment 0.1

## Context

The actual capture/interface format is not established. A logical integer does not identify byte order, padding, or transport framing.

## Decision

Keep recording readers, interface adapters, normalizers, assemblers, field interpretation, and output adapters distinct. The current library starts at a checked normalized integer.

## Consequences

Real captures require an authoritative interface description and independent packing vectors. The existing toy CLI is not a capture parser.
