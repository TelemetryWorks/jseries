# Performance policy

High throughput is a design constraint, but performance claims require repeatable measurements.

The cold path may read TOML, validate bounds, resolve references, sort catalogs, and build immutable decode plans. The hot path may normalize fixed-size inputs, perform one message lookup, extract precomputed ranges, evaluate indexed dependencies, and binary-search sorted code tables. It must not read files, parse text, resolve catalog names, or take global locks.

Criterion contains single-word latency and 1,024-message batch-throughput cases:

```text
cargo bench -p jseries-core --bench decode --locked
```

The installed-wheel benchmark separately measures the return path and the path
that accesses every decoded field. Together they expose Python integer
conversion, native calls, decoding, record wrapping, and lazy field-object
materialization:

```text
python python/benchmarks/decode.py --rows 10000 --iterations 7 --include-single
```

These answer different questions. Criterion isolates the Rust engine. The Python
benchmark reveals boundary and object-materialization costs. Capture parsing,
I/O, and output serialization require separate benchmarks so improvements cannot
be attributed to the wrong layer.

For profiles, build with symbols and profile a representative batch rather than
a microsecond single call. Use `cargo flamegraph` or `samply` for native stacks
on Linux, Windows Performance Recorder/Analyzer on Windows, and `py-spy` for the
Python caller. A combined native profiler is required when time attributed to
the extension must be resolved below the Python call frame.

Performance changes must report commit, CPU, operating system, Python version,
Rust version, release profile, schema shape, words per message, fields per
message, batch size, and whether outputs were consumed. Compare medians and
distributions, not one run. Shared GitHub runners can detect gross regressions
but should not enforce narrow thresholds; stable regression gates require a
pinned or self-hosted runner.

The `Performance` GitHub Actions workflow runs both harnesses when core or
Python performance-sensitive files change and can be dispatched with a custom
row count. It retains text output and the Criterion report for 30 days. Compare
artifacts from similar hosted hardware only; promote thresholds to required
checks only after establishing a stable runner and an agreed regression budget.

The current homogeneous batch API resolves the message plan once, preserves
order, releases the Python interpreter during native work, and preallocates its
record vector. Next measurements should guide compact columnar output,
caller-owned/reused buffers, chunk sizing, and optional parallel execution.
Parallelism should only be added after one-thread scaling and memory bandwidth
are measured; otherwise it can amplify allocation costs without improving useful
throughput.
