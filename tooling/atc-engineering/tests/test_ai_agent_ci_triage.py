import hashlib
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

MODULE_PATH = Path(__file__).resolve().parents[1] / "scripts" / "ai_agent_ci_triage.py"
SPEC = importlib.util.spec_from_file_location("ai_agent_ci_triage", MODULE_PATH)
triage = importlib.util.module_from_spec(SPEC)
assert SPEC and SPEC.loader
SPEC.loader.exec_module(triage)


class TriagePolicyTests(unittest.TestCase):
    def test_classifies_merge_conflict(self):
        self.assertEqual(
            triage.classify_finding("PR checks", "merge conflict detector", [], "failure"),
            "MERGE_CONFLICT",
        )

    def test_classifies_test_failure(self):
        self.assertEqual(
            triage.classify_finding("unit tests", "workspace tests", ["Run regression tests"], "failure"),
            "TEST_FAILURE",
        )

    def test_unknown_failure_is_not_misrepresented_as_root_cause(self):
        self.assertEqual(triage.classify_finding("custom", "job", ["run command"], "failure"), "UNCLASSIFIED")

    def test_finding_id_is_stable_and_sha_bound(self):
        first = triage.stable_finding_id("org/repo", "CI", "a" * 40, 123, ["cargo test"])
        same = triage.stable_finding_id("org/repo", "CI", "a" * 40, 123, ["cargo test"])
        other_sha = triage.stable_finding_id("org/repo", "CI", "b" * 40, 123, ["cargo test"])
        self.assertEqual(first, same)
        self.assertNotEqual(first, other_sha)
        self.assertTrue(first.startswith("ATC-FINDING-"))

    def test_missing_credentials_fail_closed(self):
        with patch.object(triage, "TOKEN", ""), patch.object(triage, "REPOSITORY", "org/repo"):
            with self.assertRaises(triage.ApiError):
                triage.api_get("/repos/org/repo/actions/runs")

    def test_evidence_adapter_matches_existing_schema_and_fails_closed(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            triage.OUT = Path(temp_dir)
            triage.REPOSITORY = "org/repo"
            triage.CURRENT_SHA = "a" * 40
            triage.CURRENT_RUN_ID = "123456789"
            source = Path(temp_dir) / "triage.json"
            source.write_text('{"status":"FINDINGS","finding_count":1}', encoding="utf-8")
            triage.write_evidence_record(source, {"status": "FINDINGS", "finding_count": 1}, "now")
            record = json.loads((Path(temp_dir) / "evidence-record.json").read_text(encoding="utf-8"))
            self.assertEqual(record["schema"], "ATC-EVD-001")
            self.assertEqual(record["status"], "UNKNOWN")
            self.assertEqual(record["commit"], "a" * 40)
            self.assertEqual(record["integrity"]["digest"], hashlib.sha256(source.read_bytes()).hexdigest())
            self.assertTrue(record["immutable"])

    def test_blocked_report_never_claims_zero_attempts(self):
        report = triage.make_blocked_report("now", "API unavailable")
        self.assertEqual(report["status"], "BLOCKED")
        self.assertIsNone(report["repair_policy"]["attempts_used"])
        self.assertFalse(report["repair_policy"]["repair_execution_enabled"])


if __name__ == "__main__":
    unittest.main()
