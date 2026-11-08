#!/usr/bin/env python3
"""
ATC Layer Registry Validator — validates the layer architecture
against the component-function catalog. Fail-closed: any
inconsistency is reported as an error, not a warning.

Usage:
    python3 tools/validate_layer_registry.py [--registry PATH] [--mapping PATH] [--catalog PATH]
"""

import json
import sys
import argparse
from pathlib import Path

try:
    import yaml
except ImportError:
    print("FAIL: PyYAML is required (pip install pyyaml)")
    sys.exit(2)


REPO_ROOT = Path(__file__).resolve().parent.parent
DEFAULT_REGISTRY = REPO_ROOT / "docs/architecture/layers/ATC-LAYER-REGISTRY-001.yaml"
DEFAULT_MAPPING = REPO_ROOT / "docs/architecture/layers/ATC-LAYER-MAPPING-001.yaml"
DEFAULT_CATALOG = REPO_ROOT / "docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json"
DEFAULT_L0_CATALOG = REPO_ROOT / "docs/architecture/layers/ATC-L0-FUNCTION-CATALOG-001.yaml"
DEFAULT_L1_CATALOG = REPO_ROOT / "docs/architecture/layers/ATC-L1-FUNCTION-CATALOG-001.yaml"
DEFAULT_L2_CATALOG = REPO_ROOT / "docs/architecture/layers/ATC-L2-FUNCTION-CATALOG-001.yaml"

VALID_STATUS = {"MAPPED", "IMPLEMENTED", "VERIFIED", "RESIDUAL", "UNRESOLVED", "BLOCKED", "EXCLUDED"}
VALID_INSTALL_MODELS = {"A", "B", "C", "D"}
KERNEL_LAYERS = {"L0", "L1", "L2", "L3", "L4"}  # Cannot claim Windows/Linux install models
HOST_APP_MODELS = {"B", "C"}


class ValidationResult:
    def __init__(self):
        self.errors = []
        self.warnings = []
        self.checks_passed = 0

    def error(self, code, message):
        self.errors.append(f"[{code}] {message}")

    def warn(self, code, message):
        self.warnings.append(f"[{code}] {message}")

    def ok(self, message):
        self.checks_passed += 1


def validate_registry(registry_path):
    """Validate the layer registry structure."""
    result = ValidationResult()
    result.ok(f"Registry file found: {registry_path}")

    with open(registry_path) as f:
        registry = yaml.safe_load(f)

    # Check schema identity
    assert registry.get("schema_id") == "ATC-LAYER-REGISTRY-001", \
        f"Unexpected schema_id: {registry.get('schema_id')}"
    result.ok("Schema ID matches ATC-LAYER-REGISTRY-001")

    # Collect all layer IDs
    layers = registry.get("layers", [])
    cross = registry.get("cross_cutting_layers", [])
    all_layer_ids = [l["id"] for l in layers] + [c["id"] for c in cross]

    # VAL-LR-001: Main layers must be L0-L10
    expected_main = {f"L{i}" for i in range(11)}
    actual_main = {l["id"] for l in layers}
    if expected_main != actual_main:
        result.error("VAL-LR-001", f"Main layers mismatch: expected {sorted(expected_main)}, got {sorted(actual_main)}")
    else:
        result.ok("All 11 main layers (L0-L10) present")

    # VAL-LR-002: Cross-cutting layers must be X1-X8
    expected_cross = {f"X{i}" for i in range(1, 9)}
    actual_cross = {c["id"] for c in cross}
    if expected_cross != actual_cross:
        result.error("VAL-LR-002", f"Cross-cutting layers mismatch: expected {sorted(expected_cross)}, got {sorted(actual_cross)}")
    else:
        result.ok("All 8 cross-cutting layers (X1-X8) present")

    # Check no duplicate layer IDs
    if len(all_layer_ids) != len(set(all_layer_ids)):
        result.error("VAL-LR-003", f"Duplicate layer IDs: {all_layer_ids}")
    else:
        result.ok("No duplicate layer IDs")

    # VAL-LR-003: Sub-layer ID uniqueness
    all_sub_ids = []
    for layer in layers + cross:
        for sub in layer.get("sub_layers", []):
            if isinstance(sub, dict) and "id" in sub:
                all_sub_ids.append(sub["id"])
            elif isinstance(sub, dict):
                # Some cross-cutting sub-layers use a different format
                for k in sub:
                    if k[0].isdigit() or "X" in k[:2]:
                        all_sub_ids.append(f"{layer['id']}-{k}" if not str(k).startswith(layer["id"]) else str(k))
    if len(all_sub_ids) != len(set(all_sub_ids)):
        from collections import Counter
        dupes = [k for k, v in Counter(all_sub_ids).items() if v > 1]
        result.error("VAL-LR-003", f"Duplicate sub-layer IDs: {dupes}")
    else:
        result.ok(f"Sub-layer IDs unique: {len(all_sub_ids)} sub-layers")

    # Each layer must have required fields
    for layer in layers + cross:
        lid = layer.get("id", "?")
        if "name" not in layer:
            result.error("VAL-LR-004", f"Layer {lid} missing 'name'")
        if "responsibility" not in layer:
            result.error("VAL-LR-004", f"Layer {lid} missing 'responsibility'")

    # Installation models
    install_models = registry.get("installation_models", [])
    model_ids = {im["id"] for im in install_models}
    if model_ids != VALID_INSTALL_MODELS:
        result.error("VAL-LR-005", f"Installation models mismatch: expected {VALID_INSTALL_MODELS}, got {model_ids}")
    else:
        result.ok("All 4 installation models (A/B/C/D) defined")

    # Validation rules must be non-empty
    rules = registry.get("validation_rules", [])
    if not rules:
        result.error("VAL-LR-006", "No validation rules defined")
    else:
        result.ok(f"Validation rules defined: {len(rules)}")

    return registry, all_layer_ids, result


def validate_mapping(mapping_path, all_layer_ids, catalog_ids):
    """Validate the component-to-layer mapping."""
    result = ValidationResult()
    result.ok(f"Mapping file found: {mapping_path}")

    with open(mapping_path) as f:
        mapping = yaml.safe_load(f)

    assert mapping.get("schema_id") == "ATC-LAYER-MAPPING-001", \
        f"Unexpected schema_id: {mapping.get('schema_id')}"
    result.ok("Schema ID matches ATC-LAYER-MAPPING-001")

    entries = mapping.get("component_mappings", [])
    if not entries:
        result.error("VAL-LR-007", "Mapping is empty")
        return mapping, result

    # Collect mapped component IDs
    mapped_ids = set()
    for entry in entries:
        cid = entry.get("component_id")
        if not cid:
            result.error("VAL-LR-007", f"Entry without component_id: {entry}")
            continue
        mapped_ids.add(cid)

        # VAL-LR-001: Primary layer must be valid
        pl = entry.get("primary_layer")
        if pl not in all_layer_ids:
            result.error("VAL-LR-002", f"Component {cid}: invalid primary layer '{pl}'")

        # VAL-LR-004: Must have installation models
        models = entry.get("install_models", [])
        if not models:
            result.error("VAL-LR-004", f"Component {cid}: no installation models declared")
        else:
            for m in models:
                if m not in VALID_INSTALL_MODELS:
                    result.error("VAL-LR-004", f"Component {cid}: invalid install model '{m}'")

            # VAL-LR-005: Kernel layers cannot claim Windows/Linux models
            if pl in KERNEL_LAYERS:
                host_models = set(models) & HOST_APP_MODELS
                if host_models:
                    result.error("VAL-LR-005",
                        f"Component {cid}: layer {pl} (kernel) claims host-app models {host_models}")

        # Cross-cutting layers must be valid X-layer IDs
        for cl in entry.get("cross_cutting_layers", []):
            if cl not in all_layer_ids:
                result.error("VAL-LR-002", f"Component {cid}: invalid cross-cutting layer '{cl}'")

        # Readiness must be a valid status value
        readiness = entry.get("readiness")
        if readiness and readiness not in VALID_STATUS:
            result.warn("VAL-STATUS", f"Component {cid}: non-standard readiness '{readiness}'")

    # VAL-LR-006: All catalog components must be mapped
    unmapped = catalog_ids - mapped_ids
    if unmapped:
        result.error("VAL-LR-006", f"Catalog components not in mapping: {sorted(unmapped)}")
    else:
        result.ok(f"All {len(catalog_ids)} catalog components are mapped")

    # No extra entries that don't exist in the catalog
    extra = mapped_ids - catalog_ids
    if extra:
        result.warn("VAL-LR-006", f"Mapping entries not in catalog: {sorted(extra)}")

    return mapping, result


def validate_catalog_reference(catalog_path, registry):
    """Cross-reference catalog components with registry layers."""
    result = ValidationResult()
    result.ok(f"Catalog file found: {catalog_path}")

    with open(catalog_path) as f:
        catalog = json.load(f)

    comps = catalog.get("components", [])
    if isinstance(comps, dict):
        comp_ids = set(comps.keys())
    elif isinstance(comps, list):
        comp_ids = {c["id"] for c in comps}
    else:
        result.error("VAL-LR-008", f"Unexpected catalog structure: {type(comps)}")
        comp_ids = set()

    result.ok(f"Catalog contains {len(comp_ids)} components")

    # Check VERIFIED claims have evidence
    for comp in comps:
        if isinstance(comp, dict):
            cid = comp.get("id", "?")
            readiness = comp.get("readiness", "")
            if readiness == "VERIFIED":
                # Must have source_inventory with SHA evidence
                si = comp.get("source_inventory", {})
                if not si:
                    result.warn("VAL-VERIFIED", f"Component {cid} claims VERIFIED but has no source_inventory")

    return comp_ids, result


def validate_function_catalog(cat_path, registry, layer_id, schema_id):
    """Validate a layer function catalog (L0/L1/...) against the registry."""
    result = ValidationResult()
    result.ok(f"Function catalog found: {cat_path} (layer {layer_id})")

    with open(cat_path) as f:
        cat = yaml.safe_load(f)

    if cat.get("schema_id") != schema_id:
        result.error("VAL-FC-000", f"Unexpected schema_id: {cat.get('schema_id')}, expected {schema_id}")
        return cat, result
    result.ok(f"Schema ID matches {schema_id}")

    # Registry L0 sub-layer IDs (cross-check)
    layer_entry = next((l for l in registry.get("layers", []) if l.get("id") == layer_id), None)
    if not layer_entry:
        result.error("VAL-FC-000", f"{layer_id} not found in registry")
        return cat, result
    reg_sub_ids = set()
    for sub in layer_entry.get("sub_layers", []):
        if isinstance(sub, dict) and "id" in sub:
            reg_sub_ids.add(sub["id"])
    result.ok(f"Registry {layer_id} sub-layers: {len(reg_sub_ids)}")

    functions = cat.get("functions", [])
    if not functions:
        result.error("VAL-FC-001", f"{layer_id} function catalog is empty")
        return cat, result

    seen = set()
    sub_coverage = {}
    required_fields = ["function_id", "name", "layer_id", "sub_layer_id", "contract", "status"]

    for fn in functions:
        fid = fn.get("function_id", "?")

        # VAL-L0-001: unique function IDs
        if fid in seen:
            result.error("VAL-L0-001", f"Duplicate function ID: {fid}")
        seen.add(fid)

        # VAL-L0-002: required fields
        for field in required_fields:
            if field not in fn or fn[field] in (None, ""):
                result.error("VAL-L0-002", f"Function {fid}: missing required field '{field}'")

        # VAL-L0-003: layer assignment
        if fn.get("layer_id") != layer_id:
            result.error("VAL-FC-003", f"Function {fid}: layer_id must be '{layer_id}', got '{fn.get('layer_id')}'")

        # VAL-L0-004: sub-layer valid against registry
        sl = fn.get("sub_layer_id")
        if sl not in reg_sub_ids:
            result.error("VAL-FC-004", f"Function {fid}: sub_layer_id '{sl}' not in registry {layer_id} sub-layers")
        else:
            sub_coverage[sl] = sub_coverage.get(sl, 0) + 1

        # VAL-L0-005: status value binding
        st = fn.get("status")
        if st not in VALID_STATUS:
            result.error("VAL-L0-005", f"Function {fid}: invalid status '{st}'")
        elif st == "VERIFIED":
            v = fn.get("verification", {})
            if not v.get("test_references") or not v.get("workflow_runs"):
                result.error("VAL-L0-006",
                    f"Function {fid}: VERIFIED without test/CI evidence (fail-closed rule)")
        elif st in ("UNRESOLVED", "BLOCKED"):
            if not fn.get("status_reason"):
                result.error("VAL-LR-009", f"Function {fid}: status {st} requires status_reason")

        # VAL-L0-007: contract completeness
        c = fn.get("contract", {})
        if isinstance(c, dict):
            for cf in ("inputs", "outputs", "errors"):
                if cf not in c:
                    result.warn("VAL-L0-007", f"Function {fid}: contract missing '{cf}'")

    result.ok(f"Functions validated: {len(functions)} entries")

    # VAL-L0-008: sub-layer coverage
    uncovered = reg_sub_ids - set(sub_coverage)
    if uncovered:
        result.warn("VAL-FC-008", f"{layer_id} sub-layers without functions: {sorted(uncovered)}")
    else:
        result.ok(f"All {len(reg_sub_ids)} {layer_id} sub-layers covered by functions")

    # VAL-L0-009: layer dependency targets valid
    valid_layers = {l["id"] for l in registry.get("layers", [])}
    valid_layers |= {c["id"] for c in registry.get("cross_cutting_layers", [])}
    for dep in cat.get("layer_dependencies", []):
        tl = dep.get("target_layer")
        if tl not in valid_layers:
            result.error("VAL-L0-009", f"Layer dependency with invalid target: '{tl}'")
    result.ok(f"Layer dependencies validated: {len(cat.get('layer_dependencies', []))} targets")

    return cat, result


def main():
    parser = argparse.ArgumentParser(description="ATC Layer Registry Validator")
    parser.add_argument("--registry", default=str(DEFAULT_REGISTRY), help="Path to layer registry YAML")
    parser.add_argument("--mapping", default=str(DEFAULT_MAPPING), help="Path to layer mapping YAML")
    parser.add_argument("--l1_catalog", default=str(DEFAULT_L1_CATALOG), help="Path to L1 function catalog YAML")
    parser.add_argument("--l2_catalog", default=str(DEFAULT_L2_CATALOG), help="Path to L2 function catalog YAML")
    parser.add_argument("--l0_catalog", default=str(DEFAULT_L0_CATALOG), help="Path to L0 function catalog YAML")
    parser.add_argument("--catalog", default=str(DEFAULT_CATALOG), help="Path to component catalog JSON")
    parser.add_argument("--quiet", action="store_true", help="Only show summary")
    args = parser.parse_args()

    print("=" * 70)
    print("ATC Layer Registry Validator")
    print("=" * 70)

    all_errors = []
    all_warnings = []
    total_checks = 0

    # 1. Validate registry
    print("\n[1/3] Validating layer registry...")
    try:
        registry, all_layer_ids, r1 = validate_registry(args.registry)
        all_errors.extend(r1.errors)
        all_warnings.extend(r1.warnings)
        total_checks += r1.checks_passed
        if not args.quiet:
            for e in r1.errors:
                print(f"  ERROR {e}")
            for w in r1.warnings:
                print(f"  WARN  {w}")
        print(f"  Checks passed: {r1.checks_passed}, Errors: {len(r1.errors)}, Warnings: {len(r1.warnings)}")
    except Exception as e:
        print(f"  FATAL: {e}")
        sys.exit(2)

    # 2. Validate catalog reference
    print("\n[2/3] Validating component catalog...")
    try:
        catalog_ids, r2 = validate_catalog_reference(args.catalog, registry)
        all_errors.extend(r2.errors)
        all_warnings.extend(r2.warnings)
        total_checks += r2.checks_passed
        if not args.quiet:
            for e in r2.errors:
                print(f"  ERROR {e}")
            for w in r2.warnings:
                print(f"  WARN  {w}")
        print(f"  Checks passed: {r2.checks_passed}, Errors: {len(r2.errors)}, Warnings: {len(r2.warnings)}")
    except Exception as e:
        print(f"  FATAL: {e}")
        sys.exit(2)

    # 3. Validate mapping
    print("\n[3/3] Validating component-to-layer mapping...")
    try:
        _, r3 = validate_mapping(args.mapping, all_layer_ids, catalog_ids)
        all_errors.extend(r3.errors)
        all_warnings.extend(r3.warnings)
        total_checks += r3.checks_passed
        if not args.quiet:
            for e in r3.errors:
                print(f"  ERROR {e}")
            for w in r3.warnings:
                print(f"  WARN  {w}")
        print(f"  Checks passed: {r3.checks_passed}, Errors: {len(r3.errors)}, Warnings: {len(r3.warnings)}")
    except Exception as e:
        print(f"  FATAL: {e}")
        sys.exit(2)


    # 4. Validate layer function catalogs (L0, L1)
    for layer_id, schema_id, cat_path in [
        ("L0", "ATC-L0-FUNCTION-CATALOG-001", args.l0_catalog),
        ("L1", "ATC-L1-FUNCTION-CATALOG-001", args.l1_catalog),
        ("L2", "ATC-L2-FUNCTION-CATALOG-001", args.l2_catalog),
    ]:
        print(f"\n[4/4] Validating {layer_id} function catalog...")
        try:
            _, r4 = validate_function_catalog(cat_path, registry, layer_id, schema_id)
            all_errors.extend(r4.errors)
            all_warnings.extend(r4.warnings)
            total_checks += r4.checks_passed
            if not args.quiet:
                for e in r4.errors:
                    print(f"  ERROR {e}")
                for w in r4.warnings:
                    print(f"  WARN  {w}")
            print(f"  Checks passed: {r4.checks_passed}, Errors: {len(r4.errors)}, Warnings: {len(r4.warnings)}")
        except Exception as e:
            print(f"  FATAL: {e}")
            sys.exit(2)

    # Summary
    print("\n" + "=" * 70)
    print("SUMMARY")
    print("=" * 70)
    print(f"Total checks passed: {total_checks}")
    print(f"Errors:   {len(all_errors)}")
    print(f"Warnings: {len(all_warnings)}")

    if all_errors:
        print("\nFAIL-CLOSED: Validation FAILED due to errors:")
        for e in all_errors:
            print(f"  {e}")
        sys.exit(1)

    print("\n✓ ALL COMPLIANT — Layer registry, mapping, and catalog are consistent.")
    sys.exit(0)


if __name__ == "__main__":
    main()
