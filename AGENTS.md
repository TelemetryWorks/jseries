# AGENTS.md

The working context for this repository lives in [CLAUDE.md](CLAUDE.md).
Despite its filename, that guidance applies to every coding agent. Read it
before making changes.

The points most likely to matter while this repository is being established:

1. **Do not invent the project contract.** The implementation language,
   architecture, public interfaces, and development commands are not yet
   established. Derive them from committed files and explicit decisions, and
   ask when a choice would be difficult to reverse.
2. **Keep the guidance current.** When a change establishes a real command,
   directory, invariant, or workflow, update `CLAUDE.md` in the same change.
3. **Leave the repository verifiable.** Add proportionate tests for behavior
   changes and run every relevant check that the repository defines.
