# Security boundaries and controls

This is a prototype threat/verification plan, not a completed certification,
compliance assessment, FIPS claim, or assertion that all malformed inputs are safe.

| Boundary or risk | Current behavior | Remaining gate |
|---|---|---|
| Wrong revision | Exact identity selection, evidence text, no fallback | Trusted source assignment and conflict handling for real recordings |
| Synthetic used as real | Standards mode always refuses; compiler accepts synthetic only | Reviewed qualification and package-installation trust chain |
| Malformed schema | Bounded source reads, strict keys/types, overlap/cycle/code checks | Rich authoritative schema, signature checks, parser fuzzing |
| Malformed logical word | Range/high-bit/length checks; unsafe Rust forbidden | Execute Rust tests; fuzz future capture adapters and assembly |
| Bad measurement | Raw retention, special/invalid/context statuses, checked scaling | Verified standard rules and complete boundary vectors |
| Resource exhaustion | One small bounded logical word; bounded toy schema sizes | Budgets for readers, sources, assemblies, queues, and batch output |
| Source tampering | Source digests, generated-code freshness, archive inventory | Authenticated release signatures and approved provenance pipeline |
| Sensitive output | Only synthetic data bundled; no network path in the core | Explicit logging/redaction and access control for real data |
| Misleading coverage | Zero real-message coverage; unknown inventory is null | Reviewed inventory and granular qualification reports |

`SelectionEvidence` and source offsets are caller-supplied assertions, not
identity authentication. The in-process `from_bundle` constructor assumes
trusted compiled code; it does not authenticate arbitrary schemas by checking
a hash string. SHA-256 identities in bundled sources come from the generation
process. A mutable `verified` flag must never become the production approval
mechanism.

A hash inventory is useful to detect changed files relative to a trusted
inventory, but it is not a signature or proof of source correctness. This
package contains neither signing keys nor a signature-verification implementation.
No crypto assurance is implied by its use of Python's hashlib.

Before adding operational captures, establish approved storage and access for
raw data, decoded outputs, logs, schema sources, and CI artifacts. Keep recording
payloads out of routine telemetry by default. Retain raw evidence through an
approved durable reference rather than leaking it into every log. Define
retention and deletion behavior per environment; none is guessed here.

Supply-chain gates for deployment should pin and record the chosen compiler,
platform build environment, source commit, schema/compiler artifacts, and
approved dependencies. Generate the organization's SBOM and provenance records
and use its signing/verification process. Those integrations are not implemented.
A dependency-free prototype does not make compilers, Python, linkers, runners,
or future schema-generation logic outside the trust boundary.
