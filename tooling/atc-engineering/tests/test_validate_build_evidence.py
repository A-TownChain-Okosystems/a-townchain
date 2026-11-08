import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).resolve().parents[1] / "scripts" / "validate_build_evidence.py"
SPEC = importlib.util.spec_from_file_location("validate_build_evidence", MODULE_PATH)
validator = importlib.util.module_from_spec(SPEC)
assert SPEC and SPEC.loader
SPEC.loader.exec_module(validator)


class BuildEvidenceValidationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.report = self.root / "gate-report.json"
        self.report.write_text(json.dumps({"schema_version": "atc.gate-report/v1", "status": "PASS"}), encoding="utf-8")
        self.sha = "a" * 40

    def tearDown(self):
        self.temp.cleanup()

    def validate(self, report_path=None, expected_sha=None, checked_out_sha=None, run_id="123", run_attempt="1"):
        return validator.validate_report(
            report_path or self.report,
            expected_sha if expected_sha is not None else self.sha,
            checked_out_sha if checked_out_sha is not None else self.sha,
            run_id,
            run_attempt,
        )

    def test_accepts_valid_report_and_exact_sha(self):
        report = self.validate()
        self.assertEqual(report["schema_version"], "atc.gate-report/v1")

    def test_missing_report_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "missing or invalid JSON"):
            self.validate(report_path=self.root / "missing.json")

    def test_invalid_json_fails_closed(self):
        self.report.write_text("{not-json", encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "missing or invalid JSON"):
            self.validate()

    def test_non_object_report_fails_closed(self):
        self.report.write_text("[]", encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "JSON object"):
            self.validate()

    def test_missing_or_wrong_schema_fails_closed(self):
        self.report.write_text(json.dumps({"schema_version": "wrong/v9"}), encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "schema_version"):
            self.validate()

    def test_malformed_expected_sha_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "full 40-character SHA"):
            self.validate(expected_sha="abc")

    def test_sha_foreign_checkout_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "does not match"):
            self.validate(checked_out_sha="b" * 40)

    def test_malformed_checkout_sha_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "does not match"):
            self.validate(checked_out_sha="not-a-sha")

    def test_missing_or_malformed_run_identity_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "run identity"):
            self.validate(run_id="")
        with self.assertRaisesRegex(ValueError, "run identity"):
            self.validate(run_attempt="x")

    def test_manifest_hashes_report_and_binary(self):
        binary = self.root / "atc-audit-cli"
        binary.write_bytes(b"test binary")
        manifest = validator.build_manifest(
            self.report, binary, self.sha, self.sha, "123", "1",
            "owner/repo", "owner/repo/.github/workflows/engineering-ci.yml@refs/heads/main",
            json.loads(self.report.read_text(encoding="utf-8")),
        )
        self.assertEqual(manifest["source_sha"], self.sha)
        self.assertEqual(len(manifest["gate_report_sha256"]), 64)
        self.assertEqual(len(manifest["release_binary_sha256"]), 64)

    def test_missing_binary_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "release binary is missing"):
            validator.build_manifest(
                self.report, self.root / "missing-binary", self.sha, self.sha, "123", "1",
                "owner/repo", "workflow-ref", json.loads(self.report.read_text(encoding="utf-8")),
            )


if __name__ == "__main__":
    unittest.main()
