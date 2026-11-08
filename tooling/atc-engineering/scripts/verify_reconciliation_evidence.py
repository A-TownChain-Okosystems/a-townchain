"""Verify external GitHub Actions evidence before accepting VERIFIED claims.

This module is intentionally separate from the status-shape validator. It queries
GitHub's API for the run, job, step, job logs, and (when claimed) artifact bytes.
Missing, inaccessible, mismatched, or unsuccessful evidence fails closed.
"""
from __future__ import annotations

import hashlib
import json
import os
import re
import sys
import urllib.error
import urllib.request
from typing import Any

SHA40 = re.compile(r"^[0-9a-f]{40}$")
SHA256 = re.compile(r"^[0-9a-f]{64}$")
API = "https://api.github.com"


class EvidenceVerificationError(ValueError):
    """External evidence cannot be independently verified."""


class GitHubActionsEvidenceVerifier:
    def __init__(self, repository: str, token: str, opener=None) -> None:
        if not re.fullmatch(r"[^/\s]+/[^/\s]+", repository):
            raise EvidenceVerificationError("repository must be owner/name")
        if not token:
            raise EvidenceVerificationError("GITHUB_TOKEN is required")
        self.repository = repository
        self.token = token
        self.opener = opener or urllib.request.urlopen

    def _get(self, path: str, *, accept: str = "application/vnd.github+json") -> bytes:
        request = urllib.request.Request(
            f"{API}/repos/{self.repository}/{path.lstrip('/')}",
            headers={
                "Accept": accept,
                "Authorization": f"Bearer {self.token}",
                "X-GitHub-Api-Version": "2022-11-28",
                "User-Agent": "atc-reconciliation-evidence-verifier",
            },
        )
        try:
            with self.opener(request, timeout=30) as response:
                body = response.read()
        except (urllib.error.URLError, TimeoutError, OSError) as exc:
            raise EvidenceVerificationError(f"evidence endpoint unavailable: {path}: {exc}") from exc
        if not body:
            raise EvidenceVerificationError(f"empty response for required evidence endpoint: {path}")
        return body

    def _json(self, path: str) -> dict[str, Any]:
        try:
            value = json.loads(self._get(path))
        except (json.JSONDecodeError, UnicodeDecodeError) as exc:
            raise EvidenceVerificationError(f"invalid JSON from evidence endpoint: {path}") from exc
        if not isinstance(value, dict):
            raise EvidenceVerificationError(f"unexpected JSON shape from evidence endpoint: {path}")
        return value

    def verify_item(self, item: dict[str, Any], checked_sha: str) -> dict[str, Any]:
        if not isinstance(item, dict):
            raise EvidenceVerificationError("each evidence item must be an object")
        run_id = item.get("workflow_run_id")
        job_name = item.get("job")
        step_name = item.get("step")
        evidence_sha = item.get("head_sha")
        if not isinstance(run_id, int) or isinstance(run_id, bool) or run_id <= 0:
            raise EvidenceVerificationError("workflow_run_id must be a positive integer")
        if not isinstance(checked_sha, str) or not SHA40.fullmatch(checked_sha):
            raise EvidenceVerificationError("checked_commit_sha must be a full lowercase SHA")
        if evidence_sha != checked_sha or not isinstance(evidence_sha, str) or not SHA40.fullmatch(evidence_sha):
            raise EvidenceVerificationError("evidence head SHA does not match checked commit SHA")
        if not isinstance(job_name, str) or not job_name.strip():
            raise EvidenceVerificationError("job identity is required")
        if not isinstance(step_name, str) or not step_name.strip():
            raise EvidenceVerificationError("step identity is required")

        run = self._json(f"actions/runs/{run_id}")
        if run.get("id") != run_id:
            raise EvidenceVerificationError("workflow run ID mismatch")
        if run.get("head_sha") != checked_sha:
            raise EvidenceVerificationError("workflow run head SHA mismatch")
        if run.get("status") != "completed" or run.get("conclusion") != "success":
            raise EvidenceVerificationError("workflow run is not completed successfully")

        jobs_payload = self._json(f"actions/runs/{run_id}/jobs?per_page=100")
        jobs = jobs_payload.get("jobs")
        if not isinstance(jobs, list):
            raise EvidenceVerificationError("workflow jobs list is missing")
        total_jobs = jobs_payload.get("total_count")
        if not isinstance(total_jobs, int) or total_jobs != len(jobs):
            raise EvidenceVerificationError("workflow jobs response is incomplete or paginated; refusing partial evidence")
        matches = [job for job in jobs if isinstance(job, dict) and job.get("name") == job_name]
        if len(matches) != 1:
            raise EvidenceVerificationError("named job is missing or ambiguous")
        job = matches[0]
        if job.get("head_sha") not in (None, checked_sha):
            raise EvidenceVerificationError("job head SHA mismatch")
        if job.get("status") != "completed" or job.get("conclusion") != "success":
            raise EvidenceVerificationError("named job did not complete successfully")
        job_id = job.get("id")
        if not isinstance(job_id, int) or job_id <= 0:
            raise EvidenceVerificationError("named job has no valid job ID")
        steps = job.get("steps")
        if not isinstance(steps, list):
            raise EvidenceVerificationError("job step list is missing")
        step_matches = [step for step in steps if isinstance(step, dict) and step.get("name") == step_name]
        if len(step_matches) != 1:
            raise EvidenceVerificationError("named step is missing or ambiguous")
        step = step_matches[0]
        if step.get("status") != "completed" or step.get("conclusion") != "success":
            raise EvidenceVerificationError("named step did not complete successfully")

        logs = self._get(f"actions/jobs/{job_id}/logs", accept="application/vnd.github+json")
        log_sha256 = hashlib.sha256(logs).hexdigest()
        if not logs.strip():
            raise EvidenceVerificationError("job logs are empty")
        expected_log_hash = item.get("log_sha256")
        if not isinstance(expected_log_hash, str) or not SHA256.fullmatch(expected_log_hash):
            raise EvidenceVerificationError("log_sha256 must be supplied as a full lowercase SHA-256")
        if log_sha256 != expected_log_hash:
            raise EvidenceVerificationError("job log SHA-256 mismatch")

        artifact_verified = False
        artifact_id = item.get("artifact_id")
        artifact_sha256 = item.get("artifact_sha256")
        if artifact_id is not None or artifact_sha256 is not None:
            if not isinstance(artifact_id, int) or isinstance(artifact_id, bool) or artifact_id <= 0:
                raise EvidenceVerificationError("artifact_id must be a positive integer")
            if not isinstance(artifact_sha256, str) or not SHA256.fullmatch(artifact_sha256):
                raise EvidenceVerificationError("artifact_sha256 must be a full lowercase SHA-256")
            artifacts_payload = self._json(f"actions/runs/{run_id}/artifacts?per_page=100")
            artifacts = artifacts_payload.get("artifacts")
            if not isinstance(artifacts, list):
                raise EvidenceVerificationError("workflow artifact list is missing")
            total_artifacts = artifacts_payload.get("total_count")
            if not isinstance(total_artifacts, int) or total_artifacts != len(artifacts):
                raise EvidenceVerificationError("workflow artifact response is incomplete or paginated; refusing partial evidence")
            found = [a for a in artifacts if isinstance(a, dict) and a.get("id") == artifact_id]
            if len(found) != 1:
                raise EvidenceVerificationError("artifact is missing or ambiguous for the referenced run")
            artifact = found[0]
            if artifact.get("expired") is True or artifact.get("workflow_run", {}).get("id") != run_id:
                raise EvidenceVerificationError("artifact is expired or belongs to another workflow run")
            archive = self._get(f"actions/artifacts/{artifact_id}/zip", accept="application/vnd.github+json")
            actual_hash = hashlib.sha256(archive).hexdigest()
            if actual_hash != artifact_sha256:
                raise EvidenceVerificationError("artifact ZIP SHA-256 mismatch")
            artifact_verified = True

        return {
            "workflow_run_id": run_id,
            "run_conclusion": run["conclusion"],
            "job_id": job_id,
            "job": job_name,
            "step": step_name,
            "step_conclusion": step["conclusion"],
            "job_log_sha256": log_sha256,
            "artifact_verified": artifact_verified,
            "result": "VERIFIED",
        }

    def verify_claim(self, claim: dict[str, Any]) -> list[dict[str, Any]]:
        if not isinstance(claim, dict) or claim.get("status") != "VERIFIED" or claim.get("decision") != "PASS":
            raise EvidenceVerificationError("only explicit VERIFIED/PASS claims can be externally verified")
        checked_sha = claim.get("checked_commit_sha")
        evidence = claim.get("evidence")
        if not isinstance(checked_sha, str) or not SHA40.fullmatch(checked_sha):
            raise EvidenceVerificationError("checked_commit_sha must be a full lowercase SHA")
        if not isinstance(evidence, list) or not evidence:
            raise EvidenceVerificationError("claim must contain non-empty evidence")
        return [self.verify_item(item, checked_sha) for item in evidence]


def main(argv: list[str] | None = None) -> int:
    argv = argv if argv is not None else sys.argv[1:]
    if len(argv) != 2:
        print("usage: python scripts/verify_reconciliation_evidence.py OWNER/REPO CLAIM.json", file=sys.stderr)
        return 2
    token = os.environ.get("GITHUB_TOKEN")
    if not token:
        print("BLOCKED: GITHUB_TOKEN is required", file=sys.stderr)
        return 2
    try:
        with open(argv[1], encoding="utf-8") as source:
            claim = json.load(source)
        verifier = GitHubActionsEvidenceVerifier(argv[0], token)
        result = verifier.verify_claim(claim)
    except (OSError, json.JSONDecodeError, EvidenceVerificationError) as exc:
        print(f"BLOCKED: {exc}", file=sys.stderr)
        return 1
    print(json.dumps({"status": "VERIFIED", "evidence": result}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
