# Dependabot Python compatibility — 2026-11-15

Follow-up to PR #9, base `2786ded66b63729dd0820728dccacf0ed4618dd6`.
Scope: Dependabot configuration and documentation only. Keep the manifest,
lockfile, uv required-version, CI workflows, Sonar behavior and settings intact.

## Compatibility decision

The repository requires exactly `uv==0.12.19` in `python/pyproject.toml` and CI.
[GitHub's current supported-ecosystems table](https://docs.github.com/en/code-security/reference/supply-chain-security/supported-ecosystems-and-repositories)
lists the uv ecosystem with supported version `v0.11`; it does not establish
support for this exact pin.

For additional evidence, upstream dependabot-core at
[`1a0125f11e75859a0957b4d73ea6b8656570b30e`](https://github.com/dependabot/dependabot-core/tree/1a0125f11e75859a0957b4d73ea6b8656570b30e/uv)
[builds the updater with uv 0.13.7](https://github.com/dependabot/dependabot-core/blob/1a0125f11e75859a0957b4d73ea6b8656570b30e/uv/Dockerfile#L16)
and [runs uv lock during updates](https://github.com/dependabot/dependabot-core/blob/1a0125f11e75859a0957b4d73ea6b8656570b30e/uv/lib/dependabot/uv/file_updater/lock_file_updater.rb).
This is inspected upstream source, not proof of the deployed GitHub updater image.

An isolated local probe used uv 0.13.7 and a minimal scratch `pyproject.toml`
with the repository's exact required-version. `uv lock --offline
--no-python-downloads` exited 2 before resolving dependencies:

```text
error: Required uv version `==0.12.19` does not match the running version `0.13.7`.
```

Neither the documented version nor the inspected updater can establish
compatibility with the exact pin. Do not switch to uv or change that pin on this
evidence. Reconsider when documented/deployed updater support can be verified.

## Authorized fallback and limitation

Retain pip for `/python` and add `versioning-strategy: lockfile-only`.
[GitHub documents this strategy for pip](https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference#versioning-strategy):
it limits PRs to lockfile changes and skips versions needing manifest changes.
This prevents pip's manifest-only range updates from leaving `uv.lock` stale.
Weekly cadence, five-PR limit, minor/patch grouping and commit prefix are retained;
Actions and Cargo entries are unchanged.

This is a safeguard, not automatic uv lock maintenance. pip does not update this
repository's uv lockfile; with no pip-managed lockfile, useful Python version PRs
may cease. Python security/version updates still need maintainer attention:
update manifest and uv lock together with uv 0.12.19 and run existing locked CI.
No automatic lock regeneration, vulnerability remediation, successful hosted
Dependabot job, or full runtime test execution is claimed.

Validation: the scratch incompatibility probe executed as described; structured
YAML comparison confirms only the Python entry's versioning strategy changes.
Manifest/lock/workflow bytes and the uv pin are unchanged. YAML parsing,
actionlint 1.7.12, offline regular-persona zizmor 1.32.0 workflow audit and
`git diff --check` pass. The existing three supplemental Dependabot cooldown
advisories remain outside the assigned workflow audit, without new suppressions.
