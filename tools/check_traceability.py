#!/usr/bin/env python3
"""Validate requirement hierarchy and references; does not execute Rust tests."""

import json
import re
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]


def _validate_parent(record: dict[str, Any], index: dict[str, dict[str, Any]]) -> None:
    """Validate one parent before reading that record's test references."""
    if record["level"] == 1:
        if record["parent"] is not None:
            raise ValueError("L1 has a parent")
    elif record["parent"] not in index or index[record["parent"]]["level"] != record["level"] - 1:
        raise ValueError(f"invalid parent for {record['id']}")


def _validate_test_reference(locator: str, kind: str) -> None:
    """Check one locator using the existing language-specific test pattern."""
    path, qualified = locator.split("::", 1)
    name = qualified.split(".")[-1]
    content = (ROOT / path).read_text()
    pattern = (
        rf"#\[test\]\s*fn\s+{re.escape(name)}\s*\("
        if kind == "rust_tests"
        else rf"def\s+{re.escape(name)}\s*\("
    )
    if not re.search(pattern, content):
        raise ValueError(f"missing test: {locator}")


def _collect_references(
    records: list[dict[str, Any]], index: dict[str, dict[str, Any]]
) -> set[str]:
    """Validate records in source order and collect their mapped Rust tests."""
    referenced = set()
    for record in records:
        _validate_parent(record, index)
        for kind in ("rust_tests", "python_tests"):
            for locator in record[kind]:
                _validate_test_reference(locator, kind)
                if kind == "rust_tests":
                    referenced.add(locator)
    return referenced


def main() -> int:
    """Validate requirement parents and mapped Rust and Python test names."""
    records = json.loads((ROOT / "docs/requirements/requirements.json").read_text())["requirements"]
    index = {r["id"]: r for r in records}
    if len(index) != len(records):
        raise ValueError("duplicate requirement id")
    referenced = _collect_references(records, index)
    actual = set()
    for path in (ROOT / "crates").glob("*/tests/*.rs"):
        for name in re.findall(r"#\[test\]\s*fn\s+(\w+)", path.read_text()):
            actual.add(f"{path.relative_to(ROOT).as_posix()}::{name}")
    if actual != referenced:
        raise ValueError("Rust tests and requirement mapping differ")
    print(
        f"Traceability valid: {len(records)} requirements; "
        f"{len(actual)} Rust test functions mapped (not executed)."
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, KeyError) as error:
        print(f"Traceability error: {error}", file=sys.stderr)
        raise SystemExit(1) from error
