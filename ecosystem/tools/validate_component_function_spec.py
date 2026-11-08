#!/usr/bin/env python3
"""Fail-closed structural validator for ATC-COMPONENT-FUNCTION-SPEC-001.json.

This validates catalog integrity, not implementation correctness. A successful run
must never be interpreted as function-level implementation or verification evidence.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

DEFAULT_CATALOG = Path("docs/architecture/ATC-COMPONENT-FUNCTION-SPEC-001.json")
FUNCTION_STATES = {
    "PROPOSED", "SPECIFIED", "IMPLEMENTED", "VERIFIED", "RESIDUAL",
    "BLOCKED", "AUDIT_REQUIRED", "DEPRECATED"
}
IST_STATES = {
    "NOT_ASSESSED", "MISSING", "PARTIAL", "PRESENT_UNVERIFIED",
    "IMPLEMENTED", "VERIFIED", "RESIDUAL", "BLOCKED"
}
DELTA_STATES = {"NOT_ASSESSED", "MATCH", "PARTIAL", "GAP", "CONFLICT", "BLOCKED"}
REQUIRED_EVIDENCE_FIELDS = {
    "repository_full_name", "branch", "commit_sha", "source_paths",
    "test_refs", "ci_evidence"
}


def fail(message: str, errors: list[str]) -> None:
    errors.append(message)


def validate(data: dict) -> dict:
    errors: list[str] = []
    warnings: list[str] = []
    if data.get("schema_id") != "ATC-COMPONENT-FUNCTION-SPEC-001":
        fail("schema_id must equal ATC-COMPONENT-FUNCTION-SPEC-001", errors)
    if not isinstance(data.get("systems"), list) or not data["systems"]:
        fail("systems must be a non-empty array", errors)
        return {"status": "FAIL", "errors": errors, "warnings": warnings}

    system_ids: set[str] = set()
    function_ids: set[str] = set()
    for system in data["systems"]:
        sid = system.get("id")
        if not sid or sid in system_ids:
            fail(f"missing or duplicate system id: {sid!r}", errors)
        system_ids.add(sid)
        if not system.get("repository") or system.get("repository") == "AUDIT_REQUIRED":
            fail(f"{sid}: repository mapping is unresolved", errors)
        system_state = system.get("soll_ist", {})
        ist = system_state.get("ist", {})
        delta = system_state.get("delta", {})
        if not ist:
            fail(f"{sid}: missing soll_ist.ist", errors)
        else:
            state = ist.get("assessment")
            if state not in IST_STATES:
                fail(f"{sid}: invalid system Ist assessment {state!r}", errors)
            if state == "VERIFIED":
                missing = [k for k in REQUIRED_EVIDENCE_FIELDS if not ist.get(k)]
                if missing:
                    fail(f"{sid}: VERIFIED without evidence fields {missing}", errors)
                if not any(
                    e.get("workflow_run_id") and e.get("job_name") and
                    e.get("step_name") and e.get("exit_code") == 0 and
                    (e.get("log_url") or e.get("artifact_url")) and
                    e.get("commit_sha") == ist.get("commit_sha")
                    for e in (ist.get("ci_evidence") or []) if isinstance(e, dict)
                ):
                    fail(f"{sid}: VERIFIED without exact Run/Job/Step/exit/log evidence", errors)
            elif state in {"NOT_ASSESSED", "PRESENT_UNVERIFIED", "PARTIAL"}:
                pass
        if delta and delta.get("status") not in DELTA_STATES:
            fail(f"{sid}: invalid delta status {delta.get('status')!r}", errors)

        for fn in system.get("functions", []):
            fid = fn.get("id")
            if not fid or fid in function_ids:
                fail(f"missing or duplicate function id: {fid!r}", errors)
            function_ids.add(fid)
            status = fn.get("status")
            if status not in FUNCTION_STATES:
                fail(f"{fid}: invalid catalog status {status!r}", errors)
            fs = fn.get("soll_ist")
            if not isinstance(fs, dict) or not isinstance(fs.get("soll"), dict) or not isinstance(fs.get("ist"), dict) or not isinstance(fs.get("delta"), dict):
                fail(f"{fid}: missing Soll/Ist/Delta object", errors)
                continue
            fi = fs["ist"]
            ds = fs["delta"].get("status")
            if fi.get("assessment") not in IST_STATES:
                fail(f"{fid}: invalid Ist assessment {fi.get('assessment')!r}", errors)
            if ds not in DELTA_STATES:
                fail(f"{fid}: invalid delta status {ds!r}", errors)
            if status == "VERIFIED" or fi.get("assessment") == "VERIFIED":
                missing = [k for k in REQUIRED_EVIDENCE_FIELDS if not fi.get(k)]
                if missing:
                    fail(f"{fid}: VERIFIED without evidence fields {missing}", errors)
                evidence = fi.get("ci_evidence") or []
                if not any(
                    isinstance(e, dict) and e.get("workflow_run_id") and e.get("job_name") and
                    e.get("step_name") and e.get("exit_code") == 0 and
                    (e.get("log_url") or e.get("artifact_url")) and
                    e.get("commit_sha") == fi.get("commit_sha")
                    for e in evidence
                ):
                    fail(f"{fid}: VERIFIED without exact-SHA Run/Job/Step/exit/log evidence", errors)
            if ds == "MATCH" and fi.get("assessment") not in {"IMPLEMENTED", "VERIFIED"}:
                fail(f"{fid}: MATCH requires IMPLEMENTED or VERIFIED Ist", errors)
            if status == "IMPLEMENTED" and fi.get("assessment") in {"NOT_ASSESSED", "MISSING", "BLOCKED"}:
                warnings.append(f"{fid}: catalog status says IMPLEMENTED while Ist is {fi.get('assessment')}")

    registry = data.get("dependency_registry", {})
    known_ids = system_ids
    for system in data["systems"]:
        for dep in system.get("dependencies", []):
            if dep not in registry and dep not in known_ids:
                fail(f"{system.get('id')}: unresolved dependency reference {dep!r}", errors)
            elif dep in registry and registry[dep].get("kind") == "catalog_system":
                target = registry[dep].get("system_id")
                if target not in known_ids:
                    fail(f"{system.get('id')}: dependency {dep!r} points to missing system {target!r}", errors)

    all_functions = [
        fn for system in data["systems"] for fn in system.get("functions", [])
        if isinstance(fn, dict)
    ]
    source_mapped = sum(bool(fn.get("soll_ist", {}).get("ist", {}).get("source_paths")) for fn in all_functions)
    tests_mapped = sum(bool(fn.get("soll_ist", {}).get("ist", {}).get("test_refs")) for fn in all_functions)
    specs_mapped = sum(bool(fn.get("soll_ist", {}).get("ist", {}).get("spec_refs")) for fn in all_functions)
    verified_functions = sum(
        fn.get("soll_ist", {}).get("ist", {}).get("assessment") == "VERIFIED"
        for fn in all_functions
    )
    system_inventory_count = sum(
        bool(system.get("soll_ist", {}).get("ist", {}).get("repository_full_name")) and
        bool(system.get("soll_ist", {}).get("ist", {}).get("commit_sha"))
        for system in data["systems"]
    )

    return {
        "schema_id": data.get("schema_id"),
        "schema_version": data.get("schema_version"),
        "system_count": len(data["systems"]),
        "function_count": len(function_ids),
        "unique_system_ids": len(system_ids),
        "unique_function_ids": len(function_ids),
        "repository_inventory": {
            "systems_with_repository_and_sha": system_inventory_count,
            "systems_total": len(data["systems"]),
            "repository_snapshots": len(data.get("repository_inventory", {}).get("snapshots", [])),
        },
        "function_coverage": {
            "source_refs_mapped": source_mapped,
            "test_refs_mapped": tests_mapped,
            "spec_refs_mapped": specs_mapped,
            "verified_functions": verified_functions,
            "functions_total": len(all_functions),
            "source_ref_gaps": len(all_functions) - source_mapped,
            "test_ref_gaps": len(all_functions) - tests_mapped,
            "spec_ref_gaps": len(all_functions) - specs_mapped,
            "note": "Unmapped references mean function-level reconciliation is incomplete; they do not prove the implementation or tests are absent.",
        },
        "catalog_validation": "PASS" if not errors else "FAIL",
        "implementation_audit": "NOT_PERFORMED_BY_STRUCTURAL_VALIDATOR",
        "status": "PASS" if not errors else "FAIL",
        "errors": errors,
        "warnings": warnings,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--catalog", default=str(DEFAULT_CATALOG))
    parser.add_argument("--output", help="Optional path to write a JSON validation report")
    args = parser.parse_args()
    try:
        data = json.loads(Path(args.catalog).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        print(json.dumps({"status": "FAIL", "errors": [str(exc)]}, indent=2))
        return 2
    report = validate(data)
    rendered = json.dumps(report, indent=2, sort_keys=True)
    print(rendered)
    if args.output:
        out = Path(args.output)
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(rendered + "\n", encoding="utf-8")
    return 0 if report["status"] == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
