#!/usr/bin/env python3
"""Run repository checks; Rust execution is never silently skipped."""

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def run(command: list[str]) -> int:
    """Run one repository command and return its exit status."""
    print("+", " ".join(command), flush=True)
    return subprocess.run(command, cwd=ROOT, check=False).returncode


def main() -> int:
    """Run every required local verification command in fail-fast order."""
    # Refuse to create an environment during verification.
    environment = ROOT / "python" / ".venv"
    if not (environment / "pyvenv.cfg").is_file():
        print("missing python/.venv: provision the locked tools first", file=sys.stderr)
        return 1
    python_dev = [
        "uv",
        "run",
        "--directory",
        "python",
        "--locked",
        "--no-sync",
        "--offline",
        "--no-python-downloads",
        "python",
        "-m",
    ]
    commands = [
        ["uv", "lock", "--directory", "python", "--check", "--offline", "--no-python-downloads"],
        ["taplo", "fmt", "--check"],
        ["taplo", "check"],
        [*python_dev[:-1], "../tools/check_toml.py"],
        [*python_dev[:-1], "../tools/check_traceability.py"],
        [*python_dev[:-1], "../tools/check_python.py"],
        [*python_dev, "ruff", "format", "--check", ".", "../tools"],
        [*python_dev, "ruff", "check", ".", "../tools"],
        [*python_dev, "mypy"],
        [*python_dev, "pylint", "jseries", "tests", "benchmarks", "../tools"],
        [
            *python_dev,
            "pytest",
            "--cov=jseries",
            "--cov-branch",
            "--cov-report=term-missing",
            "--cov-report=xml:coverage.xml",
        ],
        [*python_dev, "mypy.stubtest", "jseries"],
        ["cargo", "run", "--locked", "-p", "jseries-cli", "--", "schema", "validate", "schemas"],
        ["cargo", "fmt", "--all", "--", "--check"],
        [
            "cargo",
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
        ["cargo", "test", "--workspace", "--locked"],
        ["cargo", "test", "--workspace", "--locked", "--release"],
        ["cargo", "bench", "--workspace", "--no-run", "--locked"],
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
