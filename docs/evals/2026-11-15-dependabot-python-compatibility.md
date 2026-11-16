# Dependabot Python compatibility — owner decision 2026-11-15

Follow-up to PR #9, base `2786ded66b63729dd0820728dccacf0ed4618dd6`.
Scope: Dependabot configuration and documentation only. Manifest, lockfile,
uv required-version, CI workflows, Sonar behavior and settings remain unchanged.

## Decision: no Python Dependabot entry

**Dependabot creates NO Python dependency PRs for jseries.** Python dependencies
and `python/uv.lock` are maintained manually with pinned uv 0.12.19. Do not relax
that pin to enable automation. Revisit when Dependabot supports it or the pin is
deliberately moved through separate review.

Remove the pip entry entirely; keep Actions and Cargo entries unchanged.
The earlier pip plus `lockfile-only` fallback was rejected in review: pip has no
uv.lock support, and all jseries Python dependency requirements are ranges.
That combination cannot create Python version or security PRs here. Retaining
it would imply dependency coverage that does not exist.

## Source evidence

At inspected dependabot-core commit
[`1a0125f11e75859a0957b4d73ea6b8656570b30e`](https://github.com/dependabot/dependabot-core/tree/1a0125f11e75859a0957b4d73ea6b8656570b30e):

- The [pip file fetcher](https://github.com/dependabot/dependabot-core/blob/1a0125f11e75859a0957b4d73ea6b8656570b30e/python/lib/dependabot/python/file_fetcher.rb#L45)
  collects pyproject.toml with Poetry/PDM locks, not uv.lock. This repository has
  only uv.lock for its Python dependencies, with ranged requirements in
  `python/pyproject.toml`.
- [Dependabot's lockfile-only documentation](https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference#versioning-strategy)
  limits PRs to lockfile updates and skips versions requiring manifest changes.
  It does not give pip support for uv.lock.
- The [uv updater image](https://github.com/dependabot/dependabot-core/blob/1a0125f11e75859a0957b4d73ea6b8656570b30e/uv/Dockerfile#L16)
  bundles uv 0.13.7 and [runs uv lock](https://github.com/dependabot/dependabot-core/blob/1a0125f11e75859a0957b4d73ea6b8656570b30e/uv/lib/dependabot/uv/file_updater/lock_file_updater.rb).
- Its [required-version error handler](https://github.com/dependabot/dependabot-core/blob/1a0125f11e75859a0957b4d73ea6b8656570b30e/uv/lib/dependabot/uv/file_updater/lock_file_error_handler.rb#L86)
  matches `Required uv version ... does not match` and raises
  `Dependabot::ToolVersionNotSupported`. Thus that updater rejects the
  repository's exact `required-version = "==0.12.19"`.

[GitHub's supported-ecosystems table](https://docs.github.com/en/code-security/reference/supply-chain-security/supported-ecosystems-and-repositories)
also lists uv v0.11, not a compatibility guarantee for this exact pin. The
upstream code above is inspected source, not a claim about the deployed GitHub
image. No successful hosted Dependabot job is claimed.

## Executed validation

The earlier isolated uv 0.13.7 probe against a minimal scratch pyproject with
`required-version = "==0.12.19"` exited 2 before resolution:

```text
error: Required uv version `==0.12.19` does not match the running version `0.13.7`.
```

Structured YAML comparison verifies removal of the Python entry only, with
Actions/Cargo entries unchanged. Repository dependency requirements remain
ranges; manifest/lock/workflow bytes and the uv pin are unchanged. YAML parsing,
actionlint 1.7.12, offline regular-persona zizmor 1.32.0 workflow audit and
`git diff --check` pass. Supplemental Dependabot auditing reports the two
remaining pre-existing cooldown advisories (Actions/Cargo), without suppressions.
No runtime test, lock regeneration, vulnerability remediation or settings change
is claimed. Manual Python updates must synchronize manifest/lock with uv 0.12.19
and pass existing locked CI.
