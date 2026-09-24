# Verification status — increment 0.1

Date: September 21, 2026.

| Check | Actual result |
|---|---|
| Python schema/fixture tests | **27 passed** |
| Synthetic schema validation | **6 source bundles passed** |
| Generated Rust freshness | **Passed**: generated data matches the six JSON sources |
| Requirement hierarchy and test mapping | **Passed**: 59 requirements; all 47 Rust test functions mapped |
| Python, JSON, Cargo TOML/lockfile syntax | **Passed** |
| GitLab YAML syntax | **Parsed**; jobs were not executed |
| Rust compilation | **Not run**: rustc and Cargo unavailable |
| Rust integration tests | **47 supplied, not run**: 38 core and 9 CLI |
| Rust formatting / lints | **Not run** |
| RHEL and Windows execution | **Not run** |
| Performance / memory benchmarks | **Not measured** |
| Real MIL-STD-6016 message coverage | **0 implemented; 0 verified** |
| Authoritative baseline inventory totals | **Unknown**, not zero |

The logs in `python-checks.txt` and `rust-preflight.txt` are actual command output
from this authoring environment. The Rust-required command returned nonzero
because a toolchain is absent; it did not silently report success or execute
Rust tests. Run `python tools/check.py --rust` on the provisioned target runners
before accepting the Rust increment.

No compiled executable, benchmark result, formal security assessment, standards
conformance certificate, or full-text standard is supplied. The package contains
source code and test specifications, not a verified operational Link 16 decoder.
