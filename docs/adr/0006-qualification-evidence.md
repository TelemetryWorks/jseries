# ADR-0006: Qualification is an evidence state, not a coverage slogan

Status: Proposed; reflected in increment 0.1

## Context

A parser can pass its own generated tests while sharing a transcription mistake. Baseline inventory totals are not yet known.

## Decision

Track source registration, transcription, peer review, schema validation, independent vectors, and qualification separately. Distinguish complete inventory from declared subset. Null means unknown, never 0/0 coverage.

## Consequences

Qualification records and evidence will be larger than a single boolean. Release reports must name unresolved limits and cannot count synthetic tests as actual message coverage.
