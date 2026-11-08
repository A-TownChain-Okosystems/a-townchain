import hashlib
import json
import unittest

from scripts.verify_reconciliation_evidence import (
    EvidenceVerificationError,
    GitHubActionsEvidenceVerifier,
)

SHA = "a" * 40
LOGS = b"Step: verify-status-contract\nexit code: 0\n"
ARCHIVE = b"fake-but-deterministic-zip-bytes"


class Response:
    def __init__(self, body):
        self.body = body

    def __enter__(self):
        return self

    def __exit__(self, *args):
        return False

    def read(self):
        return self.body


def fixture_responses():
    return {
        "/actions/runs/123": {
            "id": 123, "head_sha": SHA, "status": "completed", "conclusion": "success"
        },
        "/actions/runs/123/jobs?per_page=100": {
            "total_count": 1,
            "jobs": [{
                "id": 456, "name": "contract-tests", "head_sha": SHA,
                "status": "completed", "conclusion": "success",
                "steps": [{
                    "name": "verify-status-contract", "status": "completed", "conclusion": "success"
                }],
            }]
        },
        "/actions/jobs/456/logs": LOGS,
        "/actions/runs/123/artifacts?per_page=100": {
            "total_count": 1,
            "artifacts": [{
                "id": 789, "expired": False, "workflow_run": {"id": 123}
            }]
        },
        "/actions/artifacts/789/zip": ARCHIVE,
    }


def valid_claim():
    return {
        "status": "VERIFIED",
        "decision": "PASS",
        "checked_commit_sha": SHA,
        "evidence": [{
            "evidence_id": "e1", "head_sha": SHA, "workflow_run_id": 123,
            "job": "contract-tests", "step": "verify-status-contract",
            "log_sha256": hashlib.sha256(LOGS).hexdigest(),
            "artifact_id": 789, "artifact_sha256": hashlib.sha256(ARCHIVE).hexdigest(),
        }],
    }


def verifier(responses=None):
    responses = responses if responses is not None else fixture_responses()

    def open_request(request, timeout=30):
        path = "/" + request.full_url.split("/repos/owner/repo/", 1)[1]
        if path not in responses:
            raise AssertionError(f"unexpected API request: {path}")
        value = responses[path]
        body = value if isinstance(value, bytes) else json.dumps(value).encode()
        return Response(body)

    return GitHubActionsEvidenceVerifier("owner/repo", "test-token", opener=open_request)


class VerifyExternalEvidenceTests(unittest.TestCase):
    def test_success_requires_real_run_job_step_logs_and_artifact_hash(self):
        result = verifier().verify_claim(valid_claim())
        self.assertEqual(result[0]["result"], "VERIFIED")
        self.assertEqual(result[0]["job_log_sha256"], hashlib.sha256(LOGS).hexdigest())
        self.assertTrue(result[0]["artifact_verified"])

    def test_wrong_run_sha_blocks(self):
        responses = fixture_responses()
        responses["/actions/runs/123"]["head_sha"] = "c" * 40
        with self.assertRaisesRegex(EvidenceVerificationError, "run head SHA mismatch"):
            verifier(responses).verify_claim(valid_claim())

    def test_failed_job_blocks(self):
        responses = fixture_responses()
        responses["/actions/runs/123/jobs?per_page=100"]["jobs"][0]["conclusion"] = "failure"
        with self.assertRaisesRegex(EvidenceVerificationError, "job did not complete successfully"):
            verifier(responses).verify_claim(valid_claim())

    def test_missing_log_hash_blocks(self):
        claim = valid_claim()
        del claim["evidence"][0]["log_sha256"]
        with self.assertRaisesRegex(EvidenceVerificationError, "log_sha256 must be supplied"):
            verifier().verify_claim(claim)

    def test_incomplete_jobs_page_blocks(self):
        responses = fixture_responses()
        responses["/actions/runs/123/jobs?per_page=100"]["total_count"] = 2
        with self.assertRaisesRegex(EvidenceVerificationError, "jobs response is incomplete"):
            verifier(responses).verify_claim(valid_claim())

    def test_incomplete_artifacts_page_blocks(self):
        responses = fixture_responses()
        responses["/actions/runs/123/artifacts?per_page=100"]["total_count"] = 2
        with self.assertRaisesRegex(EvidenceVerificationError, "artifact response is incomplete"):
            verifier(responses).verify_claim(valid_claim())

    def test_missing_step_blocks(self):
        responses = fixture_responses()
        responses["/actions/runs/123/jobs?per_page=100"]["jobs"][0]["steps"] = []
        with self.assertRaisesRegex(EvidenceVerificationError, "step is missing"):
            verifier(responses).verify_claim(valid_claim())

    def test_failed_step_blocks(self):
        responses = fixture_responses()
        responses["/actions/runs/123/jobs?per_page=100"]["jobs"][0]["steps"][0]["conclusion"] = "failure"
        with self.assertRaisesRegex(EvidenceVerificationError, "step did not complete successfully"):
            verifier(responses).verify_claim(valid_claim())

    def test_missing_logs_blocks(self):
        responses = fixture_responses()
        responses["/actions/jobs/456/logs"] = b""
        with self.assertRaisesRegex(EvidenceVerificationError, "empty"):
            verifier(responses).verify_claim(valid_claim())

    def test_missing_artifact_blocks(self):
        responses = fixture_responses()
        responses["/actions/runs/123/artifacts?per_page=100"]["artifacts"] = []
        responses["/actions/runs/123/artifacts?per_page=100"]["total_count"] = 0
        with self.assertRaisesRegex(EvidenceVerificationError, "artifact is missing"):
            verifier(responses).verify_claim(valid_claim())

    def test_artifact_hash_mismatch_blocks(self):
        claim = valid_claim()
        claim["evidence"][0]["artifact_sha256"] = "0" * 64
        with self.assertRaisesRegex(EvidenceVerificationError, "artifact ZIP SHA-256 mismatch"):
            verifier().verify_claim(claim)

    def test_unavailable_endpoint_blocks_closed(self):
        def unavailable(request, timeout=30):
            raise OSError("simulated unavailable endpoint")
        subject = GitHubActionsEvidenceVerifier("owner/repo", "test-token", opener=unavailable)
        with self.assertRaisesRegex(EvidenceVerificationError, "endpoint unavailable"):
            subject.verify_claim(valid_claim())

    def test_mismatched_evidence_sha_blocks_before_api_access(self):
        claim = valid_claim()
        claim["evidence"][0]["head_sha"] = "c" * 40
        with self.assertRaisesRegex(EvidenceVerificationError, "does not match checked commit"):
            verifier().verify_claim(claim)


if __name__ == "__main__":
    unittest.main()
