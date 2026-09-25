# ADR-0008: GitHub Actions and reproducible local checks

Status: Accepted

Use GitHub Actions for hosted CI and `tools/check.py` as the matching local gate. Required checks cover formatting, Clippy, requirement traceability, locked debug/release tests, Windows/Linux execution, and Rust 1.86 MSRV. The workspace uses Rust edition 2024 and third-party crates are locked in `Cargo.lock`.

Offline execution is supported after dependencies and the toolchain are provisioned; offline flags are not an installer. Generated workflow evidence is retained as run artifacts under the evidence plan rather than copied forward as a timeless passing status.
