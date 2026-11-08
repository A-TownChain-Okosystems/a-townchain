import importlib.util
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "validate_catalog_exclusions.py"
SPEC = importlib.util.spec_from_file_location("validate_catalog_exclusions", SCRIPT)
audit_module = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(audit_module)


def exclusion(path, blob, reason="UNSUPPORTED_FORMAT", evidence=None):
    evidence = evidence if evidence is not None else [{"kind": "source_or_policy_reference", "reference": f"{path}#{blob}"}]
    return {
        "exclusion_id": "a" * 64,
        "repository": "owner/repo",
        "path": path,
        "reason_code": reason,
        "category": "unsupported_format" if reason == "UNSUPPORTED_FORMAT" else "scanner_policy",
        "reason": "file is indexed but parser is unavailable",
        "rationale": "file is indexed but parser is unavailable",
        "source_blob_sha": blob,
        "evidence": evidence,
    }


class TestExclusionAudit(unittest.TestCase):
    def test_matching_file_blob_and_path_evidence_pass(self):
        blob = "b" * 40
        report = audit_module.audit_inventory({
            "repository": "owner/repo", "observed_commit_sha": "c" * 40,
            "files": [{"path": "schema.json", "blob_sha": blob, "status": "UNSUPPORTED"}],
            "exclusions": [exclusion("schema.json", blob)],
        })
        assert report["status"] == "PASS"
        assert report["counts"]["errors"] == 0

    def test_blob_mismatch_blocks(self):
        report = audit_module.audit_inventory({
            "repository": "owner/repo", "observed_commit_sha": "c" * 40,
            "files": [{"path": "schema.json", "blob_sha": "b" * 40, "status": "UNSUPPORTED"}],
            "exclusions": [exclusion("schema.json", "d" * 40)],
        })
        codes = {error["code"] for error in report["errors"]}
        assert report["status"] == "BLOCKED"
        assert "EXCLUSION_BLOB_MISMATCH" in codes
        assert "MISSING_PATH_BLOB_EVIDENCE" not in codes


    def test_missing_path_blob_evidence_blocks(self):
        blob = "b" * 40
        item = exclusion("schema.json", blob, evidence=[
            {"kind": "source_or_policy_reference", "reference": "schema.json#wrong"}
        ])
        report = audit_module.audit_inventory({
            "repository": "owner/repo", "observed_commit_sha": "c" * 40,
            "files": [{"path": "schema.json", "blob_sha": blob, "status": "UNSUPPORTED"}],
            "exclusions": [item],
        })
        assert report["status"] == "BLOCKED"
        assert any(error["code"] == "MISSING_PATH_BLOB_EVIDENCE" for error in report["errors"])

    def test_unsupported_file_without_exclusion_blocks(self):
        report = audit_module.audit_inventory({
            "repository": "owner/repo", "observed_commit_sha": "c" * 40,
            "files": [{"path": "schema.json", "blob_sha": "b" * 40, "status": "UNSUPPORTED"}],
            "exclusions": [],
        })
        assert report["status"] == "BLOCKED"
        assert any(error["code"] == "UNEXPLAINED_FILE_STATUS" for error in report["errors"])

    def test_directory_policy_exclusion_must_not_claim_file_blob(self):
        item = exclusion("target", "b" * 40, "EXCLUDED_DIRECTORY",
                         [{"kind": "source_or_policy_reference", "reference": "SKIP_DIRS:target"}])
        report = audit_module.audit_inventory({
            "repository": "owner/repo", "observed_commit_sha": "c" * 40,
            "files": [], "exclusions": [item],
        })
        assert report["status"] == "BLOCKED"
        assert any(error["code"] == "DIRECTORY_EXCLUSION_HAS_FILE_BLOB" for error in report["errors"])
