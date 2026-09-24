#!/usr/bin/env python3
"""Capture the actual runner's validation log; no stored session log is reused."""
from pathlib import Path
import platform
import subprocess
import sys

ROOT=Path(__file__).resolve().parents[1]
def main()->int:
    out=ROOT/"ci-evidence"
    out.mkdir(exist_ok=True)
    (out/"environment.txt").write_text(
        f"platform={platform.platform()}\npython={sys.version}\n",encoding="utf-8")
    log=out/"verification.txt"
    with log.open("w",encoding="utf-8") as handle:
        proc=subprocess.run([sys.executable,"tools/check.py","--rust"],cwd=ROOT,
            stdout=handle,stderr=subprocess.STDOUT,check=False)
    print(log.read_text(encoding="utf-8"))
    return proc.returncode
if __name__=="__main__":
    raise SystemExit(main())
