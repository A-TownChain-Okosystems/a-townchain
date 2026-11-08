import json
import unittest
from pathlib import Path

import jsonschema

from scripts.reconciliation_status_contract import (
    StatusContractError,
    require_known_status,
    status_conclusion,
    validate_verification_claim,
)


def valid_claim():
    sha = "a" * 40
    return {
        "status": "VERIFIED",
        "claim_id": "claim-source-census-v1",
        "scope": {"kind": "repository", "repository": "owner/repo", "paths": ["src/"]},
        "checked_commit_sha": sha,
        "decision": "PASS",
        "criteria": [{"criterion_id": "all-required-checks", "result": "PASS", "evidence_ids": ["run-1"]}],
        "evidence": [{
            "evidence_id": "run-1",
            "head_sha": sha,
            "workflow_run_id": 12345,
            "job": "contract-tests",
            "step": "verify-status-contract",
            "artifact_id": "artifact-77",
            "artifact_sha256": "b" * 64,
        }],
    }


class ReconciliationStatusContractTests(unittest.TestCase):
    def test_detected_is_not_verified(self):
        claim = valid_claim()
        claim["status"] = "DETECTED"
        with self.assertRaises(StatusContractError):
            validate_verification_claim(claim)

    def test_reconciled_alone_is_not_verified(self):
        claim = valid_claim()
        claim["status"] = "RECONCILED"
        with self.assertRaises(StatusContractError):
            validate_verification_claim(claim)

    def test_missing_evidence_blocks(self):
        claim = valid_claim()
        claim["evidence"] = []
        with self.assertRaises(StatusContractError):
            validate_verification_claim(claim)

    def test_malformed_artifact_hash_blocks(self):
        claim = valid_claim()
        claim["evidence"][0]["artifact_sha256"] = "not-a-hash"
        with self.assertRaises(StatusContractError):
            validate_verification_claim(claim)

    def test_artifact_hash_without_artifact_identity_blocks(self):
        claim = valid_claim()
        claim["evidence"][0].pop("artifact_id")
        with self.assertRaises(StatusContractError):
            validate_verification_claim(claim)

    def test_reproducible_https_log_reference_is_accepted(self):
        claim = valid_claim()
        claim["evidence"][0].pop("artifact_id")
        claim["evidence"][0].pop("artifact_sha256")
        claim["evidence"][0]["log_reference"] = "https://github.com/owner/repo/actions/runs/12345"
        validate_verification_claim(claim)

    def test_non_https_log_reference_blocks(self):
        claim = valid_claim()
        claim["evidence"][0].pop("artifact_id")
        claim["evidence"][0].pop("artifact_sha256")
        claim["evidence"][0]["log_reference"] = "run-12345"
        with self.assertRaises(StatusContractError):
            validate_verification_claim(claim)

    def test_unknown_status_fails_closed(self):
        with self.assertRaises(StatusContractError):
            require_known_status("GREEN")
        with self.assertRaises(StatusContractError):
            require_known_status(None)

    def test_single_workflow_does_not_verify_organization_scope(self):
        claim = valid_claim()
        claim["scope"] = {
            "kind": "organization",
            "repositories": ["owner/a", "owner/b"],
            "evidence_repositories": ["owner/a"],
        }
        with self.assertRaises(StatusContractError):
            validate_verification_claim(claim)

    def test_evidence_head_must_match_checked_full_sha(self):
        claim = valid_claim()
        claim["evidence"][0]["head_sha"] = "c" * 40
        with self.assertRaises(StatusContractError):
            validate_verification_claim(claim)

    def test_each_criterion_must_pass_and_reference_evidence(self):
        claim = valid_claim()
        claim["criteria"][0]["result"] = "UNKNOWN"
        with self.assertRaises(StatusContractError):
            validate_verification_claim(claim)
        claim = valid_claim()
        claim["criteria"][0]["evidence_ids"] = ["missing"]
        with self.assertRaises(StatusContractError):
            validate_verification_claim(claim)

    def test_versioned_json_schema_accepts_valid_claim_and_rejects_promotions(self):
        schema_path = Path(__file__).resolve().parents[1] / "schemas" / "reconciliation-verification-claim.schema.json"
        schema = json.loads(schema_path.read_text(encoding="utf-8"))
        jsonschema.Draft202012Validator.check_schema(schema)
        jsonschema.validate(valid_claim(), schema)
        for status in ("DETECTED", "RECONCILED", "GREEN"):
            invalid = valid_claim()
            invalid["status"] = status
            with self.assertRaises(jsonschema.ValidationError):
                jsonschema.validate(invalid, schema)

    def test_explicitly_scoped_verification_claim_passes(self):
        validate_verification_claim(valid_claim())
        self.assertEqual(status_conclusion("DETECTED"), "ASSESS_FINDINGS")
        self.assertEqual(status_conclusion("RECONCILED"), "SCOPED_RECONCILIATION_PASSED")
        self.assertEqual(status_conclusion("VERIFIED"), "SCOPED_CLAIM_VERIFIED_ONLY")

    def test_legacy_detected_is_not_rewritten_by_status_mapping(self):
        legacy = {"report_type": "legacy_reconciliation_report", "status": "DETECTED"}
        self.assertEqual(legacy["status"], "DETECTED")
        self.assertEqual(status_conclusion(legacy["status"]), "ASSESS_FINDINGS")


if __name__ == "__main__":
    unittest.main()
