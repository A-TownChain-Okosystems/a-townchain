#!/usr/bin/env python3
"""Fail-closed structural validator for ATC-PRIVACY-POLICY-001 JSON exports.

This tool validates metadata, stable requirement IDs, status vocabulary and
evidence completeness. It does not assess GDPR applicability or legal compliance.
Only Python's standard library is required.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

SCHEMA_ID = "ATC-PRIVACY-POLICY-001"
POLICY_STATUSES = {"DRAFT", "REVIEW", "APPROVED", "DEPRECATED"}
NORMATIVE_LEVELS = {"MUST", "SHOULD", "MAY"}
MAPPING_STATUSES = {"MAPPED", "MISSING", "DUPLICATE", "UNRESOLVED", "EXCLUDED"}
IMPLEMENTATION_STATUSES = {
    "NOT_ASSESSED", "PLANNED", "IMPLEMENTED", "VERIFIED",
    "RESIDUAL", "BLOCKED", "EXCLUDED",
}
REQUIREMENT_ID = re.compile(r"^ATC-PRIV-[0-9]{3}$")
FULL_SHA = re.compile(r"^[0-9a-f]{40}$")
HEX_SHA = re.compile(r"^[0-9a-f]{40,64}$")


def _nonempty(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def validate(document: Any) -> list[str]:
    """Return deterministic validation errors; empty means structurally valid."""
    errors: list[str] = []
    if not isinstance(document, dict):
        return ["root must be a JSON object"]

    if document.get("schema_id") != SCHEMA_ID:
        errors.append(f"schema_id must be {SCHEMA_ID}")
    if not _nonempty(document.get("version")):
        errors.append("version must be a non-empty string")
    if document.get("status") not in POLICY_STATUSES:
        errors.append(f"status must be one of {sorted(POLICY_STATUSES)}")
    if not isinstance(document.get("requirements"), list) or not document["requirements"]:
        errors.append("requirements must be a non-empty array")
        return errors

    requirements = document["requirements"]
    seen: set[str] = set()
    for index, req in enumerate(requirements):
        prefix = f"requirements[{index}]"
        if not isinstance(req, dict):
            errors.append(f"{prefix} must be an object")
            continue

        rid = req.get("id")
        if not isinstance(rid, str) or not REQUIREMENT_ID.fullmatch(rid):
            errors.append(f"{prefix}.id must match ATC-PRIV-NNN")
        elif rid in seen:
            errors.append(f"duplicate requirement id: {rid}")
        else:
            seen.add(rid)

        for field in ("title", "statement", "category"):
            if not _nonempty(req.get(field)):
                errors.append(f"{prefix}.{field} must be a non-empty string")
        if req.get("normative_level") not in NORMATIVE_LEVELS:
            errors.append(f"{prefix}.normative_level must be one of {sorted(NORMATIVE_LEVELS)}")
        if req.get("mapping_status") not in MAPPING_STATUSES:
            errors.append(f"{prefix}.mapping_status must be one of {sorted(MAPPING_STATUSES)}")
        impl = req.get("implementation_status")
        if impl not in IMPLEMENTATION_STATUSES:
            errors.append(f"{prefix}.implementation_status must be one of {sorted(IMPLEMENTATION_STATUSES)}")

        targets = req.get("catalog_refs", [])
        if not isinstance(targets, list) or any(not _nonempty(x) for x in targets):
            errors.append(f"{prefix}.catalog_refs must be an array of non-empty strings")
        if req.get("mapping_status") == "MAPPED" and not targets:
            errors.append(f"{prefix}: MAPPED requires at least one catalog_refs entry")
        if req.get("mapping_status") in {"MISSING", "UNRESOLVED"} and targets:
            errors.append(f"{prefix}: {req.get('mapping_status')} must not claim resolved catalog_refs")
        if req.get("mapping_status") == "EXCLUDED" and not _nonempty(req.get("exclusion_reason")):
            errors.append(f"{prefix}: EXCLUDED requires exclusion_reason")
        if req.get("mapping_status") == "DUPLICATE" and not _nonempty(req.get("duplicate_of")):
            errors.append(f"{prefix}: DUPLICATE requires duplicate_of")

        evidence = req.get("evidence_refs", [])
        if not isinstance(evidence, list):
            errors.append(f"{prefix}.evidence_refs must be an array")
            evidence = []
        if impl in {"IMPLEMENTED", "VERIFIED"}:
            if not evidence:
                errors.append(f"{prefix}: {impl} requires evidence_refs; missing evidence is fail-closed")
            for eidx, item in enumerate(evidence):
                errors.extend(_validate_evidence(item, f"{prefix}.evidence_refs[{eidx}]"))
        if impl == "VERIFIED" and req.get("mapping_status") != "MAPPED":
            errors.append(f"{prefix}: VERIFIED requires mapping_status MAPPED")

    # Never assume the IDs 001..144 are 144 independent functions. Validate only
    # an explicit declared count and report coverage gaps without inventing entries.
    declared = document.get("declared_requirement_count")
    if declared is not None and (not isinstance(declared, int) or isinstance(declared, bool) or declared < 1):
        errors.append("declared_requirement_count must be a positive integer")
    elif isinstance(declared, int) and declared != len(requirements):
        errors.append(
            f"declared_requirement_count={declared} but requirements contains {len(requirements)} entries"
        )

    if document.get("status") == "APPROVED":
        approval = document.get("approval")
        if not isinstance(approval, dict):
            errors.append("APPROVED policy requires an approval object")
        else:
            for field in ("approver", "approved_at", "review_ref"):
                if not _nonempty(approval.get(field)):
                    errors.append(f"APPROVED policy requires approval.{field}")
    return errors


def _validate_evidence(item: Any, path: str) -> list[str]:
    errors: list[str] = []
    if not isinstance(item, dict):
        return [f"{path} must be an object"]
    for field in ("repository", "commit_sha", "path", "symbol", "blob_sha",
                  "test_id", "workflow_run_id", "job", "step", "review_ref"):
        if not _nonempty(item.get(field)):
            errors.append(f"{path}.{field} is required")
    if item.get("repository") and (
        not isinstance(item["repository"], str) or "/" not in item["repository"]
    ):
        errors.append(f"{path}.repository must be owner/repository")
    if item.get("commit_sha") and not (
        isinstance(item["commit_sha"], str) and FULL_SHA.fullmatch(item["commit_sha"])
    ):
        errors.append(f"{path}.commit_sha must be a full 40-character lowercase SHA")
    if item.get("blob_sha") and not (
        isinstance(item["blob_sha"], str) and HEX_SHA.fullmatch(item["blob_sha"])
    ):
        errors.append(f"{path}.blob_sha must be a full hexadecimal Git object SHA")
    if item.get("exit_code") != 0:
        errors.append(f"{path}.exit_code must be exactly 0")
    hashes = item.get("artifact_sha256", [])
    if not isinstance(hashes, list) or any(
        not isinstance(value, str) or not re.fullmatch(r"[0-9a-f]{64}", value)
        for value in hashes
    ):
        errors.append(f"{path}.artifact_sha256 must be an array of 64-character lowercase SHA-256 hashes")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", help="Path to the machine-readable privacy policy JSON")
    parser.add_argument("--report", help="Optional path for a JSON validation report")
    args = parser.parse_args()
    try:
        document = json.loads(Path(args.input).read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        print(json.dumps({"status": "BLOCKED", "errors": [str(exc)]}, indent=2), file=sys.stderr)
        return 2

    errors = validate(document)
    report = {
        "schema_id": SCHEMA_ID,
        "status": "FAIL" if errors else "PASS_STRUCTURAL_ONLY",
        "requirements_total": len(document.get("requirements", [])) if isinstance(document, dict) and isinstance(document.get("requirements"), list) else 0,
        "errors": errors,
        "legal_compliance": "NOT_ASSESSED",
        "warning": "Structural validity does not establish implementation, security, GDPR applicability, or legal compliance.",
    }
    rendered = json.dumps(report, indent=2, ensure_ascii=False) + "\n"
    print(rendered, end="")
    if args.report:
        try:
            Path(args.report).write_text(rendered, encoding="utf-8")
        except OSError as exc:
            print(f"cannot write report: {exc}", file=sys.stderr)
            return 2
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
