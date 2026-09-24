# ADR-0001: Explicit baseline identities and no fallback

Status: Proposed; reflected in increment 0.1

## Context

Revision ambiguity can turn plausible bytes into a wrong interpretation. F and F Change 1 must remain distinguishable.

## Decision

Use D, E, F, F-C1, G, and H as exact identities. Bind each decoder to one immutable bundle. Require selection evidence and never fall back to a later revision.

## Consequences

Callers must resolve source/profile assignment explicitly. A comparison mode, if added later, cannot silently choose an operational baseline.
