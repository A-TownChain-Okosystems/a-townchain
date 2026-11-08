#!/usr/bin/env python3
"""Build the machine-checkable runtime symbol registry from census evidence.

Every candidate public symbol discovered by the organization source census is
accounted for exactly once:

  * BOUND_TO_CATALOG_FUNCTION   - already carries a catalog FN-* id (no new id)
  * RUNTIME_SYMBOL              - gets a stable unique RTS-* id and awaits
                                 functional review
  * MIRRORED_REFERENCE_NON_SSOT - umbrella subtree snapshot of a live quellrepo
  * MIRRORED_ARCHIVED_COMPONENT - umbrella subtree snapshot of an archived repo
  * EXCLUDED_*                  - documented exclusion (tests, docs, tooling,
                                 CI config, archive, examples, generated)

Ids are content-derived (SHA-256 over repository|source_path|kind|symbol) and
therefore stable across registry rebuilds. Lexical discovery is not a
function contract: runtime entries remain pending functional review.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sys
from collections import Counter, defaultdict
from pathlib import Path

ARCHIVED_REPOSITORIES = {
    "atc-wallet", "atc-explorer", "atc-indexer", "atc-mining",
    "atc-interop", "atc-oracle", "atc-storage",
}
UMBRELLA_REPO = "a-townchain-ecosystem"
RUNTIME_KINDS = {"function": "RUNTIME_FUNCTION_PENDING_FUNCTIONAL_REVIEW",
                "type": "RUNTIME_TYPE_PENDING_FUNCTIONAL_REVIEW",
                "value": "RUNTIME_VALUE_PENDING_FUNCTIONAL_REVIEW",
                "macro": "RUNTIME_MACRO_PENDING_FUNCTIONAL_REVIEW"}


def classify_path(path: str) -> str:
    p = path.lower()
    if "/archive/" in p or "archiv" in p:
        return "archive"
    if ("/test" in p or p.startswith("tests/") or "/tests/" in p
            or "test_" in p.rsplit("/", 1)[-1]
            or p.rsplit("/", 1)[-1].endswith("_test.rs")
            or "/__tests__/" in p or ".spec." in p or ".test." in p):
        return "test"
    if "/docs/" in p or p.startswith("docs/") or p.endswith(".md"):
        return "docs"
    if "/examples/" in p or p.startswith("examples/"):
        return "example"
    if "generated" in p or "/target/" in p or "/dist/" in p or "/node_modules/" in p:
        return "generated_or_build"
    if "/tools/" in p or p.startswith("tools/"):
        return "tooling"
    if "/.github/" in p or "workflow" in p:
        return "ci_config"
    return "source"


def runtime_id(repository: str, source_path: str, kind: str, symbol: str) -> str:
    digest = hashlib.sha256(f"{repository}|{source_path}|{kind}|{symbol}".encode("utf-8")).hexdigest()
    return f"RTS-{digest[:12]}"


def catalog_bindings(catalog: dict) -> tuple[set, list, list, list]:
    """Return (bound keys, duplicate findings, delegated ids, duplicate fn ids)."""
    bound: dict = {}
    delegated: list = []
    for component in catalog.get("components", []):
        inventory = component.get("source_inventory") or {}
        repo = inventory.get("repository") or component.get("canonical_owner")
        for function in component.get("functions", []):
            binding = function.get("source_binding") or {}
            if binding.get("delegates_to_function_id"):
                delegated.append({"function_id": function["id"],
                                  "delegates_to_function_id": binding["delegates_to_function_id"]})
                continue
            path = binding.get("source_path")
            symbol = binding.get("symbol_or_entrypoint")
            if path and symbol:
                bound.setdefault((repo, path, symbol), []).append(function["id"])
    duplicate_fn_ids = [fid for fid, n in Counter(
        function["id"] for component in catalog.get("components", [])
        for function in component.get("functions", [])).items() if n > 1]
    duplicates = [{"repository": repo, "source_path": path, "symbol": symbol, "function_ids": ids}
                  for (repo, path, symbol), ids in bound.items() if len(ids) > 1]
    bound_keys = set(bound)
    return bound_keys, duplicates, delegated, duplicate_fn_ids


def build_registry(census: dict, catalog: dict, build_manifest: dict) -> dict:
    bound_keys, duplicate_bindings, delegated_bindings, duplicate_fn_ids = catalog_bindings(catalog)
    manifest_by_repo = {entry.get("repo"): entry for entry in build_manifest.get("repos", [])}
    quellrepo_canonical = {}
    for repo, entry in manifest_by_repo.items():
        if entry.get("status") == "CANONICAL_ELSEWHERE":
            quellrepo_canonical[repo] = entry.get("canonical_source_path")

    entries = []
    classification_counts = Counter()
    per_repo: dict = defaultdict(Counter)
    per_kind: Counter = Counter()
    census_repo_shas = {}

    for repository_entry in census.get("repositories", []):
        repository = repository_entry.get("repository")
        name = repository_entry.get("name")
        census_repo_shas[repository] = repository_entry.get("source_sha")
        report = repository_entry.get("report") or {}
        for file in report.get("files", []):
            path_class = classify_path(file.get("path", ""))
            for symbol in file.get("candidate_public_symbols", []):
                key = (repository, file.get("path", ""), symbol.get("name", ""))
                if key in bound_keys:
                    classification = "BOUND_TO_CATALOG_FUNCTION"
                elif name == UMBRELLA_REPO and file.get("path", "").startswith("components/"):
                    sub = file.get("path", "").split("/")[1]
                    classification = ("MIRRORED_ARCHIVED_COMPONENT" if sub in ARCHIVED_REPOSITORIES
                                      else "MIRRORED_REFERENCE_NON_SSOT")
                elif path_class != "source":
                    classification = f"EXCLUDED_{path_class.upper()}"
                else:
                    classification = "RUNTIME_SYMBOL"
                classification_counts[classification] += 1
                per_repo[repository][classification] += 1
                if classification != "RUNTIME_SYMBOL":
                    continue
                kind = symbol.get("kind", "unknown")
                per_kind[kind] += 1
                ssot = None
                if name in quellrepo_canonical:
                    ssot = {"finding": "QUERLREPO_REFERENCE_NON_SSOT",
                            "canonical_source_path": quellrepo_canonical[name],
                            "policy": "quellrepo is reference/verification only per build manifest"}
                entries.append({
                    "id": runtime_id(repository, file.get("path", ""), kind, symbol.get("name", "")),
                    "repository": repository,
                    "source_path": file.get("path", ""),
                    "language": file.get("language"),
                    "kind": kind,
                    "symbol": symbol.get("name", ""),
                    "file_sha256": file.get("file_sha256"),
                    "census_source_sha": repository_entry.get("source_sha"),
                    "status": RUNTIME_KINDS.get(kind, "RUNTIME_SYMBOL_PENDING_FUNCTIONAL_REVIEW"),
                    "ssot": ssot,
                })

    entries.sort(key=lambda e: (e["repository"], e["source_path"], e["symbol"]))
    id_counts = Counter(entry["id"] for entry in entries)
    registry_duplicate_ids = sorted(fid for fid, n in id_counts.items() if n > 1)
    blocking = census.get("status") == "BLOCKED" or duplicate_fn_ids or duplicate_bindings or registry_duplicate_ids
    return {
        "schema_id": "ATC-RUNTIME-SYMBOL-REGISTRY-001",
        "schema_version": "1.0.0",
        "status": "BLOCKED" if blocking else "RESIDUAL",
        "census_schema": census.get("schema_id"),
        "census_source_shas": census_repo_shas,
        "catalog_schema": catalog.get("schema_id"),
        "functional_verification": "NOT_PERFORMED",
        "summary": {
            "candidate_symbols_total": sum(classification_counts.values()),
            "bound_to_catalog_functions": classification_counts["BOUND_TO_CATALOG_FUNCTION"],
            "runtime_registry_entries": len(entries),
            "runtime_entries_by_kind": dict(per_kind),
            "mirrored_reference_non_ssot": classification_counts["MIRRORED_REFERENCE_NON_SSOT"],
            "mirrored_archived_component": classification_counts["MIRRORED_ARCHIVED_COMPONENT"],
            "exclusions": {k: v for k, v in sorted(classification_counts.items()) if k.startswith("EXCLUDED_")},
            "per_repository": {repo: dict(counts) for repo, counts in sorted(per_repo.items())},
        },
        "catalog_duplicate_findings": {
            "duplicate_function_ids": duplicate_fn_ids,
            "duplicate_source_bindings": duplicate_bindings,
            "delegated_bindings": delegated_bindings,
        },
        "entries": entries,
        "limitations": [
            "Lexical discovery is inventory evidence, not a function contract or proof of behavior.",
            "Runtime entries await functional review; no entry is production evidence on its own.",
            "Umbrella component mirrors are subtree snapshots; the quellrepo remains the SSOT (ADR-0001).",
            "Ids are content-derived and stable, but id stability does not imply functional verification.",
            "Exclusions are classification decisions with documented reasons, not approvals.",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--census", required=True, type=Path)
    parser.add_argument("--catalog", required=True, type=Path)
    parser.add_argument("--build-manifest", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    census = json.loads(args.census.read_text(encoding="utf-8"))
    catalog = json.loads(args.catalog.read_text(encoding="utf-8"))
    manifest = json.loads(args.build_manifest.read_text(encoding="utf-8"))
    if census.get("schema_id") != "ATC-ORG-SOURCE-CENSUS-001":
        parser.error("unexpected census schema")
    if catalog.get("schema_id") != "ATC-COMPONENT-FUNCTION-CATALOG-001":
        parser.error("unexpected catalog schema")
    if manifest.get("schema_id") != "ATC-BUILD-MANIFEST-001":
        parser.error("unexpected build manifest schema")
    registry = build_registry(census, catalog, manifest)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(registry, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps({"status": registry["status"], **registry["summary"]}, sort_keys=True)[:2000])
    return 0 if registry["status"] == "RESIDUAL" else 2


if __name__ == "__main__":
    sys.exit(main())
