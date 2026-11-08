#!/usr/bin/env python3
"""Fail-closed structural validator for the organization-wide repository integration baseline."""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ALLOWED_REPO_STATUS = {"AUDIT_REQUIRED", "ARCHIVED_REFERENCE_ONLY", "VERIFIED", "BLOCKED", "RESIDUAL"}
ALLOWED_GATE_STATUS = {"NOT_RUN", "VERIFIED", "BLOCKED", "RESIDUAL"}


def validate(doc: dict) -> tuple[list[str], dict]:
    errors: list[str] = []
    repos = doc.get("repositories")
    gates = doc.get("integration_gates")
    if doc.get("schema_id") != "ATC-ORG-REPOSITORY-INTEGRATION-001":
        errors.append("schema_id must be ATC-ORG-REPOSITORY-INTEGRATION-001")
    if not isinstance(repos, list):
        errors.append("repositories must be an array")
        repos = []
    if not isinstance(gates, list):
        errors.append("integration_gates must be an array")
        gates = []

    names = [r.get("full_name") for r in repos if isinstance(r, dict)]
    if len(names) != len(repos):
        errors.append("every repository entry must be an object with full_name")
    if len(names) != len(set(names)):
        errors.append("duplicate full_name values found")
    if len(repos) != doc.get("inventory", {}).get("observed_repository_count"):
        errors.append("inventory observed_repository_count does not match repositories length")

    active = archived = 0
    verified = 0
    for i, repo in enumerate(repos):
        if not isinstance(repo, dict):
            continue
        name = repo.get("full_name", f"index:{i}")
        status = repo.get("integration_status")
        archive = repo.get("observed_archive_state")
        if status not in ALLOWED_REPO_STATUS:
            errors.append(f"{name}: invalid integration_status {status!r}")
        if archive not in {"ACTIVE", "ARCHIVED"}:
            errors.append(f"{name}: observed_archive_state must be ACTIVE or ARCHIVED")
        if archive == "ARCHIVED":
            archived += 1
            if status != "ARCHIVED_REFERENCE_ONLY":
                errors.append(f"{name}: archived repository must be ARCHIVED_REFERENCE_ONLY")
        else:
            active += 1
            if status == "ARCHIVED_REFERENCE_ONLY":
                errors.append(f"{name}: active repository cannot be ARCHIVED_REFERENCE_ONLY")
        if status == "VERIFIED":
            verified += 1
            evidence = repo.get("required_evidence") or {}
            required = {
                "exact_head_sha": repo.get("last_verified_head_sha"),
                "tree_sha": repo.get("tree_sha"),
                "required_ci_run": evidence.get("required_ci_run"),
                "required_ci_job": evidence.get("required_ci_job"),
                "required_ci_step": evidence.get("required_ci_step"),
                "exit_code": evidence.get("exit_code"),
                "log_or_artifact_url": evidence.get("log_or_artifact_url"),
            }
            missing = [k for k, v in required.items() if v is None or v == ""]
            if missing:
                errors.append(f"{name}: VERIFIED without required exact-SHA evidence fields: {', '.join(missing)}")
            if evidence.get("exit_code") != 0:
                errors.append(f"{name}: VERIFIED requires exit_code 0")

    inventory = doc.get("inventory", {})
    if active != inventory.get("active_repository_count"):
        errors.append("inventory active_repository_count does not match observed active repositories")
    if archived != inventory.get("archived_repository_count"):
        errors.append("inventory archived_repository_count does not match observed archived repositories")

    gate_ids = [g.get("id") for g in gates if isinstance(g, dict)]
    if len(gate_ids) != len(gates):
        errors.append("every integration gate must be an object")
    if len(gate_ids) != len(set(gate_ids)):
        errors.append("duplicate integration gate IDs found")
    for gate in gates:
        if not isinstance(gate, dict):
            continue
        if gate.get("status") not in ALLOWED_GATE_STATUS:
            errors.append(f"{gate.get('id')}: invalid gate status {gate.get('status')!r}")
        if gate.get("status") == "VERIFIED" and gate.get("required_terminal_status") != "VERIFIED":
            errors.append(f"{gate.get('id')}: verified gate must declare required_terminal_status VERIFIED")

    metrics = {
        "schema_id": doc.get("schema_id"),
        "schema_version": doc.get("schema_version"),
        "repositories_total": len(repos),
        "repositories_active": active,
        "repositories_archived": archived,
        "repositories_verified": verified,
        "repositories_audit_required": sum(1 for r in repos if isinstance(r, dict) and r.get("integration_status") == "AUDIT_REQUIRED"),
        "integration_gates_total": len(gates),
        "integration_gates_verified": sum(1 for g in gates if isinstance(g, dict) and g.get("status") == "VERIFIED"),
        "integration_gates_not_run": sum(1 for g in gates if isinstance(g, dict) and g.get("status") == "NOT_RUN"),
        "validation": "FAIL" if errors else "PASS_STRUCTURAL_ONLY",
        "errors": errors,
        "warning": "Structural validation does not prove repository integration, source correctness, CI success, or release readiness."
    }
    return errors, metrics


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", default="docs/integration/ATC-ORG-REPOSITORY-INTEGRATION-001.json")
    parser.add_argument("--report", default="org-repository-integration-report.json")
    args = parser.parse_args()
    try:
        doc = json.loads(Path(args.input).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        print(f"INPUT_INCOMPLETE: {exc}", file=sys.stderr)
        return 2
    errors, metrics = validate(doc)
    Path(args.report).write_text(json.dumps(metrics, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(metrics, indent=2))
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
