# ADR-0004: Raw evidence and interpretation status are separate

Status: Proposed; reflected in increment 0.1

## Context

A defined special value, an invalid encoding, and a missing interpretation are not ordinary numeric measurements.

## Decision

Retain raw values and ranges. Evaluate applicability in dependency order, special values before ordinary conversions, and report unresolved context rather than guessing. Use exact rationals for the supported scaling subset.

## Consequences

Downstream consumers must process tagged states explicitly. The current rule subset cannot be presumed to cover the standard; new encodings require reviewed semantics and tests.
