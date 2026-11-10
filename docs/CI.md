# Continuous integration

GitHub Actions provides five complementary workflows:

- `CI` runs Taplo, the repository TOML and schema checks, rustfmt, Clippy,
  traceability, debug and release tests, benchmark compilation, cross-platform
  tests, and the declared MSRV test.
- `CodeQL` analyzes Rust changes on pull requests, pushes to `main`, manual
  runs, and a weekly schedule.
- `SonarCloud` generates Rust LCOV and Python branch-coverage XML, uploads both
  as a 14-day workflow artifact, analyzes the repository, and waits for the
  SonarCloud quality gate.
- `Python` installs the committed uv lockfile, enforces Ruff formatting and
  linting, Pylint, strict mypy, native-stub consistency, pytest, and Python
  branch coverage. It also builds and installs the PyO3 package on Python 3.10
  and 3.14 on Windows and Linux, then builds, installs, tests, and retains
  release abi3 wheels for 64-bit Windows and manylinux x86-64.
- `Performance` runs when performance-sensitive files change and on manual
  dispatch. It retains Criterion reports plus Rust and installed-wheel Python
  benchmark output for 30 days. Results are evidence, not a pass/fail threshold
  on variable shared runners.

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
cargo llvm-cov --workspace --all-features --exclude jseries-python --locked --lcov --output-path lcov.info
uv sync --directory python --locked --only-group dev
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m maturin develop --uv --locked
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m ruff format --check . ../tools
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m ruff check . ../tools
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m pylint jseries tests benchmarks ../tools
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m mypy
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m pytest --cov=jseries --cov-branch
uv run --directory python --locked --no-sync --offline --no-python-downloads python -m mypy.stubtest jseries
```

CI pins uv 0.12.19, Taplo CLI 0.10.0, and cargo-llvm-cov 0.8.7. Python tools
are pinned transitively by `python/uv.lock`. Local installations should use the
same versions when reproducing a workflow result. The wheel builder uses
maturin 1.15.0, matching `python/uv.lock`; update both together. Some actions use
immutable commit SHAs with readable version comments; others retain version tags.

The Python workflow runs for every pull request, including stacked tooling PRs.
Its matrix selects the interpreter provisioned by setup-python explicitly and
records console-launcher diagnostics without treating them as test results.
Installed-wheel checks force replacement, verify package/native provenance and
wheel bytes, and run isolated tests and native-stub checks. The other main-target
workflows must also pass on the complete feature PR before merging to `main`.

Rust LCOV generation excludes `jseries-python`: enabling PyO3's extension-module
link mode inside a Rust test harness is not a representative Python test. The
Python workflow instead installs and imports the real native wheels on both
supported operating-system families. SonarCloud imports `python/coverage.xml`
for the Python facade; native binding coverage remains excluded from the Rust
LCOV report because it requires execution through an embedded Python process.
