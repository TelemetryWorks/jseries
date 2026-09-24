"""Python checks of schema authoring and toy arithmetic, not execution of Rust."""
from __future__ import annotations
import copy
import hashlib
import json
from pathlib import Path
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
import schema_compile as compiler

def reference_decode(document: dict, message_id: str, word: int) -> dict:
    """Independent small Python evaluator for the documented synthetic format."""
    spec = next(m for m in document["messages"] if m["id"] == message_id)
    if word < 0 or word >= 1 << spec["word_bits"]:
        raise ValueError("word outside format")
    fields = {f["id"]: f for f in spec["fields"]}
    raw = {name: (word >> f["lsb"]) % (1 << f["width"]) for name, f in fields.items()}
    result = {}
    pending = set(fields)
    while pending:
        before = len(pending)
        for name in sorted(pending):
            f = fields[name]
            value = raw[name]
            cond = f.get("condition")
            if cond and cond["selector"] not in result:
                continue
            output = {"raw": str(value)}
            if cond and result[cond["selector"]]["status"] != "value":
                output["status"] = "unresolved_context"
            elif cond and raw[cond["selector"]] != int(cond["equals"]):
                output["status"] = "not_applicable"
            else:
                special = {int(s["raw"]): s["label"] for s in f["specials"]}
                rule = f["interpretation"]
                if value in special:
                    output.update(status="special", label=special[value])
                elif rule["kind"] == "unsigned":
                    output.update(status="value", unsigned=str(value))
                elif rule["kind"] == "twos-complement":
                    signed = value if value < 1 << (f["width"] - 1) else value - (1 << f["width"])
                    output.update(status="value", signed=str(signed))
                elif rule["kind"] == "scaled-unsigned":
                    output.update(status="value", numerator=str(value * rule["numerator"]),
                                  denominator=str(rule["denominator"]), unit=rule["unit"])
                elif rule["kind"] == "enumeration":
                    values = {int(s["raw"]): s["label"] for s in rule["entries"]}
                    if value in values:
                        output.update(status="value", label=values[value])
                    else:
                        output["status"] = "invalid_encoding"
                else:
                    raise ValueError("unsupported rule")
            result[name] = output
            pending.remove(name)
        if len(pending) == before:
            raise ValueError("unresolved dependency graph")
    return result

class SchemaChecks(unittest.TestCase):
    def setUp(self):
        self.schema, _ = compiler.load(ROOT / "schemas/synthetic/D.json")
    def test_six_synthetic_schemas_validate(self):
        for baseline in compiler.BASELINES:
            with self.subTest(baseline=baseline):
                doc, _ = compiler.load(ROOT / f"schemas/synthetic/{baseline}.json")
                self.assertEqual(doc["baseline"], baseline)
                self.assertEqual(doc["kind"], "synthetic")
    def test_generated_rust_is_current(self):
        compiler.compile_all(check=True)
    def test_all_six_hand_established_golden_cases(self):
        cases = json.loads((ROOT / "fixtures/normal-cases.json").read_text())
        for case in cases:
            doc, _ = compiler.load(ROOT / f'schemas/synthetic/{case["baseline"]}.json')
            with self.subTest(case=case["id"]):
                self.assertEqual(reference_decode(doc, case["message"], int(case["word_hex"], 16)), case["expected"])
    def test_digests_are_exact_source_bytes_and_unique(self):
        digests = set()
        for b in compiler.BASELINES:
            path = ROOT / f"schemas/synthetic/{b}.json"
            _, digest = compiler.load(path)
            self.assertEqual(digest, hashlib.sha256(path.read_bytes()).hexdigest())
            digests.add(digest)
        self.assertEqual(len(digests), 6)
    def test_non_synthetic_input_is_rejected(self):
        self.schema["kind"] = "authoritative"
        with self.assertRaises(compiler.SchemaFailure): compiler.validate(self.schema)
    def test_unknown_keys_are_rejected(self):
        self.schema["mesages"] = []
        with self.assertRaises(compiler.SchemaFailure): compiler.validate(self.schema)
    def test_duplicate_json_keys_are_rejected(self):
        with self.assertRaises(compiler.SchemaFailure):
            json.loads('{"baseline":"D","baseline":"H"}', object_pairs_hook=compiler.no_duplicate_keys)
    def test_overlap_is_rejected(self):
        self.schema["messages"][0]["fields"][1]["lsb"] = 1
        with self.assertRaises(compiler.SchemaFailure): compiler.validate(self.schema)
    def test_bounds_are_rejected(self):
        self.schema["messages"][0]["fields"][3]["width"] = 5
        with self.assertRaises(compiler.SchemaFailure): compiler.validate(self.schema)
    def test_missing_selector_is_rejected(self):
        self.schema["messages"][0]["fields"][2]["condition"]["selector"] = "absent"
        with self.assertRaises(compiler.SchemaFailure): compiler.validate(self.schema)
    def test_dependency_cycle_is_rejected(self):
        self.schema["messages"][0]["fields"][0]["condition"] = {"selector":"auxiliary","equals":"1"}
        with self.assertRaises(compiler.SchemaFailure): compiler.validate(self.schema)
    def test_invalid_scale_is_rejected(self):
        self.schema["messages"][0]["fields"][1]["interpretation"]["denominator"] = 0
        with self.assertRaises(compiler.SchemaFailure): compiler.validate(self.schema)
    def test_special_normal_conflict_is_rejected(self):
        self.schema["messages"][0]["fields"][0]["specials"] = [{"raw":"1","label":"conflict"}]
        with self.assertRaises(compiler.SchemaFailure): compiler.validate(self.schema)
    def test_duplicate_special_is_rejected(self):
        special = self.schema["messages"][0]["fields"][0]["specials"]
        special.append(copy.deepcopy(special[0]))
        with self.assertRaises(compiler.SchemaFailure): compiler.validate(self.schema)
    def test_f_and_f_change_one_have_different_identity(self):
        f, _ = compiler.load(ROOT / "schemas/synthetic/F.json")
        c, _ = compiler.load(ROOT / "schemas/synthetic/F-C1.json")
        self.assertNotEqual(f["schema_id"], c["schema_id"])
    def test_special_before_scale(self):
        value = reference_decode(self.schema, "SYNTH-01", 0x00fc)["level"]
        self.assertEqual(value, {"raw":"63","status":"special","label":"no_statement"})
    def test_special_selector_is_not_false_condition(self):
        value = reference_decode(self.schema, "SYNTH-01", 0x0767)["auxiliary"]
        self.assertEqual(value["status"], "unresolved_context")
        self.assertEqual(value["raw"], "7")
    def test_false_condition_retains_bits(self):
        value = reference_decode(self.schema, "SYNTH-01", 0x0764)["auxiliary"]
        self.assertEqual(value, {"raw":"7","status":"not_applicable"})
    def test_invalid_selector_is_unresolved(self):
        self.assertEqual(reference_decode(self.schema, "SYNTH-01", 0x0766)["auxiliary"]["status"], "unresolved_context")
    def test_enum_unknown_not_silently_accepted(self):
        self.assertEqual(reference_decode(self.schema, "SYNTH-01", 0x2765)["flags"]["status"], "invalid_encoding")
    def test_h_only_message_is_not_in_d(self):
        self.assertNotIn("SYNTH-H-ONLY", [m["id"] for m in self.schema["messages"]])
        h, _ = compiler.load(ROOT / "schemas/synthetic/H.json")
        self.assertIn("SYNTH-H-ONLY", [m["id"] for m in h["messages"]])
    def test_file_size_is_bounded(self):
        with tempfile.TemporaryDirectory() as d:
            path = Path(d) / "oversize.json"
            path.write_bytes(b" " * (compiler.MAX_FILE_BYTES + 1))
            with self.assertRaises(compiler.SchemaFailure): compiler.load(path)
    def test_bool_is_not_accepted_as_numeric_width(self):
        self.schema["messages"][0]["fields"][0]["width"] = True
        with self.assertRaises(compiler.SchemaFailure): compiler.validate(self.schema)
    def test_noncanonical_code_is_rejected(self):
        self.schema["messages"][0]["fields"][0]["specials"][0]["raw"] = "03"
        with self.assertRaises(compiler.SchemaFailure): compiler.validate(self.schema)
    def test_reordered_fields_preserve_interpretation(self):
        expected=reference_decode(self.schema,"SYNTH-01",0x0765)
        self.schema["messages"][0]["fields"].reverse()
        compiler.validate(self.schema)
        self.assertEqual(reference_decode(self.schema,"SYNTH-01",0x0765),expected)
    def test_source_register_has_no_invented_document_hashes(self):
        registry=json.loads((ROOT/"schemas/authoritative/source-register.json").read_text())
        self.assertEqual([x["baseline"] for x in registry["baselines"]],list(compiler.BASELINES))
        self.assertTrue(all(x["document_sha256"] is None for x in registry["baselines"]))
    def test_coverage_denominators_remain_unknown(self):
        coverage=json.loads((ROOT/"schemas/authoritative/coverage.json").read_text())
        for item in coverage["baselines"]:
            self.assertIsNone(item["inventory_total"])
            self.assertEqual(item["verified_standard_messages"],0)

if __name__ == "__main__":
    unittest.main(verbosity=2)
