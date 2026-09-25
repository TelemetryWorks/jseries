# ADR-0003: Immutable decode plans

Status: Amended by ADR-0009

Runtime field decoding must not repeatedly parse an authoring format or resolve symbolic references. ADR-0009 replaces generated Rust tables with user-supplied TOML, while retaining this performance decision: package loading is a cold path that produces immutable validated core structures and precomputed decoder indices.

Schema changes no longer require rebuilding the application. Construction cost is paid when a package is selected; decoding uses the resolved plan.
