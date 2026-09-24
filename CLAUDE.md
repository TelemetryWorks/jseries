# CLAUDE.md

This file provides guidance to coding agents working in this repository. It is
tool-neutral: keep project knowledge here rather than maintaining separate,
divergent instructions for individual assistants.

## Project overview

`jseries` is a new TelemetryWorks repository. Its product scope, implementation
language, architecture, and supported environments have not yet been recorded
in the repository. Do not infer those decisions from the repository name.

At present, the repository contains no implementation or committed history.
Treat this as a deliberate bootstrap phase: make foundational decisions
explicit and easy to revise, but do not add speculative structure or tooling
before it serves an agreed requirement.

As the project takes shape, replace this section with a concrete description
of what the software does, who uses it, and which boundaries or external
systems matter.

## Sources of truth

Use this order when repository information conflicts:

1. The user's current request and explicitly recorded project decisions.
2. Executable behavior and tests.
3. Project configuration and automation.
4. Maintainer documentation.
5. Comments and historical notes.

Raise consequential contradictions instead of silently choosing one. Do not
change product behavior, public interfaces, data formats, compatibility
targets, security posture, or dependency policy merely to make a check pass.

`CONVERSATION.md` currently has no content and establishes no requirements. If
it later records decisions, preserve their context and reconcile them with the
sources above rather than treating every discussion as settled policy.

## Before making changes

- Read this file and any more specific `AGENTS.md` or `CLAUDE.md` in the area
  being changed.
- Inspect the repository status and preserve unrelated user work.
- Read the relevant implementation, tests, configuration, and documentation;
  do not reason from a diff alone.
- Identify the smallest coherent change that satisfies the request.
- Ask before making a choice that establishes a hard-to-reverse public API,
  data model, platform commitment, or external service dependency when the
  requirement does not already determine it.

## Development commands

No build, test, lint, format, or run commands are defined yet. Do not present a
tool's default command as a repository convention until the corresponding
configuration is committed.

When introducing the first development workflow:

- provide one obvious setup path and one obvious verification command;
- prefer commands that behave consistently locally and in CI;
- pin or constrain tool versions where reproducibility depends on them;
- document the commands here and in the user-facing README as appropriate;
- avoid adding a dependency or framework solely for a trivial task that the
  chosen platform already handles well.

Once commands exist, replace this section with exact, runnable commands and a
short explanation of what each one verifies.

## Architecture and repository layout

No architecture or source layout is established. Let the first real use cases
shape them. Prefer clear boundaries and straightforward code over abstractions
created in anticipation of hypothetical requirements.

When a stable layout emerges, document the important paths here. Add scoped
instruction files only when a subtree genuinely needs different rules; the
nearest instruction file should refine, not duplicate or contradict, this one.

## Implementation standards

- Match conventions already established by the repository. If none exist,
  favor idiomatic, readable code for the selected language.
- Keep changes focused. Do not combine feature work with unrelated cleanup.
- Make invalid states and error paths explicit. Preserve useful diagnostic
  context without exposing secrets or sensitive data.
- Treat persisted data, serialized output, command-line behavior, and public
  APIs as compatibility surfaces once released or consumed.
- Prefer deterministic behavior in builds, tests, generated artifacts, and
  output intended for comparison or automation.
- Add dependencies deliberately. Record why a non-obvious dependency is
  needed, keep its scope narrow, and commit the ecosystem's lockfile when one
  is used.
- Never commit credentials, tokens, private keys, production data, or local
  environment files. Use documented examples with inert placeholder values.
- Do not suppress lint, type, security, or test findings without a narrow,
  documented reason.

## Testing and verification

Every behavior change should have proportionate automated coverage when the
project has a test framework. A fix should normally include a regression test
that fails for the original defect. Test observable behavior and boundary
conditions rather than implementation details.

Before handing off work:

1. Run the narrowest relevant checks while iterating.
2. Run the repository's full verification command when one exists.
3. Inspect the final diff and repository status for accidental files, secrets,
   generated output, or unrelated edits.
4. Report exactly what was verified and any check that could not be run.

Do not weaken assertions, refresh expected output, reduce coverage, or disable
a check merely to obtain a green result. If expected behavior intentionally
changes, make the resulting test or fixture diff part of the reviewable
change.

## Documentation and decisions

Documentation is part of the change when setup, behavior, an interface, or an
operator workflow changes. Use a lightweight decision record for choices with
long-lived tradeoffs, especially architecture, storage, interoperability,
security, and dependency decisions.

Keep this file concrete. Remove bootstrap language as facts become known, and
record the commands and invariants most likely to surprise a future
contributor. Avoid turning it into a generic style guide that merely repeats
formatter or linter configuration.

## Git conventions

Do **not** add `Co-Authored-By: Claude ...` trailers to commit messages on this
repo, even if the harness's default instructions suggest it. Commit messages
are the human-authored record of intent; tool attribution belongs in tool logs,
not history. This overrides the default trailer behavior.

- Do not discard, overwrite, stage, or commit unrelated user changes.
- Do not rewrite history or use destructive Git commands unless explicitly
  requested.
- Keep commits cohesive and use imperative commit subjects that explain the
  change.
- Do not create a commit unless the user asks for one.
