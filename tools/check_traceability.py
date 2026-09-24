#!/usr/bin/env python3
"""Validate requirement hierarchy and references; does not execute Rust tests."""
import json
from pathlib import Path
import re
import sys
ROOT=Path(__file__).resolve().parents[1]
def main()->int:
    records=json.loads((ROOT/"docs/requirements/requirements.json").read_text())["requirements"]
    index={r["id"]:r for r in records}
    if len(index)!=len(records): raise ValueError("duplicate requirement id")
    referenced=set()
    for r in records:
        if r["level"]==1:
            if r["parent"] is not None: raise ValueError("L1 has a parent")
        elif r["parent"] not in index or index[r["parent"]]["level"]!=r["level"]-1:
            raise ValueError(f'invalid parent for {r["id"]}')
        for kind in ("rust_tests","python_tests"):
            for locator in r[kind]:
                path, qualified=locator.split("::",1)
                name=qualified.split(".")[-1]
                content=(ROOT/path).read_text()
                pattern=(rf'#\[test\]\s*fn\s+{re.escape(name)}\s*\('
                    if kind=="rust_tests" else rf'def\s+{re.escape(name)}\s*\(')
                if not re.search(pattern,content): raise ValueError(f"missing test: {locator}")
                if kind=="rust_tests": referenced.add(locator)
    actual=set()
    for path in (ROOT/"crates").glob("*/tests/*.rs"):
        for name in re.findall(r'#\[test\]\s*fn\s+(\w+)',path.read_text()):
            actual.add(f'{path.relative_to(ROOT).as_posix()}::{name}')
    if actual!=referenced: raise ValueError("Rust tests and requirement mapping differ")
    print(f"Traceability valid: {len(records)} requirements; {len(actual)} Rust test functions mapped (not executed).")
    return 0
if __name__=="__main__":
    try: raise SystemExit(main())
    except (OSError,ValueError,KeyError) as e:
        print(f"Traceability error: {e}",file=sys.stderr)
        raise SystemExit(1)
