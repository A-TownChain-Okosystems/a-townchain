#!/usr/bin/env python3
"""Reconcile lexical source candidates with catalog function bindings.

This report is deliberately conservative: lexical symbol hits are not contracts,
and matching names do not constitute functional verification.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path


def reconcile(catalog: dict, census: dict) -> dict:
    census_repos = {
        entry.get("repository"): entry
        for entry in census.get("repositories", [])
        if isinstance(entry, dict) and entry.get("repository")
    }
    symbols_by_repo: dict[str, set[tuple[str, str]]] = {}
    for repository, entry in census_repos.items():
        report = entry.get("report") or {}
        found: set[tuple[str, str]] = set()
        for file in report.get("files", []):
            for symbol in file.get("candidate_public_symbols", []):
                found.add((file.get("path", ""), symbol.get("name", "")))
        symbols_by_repo[repository] = found

    bound: dict[str, set[tuple[str, str]]] = {}
    function_findings = []
    component_findings = []
    total_functions = 0
    for component in catalog.get("components", []):
        owner = component.get("canonical_owner")
        source_inventory = component.get("source_inventory") or {}
        inventory_repo = source_inventory.get("repository")
        repo_entry = census_repos.get(inventory_repo or owner)
        if not repo_entry:
            component_findings.append({
                "component_id": component.get("id"),
                "component": component.get("name"),
                "finding": "SOURCE_REPOSITORY_NOT_IN_CENSUS",
                "repository": inventory_repo or owner,
                "status": "BLOCKED",
            })
        else:
            observed = source_inventory.get("observed_head_sha")
            actual = repo_entry.get("source_sha")
            if observed and actual and observed != actual:
                if source_inventory.get("self_referential_head"):
                    # The catalog lives inside this repository, so its own HEAD
                    # moves with every catalog revision. The binding evidence is
                    # the census run itself; the drift is documented, not hidden.
                    component_findings.append({
                        "component_id": component.get("id"),
                        "component": component.get("name"),
                        "finding": "SELF_REFERENTIAL_HEAD_BOUND_AT_CENSUS_RUN",
                        "catalog_sha": observed,
                        "census_sha": actual,
                        "status": "RESIDUAL",
                    })
                else:
                    component_findings.append({
                        "component_id": component.get("id"),
                        "component": component.get("name"),
                        "finding": "CATALOG_CENSUS_SHA_MISMATCH",
                        "catalog_sha": observed,
                        "census_sha": actual,
                        "status": "RESIDUAL",
                    })
        for function in component.get("functions", []):
            total_functions += 1
            binding = function.get("source_binding") or {}
            repo = inventory_repo or owner
            path = binding.get("source_path")
            symbol = binding.get("symbol_or_entrypoint")
            key = (path or "", symbol or "")
            item = {
                "component_id": component.get("id"),
                "component": component.get("name"),
                "function_id": function.get("id"),
                "function": function.get("name"),
                "repository": repo,
                "source_path": path,
                "symbol_or_entrypoint": symbol,
                "catalog_source_sha": binding.get("exact_source_sha"),
                "census_source_sha": (census_repos.get(repo) or {}).get("source_sha"),
                "status": "RESIDUAL",
                "finding": None,
            }
            binding_kind = binding.get("binding_kind", "code_symbol")
            if binding.get("delegates_to_function_id"):
                item["finding"] = "DELEGATED_BINDING_DOCUMENTED"
                item["delegates_to_function_id"] = binding["delegates_to_function_id"]
            elif not path or not symbol:
                if binding_kind == "exempt_no_source_binding":
                    item["finding"] = "EXEMPT_NO_SOURCE_BINDING_DOCUMENTED"
                else:
                    item["finding"] = "FUNCTION_BINDING_MISSING"
            elif repo not in census_repos or not (census_repos[repo].get("report")):
                item["finding"] = "SOURCE_CENSUS_UNAVAILABLE"
                item["status"] = "BLOCKED"
            elif key not in symbols_by_repo.get(repo, set()):
                if binding_kind == "non_public_internal_helper":
                    item["finding"] = "NON_PUBLIC_INTERNAL_HELPER_EXPECTED_ABSENT"
                elif binding_kind == "data_artifact":
                    item["finding"] = "DATA_ARTIFACT_BINDING_DOCUMENTED"
                elif binding_kind == "specification_document":
                    item["finding"] = "SPECIFICATION_DOCUMENT_BINDING_DOCUMENTED"
                elif binding_kind == "test_surface":
                    item["finding"] = "TEST_SURFACE_BINDING_DOCUMENTED"
                else:
                    item["finding"] = "BOUND_SYMBOL_NOT_FOUND_IN_LEXICAL_SCAN"
            elif binding.get("exact_source_sha") != census_repos[repo].get("source_sha"):
                item["finding"] = "FUNCTION_BINDING_SHA_MISMATCH"
            else:
                item["finding"] = "LEXICAL_MATCH_ONLY_FUNCTIONAL_VERIFICATION_PENDING"
                bound.setdefault(repo, set()).add(key)
            function_findings.append(item)

    unmatched_candidates = []
    for repo, found in sorted(symbols_by_repo.items()):
        for path, symbol in sorted(found - bound.get(repo, set())):
            unmatched_candidates.append({
                "repository": repo,
                "source_path": path,
                "symbol_or_entrypoint": symbol,
                "status": "RESIDUAL",
                "finding": "CANDIDATE_NOT_BOUND_TO_CATALOG_FUNCTION",
            })

    blocking = census.get("status") == "BLOCKED" or any(
        item.get("status") == "BLOCKED" for item in component_findings + function_findings
    )
    return {
        "schema_id": "ATC-COMPONENT-FUNCTION-RECONCILIATION-001",
        "schema_version": "1.0.0",
        "status": "BLOCKED" if blocking else "RESIDUAL",
        "catalog_schema": catalog.get("schema_id"),
        "census_schema": census.get("schema_id"),
        "census_status": census.get("status"),
        "summary": {
            "components": len(catalog.get("components", [])),
            "functions": total_functions,
            "function_bindings_residual_or_blocked": sum(
                item["finding"] != "LEXICAL_MATCH_ONLY_FUNCTIONAL_VERIFICATION_PENDING"
                for item in function_findings
            ),
            "function_bindings_documented_exclusions": sum(
                item["finding"] in {
                    "DELEGATED_BINDING_DOCUMENTED",
                    "EXEMPT_NO_SOURCE_BINDING_DOCUMENTED",
                    "NON_PUBLIC_INTERNAL_HELPER_EXPECTED_ABSENT",
                    "DATA_ARTIFACT_BINDING_DOCUMENTED",
                    "SPECIFICATION_DOCUMENT_BINDING_DOCUMENTED",
                    "TEST_SURFACE_BINDING_DOCUMENTED",
                }
                for item in function_findings
            ),
            "self_referential_component_heads": sum(
                item["finding"] == "SELF_REFERENTIAL_HEAD_BOUND_AT_CENSUS_RUN"
                for item in component_findings
            ),
            "component_findings": len(component_findings),
            "candidate_symbols_unbound": len(unmatched_candidates),
            "functional_verification": "NOT_PERFORMED",
        },
        "component_findings": component_findings,
        "function_findings": function_findings,
        "unmatched_candidates": unmatched_candidates,
        "limitations": [
            "A lexical name/path match is discovery evidence only, not a function contract or proof of behavior.",
            "Dynamic, generated, macro-expanded, conditional, indirect, and non-public entrypoints may be absent from the scan.",
            "A repository census SHA mismatch must be reconciled before any source binding can be considered current.",
            "Tests and workflow evidence must be separately linked and verified at the exact source SHA.",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--catalog", required=True, type=Path)
    parser.add_argument("--census", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    catalog = json.loads(args.catalog.read_text(encoding="utf-8"))
    census = json.loads(args.census.read_text(encoding="utf-8"))
    if catalog.get("schema_id") != "ATC-COMPONENT-FUNCTION-CATALOG-001":
        parser.error("unexpected component/function catalog schema")
    if census.get("schema_id") != "ATC-ORG-SOURCE-CENSUS-001":
        parser.error("unexpected organization census schema")
    report = reconcile(catalog, census)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": report["status"], **report["summary"], "output": str(args.output)}, sort_keys=True))
    return 0 if report["status"] == "RESIDUAL" else 2


if __name__ == "__main__":
    sys.exit(main())
