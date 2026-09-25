import importlib.metadata
from pathlib import Path
import unittest

import jseries

ROOT = Path(__file__).resolve().parents[2]


class PackageTests(unittest.TestCase):
    def test_version_matches_distribution_metadata(self) -> None:
        self.assertEqual(jseries.__version__, importlib.metadata.version("jseries"))


class DecoderTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.decoder = jseries.Decoder(ROOT / "schemas")

    def test_loads_schema_and_decodes_logical70(self) -> None:
        self.assertEqual(self.decoder.message_ids(), ["EXAMPLE-70"])

        record = self.decoder.decode_logical70(
            "EXAMPLE-70", [0x1C94], source_id="unit-test", source_offset=7
        )

        self.assertEqual(record.message_id, "EXAMPLE-70")
        self.assertEqual(record.source_id, "unit-test")
        self.assertEqual(record.source_offset, 7)
        self.assertEqual(record.fields[1].status, "enumeration")
        self.assertEqual(record.fields[1].label, "active")
        self.assertEqual(record.fields[2].numerator, 9)
        self.assertEqual(record.fields[2].denominator, 2)
        self.assertEqual(record.fields[3].unsigned_value, 7)

    def test_batch_decode_preserves_rows_and_assigns_offsets(self) -> None:
        records = self.decoder.decode_many_logical70(
            "EXAMPLE-70",
            [[0x1C94], [0x1494], [0x0C94]],
            source_id="batch-test",
            start_offset=100,
        )

        self.assertEqual([record.source_offset for record in records], [100, 101, 102])
        self.assertEqual([record.fields[3].raw for record in records], [7, 5, 3])

    def test_rejects_out_of_range_logical70_word(self) -> None:
        with self.assertRaises(ValueError):
            self.decoder.decode_logical70("EXAMPLE-70", [1 << 70])


if __name__ == "__main__":
    unittest.main()
