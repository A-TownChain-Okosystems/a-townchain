#!/usr/bin/env python3
"""Fail-closed structural validator for ATC-CATALOG-TAXONOMY-001.

This validates taxonomy vocabulary and any taxonomy bindings present in the
canonical catalog. It deliberately does not claim source reconciliation or
runtime verification.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

TAXONOMY_SCHEMA_ID = "ATC-CATALOG-TAXONOMY-001"
CATALOG_SCHEMA_ID = "ATC-COMPONENT-FUNCTION-CATALOG-001"
CLASSIFICATION_STATES = {"MAPPED", "EXCLUDED", "UNRESOLVED"}
IMPLEMENTATION_STATES = {"IMPLEMENTED", "RESIDUAL", "BLOCKED"}
VERIFICATION_STATES = {"NOT_VERIFIED", "VERIFIED"}
SHA40 = re.compile(r"^[0-9a-f]{40}$")
SHA256 = re.compile(r"^[0-9a-f]{64}$")
EXPECTED_PRIMARY_LAYERS = {"COMP-ATC-ENG":"X","COMP-STANDARDS":"X","COMP-LANG":"L3","COMP-SHIVA":"L0","COMP-CHAIN":"L2","COMP-ALGO":"L2","COMP-VM":"L3","COMP-GLOBUS":"L0","COMP-AURORA":"L6","COMP-TOOLCHAIN":"L7","COMP-GENESIS":"L7","COMP-ECOSYSTEM":"X","COMP-SDK":"L7","COMP-NODE":"L0","COMP-CONTRACTS":"L3","COMP-ZKP":"L4","COMP-STORAGE":"L1","COMP-COMPUTE":"L5","COMP-LAUNCHPAD":"L5","COMP-MARKET":"L5","COMP-IDE":"L7","COMP-CHRONICLES":"L7","COMP-GFF":"L7","COMP-OS-INTEGRATION":"L0","COMP-SHIVA-SPEC":"X","COMP-VM-SPEC":"X","COMP-ALGO-SPEC":"X","COMP-DOCS":"X","COMP-ORG-GOV":"X","COMP-DEMO":"L7","COMP-XCHAIN-INTEROP":"L5"}


def _unique_ids(items: Any, label: str, errors: list[str]) -> set[str]:
    if not isinstance(items, list):
        errors.append(f"{label} must be an array")
        return set()
    seen: set[str] = set()
    for index, item in enumerate(items):
        if not isinstance(item, dict):
            errors.append(f"{label}[{index}] must be an object")
            continue
        identifier = item.get("id")
        if not isinstance(identifier, str) or not identifier.strip():
            errors.append(f"{label}[{index}].id must be a non-empty string")
            continue
        if identifier in seen:
            errors.append(f"duplicate {label} id: {identifier}")
        seen.add(identifier)
        if not isinstance(item.get("description"), str) or not item["description"].strip():
            errors.append(f"{label}[{index}].description must be a non-empty string")
    return seen


def validate_taxonomy(taxonomy: Any, catalog: Any | None = None) -> list[str]:
    errors: list[str] = []
    if not isinstance(taxonomy, dict):
        return ["taxonomy root must be an object"]
    if taxonomy.get("schema_id") != TAXONOMY_SCHEMA_ID:
        errors.append(f"schema_id must be {TAXONOMY_SCHEMA_ID}")
    if not isinstance(taxonomy.get("schema_version"), str) or not taxonomy["schema_version"]:
        errors.append("schema_version must be a non-empty string")
    if taxonomy.get("status") != "PROPOSED_NOT_SOURCE_VERIFIED":
        errors.append("taxonomy status must remain PROPOSED_NOT_SOURCE_VERIFIED until independently reconciled")

    category_ids = _unique_ids(taxonomy.get("categories"), "categories", errors)
    family_ids = _unique_ids(taxonomy.get("families"), "families", errors)
    class_ids = _unique_ids(taxonomy.get("classes"), "classes", errors)

    for index, family in enumerate(taxonomy.get("families", []) if isinstance(taxonomy.get("families"), list) else []):
        if isinstance(family, dict) and family.get("category_id") not in category_ids:
            errors.append(f"families[{index}] references unknown category_id: {family.get('category_id')!r}")

    contract = taxonomy.get("classification_contract")
    if not isinstance(contract, dict):
        errors.append("classification_contract must be an object")
    else:
        expected = {
            "classification_status_values": CLASSIFICATION_STATES,
            "implementation_status_values": IMPLEMENTATION_STATES,
            "verification_status_values": VERIFICATION_STATES,
        }
        for field, allowed in expected.items():
            actual = contract.get(field)
            if not isinstance(actual, list) or set(actual) != allowed:
                errors.append(f"classification_contract.{field} must contain exactly {sorted(allowed)}")

    if catalog is not None:
        if not isinstance(catalog, dict):
            errors.append("catalog root must be an object")
        else:
            if catalog.get("schema_id") != CATALOG_SCHEMA_ID:
                errors.append(f"catalog schema_id must be {CATALOG_SCHEMA_ID}")
            _validate_architecture_layers(catalog, errors)
            _validate_source_binding_accounting(catalog, errors)
            _validate_catalog_records(catalog, category_ids, family_ids, class_ids, errors)

    return errors


def _validate_catalog_records(
    catalog: dict[str, Any], category_ids: set[str], family_ids: set[str],
    class_ids: set[str], errors: list[str]
) -> None:
    # Recursively inspect records, but only treat objects with known catalog
    # identity keys as component/function/interface records.
    def walk(value: Any, path: str) -> None:
        if isinstance(value, dict):
            identity_keys = {"component_id", "function_id", "interface_id"}
            if identity_keys.intersection(value):
                for field, allowed in (
                    ("category_id", category_ids),
                    ("family_id", family_ids),
                    ("class_id", class_ids),
                ):
                    if field in value and value[field] is not None and value[field] not in allowed:
                        errors.append(f"{path}.{field} references unknown taxonomy id: {value[field]!r}")
                state = value.get("classification_status")
                if state is not None and state not in CLASSIFICATION_STATES:
                    errors.append(f"{path}.classification_status is invalid: {state!r}")
                if state == "MAPPED":
                    evidence = value.get("source_evidence")
                    if not isinstance(evidence, dict):
                        errors.append(f"{path}: MAPPED record requires source_evidence object")
                    else:
                        required = ("canonical_repository", "source_sha", "source_path",
                                    "symbol_kind", "symbol_or_entrypoint", "source_file_sha256")
                        for field in required:
                            if not isinstance(evidence.get(field), str) or not evidence[field].strip():
                                errors.append(f"{path}.source_evidence.{field} is required for MAPPED")
                        if isinstance(evidence.get("source_sha"), str) and not SHA40.fullmatch(evidence["source_sha"]):
                            errors.append(f"{path}.source_evidence.source_sha must be a full 40-character lowercase SHA")
                        if isinstance(evidence.get("source_file_sha256"), str) and not SHA256.fullmatch(evidence["source_file_sha256"]):
                            errors.append(f"{path}.source_evidence.source_file_sha256 must be a 64-character lowercase SHA-256")
                for field, allowed in (
                    ("implementation_status", IMPLEMENTATION_STATES),
                    ("verification_status", VERIFICATION_STATES),
                ):
                    if field in value and value[field] not in allowed:
                        errors.append(f"{path}.{field} has invalid value: {value[field]!r}")
            for key, child in value.items():
                walk(child, f"{path}.{key}")
        elif isinstance(value, list):
            for index, child in enumerate(value):
                walk(child, f"{path}[{index}]")
    walk(catalog, "catalog")


def _validate_architecture_layers(catalog: dict[str, Any], errors: list[str]) -> None:
    binding = catalog.get('architecture_binding')
    if not isinstance(binding, dict) or binding.get('source_path') != 'ARCHITECTURE.md' or binding.get('model') != 'L0-L7 plus X':
        errors.append('catalog architecture_binding must point to ARCHITECTURE.md and declare L0-L7 plus X')
    elif not SHA40.fullmatch(str(binding.get('source_commit_sha', ''))):
        errors.append('catalog architecture_binding.source_commit_sha must be a full 40-character lowercase SHA')
    allowed = {'L0', 'L1', 'L2', 'L3', 'L4', 'L5', 'L6', 'L7', 'X'}
    for component in catalog.get('components', []):
        if not isinstance(component, dict):
            continue
        cid = component.get('id')
        expected = EXPECTED_PRIMARY_LAYERS.get(cid)
        if expected is None or component.get('layer') != expected:
            errors.append(f'{cid}: primary layer conflicts with ARCHITECTURE.md; expected {expected!r}')
        layers = component.get('architectural_layers')
        if not isinstance(layers, list) or not layers or any(x not in allowed for x in layers) or component.get('layer') not in layers or len(layers) != len(set(layers)):
            errors.append(f'{cid}: architectural_layers must be unique valid L0-L7/X values and include the primary layer')
        if cid == 'COMP-NODE' and set(layers or []) != {'L0', 'L2'}:
            errors.append('COMP-NODE must preserve both L0 runtime and L2 blockchain responsibilities')
        if cid == 'COMP-DEMO' and (component.get('catalog_scope') != 'EXCLUDED_FROM_PRODUCTION_READINESS' or not component.get('scope_exclusion_reason')):
            errors.append('COMP-DEMO must be explicitly excluded from production-readiness interpretation')

def _validate_source_binding_accounting(catalog: dict[str, Any], errors: list[str]) -> None:
    functions = [fn for c in catalog.get('components', []) if isinstance(c, dict) for fn in c.get('functions', []) if isinstance(fn, dict)]
    scope = catalog.get('scope', {})
    derived = {
        'function_source_bindings': sum(isinstance(fn.get('source_binding'), dict) for fn in functions),
        'function_source_paths': sum(bool((fn.get('source_binding') or {}).get('source_path')) for fn in functions),
        'function_exact_source_shas': sum(bool(SHA40.fullmatch(str((fn.get('source_binding') or {}).get('exact_source_sha', '')))) for fn in functions),
    }
    for field, count in derived.items():
        if scope.get(field) != count:
            errors.append(f'scope.{field} is stale: declared {scope.get(field)!r}, derived {count}')
    counts = {}
    for fn in functions:
        status = (fn.get('source_binding') or {}).get('status', 'NO_SOURCE_BINDING')
        counts[status] = counts.get(status, 0) + 1
    if scope.get('source_binding_status_counts') != counts:
        errors.append('scope.source_binding_status_counts does not match function source_binding statuses')

def _load(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--taxonomy", type=Path, default=Path("docs/architecture/ATC-CATALOG-TAXONOMY-001.json"))
    parser.add_argument("--catalog", type=Path, default=Path("docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json"))
    args = parser.parse_args(argv)
    try:
        taxonomy = _load(args.taxonomy)
        catalog = _load(args.catalog) if args.catalog.exists() else None
    except (OSError, json.JSONDecodeError) as exc:
        print(f"FAIL: cannot load JSON input: {exc}", file=sys.stderr)
        return 2
    errors = validate_taxonomy(taxonomy, catalog)
    if errors:
        print("FAIL: taxonomy/catalog validation")
        for error in errors:
            print(f"- {error}")
        return 1
    print("PASS: taxonomy structure and existing taxonomy bindings are valid")
    print("NOTE: structural validation is not source reconciliation or runtime verification")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
