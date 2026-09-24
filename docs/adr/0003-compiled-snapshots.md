# ADR-0003: Generate immutable runtime tables from reviewed sources

Status: Proposed; reflected in increment 0.1

## Context

Runtime field decoding should not parse JSON or resolve document changes repeatedly. Offline operation should avoid unnecessary dependencies.

## Decision

Use Python standard-library tooling to validate toy JSON and generate Rust tables before building. Embed exact source-byte SHA-256 and verify generated-code freshness.

## Consequences

Schema edits require regeneration and rebuild in this increment. Dynamic signed schema loading remains future work. Generator identity and build identity must be included in production provenance.
