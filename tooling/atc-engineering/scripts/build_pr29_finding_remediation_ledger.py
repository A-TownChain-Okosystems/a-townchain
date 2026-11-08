#!/usr/bin/env python3
"""Map every finding in the pinned 361-finding baseline to current evidence."""
from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter, defaultdict
from pathlib import Path

PARSER_MODES = {
    "json": "stdlib_json",
    "markdown": "commonmark_ast",
    "python": "python_ast",
    "rust": "tree_sitter_rust_cst",
    "shell": "tree_sitter_bash_cst",
    "toml": "stdlib_tomllib",
    "unknown": "explicit_unknown_classifier",
    "yaml": "pyyaml_ast",
}

FIX_TESTS = {
    "MISSING_SYMBOL_BINDING": {
        "root_cause": "The heuristic extractor derived the declaration line from a regex match whose leading whitespace could consume blank lines, producing empty signatures; authoritative Rust/Bash parsers now bind signatures to syntax-tree declarations.",
        "fix_refs": ["scripts/org_function_catalog.py#extract_heuristic", "scripts/org_function_catalog.py#extract_tree_sitter"],
        "test_refs": ["tests/test_org_function_catalog.py#test_heuristic_signature_uses_declaration_line_after_blank_lines", "tests/test_org_function_catalog.py#test_rust_tree_sitter_extraction_uses_ast_ranges_and_nonempty_signatures"],
    },
    "UNRESOLVED_FILE_STATUS": {
        "root_cause": "The original scanner did not parse required document formats and lacked explicit, evidence-backed classification for metadata-only files.",
        "fix_refs": ["scripts/org_function_catalog.py#validate_document_format", "scripts/org_function_catalog.py#explicit_unknown_classification", "schemas/org-function-catalog.schema.json", "scripts/validate_catalog_exclusions.py"],
        "test_refs": ["tests/test_org_function_catalog.py#test_document_parsers_accept_valid_and_reject_malformed_inputs", "tests/test_org_function_catalog.py#test_scan_indexes_unsupported_files_with_reason_coded_exclusion", "tests/test_org_function_catalog.py#test_symlink_is_inventoried_as_git_blob_without_following_target"],
    },
    "PARSER_NOT_AUTHORITATIVE": {
        "root_cause": "The baseline reported unsupported or non-authoritative parser coverage for the required formats; current inventories use explicit parser identities and fail closed on parse errors.",
        "fix_refs": ["scripts/org_function_catalog.py#validate_document_format", "scripts/org_function_catalog.py#extract_tree_sitter", "schemas/org-function-catalog.schema.json", "scripts/validate_bidirectional_reconciliation.py"],
        "test_refs": ["tests/test_org_function_catalog.py#test_document_parsers_accept_valid_and_reject_malformed_inputs", "tests/test_org_function_catalog.py#test_scan_reports_parser_coverage_per_language_and_does_not_claim_verification", "tests/test_bidirectional_reconciliation.py#test_heuristic_parser_cannot_be_authoritative"],
    },
    "CATALOG_FILE_NOT_IN_SOURCE_CENSUS": {
        "root_cause": "The pinned catalog commit contains a reviewed added evidence/schema file that does not exist in the earlier pinned source commit; the difference is bound by the exact Git trees and reviewed expected-delta contract.",
        "fix_refs": ["scripts/git_tree_manifest.py", "scripts/validate_git_tree_inventory.py", "docs/architecture/source-catalog-expected-tree-delta.v1.json"],
        "test_refs": ["tests/test_git_tree_manifest.py#test_exact_tree_file_directory_and_submodule_agreement_passes", "tests/test_bidirectional_reconciliation.py#test_reviewed_expected_file_delta_is_exactly_bound_to_both_inventories"],
    },
    "FILE_BLOB_MISMATCH": {
        "root_cause": "The same workflow path has different blob contents in the two pinned commits; the reviewed expected-delta contract binds both exact blob SHAs and the Git-tree validator authenticates them.",
        "fix_refs": ["scripts/git_tree_manifest.py", "scripts/validate_git_tree_inventory.py", "docs/architecture/source-catalog-expected-tree-delta.v1.json"],
        "test_refs": ["tests/test_git_tree_manifest.py#test_blob_sha_mismatch_blocks", "tests/test_bidirectional_reconciliation.py#test_reviewed_expected_file_delta_is_exactly_bound_to_both_inventories"],
    },
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def read_json(path: Path) -> tuple[dict, bytes]:
    raw = path.read_bytes()
    return json.loads(raw), raw


def base_name(name: str) -> str:
    return name.replace("::", ".").split(".")[-1]


def make_finding_id(finding: dict) -> str:
    return sha256(json.dumps(finding, ensure_ascii=True, sort_keys=True, separators=(",", ":")).encode("utf-8"))


def symbol_index(inventory: dict) -> dict:
    result = defaultdict(list)
    for symbol in inventory.get("symbols", []):
        key = (symbol.get("path"), base_name(str(symbol.get("qualified_name", ""))), symbol.get("source_blob_sha"))
        result[key].append(symbol)
    return result


def resolve_symbol_finding(finding: dict, baseline_inventories: dict, current_inventories: dict) -> dict:
    side = finding.get("side")
    baseline = baseline_inventories[side]
    current = current_inventories[side]
    symbol_id = str(finding.get("detail", "")).split(":", 1)[0]
    old = next((item for item in baseline.get("symbols", []) if item.get("symbol_id") == symbol_id), None)
    if old is None:
        return {"status": "UNRESOLVED", "reason": "baseline symbol id not present in pinned baseline inventory"}
    key = (old.get("path"), base_name(str(old.get("qualified_name", ""))), old.get("source_blob_sha"))
    candidates = [
        item for item in symbol_index(current).get(key, [])
        if abs(int(item.get("source_start_line", 0)) - int(old.get("source_start_line", 0))) <= 1
        and isinstance(item.get("signature"), str) and item["signature"].strip()
    ]
    if len(candidates) != 1:
        return {"status": "UNRESOLVED", "reason": f"expected one current symbol match by path/name/blob/line, found {len(candidates)}"}
    symbol = candidates[0]
    return {
        "status": "RESOLVED",
        "match_method": "unique path + terminal qualified name + source blob SHA + source-start-line delta <= 1",
        "current_symbol_id": symbol.get("symbol_id"),
        "path": symbol.get("path"),
        "source_blob_sha": symbol.get("source_blob_sha"),
        "source_start_line": symbol.get("source_start_line"),
        "source_end_line": symbol.get("source_end_line"),
        "signature": symbol.get("signature"),
        "signature_sha256": sha256(symbol["signature"].encode("utf-8")),
    }


def resolve_file_status_finding(finding: dict, baseline_inventories: dict, current_inventories: dict) -> dict:
    side = finding.get("side")
    detail = str(finding.get("detail", ""))
    path, old_status = detail.rsplit(":", 1)
    old_file = next((item for item in baseline_inventories[side].get("files", []) if item.get("path") == path), None)
    current_file = next((item for item in current_inventories[side].get("files", []) if item.get("path") == path), None)
    if old_file is None or current_file is None:
        return {"status": "UNRESOLVED", "reason": "file path missing from baseline or current inventory"}
    if old_file.get("blob_sha") != current_file.get("blob_sha"):
        return {"status": "UNRESOLVED", "reason": "file blob changed while resolving a file-status finding"}
    current_status = current_file.get("status")
    if current_status == "ANALYSED":
        coverage = current_inventories[side].get("parser_coverage", {}).get(current_file.get("language"), {})
        if coverage.get("status") == "AUTHORITATIVE_COMPLETE" and coverage.get("unsupported_files") == 0 and coverage.get("parse_errors") == 0:
            return {"status": "RESOLVED", "path": path, "old_status": old_status, "current_status": current_status, "blob_sha": current_file["blob_sha"], "parser_mode": coverage.get("parser_mode")}
    if current_status in {"SKIPPED", "EXCLUDED"} and str(current_file.get("reason", "")).strip():
        exclusion = next((item for item in current_inventories[side].get("exclusions", []) if item.get("path") == path and item.get("source_blob_sha") == current_file.get("blob_sha") and item.get("reason")), None)
        if exclusion:
            return {"status": "RESOLVED", "path": path, "old_status": old_status, "current_status": current_status, "blob_sha": current_file["blob_sha"], "exclusion_id": exclusion.get("exclusion_id"), "reason_code": exclusion.get("reason_code")}
    return {"status": "UNRESOLVED", "reason": f"current status {current_status!r} has no authoritative parser or matching evidence-backed exclusion"}


def resolve_parser_finding(finding: dict, current_inventories: dict) -> dict:
    side = finding.get("side")
    language = str(finding.get("detail", "")).split(":", 1)[0]
    coverage = current_inventories[side].get("parser_coverage", {}).get(language)
    expected_mode = PARSER_MODES.get(language)
    if isinstance(coverage, dict) and coverage.get("parser_mode") == expected_mode and coverage.get("status") == "AUTHORITATIVE_COMPLETE" and coverage.get("unsupported_files") == 0 and coverage.get("parse_errors") == 0:
        return {"status": "RESOLVED", "language": language, "parser_mode": expected_mode, "coverage_status": coverage.get("status"), "files_seen": coverage.get("files_seen"), "symbols_detected": coverage.get("symbols_detected")}
    return {"status": "UNRESOLVED", "reason": f"current parser coverage does not satisfy the allowlisted mode for {language}"}


def resolve_delta_finding(finding: dict, current_inventories: dict, expected_delta: dict, tree_report: dict, current_report: dict) -> dict:
    path = str(finding.get("detail", ""))
    if finding.get("code") == "FILE_BLOB_MISMATCH":
        path = path.split(":", 1)[0]
    entry = next((item for item in expected_delta.get("expected_deltas", []) if item.get("path") == path), None)
    if entry is None:
        return {"status": "UNRESOLVED", "reason": "path absent from expected-delta contract"}
    source = next((item for item in current_inventories["source"].get("files", []) if item.get("path") == path), None)
    catalog = next((item for item in current_inventories["catalog"].get("files", []) if item.get("path") == path), None)
    if (entry.get("source_blob_sha") != (source.get("blob_sha") if source else None)
        or entry.get("catalog_blob_sha") != (catalog.get("blob_sha") if catalog else None)):
        return {"status": "UNRESOLVED", "reason": "expected-delta blob bindings do not match current inventories"}
    if tree_report.get("status") != "PASS" or tree_report.get("counts", {}).get("blocking_findings") != 0:
        return {"status": "UNRESOLVED", "reason": "independent Git-tree reconciliation is not PASS"}
    if current_report.get("status") != "RECONCILED" or current_report.get("counts", {}).get("blocking_findings") != 0:
        return {"status": "UNRESOLVED", "reason": "current independent bidirectional reconciliation is not RECONCILED"}
    return {"status": "RESOLVED_AS_EXPECTED_DELTA", "path": path, "change_type": entry.get("change_type"), "source_blob_sha": entry.get("source_blob_sha"), "catalog_blob_sha": entry.get("catalog_blob_sha"), "rationale": entry.get("rationale"), "evidence": entry.get("evidence")}


def build_ledger(args) -> dict:
    contract, contract_raw = read_json(Path(args.ledger_contract))
    expected_delta, delta_raw = read_json(Path(args.expected_deltas))
    baseline_dir = Path(args.baseline_dir)
    baseline_report, baseline_report_raw = read_json(baseline_dir / "independent-bidirectional-reconciliation.json")
    baseline_source, baseline_source_raw = read_json(baseline_dir / "source-inventory.json")
    baseline_catalog, baseline_catalog_raw = read_json(baseline_dir / "catalog-inventory.json")
    baseline_legacy_report, baseline_legacy_raw = read_json(baseline_dir / "reconciliation-report.json")
    baseline_member_bytes = {
        "independent-bidirectional-reconciliation.json": baseline_report_raw,
        "source-inventory.json": baseline_source_raw,
        "catalog-inventory.json": baseline_catalog_raw,
        "reconciliation-report.json": baseline_legacy_raw,
    }
    expected_member_hashes = contract.get("baseline_file_sha256", {})
    for name, raw in baseline_member_bytes.items():
        if sha256(raw) != expected_member_hashes.get(name):
            raise ValueError(f"baseline artifact member SHA-256 mismatch: {name}")
    baseline_inventories = {"source": baseline_source, "catalog": baseline_catalog}
    current_inventories = {
        "source": read_json(Path(args.source_inventory))[0],
        "catalog": read_json(Path(args.catalog_inventory))[0],
    }
    current_report, current_report_raw = read_json(Path(args.current_reconciliation))
    tree_report, tree_report_raw = read_json(Path(args.tree_reconciliation))

    if baseline_report.get("counts", {}).get("blocking_findings") != contract.get("baseline_findings_count"):
        raise ValueError("baseline finding count does not match the pinned ledger contract")
    if len(baseline_report.get("findings", [])) != contract.get("baseline_findings_count"):
        raise ValueError("baseline findings array length does not match the pinned ledger contract")
    for side, key in (("source", "source_commit_sha"), ("catalog", "catalog_commit_sha")):
        if baseline_inventories[side].get("observed_commit_sha") != contract.get(key):
            raise ValueError(f"{side} baseline commit does not match the ledger contract")
        if current_inventories[side].get("observed_commit_sha") != contract.get(key):
            raise ValueError(f"{side} current commit does not match the ledger contract")

    records = []
    for finding in baseline_report["findings"]:
        code = finding.get("code")
        resolution = {"status": "UNRESOLVED", "reason": f"no resolver for original finding code {code}"}
        if code == "MISSING_SYMBOL_BINDING":
            resolution = resolve_symbol_finding(finding, baseline_inventories, current_inventories)
        elif code == "UNRESOLVED_FILE_STATUS":
            resolution = resolve_file_status_finding(finding, baseline_inventories, current_inventories)
        elif code == "PARSER_NOT_AUTHORITATIVE":
            resolution = resolve_parser_finding(finding, current_inventories)
        elif code in {"CATALOG_FILE_NOT_IN_SOURCE_CENSUS", "FILE_BLOB_MISMATCH"}:
            resolution = resolve_delta_finding(finding, current_inventories, expected_delta, tree_report, current_report)
        mapping = FIX_TESTS.get(code, {})
        records.append({
            "finding_id": make_finding_id(finding),
            "original_finding": finding,
            "root_cause": mapping.get("root_cause", resolution.get("reason", "unmapped")),
            "resolution_status": resolution.get("status", "UNRESOLVED"),
            "resolution": resolution,
            "fix_refs": mapping.get("fix_refs", []),
            "regression_test_refs": mapping.get("test_refs", []),
        })

    status_counts = Counter(record["resolution_status"] for record in records)
    unresolved = [record for record in records if record["resolution_status"] == "UNRESOLVED"]
    current_gate_pass = (
        current_report.get("status") == "RECONCILED"
        and current_report.get("counts", {}).get("blocking_findings") == 0
        and tree_report.get("status") == "PASS"
        and tree_report.get("counts", {}).get("blocking_findings") == 0
    )
    if not current_gate_pass:
        unresolved.append({"finding_id": "CURRENT_GATE_NOT_PASS", "resolution_status": "UNRESOLVED", "resolution": {"reason": "current reconciliation or Git-tree gate did not pass"}})
    return {
        "schema_version": "1.0.0",
        "repository": contract.get("repository"),
        "baseline": {
            "run_id": contract.get("baseline_run_id"),
            "artifact_id": contract.get("baseline_artifact_id"),
            "artifact_name": contract.get("baseline_artifact_name"),
            "artifact_sha256": contract.get("baseline_artifact_sha256"),
            "independent_report_sha256": sha256(baseline_report_raw),
            "legacy_reconciliation_report_sha256": sha256(baseline_legacy_raw),
            "member_hashes_verified": True,
            "finding_count": len(baseline_report.get("findings", [])),
            "source_commit_sha": contract.get("source_commit_sha"),
            "catalog_commit_sha": contract.get("catalog_commit_sha"),
        },
        "current": {
            "head_sha": args.current_head_sha,
            "source_commit_sha": current_inventories["source"].get("observed_commit_sha"),
            "catalog_commit_sha": current_inventories["catalog"].get("observed_commit_sha"),
            "current_reconciliation_status": current_report.get("status"),
            "current_reconciliation_sha256": sha256(current_report_raw),
            "git_tree_status": tree_report.get("status"),
            "git_tree_report_sha256": sha256(tree_report_raw),
            "expected_delta_contract_sha256": sha256(delta_raw),
            "ledger_contract_sha256": sha256(contract_raw),
        },
        "status": "RESOLVED" if not unresolved and len(records) == contract.get("baseline_findings_count") else "BLOCKED",
        "counts": {
            "baseline_findings": len(records),
            "resolved": status_counts.get("RESOLVED", 0),
            "resolved_as_expected_delta": status_counts.get("RESOLVED_AS_EXPECTED_DELTA", 0),
            "unresolved": len(unresolved),
            "by_original_code": dict(sorted(Counter(record["original_finding"]["code"] for record in records).items())),
            "by_resolution_status": dict(sorted(status_counts.items())),
        },
        "findings": records,
        "unresolved_findings": unresolved,
        "governance": {
            "merge_approval": "NOT_GRANTED",
            "pr_state": "DRAFT",
            "meaning": "RESOLVED means each baseline finding has a unique evidence-backed disposition against the current pinned inventories and exact tree artifacts. It does not grant merge approval or assert organization-wide VERIFIED status.",
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ledger-contract", required=True)
    parser.add_argument("--baseline-dir", required=True)
    parser.add_argument("--source-inventory", required=True)
    parser.add_argument("--catalog-inventory", required=True)
    parser.add_argument("--current-reconciliation", required=True)
    parser.add_argument("--tree-reconciliation", required=True)
    parser.add_argument("--expected-deltas", required=True)
    parser.add_argument("--current-head-sha", required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    ledger = build_ledger(args)
    target = Path(args.output)
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(ledger, ensure_ascii=True, sort_keys=True, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"status": ledger["status"], "counts": ledger["counts"], "output": str(target)}, sort_keys=True))
    return 0 if ledger["status"] == "RESOLVED" else 1


if __name__ == "__main__":
    raise SystemExit(main())
