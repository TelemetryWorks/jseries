#!/usr/bin/env python3
"""Run repository checks; Rust execution is never silently skipped."""
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]

def run(command: list[str]) -> int:
    print("+", " ".join(command), flush=True)
    return subprocess.run(command, cwd=ROOT, check=False).returncode

def main() -> int:
    commands = [
        [sys.executable, "tools/check_traceability.py"],
        ["cargo", "fmt", "--all", "--", "--check"],
        ["cargo", "clippy", "--workspace", "--all-targets", "--all-features", "--", "-D", "warnings"],
        ["cargo", "test", "--workspace", "--locked"],
        ["cargo", "test", "--workspace", "--locked", "--release"],
    ]
    for command in commands:
        try:
            code = run(command)
        except FileNotFoundError as error:
            print(f"required tool unavailable: {error.filename}", file=sys.stderr)
            return 1
        if code:
            return code
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
