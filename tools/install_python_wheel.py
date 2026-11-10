#!/usr/bin/env python3
"""Install exactly one locally built jseries wheel without resolving dependencies."""

import subprocess
import sys
from pathlib import Path

EXPECTED_ARGUMENT_COUNT = 2


def main() -> int:
    """Install the sole wheel found in the requested directory."""
    if len(sys.argv) != EXPECTED_ARGUMENT_COUNT:
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
            "uv",
            "pip",
            "install",
            "--python",
            sys.executable,
            "--no-deps",
            "--no-index",
            str(wheels[0]),
        ],
        check=True,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
