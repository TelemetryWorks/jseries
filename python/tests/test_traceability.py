"""Preserve traceability diagnostics and validation order across helper extraction."""

import importlib.util
import json
from pathlib import Path
from types import ModuleType
from typing import Any

import pytest


@pytest.fixture(name="checker")
def checker_fixture(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> ModuleType:
    """Load the checker against an isolated synthetic repository tree."""
    source = Path(__file__).resolve().parents[2] / "tools/check_traceability.py"
    spec = importlib.util.spec_from_file_location("traceability_test", source)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    monkeypatch.setattr(module, "ROOT", tmp_path)
    return module


def records() -> list[dict[str, Any]]:
    """Return a valid parent hierarchy with Rust and qualified Python locators."""
    return [
        {"id": "L1", "level": 1, "parent": None, "rust_tests": [], "python_tests": []},
        {
            "id": "L2",
            "level": 2,
            "parent": "L1",
            "rust_tests": ["crates/example/tests/sample.rs::rust_test"],
            "python_tests": ["python/tests/sample.py::Cases.python_test"],
        },
    ]


def write_tree(
    root: Path, requirements: list[dict[str, Any]], rust: str = "#[test]\nfn rust_test() {}\n"
) -> None:
    """Write the smallest tree needed by the unchanged traceability contract."""
    path = root / "docs/requirements/requirements.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps({"requirements": requirements}), encoding="utf-8")
    path = root / "crates/example/tests/sample.rs"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(rust, encoding="utf-8")
    path = root / "python/tests/sample.py"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("def python_test(): pass\n", encoding="utf-8")


def test_valid_mapping_preserves_summary(
    checker: ModuleType, capsys: pytest.CaptureFixture[str]
) -> None:
    """Class-qualified Python locators and repeated Rust references remain accepted."""
    requirements = records()
    requirements[1]["rust_tests"] *= 2
    write_tree(checker.ROOT, requirements)
    assert checker.main() == 0
    captured = capsys.readouterr()
    assert (
        captured.out
        == "Traceability valid: 2 requirements; 1 Rust test functions mapped (not executed).\n"
    )
    assert captured.err == ""


def test_duplicate_ids_precede_parent_and_reference_errors(checker: ModuleType) -> None:
    """Duplicate IDs still fail before validating parent links or reading tests."""
    requirements = records()
    requirements[1]["id"] = "L1"
    requirements[1]["parent"] = "missing"
    requirements[1]["rust_tests"] = ["absent.rs::missing"]
    write_tree(checker.ROOT, requirements)
    with pytest.raises(ValueError, match=r"^duplicate requirement id$"):
        checker.main()


@pytest.mark.parametrize(
    "level,parent,expected",
    [
        (1, "L1", "L1 has a parent"),
        (2, "missing", "invalid parent for L2"),
        (3, "L1", "invalid parent for L2"),
    ],
)
def test_parent_diagnostics_precede_test_reads(
    checker: ModuleType, level: int, parent: str, expected: str
) -> None:
    """Reject missing/wrong-level parents and L1 parents with the original messages."""
    requirements = records()
    requirements[1].update(level=level, parent=parent, rust_tests=["absent.rs::missing"])
    write_tree(checker.ROOT, requirements)
    with pytest.raises(ValueError) as error:
        checker.main()
    assert str(error.value) == expected


@pytest.mark.parametrize(
    "kind,locator",
    [
        ("rust_tests", "crates/example/tests/sample.rs::absent"),
        ("python_tests", "python/tests/sample.py::Cases.absent"),
    ],
)
def test_missing_reference_preserves_locator(checker: ModuleType, kind: str, locator: str) -> None:
    """Both languages keep the complete locator in the same error category."""
    requirements = records()
    requirements[1][kind] = [locator]
    write_tree(checker.ROOT, requirements)
    with pytest.raises(ValueError) as error:
        checker.main()
    assert str(error.value) == f"missing test: {locator}"


def test_record_order_is_preserved(checker: ModuleType) -> None:
    """An earlier missing test is reported before a later record's invalid parent."""
    requirements = records()
    locator = "python/tests/sample.py::absent"
    requirements[1]["python_tests"] = [locator]
    requirements.append(
        {"id": "later", "level": 2, "parent": "missing", "rust_tests": [], "python_tests": []}
    )
    write_tree(checker.ROOT, requirements)
    with pytest.raises(ValueError) as error:
        checker.main()
    assert str(error.value) == f"missing test: {locator}"


@pytest.mark.parametrize(
    "rust",
    ["fn rust_test() {}\n", "#[test]\nfn rust_test() {}\n#[test]\nfn unmapped() {}\n"],
    ids=["missing-attribute", "unmapped-test"],
)
def test_rust_attribute_and_mapping_boundaries(checker: ModuleType, rust: str) -> None:
    """Keep attribute checks and the exact actual-vs-referenced set comparison."""
    write_tree(checker.ROOT, records(), rust)
    with pytest.raises(ValueError) as error:
        checker.main()
    expected = (
        "missing test: crates/example/tests/sample.rs::rust_test"
        if rust.startswith("fn")
        else "Rust tests and requirement mapping differ"
    )
    assert str(error.value) == expected
