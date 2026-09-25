# Performance policy

High throughput is a design constraint, but performance claims require repeatable measurements.

The cold path may read TOML, validate bounds, resolve references, sort catalogs, and build immutable decode plans. The hot path may normalize fixed-size inputs, perform one message lookup, extract precomputed ranges, evaluate indexed dependencies, and binary-search sorted code tables. It must not read files, parse text, resolve catalog names, or take global locks.

Near-term work is to add representative single-word and multiword benchmarks, record toolchain/CPU/build metadata, establish latency and throughput baselines, and make statistically meaningful regressions visible in CI. Batch decoding and caller-owned output storage should precede claims of zero allocation. Capture I/O and output serialization must be measured separately from field decoding.
