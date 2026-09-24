#!/usr/bin/env python3
"""Run schema/Python checks; --rust also requires and runs the Rust test suite."""
from pathlib import Path
import argparse
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
def main() -> int:
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rust",action="store_true")
    args=parser.parse_args()
    commands = [
        [sys.executable,"tools/schema_compile.py","--check"],
        [sys.executable,"tools/check_traceability.py"],
        [sys.executable,"-m","unittest","discover","-s","tests","-v"],
    ]
    for command in commands:
        completed = subprocess.run(command,cwd=ROOT,check=False)
        if completed.returncode:
            return completed.returncode
    if not args.rust:
        print("Rust compile/test NOT performed by this command; use --rust on a provisioned runner.")
        return 0
    if shutil.which("cargo") is None or shutil.which("rustc") is None:
        print("ERROR: --rust requires cargo and rustc already installed and on PATH.",file=sys.stderr)
        return 1
    for command in [["rustc","--version"],["cargo","--version"],
                    ["cargo","test","--workspace","--locked","--offline"],
                    ["cargo","test","--workspace","--locked","--offline","--release"]]:
        completed=subprocess.run(command,cwd=ROOT,check=False)
        if completed.returncode: return completed.returncode
    return 0
if __name__=="__main__":
    raise SystemExit(main())
