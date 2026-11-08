"""Fail-closed validation for reconciliation and verification status claims."""
from __future__ import annotations

import re
from typing import Any

SHA40 = re.compile(r"^[0-9a-f]{40}$")
SHA256 = re.compile(r"^[0-9a-f]{64}$")
KNOWN_STATUSES = {"DETECTED", "RECONCILED", "VERIFIED"}


class StatusContractError(ValueError):
    """A status claim violates the versioned reconciliation status contract."""


def require_known_status(status: Any) -> str:
    if not isinstance(status, str) or status not in KNOWN_STATUSES:
        raise StatusContractError(f"unknown or missing status: {status!r}")
    return status


def validate_verification_claim(claim: dict[str, Any]) -> None:
    """Raise StatusContractError unless claim is independently verification-ready.

    This validates the shape and binding of a claim, not the truth of external
    workflow results. Callers must fetch/verify referenced evidence independently.
    """
    if not isinstance(claim, dict):
        raise StatusContractError("claim must be an object")

    status = require_known_status(claim.get("status"))
    if status != "VERIFIED":
        raise StatusContractError(f"{status} is not a verification grant")

    claim_id = claim.get("claim_id")
    scope = claim.get("scope")
    checked_sha = claim.get("checked_commit_sha")
    criteria = claim.get("criteria")
    evidence = claim.get("evidence")
    decision = claim.get("decision")

    if not isinstance(claim_id, str) or not claim_id.strip():
        raise StatusContractError("claim_id is required")
    if not isinstance(scope, dict) or not scope:
        raise StatusContractError("explicit non-empty scope is required")
    if not isinstance(checked_sha, str) or not SHA40.fullmatch(checked_sha):
        raise StatusContractError("checked_commit_sha must be a full lowercase 40-character SHA")
    if decision != "PASS":
        raise StatusContractError("verification decision must be PASS")
    if not isinstance(criteria, list) or not criteria:
        raise StatusContractError("explicit verification criteria are required")
    for index, criterion in enumerate(criteria):
        if not isinstance(criterion, dict) or not criterion.get("criterion_id"):
            raise StatusContractError(f"criterion {index} lacks criterion_id")
        if criterion.get("result") != "PASS":
            raise StatusContractError(f"criterion {index} is not explicitly PASS")
        if not criterion.get("evidence_ids"):
            raise StatusContractError(f"criterion {index} has no evidence_ids")

    if not isinstance(evidence, list) or not evidence:
        raise StatusContractError("evidence list is required")
    evidence_ids: set[str] = set()
    for index, item in enumerate(evidence):
        if not isinstance(item, dict):
            raise StatusContractError(f"evidence {index} must be an object")
        evidence_id = item.get("evidence_id")
        if not isinstance(evidence_id, str) or not evidence_id.strip() or evidence_id in evidence_ids:
            raise StatusContractError(f"evidence {index} has missing or duplicate evidence_id")
        evidence_ids.add(evidence_id)
        head = item.get("head_sha")
        if not isinstance(head, str) or not SHA40.fullmatch(head):
            raise StatusContractError(f"evidence {index} head_sha must be a full lowercase 40-character SHA")
        if head != checked_sha:
            raise StatusContractError("evidence head SHA does not exactly match checked_commit_sha")
        if not isinstance(item.get("workflow_run_id"), int) or item["workflow_run_id"] <= 0:
            raise StatusContractError(f"evidence {index} requires a positive workflow_run_id")
        if not isinstance(item.get("job"), str) or not item["job"].strip():
            raise StatusContractError(f"evidence {index} requires a job identity")
        if not isinstance(item.get("step"), str) or not item["step"].strip():
            raise StatusContractError(f"evidence {index} requires a step identity")
        artifact_sha = item.get("artifact_sha256")
        artifact_id = item.get("artifact_id")
        log_ref = item.get("log_reference")
        if artifact_sha is not None and (not isinstance(artifact_sha, str) or not SHA256.fullmatch(artifact_sha)):
            raise StatusContractError(f"evidence {index} artifact_sha256 is malformed")
        artifact_evidence_valid = (
            isinstance(artifact_id, (str, int))
            and str(artifact_id).strip()
            and isinstance(artifact_sha, str)
            and SHA256.fullmatch(artifact_sha)
        )
        log_evidence_valid = isinstance(log_ref, str) and log_ref.startswith("https://") and len(log_ref) > 8
        if not artifact_evidence_valid and not log_evidence_valid:
            raise StatusContractError(
                f"evidence {index} requires artifact_id plus SHA-256, or an HTTPS reproducible log_reference"
            )

    for index, criterion in enumerate(criteria):
        refs = criterion["evidence_ids"]
        if not isinstance(refs, list) or not refs or any(ref not in evidence_ids for ref in refs):
            raise StatusContractError(f"criterion {index} references missing evidence")

    if scope.get("kind") == "organization":
        repositories = scope.get("repositories")
        covered = scope.get("evidence_repositories")
        if not isinstance(repositories, list) or not repositories:
            raise StatusContractError("organization-wide claim requires explicit repositories")
        if not isinstance(covered, list) or set(covered) != set(repositories):
            raise StatusContractError("organization-wide claim requires evidence for every in-scope repository")


def status_conclusion(status: Any) -> str:
    """Return the only conclusion permitted by the status contract."""
    status = require_known_status(status)
    return {
        "DETECTED": "ASSESS_FINDINGS",
        "RECONCILED": "SCOPED_RECONCILIATION_PASSED",
        "VERIFIED": "SCOPED_CLAIM_VERIFIED_ONLY",
    }[status]
