import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "validate_component_evidence.py"
SPEC = importlib.util.spec_from_file_location("validate_component_evidence", SCRIPT)
validator = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(validator)


class ComponentEvidenceValidationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.source = self.root / "src.py"
        self.source.write_text("def canonical_component():\n    return True\n", encoding="utf-8")
        self.sha = validator.git_blob_sha(self.source.read_bytes())
        self.commit = "a" * 40
        self.manifest = {
            "observed_commit_sha": self.commit,
            "candidates": [{
                "component_id": "ATCE-COMP-001",
                "status": "IMPLEMENTED",
                "decision": "REVIEW_REQUIRED",
                "evidence": [{"path": "src.py", "blob_sha": self.sha, "symbols": ["canonical_component"]}],
                "contract_refs": ["contracts/component-v1.json"],
                "runtime_wiring_evidence": ["src.py:canonical_component"],
            }],
        }

    def tearDown(self):
        self.temp.cleanup()

    def test_valid_source_binding_does_not_claim_verified(self):
        report = validator.validate(self.manifest, self.root, self.commit)
        self.assertEqual(report["status"], "RESIDUAL")
        self.assertEqual(report["finding_count"], 0)

    def test_missing_and_invalid_blob_sha_are_blocked(self):
        for sha in (None, "not-a-git-sha"):
            with self.subTest(sha=sha):
                candidate = json.loads(json.dumps(self.manifest))
                candidate["candidates"][0]["evidence"][0]["blob_sha"] = sha
                report = validator.validate(candidate, self.root, self.commit)
                self.assertEqual(report["status"], "BLOCKED")
                self.assertIn("INVALID_BLOB_SHA", {f["code"] for f in report["findings"]})

    def test_missing_source_path_is_blocked(self):
        candidate = json.loads(json.dumps(self.manifest))
        candidate["candidates"][0]["evidence"][0]["path"] = "missing.py"
        report = validator.validate(candidate, self.root, self.commit)
        self.assertIn("SOURCE_PATH_NOT_FOUND", {f["code"] for f in report["findings"]})

    def test_missing_symbol_is_blocked(self):
        candidate = json.loads(json.dumps(self.manifest))
        candidate["candidates"][0]["evidence"][0]["symbols"] = ["not_present"]
        report = validator.validate(candidate, self.root, self.commit)
        self.assertIn("SYMBOL_NOT_FOUND", {f["code"] for f in report["findings"]})

    def test_duplicate_component_id_is_blocked(self):
        candidate = json.loads(json.dumps(self.manifest))
        candidate["candidates"].append(json.loads(json.dumps(candidate["candidates"][0])))
        report = validator.validate(candidate, self.root, self.commit)
        self.assertIn("DUPLICATE_COMPONENT_ID", {f["code"] for f in report["findings"]})

    def test_stale_blob_and_stale_commit_are_blocked(self):
        candidate = json.loads(json.dumps(self.manifest))
        candidate["candidates"][0]["evidence"][0]["blob_sha"] = "b" * 40
        candidate["observed_commit_sha"] = "c" * 40
        report = validator.validate(candidate, self.root, self.commit)
        codes = {f["code"] for f in report["findings"]}
        self.assertIn("STALE_BLOB_SHA", codes)
        self.assertIn("STALE_MANIFEST_COMMIT", codes)

    def test_missing_contract_and_runtime_wiring_are_blocked(self):
        candidate = json.loads(json.dumps(self.manifest))
        candidate["candidates"][0]["contract_refs"] = []
        candidate["candidates"][0]["runtime_wiring_evidence"] = []
        report = validator.validate(candidate, self.root, self.commit)
        codes = {f["code"] for f in report["findings"]}
        self.assertIn("MISSING_CONTRACT_BINDING", codes)
        self.assertIn("MISSING_RUNTIME_WIRING", codes)

    def test_verified_without_explicit_proof_is_blocked(self):
        candidate = json.loads(json.dumps(self.manifest))
        candidate["candidates"][0]["status"] = "VERIFIED"
        report = validator.validate(candidate, self.root, self.commit)
        self.assertIn("VERIFIED_WITHOUT_PROOF", {f["code"] for f in report["findings"]})

    def test_unsupported_exclusion_is_blocked(self):
        candidate = json.loads(json.dumps(self.manifest))
        candidate["candidates"][0]["decision"] = "EXCLUDED"
        report = validator.validate(candidate, self.root, self.commit)
        self.assertIn("UNJUSTIFIED_EXCLUSION", {f["code"] for f in report["findings"]})


if __name__ == "__main__":
    unittest.main()
