import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from check_release_candidate import validate  # noqa: E402

SHA = "1" * 40


class ReleaseCandidateValidationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        (self.root / "src").mkdir()
        (self.root / "VERSION").write_text("0.4.0\n", encoding="utf-8")
        (self.root / "Cargo.toml").write_text(
            '[package]\nname = "adp-observatory"\nversion = "0.4.0"\n',
            encoding="utf-8",
        )
        (self.root / "src/main.rs").write_text(
            'schema_version: "0.4.0",\n', encoding="utf-8"
        )

    def tearDown(self):
        self.temp.cleanup()

    def evaluate(self, **kwargs):
        options = dict(
            root=self.root, expected_version="0.4.0", expected_sha=SHA,
            actual_sha=SHA, ref="refs/heads/main"
        )
        options.update(kwargs)
        return validate(**options)

    def test_valid_pinned_candidate(self):
        self.assertEqual([], self.evaluate())

    def test_wrong_source_sha_rejected(self):
        self.assertTrue(any("SHA" in x for x in self.evaluate(actual_sha="2" * 40)))

    def test_non_main_ref_rejected(self):
        self.assertTrue(any("main" in x for x in self.evaluate(ref="refs/tags/v0.4.0")))

    def test_version_mismatch_rejected(self):
        errors = self.evaluate(expected_version="0.5.0")
        self.assertTrue(any("VERSION" in x for x in errors))
        self.assertTrue(any("Cargo.toml" in x for x in errors))
        self.assertTrue(any("report schema" in x for x in errors))

    def test_invalid_source_sha_rejected(self):
        self.assertTrue(self.evaluate(expected_sha="HEAD"))

    def test_malformed_version_rejected(self):
        self.assertTrue(self.evaluate(expected_version="0.4.0/unsafe"))

    def test_missing_version_file_rejected(self):
        (self.root / "VERSION").unlink()
        self.assertTrue(any("cannot read VERSION" in x for x in self.evaluate()))

    def test_gate_does_not_change_input_files(self):
        original = (self.root / "Cargo.toml").read_bytes()
        self.assertFalse(self.evaluate())
        self.assertEqual(original, (self.root / "Cargo.toml").read_bytes())


if __name__ == "__main__":
    unittest.main()
