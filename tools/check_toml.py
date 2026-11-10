#!/usr/bin/env python3
"""Parse every repository TOML document with Python's standards-based parser."""

import sys
from pathlib import Path

import tomli

ROOT = Path(__file__).resolve().parents[1]
EXCLUDED_PARTS = {".git", ".venv", "dist", "dist-test", "target"}


def main() -> int:
    """Parse all repository TOML outside generated and VCS directories."""
    paths = sorted(
        path
        for path in ROOT.rglob("*.toml")
        if not EXCLUDED_PARTS.intersection(path.relative_to(ROOT).parts)
    )
    failures = []
    for path in paths:
        try:
            with path.open("rb") as handle:
                tomli.load(handle)
        except (OSError, tomli.TOMLDecodeError) as error:
            failures.append(f"{path.relative_to(ROOT)}: {error}")
    if failures:
        print("TOML validation failed:\n" + "\n".join(failures), file=sys.stderr)
        return 1
    print(f"TOML valid: {len(paths)} files parsed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
