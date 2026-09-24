#!/usr/bin/env python3
"""Write or verify a file inventory. Hashes detect changes, not publisher identity."""
from pathlib import Path
import argparse
import hashlib
import sys

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/"SHA256SUMS"
EXCLUDED={".git","target","__pycache__","ci-evidence"}
def paths():
    return sorted(p for p in ROOT.rglob("*") if p.is_file()
        and p != MANIFEST and not set(p.relative_to(ROOT).parts)&EXCLUDED
        and p.suffix not in {".pyc",".pyo"})
def content()->str:
    return "".join(f"{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.relative_to(ROOT).as_posix()}\n"
        for p in paths())
def main()->int:
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check",action="store_true")
    args=parser.parse_args()
    expected=content()
    if args.check:
        if not MANIFEST.exists() or MANIFEST.read_text(encoding="utf-8") != expected:
            print("File inventory differs from SHA256SUMS",file=sys.stderr)
            return 1
        print("File inventory matches SHA256SUMS")
    else:
        MANIFEST.write_text(expected,encoding="utf-8",newline="\n")
        print(f"Wrote SHA256SUMS for {len(paths())} files")
    return 0
if __name__=="__main__":
    raise SystemExit(main())
