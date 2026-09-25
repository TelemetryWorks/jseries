#!/usr/bin/env python3
"""Syntax-check repository Python without creating bytecode in package sources."""

from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    paths = sorted((ROOT / "python").glob("**/*.py"))
    paths.extend(sorted((ROOT / "tools").glob("*.py")))
    for path in paths:
        source = path.read_text(encoding="utf-8")
        compile(source, str(path), "exec")
    print(f"Python syntax valid: {len(paths)} files compiled in memory.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, SyntaxError) as error:
        print(f"Python syntax error: {error}", file=sys.stderr)
        raise SystemExit(1)
