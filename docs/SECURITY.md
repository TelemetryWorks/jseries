# Security boundaries and controls

This project is not a security certification or standards-conformance implementation.

| Boundary | Current control | Remaining work |
|---|---|---|
| Untrusted schema package | 1 MiB file and 16 MiB package limits; strict TOML keys/types; safe relative paths; bounded messages, words, and fields; overlap/reference/cycle/code validation | Reparse-point/symlink policy, parser fuzzing, signatures, approved installation path |
| Malformed word | Exact 70/75/80 bounds, zero-padding check, preserved parity, checked 70-bit field ranges, unsafe Rust forbidden | Adapter fuzzing, parity policy, source-byte retention |
| Misleading qualification | Source and qualification carried as explicit author metadata; no standards definitions bundled | Fact-level provenance, trust policy, reviewer workflow, signed releases |
| Resource exhaustion | Fixed `u128` word representation, 32-word assembly limit, bounded package/model sizes | Bounds for readers, concurrent assemblies, batches, and output queues |
| Arithmetic/meaning | Exact integer/rational interpretation, explicit special/invalid/not-applicable states | Independently qualified semantic vectors and broader rule model |
| Sensitive data | No operational captures bundled; core performs no network or file access | Logging/redaction, storage, access, retention, and deletion policy |
| Supply chain | Locked Cargo dependencies, GitHub workflow with read-only contents permission | Dependency review, CodeQL, SBOM/provenance, action SHA pinning, signing |

Package metadata is not authenticated proof. Do not decide that a package is authoritative from its path, label, or successful parse. Hashes can identify bytes but do not prove correctness or authorization.

Before operational captures are used, establish approved storage and access for raw data, decoded output, logs, schemas, and CI artifacts. Keep payloads out of routine telemetry and preserve raw evidence through controlled references.
