#!/usr/bin/env python3
"""Fail-closed structural and cross-record validation for the ATC delivery registry."""
from __future__ import annotations
import copy
import json
import sys
from pathlib import Path

from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "control-plane" / "DELIVERY_REGISTRY.json"
SCHEMA = ROOT / "control-plane" / "DELIVERY_REGISTRY.schema.json"

def validate(document: object, schema: dict) -> list[str]:
    errors: list[str] = []
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    errors.extend(f"schema:{e.json_path}: {e.message}" for e in validator.iter_errors(document))
    if not isinstance(document, dict):
        return errors
    statuses = document.get("status_enum")
    priorities = document.get("priority_enum")
    if not isinstance(statuses, list) or not statuses or any(not isinstance(x, str) for x in statuses):
        errors.append("status_enum must be a non-empty list of strings")
        statuses = []
    if not isinstance(priorities, list) or not priorities:
        errors.append("priority_enum must be a non-empty list")
        priorities = []
    def walk(value: object, path: str = "$") -> None:
        if isinstance(value, dict):
            if "status" in value and value["status"] not in statuses:
                errors.append(f"{path}.status is outside status_enum: {value['status']!r}")
            if "priority" in value and value["priority"] not in priorities:
                errors.append(f"{path}.priority is outside priority_enum: {value['priority']!r}")
            for key, child in value.items():
                walk(child, f"{path}.{key}")
        elif isinstance(value, list):
            for i, child in enumerate(value):
                walk(child, f"{path}[{i}]")
    walk(document)
    for key in ("documentation_index", "roadmap", "milestones", "checkpoints", "components", "functions", "sprints", "todos", "repository_inventory", "repository_delivery_plans"):
        records = document.get(key)
        if not isinstance(records, list):
            continue
        ids = [x.get("id") for x in records if isinstance(x, dict)]
        if len(ids) != len(set(ids)):
            errors.append(f"{key} contains duplicate IDs")
    inventory = document.get("repository_inventory", [])
    plans = document.get("repository_delivery_plans", [])
    if isinstance(inventory, list) and isinstance(plans, list):
        inv_repos = [x.get("repository") for x in inventory if isinstance(x, dict)]
        plan_repos = [x.get("repository") for x in plans if isinstance(x, dict)]
        if len(inv_repos) != len(set(inv_repos)):
            errors.append("repository_inventory contains duplicate repository names")
        if len(plan_repos) != len(set(plan_repos)):
            errors.append("repository_delivery_plans contains duplicate repository names")
        if set(inv_repos) != set(plan_repos):
            errors.append("repository inventory and delivery-plan repository sets differ")
    census = document.get("repository_census")
    if isinstance(census, dict):
        if census.get("total") != census.get("active", 0) + census.get("archived", 0):
            errors.append("repository_census.total must equal active + archived")
        if isinstance(inventory, list) and census.get("total") != len(inventory):
            errors.append("repository_census.total does not match repository_inventory length")
        if isinstance(inventory, list):
            active = sum(1 for x in inventory if isinstance(x, dict) and x.get("repository_state") == "ACTIVE")
            archived = sum(1 for x in inventory if isinstance(x, dict) and x.get("repository_state") == "ARCHIVED")
            if active != census.get("active") or archived != census.get("archived"):
                errors.append("repository_census active/archived counts do not match inventory")
    return errors

def main() -> int:
    try:
        document = json.loads(REGISTRY.read_text(encoding="utf-8"))
        schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    except Exception as exc:
        print(f"FAIL-CLOSED: cannot load registry/schema: {exc}", file=sys.stderr)
        return 2
    errors = validate(document, schema)
    if "--self-test" in sys.argv:
        mutated = copy.deepcopy(document)
        mutated["registry_id"] = "WRONG"
        if not validate(mutated, schema):
            errors.append("self-test failed: invalid registry_id was not rejected")
        mutated = copy.deepcopy(document)
        mutated["repository_census"]["total"] = -1
        if not validate(mutated, schema):
            errors.append("self-test failed: invalid census was not rejected")
        mutated = copy.deepcopy(document)
        mutated["components"][0]["status"] = "UNKNOWN_STATUS"
        if not validate(mutated, schema):
            errors.append("self-test failed: invalid nested status was not rejected")
        if errors:
            print("\n".join(errors), file=sys.stderr)
            return 1
        print("PASS: schema and semantic negative self-tests")
        return 0
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print("PASS: DELIVERY_REGISTRY.json matches Draft 2020-12 schema and semantic invariants")
    print(f"repositories={len(document['repository_inventory'])}; plans={len(document['repository_delivery_plans'])}; components={len(document['components'])}; functions={len(document['functions'])}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
