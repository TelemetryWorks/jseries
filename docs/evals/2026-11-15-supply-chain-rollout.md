# Supply-chain rollout — 2026-11-15

Scope: jseries workflows/docs and the assigned Dependabot configuration.
Base main: `9bb0506f53fa9c6a29312534ad22643f7fad4e05`.
No runtime, manifest, lockfile, release, package publishing, or repository/Sonar
settings change. Existing wheel builds remain test artifacts, not publication.

## Reviewed references and boundaries

Scorecard and zizmor workflows are copied byte for byte from the
[reviewed irig106-types pilot](https://github.com/TelemetryWorks/irig106-types/tree/2899b30e338358eb2a52d12e036ff979bc25bcf5/.github).
The [irig106-tmats rollout](https://github.com/TelemetryWorks/irig106-tmats/blob/2c99e47/docs/evals/2026-11-15-supply-chain-rollout.md)
is the implementation reference. The official Scorecard template and pinned
Scorecard/zizmor/Rust action input contracts were inspected via upstream APIs.

- Scorecard runs on main pushes and weekly, never PRs. Workflow permissions are
  `contents: read`; only its analysis job grants `security-events: write` for
  SARIF and `id-token: write` for Scorecard results. Security-result publication
  is part of the exact template, not package publication. SARIF retention is five
  days; the README badge targets jseries's public Scorecard view.
- zizmor audits the entire workflow directory on workflow-changing PRs to main,
  version 1.32.0, default regular persona and online audits. Failing audit mode
  (`advanced-security: false`) requires no new SARIF permissions.
- All seven workflows default to `contents: read`; every checkout disables
  credential persistence. No `pull_request_target` event is present. Existing
  job names, triggers, commands, matrix, tool versions and MSRV 1.86 are retained.
  Rust action implementation is pinned to its verified `v1` commit with existing
  explicit `stable`/`1.86.0` selections and components unchanged.
- Sonar's same-repository/Dependabot guard, SONAR_TOKEN use, read-only fixed-date
  preflight, scanner action, complete scanner arguments, coverage commands,
  exclusions, thresholds, wait/timeout and history documentation are preserved.
  Only action pin/comment hardening and checkout credential persistence change.
  The configured cutoff remains separate from Cloud's effective history anchor;
  no settings write, manual analysis, or baseline advance is performed.
- Dependabot follows the pilot's weekly/five-PR/grouped-minor-patch policy and
  `chore(deps)` prefix. Cargo directories enumerate the root and all four workspace
  members, including the PyO3 crate. The assigned pip entry targets `/python`'s
  PEP 621 `pyproject.toml`; it does not claim uv lockfile refresh support.
  [GitHub documents pip's PEP 621 support and uv as a separate ecosystem](https://docs.github.com/en/code-security/reference/supply-chain-security/supported-ecosystems-and-repositories).
  Future Python updates must keep manifest/lock consistency and pass the existing
  locked checks; no dependencies or lockfile are changed in this rollout.

## Direct-action inventory and upstream verification

GitHub upstream tag APIs resolved every repository/version below to its commit,
peeling annotated tags when necessary. Every direct `uses` has a full SHA and
matching version comment. Existing checkouts stay on v6; exact pilot templates
use v7.0.1. Existing taiki SHA is retained with its precise v2.87.20 comment.

| Repository | Version comment | Verified commit |
| --- | --- | --- |
| actions/checkout | v6 | `d23441a48e516b6c34aea4fa41551a30e30af803` |
| actions/checkout | v7.0.1 | `3d3c42e5aac5ba805825da76410c181273ba90b1` |
| actions/setup-python | v6 | `ece7cb06caefa5fff74198d8649806c4678c61a1` |
| actions/upload-artifact | v4 | `ea165f8d65b6e75b540449e92b4886f43607fa02` |
| actions/upload-artifact | v7.0.2 | `cf430e030ddbb5b0abf93d22962f4752f3646cd9` |
| github/codeql-action | v4 | `33d7b85da1a04171e211befe2c739303b9a4c066` |
| github/codeql-action | v4.39.3 | `33d7b85da1a04171e211befe2c739303b9a4c066` |
| dtolnay/rust-toolchain | v1 | `683b588c487a69d01f6c6ddf40f036ff682cc2c8` |
| astral-sh/setup-uv | v10.2.0 | `c18668ad3cf93ea998bef934396af7bb5c839dc7` |
| PyO3/maturin-action | v1.51.0 | `e83996d129638aa358a18fbd1dfb82f0b0fb5d3b` |
| SonarSource/sonarqube-scan-action | v8.2.1 | `22918119ff8e1ca75a623e15c8296b6ea4fbe28f` |
| ossf/scorecard-action | v2.5.0 | `fa7117db83c52d787ae5a89f350999701505d78e` |
| zizmorcore/zizmor-action | v0.8.1 | `45d7276c9ca790aabd705e31ca99c6185c3e76dd` |
| taiki-e/install-action | v2.87.20 | `9983c65e42da123ff25d1f78505eb6de315aa172` |

## Findings and validation

The initial offline audit found unpinned actions and persistent checkout
credentials. This rollout fixes both without repository suppressions: there are
no inline ignores, audit-config ignores, severity filters, or persona changes.

Executed on the aarch64 host:

- actionlint 1.7.12 passes all seven workflows. Optional shellcheck/pyflakes
  integrations were disabled because those tools are not installed.
- zizmor 1.32.0 passes offline and authenticated online regular-persona audits
  of all seven workflows, with zero unsuppressed findings. Its 25 omitted findings
  are built-in regular-persona behavior, not repository ignores. The existing CLI
  session supplied online access without displaying credentials.
- A supplemental explicit Dependabot audit parses the configuration but reports
  three `dependabot-cooldown` advisories (one per ecosystem entry). The reviewed
  pilot has no cooldown; this rollout retains that behavior without ignores.
  This supplemental result is not claimed as clean. The assigned zizmor workflow
  audits workflows, exactly as in the pilot.
- Structured YAML comparison against the base confirms all five existing
  workflows are identical after normalizing action references and removing the
  new checkout credential-persistence setting. This includes Sonar's complete
  trigger/guard/env/permission/step/input/command/scanner configuration.
- Verified exact template bytes, every direct SHA/version pair, workflow
  permission defaults, checkout inputs, absence of PR-target events, every tracked
  Cargo manifest directory, pip path, weekly/five-PR/grouping/prefix policy,
  unchanged manifest/lock files, LF and `git diff --check`.

These are executed static workflow checks, not a Rust/Python build/test or hosted
CI PASS claim. No runtime code changed or runtime dependencies were provisioned.
Normal PR checks still need to evaluate the review head. Scorecard runs only on
main pushes and schedule: its first score, OIDC issuance, SARIF upload and badge
value are unverified here. No merge, settings change or manual analysis was
performed. Freeze after reporting the PR and exact head.
