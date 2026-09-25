# Continuous integration

GitHub Actions provides three complementary workflows:

- `CI` runs Taplo, the repository TOML and schema checks, rustfmt, Clippy,
  traceability, debug and release tests, benchmark compilation, cross-platform
  tests, and the declared MSRV test.
- `CodeQL` analyzes Rust changes on pull requests, pushes to `main`, manual
  runs, and a weekly schedule.
- `SonarCloud` generates LCOV coverage with `cargo-llvm-cov`, uploads the
  report as a 14-day workflow artifact, analyzes the repository, and waits for
  the SonarCloud quality gate.

The SonarCloud workflow requires these repository secrets:

- `SONAR_TOKEN`: a SonarCloud analysis token;
- `SONAR_PROJECT_KEY`: the SonarCloud project key; and
- `SONAR_ORGANIZATION`: the SonarCloud organization key.

The workflow fails with only the missing secret's name when configuration is
incomplete; it never prints secret values. It skips pull requests from forks
and Dependabot because GitHub does not provide repository secrets to those
runs. The workflow carries the non-secret analysis scope and coverage settings
next to the scanner invocation.
The SonarCloud project must use CI-based analysis with Automatic Analysis
disabled so the uploaded LCOV report is the coverage source.

Run the added checks locally with:

```text
taplo fmt --check
taplo check
cargo llvm-cov --workspace --all-features --locked --lcov --output-path lcov.info
```

CI pins Taplo CLI 0.10.0 and cargo-llvm-cov 0.8.7. Local installations should
use the same versions when reproducing a workflow result. Third-party actions
are pinned to immutable commit SHAs with their human-readable version or branch
recorded in comments.
