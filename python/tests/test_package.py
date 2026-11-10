"""Installed-package tests for the Rust-backed Python API."""

from importlib import metadata
from pathlib import Path

import pytest

import jseries

ROOT = Path(__file__).resolve().parents[2]


@pytest.fixture(name="decoder", scope="module")
def decoder_fixture() -> jseries.Decoder:
    """Load the repository's synthetic package once for decoder tests."""
    return jseries.Decoder(ROOT / "schemas")


def test_version_matches_distribution_metadata() -> None:
    """The extension and wheel metadata share the Cargo workspace version."""
    assert jseries.__version__ == metadata.version("jseries")


def test_loads_schema_and_decodes_one_record(decoder: jseries.Decoder) -> None:
    """Decode one 70-bit information word through the installed extension."""
    assert decoder.message_ids() == ["EXAMPLE-70"]

    record = decoder.decode("EXAMPLE-70", [0x1C94], source_id="unit-test", source_offset=7)

    assert record.message_id == "EXAMPLE-70"
    assert record.source_id == "unit-test"
    assert record.source_offset == 7
    assert record.fields[1].status == "enumeration"
    assert record.fields[1].label == "active"
    assert record.fields[2].numerator == 9
    assert record.fields[2].denominator == 2
    assert record.fields[3].unsigned_value == 7


def test_batch_decode_preserves_rows_and_assigns_offsets(decoder: jseries.Decoder) -> None:
    """Batch decoding preserves row order and increments source offsets."""
    records = decoder.decode(
        "EXAMPLE-70",
        [[0x1C94], [0x1494], [0x0C94]],
        source_id="batch-test",
        source_offset=100,
    )

    assert [record.source_offset for record in records] == [100, 101, 102]
    assert [record.fields[3].raw for record in records] == [7, 5, 3]


def test_rejects_out_of_range_word(decoder: jseries.Decoder) -> None:
    """Reject input values containing bits above the 70-bit boundary."""
    with pytest.raises(ValueError):
        decoder.decode("EXAMPLE-70", [1 << 70])


def test_rejects_ambiguous_empty_input(decoder: jseries.Decoder) -> None:
    """Reject an empty value sequence because its intended shape is ambiguous."""
    with pytest.raises(ValueError, match="cannot be empty"):
        decoder.decode("EXAMPLE-70", [])


def test_schema_and_decode_errors_keep_public_hierarchy(tmp_path: Path) -> None:
    """Expose Rust failures through the documented Python exception classes."""
    assert issubclass(jseries.SchemaError, jseries.JSeriesError)
    assert issubclass(jseries.DecodeError, jseries.JSeriesError)
    with pytest.raises(jseries.SchemaError):
        jseries.Decoder(tmp_path)
    decoder = jseries.Decoder(ROOT / "schemas")
    with pytest.raises(jseries.DecodeError, match="not in the selected package"):
        decoder.decode("UNKNOWN", [0x1C94])
    with pytest.raises(jseries.DecodeError, match="source identifier"):
        decoder.decode("EXAMPLE-70", [0x1C94], source_id=" ")


def test_batch_rejects_offset_overflow(decoder: jseries.Decoder) -> None:
    """Consecutive batch offsets must not wrap or return a partial result."""
    with pytest.raises(jseries.DecodeError, match="source offset overflow"):
        decoder.decode("EXAMPLE-70", [[0x1C94], [0x1C94]], source_offset=(1 << 64) - 1)


@pytest.mark.parametrize("attribute", ["raw", "status"])
def test_field_properties_are_read_only(decoder: jseries.Decoder, attribute: str) -> None:
    """Typed native properties preserve exact values and cannot be mutated."""
    field = decoder.decode("EXAMPLE-70", (0x1C94,)).fields[2]
    assert field.status == "rational"
    assert field.unsigned_value is None
    assert field.signed_value is None
    assert field.label is None
    assert field.selector is None
    assert field.error is None
    with pytest.raises(AttributeError):
        setattr(field, attribute, 0)
