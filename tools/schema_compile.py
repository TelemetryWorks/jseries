#!/usr/bin/env python3
"""Validate toy schemas and compile immutable Rust tables. No third-party modules.

This compiler deliberately accepts ONLY synthetic schemas. It cannot confer
standards verification or transform arbitrary text into an approved definition.
The digest is SHA-256 of the exact UTF-8 schema JSON bytes, not a signature.
"""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import re
import sys
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
BASELINES = ("D", "E", "F", "F-C1", "G", "H")
VARIANTS = dict(zip(BASELINES, ("D", "E", "F", "FChange1", "G", "H")))
MAX_FILE_BYTES = 1_048_576
OUTPUT = ROOT / "crates/l16-core/src/generated.rs"

class SchemaFailure(ValueError):
    """A malformed, unsupported, or non-synthetic schema was supplied."""

def fail(message: str) -> None:
    raise SchemaFailure(message)

def keys(value: Any, required: set[str], optional: set[str] | None = None) -> None:
    if not isinstance(value, dict):
        fail("expected object")
    extra = set(value) - required - (optional or set())
    missing = required - set(value)
    if extra or missing:
        fail(f"unexpected keys={sorted(extra)}; missing keys={sorted(missing)}")

def integer(value: Any, low: int, high: int) -> int:
    if type(value) is not int or not low <= value <= high:
        fail(f"expected integer in {low}..{high}: {value!r}")
    return value

def text(value: Any, max_length: int = 512) -> str:
    if not isinstance(value, str) or not value.strip() or len(value) > max_length:
        fail("expected nonempty bounded text")
    if not all(32 <= ord(c) <= 126 for c in value):
        fail("this synthetic compiler accepts printable ASCII text only")
    return value

def code(value: Any, width: int) -> int:
    if not isinstance(value, str) or re.fullmatch(r"0|[1-9][0-9]*", value) is None:
        fail("raw codes must be canonical nonnegative decimal strings")
    return integer(int(value), 0, (1 << width) - 1)

def entries(value: Any, width: int) -> dict[int, str]:
    if not isinstance(value, list) or len(value) > 4096:
        fail("expected a bounded entry list")
    result: dict[int, str] = {}
    for item in value:
        keys(item, {"raw", "label"})
        raw, label = code(item["raw"], width), text(item["label"])
        if raw in result:
            fail("duplicate raw code")
        result[raw] = label
    return result

def validate(document: Any) -> dict[str, Any]:
    keys(document, {"format_version", "baseline", "schema_id", "schema_version", "profile_id",
                    "kind", "source", "messages"})
    integer(document["format_version"], 1, 1)
    if document["baseline"] not in BASELINES:
        fail("unrecognized baseline")
    if document["kind"] != "synthetic":
        fail("only synthetic schemas are supported; authoritative approval is not implemented")
    for name in ("schema_id", "schema_version", "profile_id"):
        text(document[name])
    if not document["schema_id"].startswith("synthetic-"):
        fail("schema identity must start with synthetic-")
    keys(document["source"], {"kind", "locator"})
    if document["source"]["kind"] != "project-authored-synthetic":
        fail("invalid source kind for toy schema")
    text(document["source"]["locator"])
    messages = document["messages"]
    if not isinstance(messages, list) or not 1 <= len(messages) <= 128:
        fail("message count must be 1..128")
    message_ids: set[str] = set()
    for message in messages:
        keys(message, {"id", "word_bits", "fields"})
        mid = text(message["id"])
        if not mid.startswith("SYNTH-") or mid in message_ids:
            fail("duplicate or non-synthetic message identifier")
        message_ids.add(mid)
        bits = integer(message["word_bits"], 1, 128)
        fields = message["fields"]
        if not isinstance(fields, list) or not 1 <= len(fields) <= 128:
            fail("field count must be 1..128")
        occupied = 0
        by_id: dict[str, Any] = {}
        for field in fields:
            keys(field, {"id", "definition_ref", "lsb", "width", "interpretation", "specials"}, {"condition"})
            fid = text(field["id"])
            if fid in by_id:
                fail("duplicate field identifier")
            by_id[fid] = field
            if not text(field["definition_ref"]).startswith("SYNTH:"):
                fail("synthetic definitions must use SYNTH: references, not DFI/DUI")
            lsb, width = integer(field["lsb"], 0, 127), integer(field["width"], 1, 128)
            if lsb + width > bits:
                fail("field exceeds word bounds")
            mask = ((1 << width) - 1) << lsb
            if mask & occupied:
                fail("field overlap is not supported in this increment")
            occupied |= mask
            special = entries(field["specials"], width)
            rule = field["interpretation"]
            if not isinstance(rule, dict):
                fail("interpretation must be an object")
            kind = rule.get("kind")
            if kind in ("unsigned", "twos-complement"):
                keys(rule, {"kind"})
            elif kind == "scaled-unsigned":
                keys(rule, {"kind", "numerator", "denominator", "unit"})
                integer(rule["numerator"], -(1 << 63), (1 << 63) - 1)
                integer(rule["denominator"], 1, (1 << 64) - 1)
                text(rule["unit"])
                if width > 63:
                    fail("scaled unsigned fields are limited to 63 bits in this increment")
            elif kind == "enumeration":
                keys(rule, {"kind", "entries"})
                normal = entries(rule["entries"], width)
                if not normal or set(normal) & set(special):
                    fail("empty enumeration or special/normal code conflict")
            else:
                fail(f"unsupported interpretation {kind!r}")
        for field in fields:
            cond = field.get("condition")
            if cond is not None:
                keys(cond, {"selector", "equals"})
                selector = text(cond["selector"])
                if selector not in by_id:
                    fail("condition references a missing selector")
                code(cond["equals"], by_id[selector]["width"])
        pending = set(by_id)
        ready: set[str] = set()
        while pending:
            batch = {fid for fid in pending if by_id[fid].get("condition") is None
                     or by_id[fid]["condition"]["selector"] in ready}
            if not batch:
                fail("conditional dependency cycle")
            pending -= batch
            ready |= batch
    return document

def no_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            fail(f"duplicate JSON key: {key}")
        result[key] = value
    return result

def load(path: Path) -> tuple[dict[str, Any], str]:
    # A bounded read avoids allocating an unbounded input, even if it changes
    # between a metadata check and the actual read.
    with path.open("rb") as handle:
        raw = handle.read(MAX_FILE_BYTES + 1)
    if len(raw) > MAX_FILE_BYTES:
        fail(f"schema exceeds {MAX_FILE_BYTES} bytes")
    document = json.loads(raw.decode("utf-8"), object_pairs_hook=no_duplicate_keys)
    return validate(document), hashlib.sha256(raw).hexdigest()

def rs(value: str) -> str:
    # validate() restricts strings to printable ASCII; JSON and Rust escaping
    # coincide for this subset (double quotes and backslashes).
    return json.dumps(value, ensure_ascii=True)

def render_entries(items: list[dict[str, Any]]) -> str:
    return "&[" + ", ".join(f'({item["raw"]}u128, {rs(item["label"])})' for item in items) + "]"

def render(documents: list[tuple[dict[str, Any], str]]) -> str:
    lines = ["// Generated by tools/schema_compile.py. DO NOT EDIT.",
             "// SYNTHETIC ONLY: no authoritative message definitions are present.",
             "use crate::{Baseline, schema::{Bundle, Condition, FieldSpec, Interpretation, MessageSpec, SchemaKind}};", ""]
    for doc, digest in documents:
        symbol = doc["baseline"].replace("-", "_")
        lines += [f"pub static BUNDLE_{symbol}: Bundle = Bundle {{",
                  f'    baseline: Baseline::{VARIANTS[doc["baseline"]]},',
                  f'    schema_id: {rs(doc["schema_id"])},',
                  f'    schema_version: {rs(doc["schema_version"])},',
                  f'    profile_id: {rs(doc["profile_id"])},',
                  f'    digest_sha256: {rs(digest)},',
                  "    kind: SchemaKind::Synthetic,", "    messages: &["]
        for message in doc["messages"]:
            lines += ["        MessageSpec {", f'            id: {rs(message["id"])},',
                      f'            word_bits: {message["word_bits"]},', "            fields: &["]
            for field in message["fields"]:
                rule = field["interpretation"]
                kind = rule["kind"]
                if kind == "unsigned":
                    interpretation = "Interpretation::Unsigned"
                elif kind == "twos-complement":
                    interpretation = "Interpretation::TwosComplement"
                elif kind == "enumeration":
                    interpretation = f'Interpretation::Enumeration({render_entries(rule["entries"])})'
                else:
                    interpretation = ("Interpretation::ScaledUnsigned { "
                        f'numerator: {rule["numerator"]}i64, denominator: {rule["denominator"]}u64, '
                        f'unit: {rs(rule["unit"])}' + " }")
                c = field.get("condition")
                condition = "None" if c is None else ("Some(Condition { "
                    f'selector: {rs(c["selector"])}, equals: {c["equals"]}u128' + " })")
                lines += ["                FieldSpec {", f'                    id: {rs(field["id"])},',
                          f'                    definition_ref: {rs(field["definition_ref"])},',
                          f'                    lsb: {field["lsb"]}, width: {field["width"]},',
                          f"                    interpretation: {interpretation},",
                          f'                    specials: {render_entries(field["specials"])},',
                          f"                    condition: {condition},", "                },"]
            lines += ["            ],", "        },"]
        lines += ["    ],", "};", ""]
    return "\n".join(lines)

def compile_all(check: bool = False) -> None:
    documents = [load(ROOT / "schemas/synthetic" / f"{b}.json") for b in BASELINES]
    actual = [doc["baseline"] for doc, _ in documents]
    if actual != list(BASELINES):
        fail("filename and declared baseline mismatch")
    if len({doc["schema_id"] for doc, _ in documents}) != len(documents):
        fail("schema identities must be unique")
    content = render(documents)
    if check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != content:
            fail("generated.rs is stale; run python tools/schema_compile.py")
    else:
        OUTPUT.write_text(content, encoding="utf-8", newline="\n")

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail rather than update generated Rust")
    args = parser.parse_args()
    try:
        compile_all(args.check)
    except (OSError, UnicodeError, ValueError, TypeError, KeyError) as error:
        print(f"schema compilation failed: {error}", file=sys.stderr)
        return 1
    print("6 synthetic schemas validated; Rust tables " + ("match" if args.check else "generated"))
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
