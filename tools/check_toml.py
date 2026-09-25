#!/usr/bin/env python3
"""Parse every repository TOML document with Python's standards-based parser."""
from pathlib import Path
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[1]
EXCLUDED_PARTS = {".git", "target"}

def main() -> int:
    paths = sorted(
        path for path in ROOT.rglob("*.toml")
        if not EXCLUDED_PARTS.intersection(path.relative_to(ROOT).parts)
    )
    failures = []
    for path in paths:
        try:
            with path.open("rb") as handle:
                tomllib.load(handle)
        except (OSError, tomllib.TOMLDecodeError) as error:
            failures.append(f"{path.relative_to(ROOT)}: {error}")
    if failures:
        print("TOML validation failed:\n" + "\n".join(failures), file=sys.stderr)
        return 1
    print(f"TOML valid: {len(paths)} files parsed.")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
