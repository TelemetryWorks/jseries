#!/usr/bin/env python3
"""Install one local wheel and verify isolated imports against its contents."""

import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path
from zipfile import ZipFile

EXPECTED_ARGUMENT_COUNT = 2
IMPORT_PROBE = """
import importlib.metadata
import json
import jseries
from jseries import _jseries

dist = importlib.metadata.distribution("jseries")
print(json.dumps({
    "package": jseries.__file__,
    "native": _jseries.__file__,
    "installed_package": str(dist.locate_file("jseries/__init__.py")),
    "version": jseries.__version__,
    "metadata_version": dist.version,
    "direct_url": json.loads(dist.read_text("direct_url.json") or "{}"),
}))
"""


def verify_installed_wheel(wheel: Path) -> None:
    """Reject editable/source imports and compare installed bytes with the wheel."""
    with tempfile.TemporaryDirectory(prefix="jseries-wheel-") as directory:
        result = subprocess.run(
            [sys.executable, "-I", "-c", IMPORT_PROBE],
            cwd=directory,
            check=True,
            capture_output=True,
            text=True,
        )
    probe = json.loads(result.stdout)
    package = Path(probe["package"]).resolve()
    native = Path(probe["native"]).resolve()
    if package != Path(probe["installed_package"]).resolve():
        raise RuntimeError(f"source-tree package fallback: {package}")
    if not package.is_relative_to(Path(sys.prefix).resolve()):
        raise RuntimeError(f"package outside target environment: {package}")
    if native.parent != package.parent:
        raise RuntimeError(f"native module outside installed package: {native}")
    if probe["direct_url"].get("dir_info", {}).get("editable", False):
        raise RuntimeError("installed distribution is editable")
    if probe["version"] != probe["metadata_version"]:
        raise RuntimeError("native version differs from wheel metadata")
    with ZipFile(wheel) as archive:
        for installed in (package, native):
            member = f"jseries/{installed.name}"
            if (
                hashlib.sha256(installed.read_bytes()).digest()
                != hashlib.sha256(archive.read(member)).digest()
            ):
                raise RuntimeError(f"installed bytes differ from selected wheel: {installed}")
    print(f"Verified wheel imports: {package}, {native}; version={probe['version']}")


def main() -> int:
    """Replace any prior installation with the sole wheel and verify provenance."""
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
            "--reinstall",
            str(wheels[0]),
        ],
        check=True,
    )
    verify_installed_wheel(wheels[0])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
