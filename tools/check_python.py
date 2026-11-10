#!/usr/bin/env python3
"""Syntax-check repository Python without creating bytecode in package sources."""

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXCLUDED_PARTS = {".venv", "__pycache__", "dist", "dist-test", "target"}


def main() -> int:
    """Compile every tracked Python source in memory."""
    paths = sorted(
        path
        for path in (ROOT / "python").glob("**/*.py")
        if not EXCLUDED_PARTS.intersection(path.relative_to(ROOT).parts)
    )
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
        raise SystemExit(1) from error
