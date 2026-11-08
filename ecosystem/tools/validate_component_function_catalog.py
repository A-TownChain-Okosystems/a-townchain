#!/usr/bin/env python3
"""Fail-closed validator for the ATC component/function catalog."""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

READINESS = {
    "PLANNED", "DESIGNED", "IMPLEMENTED", "TESTED", "AUDITED",
    "PRODUCTION_READY", "BLOCKED", "RESIDUAL", "DEPRECATED",
}
GATE_STATUS = {"NOT_RUN", "VERIFIED", "BLOCKED", "RESIDUAL"}
SHA_RE = re.compile(r"^[0-9a-f]{40}$")
REQUIRED_FUNCTION_FIELDS = {
    "id", "name", "purpose", "inputs", "outputs", "failure_modes",
    "acceptance", "test_classes", "readiness",
}


def validate(doc: dict) -> tuple[list[str], dict]:
    errors: list[str] = []
    components = doc.get("components")
    gates = doc.get("production_gates")
    interfaces = doc.get("cross_component_interfaces")
    repository_coverage = doc.get("repository_coverage")
    scope = doc.get("scope", {})
    if doc.get("schema_id") != "ATC-COMPONENT-FUNCTION-CATALOG-001":
        errors.append("schema_id must be ATC-COMPONENT-FUNCTION-CATALOG-001")
    if not isinstance(components, list) or not components:
        errors.append("components must be a non-empty array")
        components = []
    if not isinstance(gates, list) or not gates:
        errors.append("production_gates must be a non-empty array")
        gates = []
    if not isinstance(interfaces, list):
        errors.append("cross_component_interfaces must be an array")
        interfaces = []
    if not isinstance(repository_coverage, list):
        errors.append("repository_coverage must be an array")
        repository_coverage = []
    repo_names = []
    active_repos = 0
    archived_repos = 0
    for index, repo in enumerate(repository_coverage):
        if not isinstance(repo, dict):
            errors.append(f"repository_coverage[{index}] must be an object")
            continue
        name = repo.get("repository")
        if not isinstance(name, str) or "/" not in name:
            errors.append(f"repository_coverage[{index}] must include full repository name")
            continue
        repo_names.append(name)
        for field in ("role", "canonicality", "observed_archive_state", "function_inventory_status", "production_readiness"):
            if not repo.get(field):
                errors.append(f"{name}: repository_coverage missing {field}")
        archive_state = repo.get("observed_archive_state")
        if archive_state == "ACTIVE":
            active_repos += 1
            if repo.get("function_inventory_status") != "RESIDUAL":
                errors.append(f"{name}: active repository function inventory must remain RESIDUAL until audited")
            if repo.get("production_readiness") == "PRODUCTION_READY":
                errors.append(f"{name}: repository coverage cannot certify production readiness")
        elif archive_state == "ARCHIVED":
            archived_repos += 1
            if repo.get("function_inventory_status") != "REFERENCE_ONLY":
                errors.append(f"{name}: archived repository must be REFERENCE_ONLY")
            if repo.get("production_readiness") != "ARCHIVED_REFERENCE_ONLY":
                errors.append(f"{name}: archived repository must not be treated as production target")
        else:
            errors.append(f"{name}: invalid observed_archive_state {archive_state!r}")
    if len(repo_names) != len(set(repo_names)):
        errors.append("duplicate full repository names in repository_coverage")
    if scope.get("organization_repository_count") != len(repository_coverage):
        errors.append("scope organization_repository_count does not match repository_coverage")
    if scope.get("active_repository_count") != active_repos:
        errors.append("scope active_repository_count does not match observed ACTIVE repositories")
    if scope.get("archived_repository_count") != archived_repos:
        errors.append("scope archived_repository_count does not match observed ARCHIVED repositories")
    if active_repos + archived_repos != len(repository_coverage):
        errors.append("every covered repository must be ACTIVE or ARCHIVED")

    component_ids: list[str] = []
    function_ids: list[str] = []
    component_by_id: dict[str, dict] = {}
    total_functions = 0
    production_ready_components = 0
    production_ready_functions = 0

    for index, component in enumerate(components):
        if not isinstance(component, dict):
            errors.append(f"components[{index}] must be an object")
            continue
        cid = component.get("id")
        if not isinstance(cid, str) or not cid:
            errors.append(f"components[{index}] missing id")
            continue
        component_ids.append(cid)
        component_by_id[cid] = component
        for field in ("name", "layer", "canonical_owner", "role", "criticality", "depends_on", "functions"):
            if field not in component or component[field] in (None, ""):
                errors.append(f"{cid}: missing required field {field}")
        if component.get("readiness") not in READINESS:
            errors.append(f"{cid}: invalid readiness {component.get('readiness')!r}")
        if component.get("readiness") == "PRODUCTION_READY":
            production_ready_components += 1
            _check_evidence(component, cid, errors)
        deps = component.get("depends_on", [])
        if not isinstance(deps, list):
            errors.append(f"{cid}: depends_on must be an array")
            deps = []
        funcs = component.get("functions")
        if not isinstance(funcs, list) or not funcs:
            errors.append(f"{cid}: functions must be a non-empty array")
            continue
        for fidx, fn in enumerate(funcs):
            total_functions += 1
            if not isinstance(fn, dict):
                errors.append(f"{cid}: functions[{fidx}] must be an object")
                continue
            fid = fn.get("id")
            if not isinstance(fid, str) or not fid:
                errors.append(f"{cid}: functions[{fidx}] missing id")
                continue
            function_ids.append(fid)
            missing = sorted(REQUIRED_FUNCTION_FIELDS - set(fn))
            if missing:
                errors.append(f"{fid}: missing required fields: {', '.join(missing)}")
            if fn.get("readiness") not in READINESS:
                errors.append(f"{fid}: invalid readiness {fn.get('readiness')!r}")
            for list_field in ("inputs", "outputs", "failure_modes", "acceptance", "test_classes"):
                value = fn.get(list_field)
                if not isinstance(value, list) or not value or any(not isinstance(x, str) or not x.strip() for x in value):
                    errors.append(f"{fid}: {list_field} must be a non-empty array of non-empty strings")
            if fn.get("readiness") == "PRODUCTION_READY":
                production_ready_functions += 1
                _check_evidence(fn, fid, errors)

    if len(component_ids) != len(set(component_ids)):
        errors.append("duplicate component IDs found")
    if len(function_ids) != len(set(function_ids)):
        errors.append("duplicate function IDs found")
    all_component_ids = set(component_ids)
    for cid, component in component_by_id.items():
        for dep in component.get("depends_on", []) if isinstance(component.get("depends_on", []), list) else []:
            if dep not in all_component_ids:
                errors.append(f"{cid}: unknown dependency {dep}")
    gate_ids = [g.get("id") for g in gates if isinstance(g, dict)]
    if len(gate_ids) != len(gates):
        errors.append("every production gate must be an object")
    if len(gate_ids) != len(set(gate_ids)):
        errors.append("duplicate production gate IDs found")
    for gate in gates:
        if not isinstance(gate, dict):
            continue
        if gate.get("status") not in GATE_STATUS:
            errors.append(f"{gate.get('id')}: invalid gate status {gate.get('status')!r}")
        if not isinstance(gate.get("acceptance"), list) or not gate["acceptance"]:
            errors.append(f"{gate.get('id')}: acceptance must be a non-empty array")

    interface_ids: list[str] = []
    for i, interface in enumerate(interfaces):
        if not isinstance(interface, dict):
            errors.append(f"cross_component_interfaces[{i}] must be an object")
            continue
        iid = interface.get("id")
        interface_ids.append(iid)
        for field in ("producer", "consumer", "contract", "status"):
            if not interface.get(field):
                errors.append(f"{iid}: missing required field {field}")
        if interface.get("producer") not in all_component_ids:
            errors.append(f"{iid}: unknown producer {interface.get('producer')!r}")
        if interface.get("consumer") not in all_component_ids:
            errors.append(f"{iid}: unknown consumer {interface.get('consumer')!r}")
        if interface.get("status") not in GATE_STATUS:
            errors.append(f"{iid}: invalid interface status {interface.get('status')!r}")
    if len(interface_ids) != len(set(interface_ids)):
        errors.append("duplicate interface IDs found")

    release = doc.get("release_decision", {})
    if release.get("status") == "PRODUCTION_READY":
        if errors:
            errors.append("release_decision cannot be PRODUCTION_READY while validation errors exist")
        if any(g.get("status") != "VERIFIED" for g in gates if isinstance(g, dict)):
            errors.append("release_decision PRODUCTION_READY requires every production gate VERIFIED")
        if any(c.get("readiness") != "PRODUCTION_READY" for c in components if isinstance(c, dict)):
            errors.append("release_decision PRODUCTION_READY requires every listed component PRODUCTION_READY")

    metrics = {
        "schema_id": doc.get("schema_id"),
        "schema_version": doc.get("schema_version"),
        "components_total": len(components),
        "repositories_covered": len(repository_coverage),
        "active_repositories_audit_required": active_repos,
        "archived_repositories_reference_only": archived_repos,
        "functions_total": total_functions,
        "interfaces_total": len(interfaces),
        "production_gates_total": len(gates),
        "components_production_ready": production_ready_components,
        "functions_production_ready": production_ready_functions,
        "validation": "FAIL" if errors else "PASS_STRUCTURAL_ONLY",
        "errors": errors,
        "warning": "Structural validity does not prove implementation correctness, security, interface compatibility, or release readiness.",
    }
    return errors, metrics


def _check_evidence(item: dict, item_id: str, errors: list[str]) -> None:
    evidence = item.get("production_evidence")
    if not isinstance(evidence, dict):
        errors.append(f"{item_id}: PRODUCTION_READY requires production_evidence object")
        return
    for field in ("source_sha", "run_id", "job", "step", "exit_code", "log_or_artifact_url", "review_ref"):
        if evidence.get(field) in (None, ""):
            errors.append(f"{item_id}: PRODUCTION_READY evidence missing {field}")
    if evidence.get("source_sha") and not SHA_RE.fullmatch(str(evidence["source_sha"])):
        errors.append(f"{item_id}: production_evidence.source_sha must be a full 40-character SHA")
    if evidence.get("exit_code") != 0:
        errors.append(f"{item_id}: production_evidence.exit_code must be 0")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", default="docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json")
    parser.add_argument("--report", default="component-function-catalog-report.json")
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
