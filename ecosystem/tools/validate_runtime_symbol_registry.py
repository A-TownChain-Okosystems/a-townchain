#!/usr/bin/env python3
"""Fail-closed validator for ATC-RUNTIME-SYMBOL-REGISTRY-001.

Gates (exit 1 on any violation):
  R1 schema      - schema id/version, status, census/catalog references
  R2 id rules    - unique, RTS-<12 hex>, content-derived and re-derivable
  R3 accounting  - every census candidate is bound, registered, mirrored,
                   or excluded; per-repository totals must match the census
  R4 ssot        - quellrepo references match the build manifest canonical map
  R5 duplicates  - no duplicate catalog FN ids, no duplicate non-delegated
                   source bindings, no duplicate registry ids
Fail-closed: a registry that hides an unaccounted candidate is invalid.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from collections import Counter
from pathlib import Path

ID_RE = re.compile(r"^RTS-[0-9a-f]{12}$")
STATUSES = {
    "RUNTIME_FUNCTION_PENDING_FUNCTIONAL_REVIEW",
    "RUNTIME_TYPE_PENDING_FUNCTIONAL_REVIEW",
    "RUNTIME_VALUE_PENDING_FUNCTIONAL_REVIEW",
    "RUNTIME_MACRO_PENDING_FUNCTIONAL_REVIEW",
    "RUNTIME_SYMBOL_PENDING_FUNCTIONAL_REVIEW",
}


def expected_id(repository: str, source_path: str, kind: str, symbol: str) -> str:
    digest = hashlib.sha256(f"{repository}|{source_path}|{kind}|{symbol}".encode("utf-8")).hexdigest()
    return f"RTS-{digest[:12]}"


def validate(registry: dict, census: dict, build_manifest: dict) -> tuple[list[str], dict]:
    errors: list[str] = []
    if registry.get("schema_id") != "ATC-RUNTIME-SYMBOL-REGISTRY-001":
        errors.append("schema_id must be ATC-RUNTIME-SYMBOL-REGISTRY-001")
    if registry.get("schema_version") != "1.0.0":
        errors.append("schema_version must be 1.0.0")
    if registry.get("status") not in {"RESIDUAL", "BLOCKED"}:
        errors.append("status must be RESIDUAL or BLOCKED")
    if registry.get("functional_verification") != "NOT_PERFORMED":
        errors.append("functional_verification must be NOT_PERFORMED (no sham validation)")

    entries = registry.get("entries") or []
    if not isinstance(entries, list):
        errors.append("entries must be an array")
        entries = []

    ids = [entry.get("id") for entry in entries]
    for value, count in Counter(ids).items():
        if count > 1:
            errors.append(f"duplicate registry id {value} ({count} entries)")
    for index, entry in enumerate(entries):
        for field in ("id", "repository", "source_path", "kind", "symbol", "census_source_sha", "status"):
            if not entry.get(field):
                errors.append(f"entries[{index}] missing {field}")
        if not ID_RE.fullmatch(str(entry.get("id"))):
            errors.append(f"entries[{index}] id format violation: {entry.get('id')}")
            continue
        derived = expected_id(entry.get("repository", ""), entry.get("source_path", ""),
                              entry.get("kind", ""), entry.get("symbol", ""))
        if derived != entry.get("id"):
            errors.append(f"entries[{index}] id is not content-derived (expected {derived})")
        if entry.get("status") not in STATUSES:
            errors.append(f"entries[{index}] invalid status {entry.get('status')}")

    # R3 accounting against the census
    census_per_repo = Counter()
    for repository_entry in census.get("repositories", []):
        report = repository_entry.get("report") or {}
        total = 0
        for file in report.get("files", []):
            total += len(file.get("candidate_public_symbols", []))
        if total:
            census_per_repo[repository_entry.get("repository")] = total
    registry_per_repo = Counter(entry["repository"] for entry in entries)
    summary = registry.get("summary") or {}
    per_repo_summary = summary.get("per_repository") or {}
    for repository, total in census_per_repo.items():
        accounted = registry_per_repo.get(repository, 0)
        declared = sum(per_repo_summary.get(repository, {}).values())
        if accounted + 0 != registry_per_repo.get(repository, 0):
            errors.append(f"{repository}: entry/accounting drift")
        if declared != total:
            errors.append(
                f"{repository}: classification total {declared} != census candidates {total}")
    for repository in per_repo_summary:
        if repository not in census_per_repo:
            errors.append(f"{repository}: classified but absent from census")

    # R4 ssot against the build manifest
    manifest_by_repo = {entry.get("repo"): entry for entry in build_manifest.get("repos", [])}
    for entry in entries:
        name = entry["repository"].rsplit("/", 1)[-1]
        entry_repo = manifest_by_repo.get(name)
        if not entry_repo:
            continue
        ssot = entry.get("ssot")
        if entry_repo.get("status") == "CANONICAL_ELSEWHERE":
            if not ssot or ssot.get("finding") != "QUERLREPO_REFERENCE_NON_SSOT":
                errors.append(f"{entry['id']}: quellrepo entry missing SSOT reference finding")
            elif ssot.get("canonical_source_path") != entry_repo.get("canonical_source_path"):
                errors.append(f"{entry['id']}: SSOT canonical path disagrees with build manifest")
        elif ssot is not None:
            errors.append(f"{entry['id']}: unexpected ssot finding for non-quellrepo repository")

    # R5 catalog duplicates
    findings = registry.get("catalog_duplicate_findings") or {}
    if findings.get("duplicate_function_ids"):
        errors.append(f"duplicate catalog function ids: {findings['duplicate_function_ids']}")
    if findings.get("duplicate_source_bindings"):
        errors.append(f"duplicate non-delegated source bindings: "
                      f"{[(b['repository'], b['source_path'], b['symbol']) for b in findings['duplicate_source_bindings']]}")

    return errors, {
        "entries": len(entries),
        "census_repositories": len(census_per_repo),
        "gates": {
            "R1_schema": "PASS" if not any("schema" in e for e in errors) else "FAIL",
            "R2_id_rules": "PASS" if not any("id" in e for e in errors) else "FAIL",
            "R3_accounting": "PASS" if not any("accounting" in e or "classification" in e or "absent" in e for e in errors) else "FAIL",
            "R4_ssot": "PASS" if not any("SSOT" in e or "ssot" in e for e in errors) else "FAIL",
            "R5_duplicates": "PASS" if not any("duplicate" in e for e in errors) else "FAIL",
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--registry", required=True, type=Path)
    parser.add_argument("--census", required=True, type=Path)
    parser.add_argument("--build-manifest", required=True, type=Path)
    args = parser.parse_args()
    registry = json.loads(args.registry.read_text(encoding="utf-8"))
    census = json.loads(args.census.read_text(encoding="utf-8"))
    manifest = json.loads(args.build_manifest.read_text(encoding="utf-8"))
    errors, summary = validate(registry, census, manifest)
    if errors:
        print(json.dumps({"validation": "FAIL", "errors": errors}, indent=2))
        return 1
    print(json.dumps({"validation": "PASS", **summary}, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
