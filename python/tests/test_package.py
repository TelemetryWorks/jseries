import importlib.metadata
import unittest

import jseries


class PackageTests(unittest.TestCase):
    def test_version_matches_distribution_metadata(self) -> None:
        self.assertEqual(jseries.__version__, importlib.metadata.version("jseries"))


if __name__ == "__main__":
    unittest.main()
