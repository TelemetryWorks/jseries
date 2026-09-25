#!/usr/bin/env python3
"""Install exactly one locally built jseries wheel without resolving dependencies."""

from pathlib import Path
import subprocess
import sys


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: install_python_wheel.py WHEEL_DIRECTORY", file=sys.stderr)
        return 2

    wheel_directory = Path(sys.argv[1]).resolve()
    wheels = sorted(wheel_directory.glob("jseries-*.whl"))
    if len(wheels) != 1:
        print(
            f"expected exactly one jseries wheel in {wheel_directory}, found {len(wheels)}",
            file=sys.stderr,
        )
        return 1

    subprocess.run(
        [
            sys.executable,
            "-m",
            "pip",
            "install",
            "--disable-pip-version-check",
            "--no-deps",
            "--no-index",
            str(wheels[0]),
        ],
        check=True,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
