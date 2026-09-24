# ADR-0007: Decoded events are immutable evidence; track state is downstream

Status: Proposed; reflected in increment 0.1

## Context

Consumers may want carried-forward state or reconstructed tracks. Those transformations must not be mistaken for the fields present in a received message.

## Decision

Keep stateful validation and reconstruction downstream of immutable decoded events. Preserve source events and label inferred or carried-forward values.

## Consequences

A later stateful component needs ordering, expiry, source identity, and missing-history policies. None is implemented in the initial core.
