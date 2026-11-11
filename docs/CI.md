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

## Configured cutoff and evaluated new-code period

The project uses SonarQube Cloud's **Specific date** configuration, explicitly
set on `TelemetryWorks_jseries`: `sonar.leak.period=2026-09-23` and
`sonar.leak.period.type=date`. This fixed parameter is not a release or rolling
window. It was selected before repository/source dates to avoid using the
November merge date as the cutoff for September-authored feature work. The
configured parameter must not advance automatically, but it is not a guarantee
that Cloud's evaluated period starts on that date or includes the entire codebase.

### Observed main analysis

On 2026-11-11, [Sonar push run 42957341198](https://github.com/TelemetryWorks/jseries/actions/runs/42957341198)
succeeded for revision `92513b13b3010394146ba555b59d56015a266390`.
[Exact analysis `6129d4f5-2005-4a09-b978-b5d883abd26c`](https://sonarcloud.io/api/qualitygates/project_status?analysisId=6129d4f5-2005-4a09-b978-b5d883abd26c)
reported `OK`, all six conditions `OK`, and `ignoredConditions=false`.
Its period had mode `date`, parameter `2026-09-23`, and effective date
`2026-09-25T00:09:37+0000`.

That effective timestamp matches retained analysis
`9ef32886-747a-4434-8066-b836fe71f6b5`, revision
`f55812fdff6adf769f91f5698b817db1ab0f5f33`, in the project's analysis history.
This records the observed resolution to retained history; it does not assert a
universal internal selection algorithm or promise the same result for different
history/retention. Foundation code before that anchor is old code for this
period. Feature commit `650e968` begins at `2026-09-25T03:57:25Z`, after the
anchor; its later November merge does not move that earlier feature work outside
the period.

| Metric at this exact analysis | Observed value |
| --- | --- |
| New lines | 1597 |
| New lines to cover / uncovered | 81 / 0 |
| New-code coverage | 100.0% (81/81 eligible new lines covered) |
| Overall lines to cover / uncovered | 707 / 131 |
| Overall coverage | 81.5% |
| Alert status | `OK` |

New-code coverage is distinct from overall coverage. Passing the new-code gate
does not establish full coverage or reclassify all foundation code as new.
These are dated analysis results, not a timeless coverage guarantee.

Previous-version calculation requires configured version history; adding
`sonar.projectVersion` alone does not establish a new-code period. Rolling days
would age unresolved work out. Specific version lacks a meaningful historical
version here. Cloud's documented date option avoids those tradeoffs without
changing package versions. See [Cloud new-code configuration](https://docs.sonarsource.com/sonarqube-cloud/managing-your-projects/project-analysis/configuring-new-code-calculation).

The existing guarded Sonar job runs `tools/check_sonar_baseline.py` before
coverage/analysis. It makes one bounded authenticated settings GET and fails
closed on missing credentials, unreadable/malformed settings, drift, duplicates,
or inherited values. Cloud may omit `inherited` for an explicit setting: omission
and boolean false are accepted, while true and nonboolean values are rejected.
The preflight verifies configured settings only, not the resolved analysis period
or its coverage scope. Only readable project settings are required; the analysis
token need not be an administrator. No setting is written by repository
verification or analysis.
The existing same-repository/fork/Dependabot secret boundary remains unchanged.

PR analysis against its target is the primary gate; main's push analysis is the
backstop. Scanner arguments, thresholds, exclusions and `sonar.qualitygate.wait`
are unchanged. `NONE` is not a pass. Validate the exact postmerge revision using
the run log and public analysis API: record the configured parameter and actual
effective period date/history anchor separately, all six main gate conditions,
new-code measures, overall coverage, and `alert_status=OK`, preserving the exact
analysis ID and workflow run. Do not require the effective date to equal the
configured parameter or infer that all code is new. Fix measured failures rather
than moving the baseline.
Any intentional baseline change requires separate reviewed authorization.

The one-time administrative setup was performed separately in [run 42955617045](https://github.com/TelemetryWorks/jseries/actions/runs/42955617045):
its empty before-state, two HTTP 204 responses, and exact readback were independently
reviewed. The temporary setup workflow was never merged into main.

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
