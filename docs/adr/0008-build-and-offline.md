# ADR-0008: Use provisioned offline runners and report actual test evidence

Status: Proposed; reflected in increment 0.1

## Context

The user needs disconnected Linux and Windows operation. Rust is absent in the authoring environment, so compilation cannot be claimed.

## Decision

Use a dependency-free Rust workspace and standard-library Python tooling. Supply Linux/Windows GitLab shell-runner jobs that require local toolchains and execute locked/offline tests. Record missing execution as a gap.

## Consequences

The organization must provision and pin a compiler/linker. Python success does not prove Rust compiles, and generic source code does not prove RHEL or Windows support.
