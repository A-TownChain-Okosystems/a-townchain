#!/usr/bin/env python3
"""Validate ATC AI code-agent policy and SHA-bound evidence schema using stdlib only."""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
POLICY = ROOT / "config/ai-code-agent-policy.json"
PROFILES = ROOT / "config/ai-code-agent-quality-profiles.json"
SCHEMA = ROOT / "schemas/ai-agent/ATC-AI-VERIFY-001.json"

REQUIRED_STATUSES = {
    "PLANNED", "IMPLEMENTED", "VERIFYING", "VERIFIED",
    "RESIDUAL", "BLOCKED", "FAILED",
}
REQUIRED_CONTROLS = {
    "requirements_and_acceptance_criteria",
    "canonical_repository_and_implementation",
    "impact_and_dependency_review",
    "negative_boundary_and_error_path_tests",
    "regression_test_for_reproducible_bug",
    "exact_head_sha_verification",
    "record_run_job_step_exit_code_and_logs",
    "missing_stale_unreadable_or_mismatched_evidence_fails_closed",
    "preserve_existing_tests_and_assertions",
    "preserve_security_and_branch_protection_gates",
    "human_approval_required_for_merge_or_release",
}
EXPECTED_PROFILES = {
    "atclang", "a-townchain", "atc-vm", "atc-algorithm",
    "shivacore-kernel", "globus-os", "aurora-ai",
    "atc-standards", "toolchain-genesis-engine",
}


def load_json(path: Path) -> dict:
    with path.open(encoding="utf-8") as stream:
        value = json.load(stream)
    if not isinstance(value, dict):
        raise ValueError(f"{path.relative_to(ROOT)} must contain a JSON object")
    return value


def main() -> int:
    errors: list[str] = []
    try:
        policy = load_json(POLICY)
        profiles = load_json(PROFILES)
        schema = load_json(SCHEMA)
    except (OSError, json.JSONDecodeError, ValueError) as exc:
        print(f"BLOCKED: invalid or unreadable policy input: {exc}", file=sys.stderr)
        return 2

    if policy.get("policy_id") != "ATC-AI-CODE-AGENT-001":
        errors.append("unexpected policy_id")
    if set(policy.get("lifecycle", [])) != REQUIRED_STATUSES:
        errors.append("lifecycle status set does not match the required seven statuses")
    controls = policy.get("required_controls", {})
    for key in sorted(REQUIRED_CONTROLS):
        if controls.get(key) is not True:
            errors.append(f"required control must be true: {key}")
    for key in ("automatic_merge", "force_push", "gate_weakening", "test_skipping_to_hide_failure"):
        if controls.get(key) is not False:
            errors.append(f"prohibited behavior must be false: {key}")
    verification = policy.get("verification_rules", {})
    for key in ("verified_requires_all_required_gates_pass", "verified_requires_current_full_commit_sha_match", "each_retry_requires_new_diagnosis"):
        if verification.get(key) is not True:
            errors.append(f"verification rule must be true: {key}")
    if verification.get("build_success_alone_is_sufficient") is not False:
        errors.append("build-only verification must be prohibited")
    if verification.get("max_repair_attempts_per_finding") != 3:
        errors.append("repair-attempt limit must be exactly 3")
    if policy.get("evidence", {}).get("sha_length") != 40:
        errors.append("evidence SHA length must be 40")
    if policy.get("evidence", {}).get("merge_authorized_default") is not False:
        errors.append("merge authorization default must be false")
    if schema.get("$id") != "ATC-AI-VERIFY-001":
        errors.append("unexpected evidence schema id")
    if schema.get("properties", {}).get("merge_authorized", {}).get("const") is not False:
        errors.append("evidence schema must reject merge_authorized=true")
    if schema.get("properties", {}).get("commit_sha", {}).get("pattern") != "^[0-9a-f]{40}$":
        errors.append("evidence schema must require a full lowercase 40-character SHA")
    actual_profiles = set(profiles.get("profiles", {}))
    if actual_profiles != EXPECTED_PROFILES:
        errors.append(f"profile set mismatch: expected {sorted(EXPECTED_PROFILES)}, got {sorted(actual_profiles)}")
    for name, checks in profiles.get("profiles", {}).items():
        if not isinstance(checks, list) or not checks or any(not isinstance(item, str) or not item for item in checks):
            errors.append(f"profile {name} must have a non-empty list of named checks")
    if not re.fullmatch(r"^[0-9a-f]{40}$", "0" * 40):
        errors.append("internal SHA pattern self-test failed")

    if errors:
        print("BLOCKED: AI code-agent policy validation failed")
        for error in errors:
            print(f"- {error}")
        return 1
    print("PASS: AI code-agent policy, component profiles, and evidence-schema invariants are valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
