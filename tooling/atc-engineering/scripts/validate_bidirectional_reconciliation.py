#!/usr/bin/env python3
"""Independent fail-closed comparison of two source/catalog inventory artifacts.

This validates inventory evidence and both comparison directions. It deliberately
does not upgrade heuristic parser output to authoritative coverage.
"""
from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path

REQUIRED_FORMATS = ("json", "markdown", "python", "rust", "shell", "toml", "unknown", "yaml")
AUTHORITATIVE_STATUSES = {"AUTHORITATIVE", "AUTHORITATIVE_COMPLETE", "VERIFIED"}
AUTHORITATIVE_MODES = {
    "json": {"stdlib_json"},
    "markdown": {"commonmark_ast"},
    "python": {"python_ast"},
    "rust": {"tree_sitter_rust_cst"},
    "shell": {"tree_sitter_bash_cst"},
    "toml": {"stdlib_tomllib"},
    "unknown": {"explicit_unknown_classifier"},
    "yaml": {"pyyaml_ast"},
}
SHA40 = re.compile(r"^[0-9a-f]{40}$")


def _errors_for_inventory(label: str, doc: dict) -> list[dict]:
    errors: list[dict] = []
    commit = doc.get("observed_commit_sha")
    if not isinstance(commit, str) or not SHA40.fullmatch(commit):
        errors.append({"code": "INVALID_COMMIT_SHA", "side": label, "detail": str(commit)})
    for key in ("files", "symbols", "exclusions", "parser_coverage"):
        if not isinstance(doc.get(key), (list if key in {"files", "symbols", "exclusions"} else dict)):
            errors.append({"code": "INVALID_INVENTORY_FIELD", "side": label, "detail": key})
    files = doc.get("files", []) if isinstance(doc.get("files"), list) else []
    seen_paths: set[str] = set()
    for item in files:
        if not isinstance(item, dict):
            errors.append({"code": "INVALID_FILE_RECORD", "side": label, "detail": "file record is not an object"})
            continue
        path, blob = item.get("path"), item.get("blob_sha")
        if not isinstance(path, str) or not path:
            errors.append({"code": "INVALID_FILE_PATH", "side": label, "detail": str(path)})
        elif path in seen_paths:
            errors.append({"code": "DUPLICATE_FILE_PATH", "side": label, "detail": path})
        else:
            seen_paths.add(path)
        if not isinstance(blob, str) or not SHA40.fullmatch(blob):
            errors.append({"code": "INVALID_FILE_BLOB_SHA", "side": label, "detail": str(path)})
        if item.get("status") in {"UNSUPPORTED", "PARSE_ERROR", "READ_ERROR", "UNCLASSIFIED", "UNKNOWN"}:
            errors.append({"code": "UNRESOLVED_FILE_STATUS", "side": label, "detail": f"{path}:{item.get('status')}"})

    symbols = doc.get("symbols", []) if isinstance(doc.get("symbols"), list) else []
    ids = [s.get("symbol_id") for s in symbols if isinstance(s, dict)]
    for symbol in symbols:
        if not isinstance(symbol, dict):
            errors.append({"code": "INVALID_SYMBOL_RECORD", "side": label, "detail": "symbol record is not an object"})
            continue
        for key in ("symbol_id", "path", "signature", "source_blob_sha"):
            if not symbol.get(key):
                errors.append({"code": "MISSING_SYMBOL_BINDING", "side": label, "detail": f"{symbol.get('symbol_id')}:{key}"})
        if symbol.get("path") not in seen_paths:
            errors.append({"code": "SYMBOL_PATH_NOT_IN_CENSUS", "side": label, "detail": str(symbol.get("path"))})
        if not isinstance(symbol.get("source_blob_sha"), str) or not SHA40.fullmatch(symbol.get("source_blob_sha", "")):
            errors.append({"code": "INVALID_SYMBOL_BLOB_SHA", "side": label, "detail": str(symbol.get("symbol_id"))})
    for symbol_id, count in Counter(ids).items():
        if count > 1:
            errors.append({"code": "DUPLICATE_SYMBOL_ID", "side": label, "detail": f"{symbol_id}:{count}"})

    coverage = doc.get("parser_coverage", {}) if isinstance(doc.get("parser_coverage"), dict) else {}
    for fmt in REQUIRED_FORMATS:
        entry = coverage.get(fmt)
        if not isinstance(entry, dict):
            errors.append({"code": "PARSER_GROUP_MISSING", "side": label, "detail": fmt})
        elif entry.get("status") not in AUTHORITATIVE_STATUSES or entry.get("parser_mode") not in AUTHORITATIVE_MODES[fmt]:
            errors.append({"code": "PARSER_NOT_AUTHORITATIVE", "side": label, "detail": f"{fmt}:{entry.get('parser_mode')}:{entry.get('status')}"})

    exclusions = doc.get("exclusions", []) if isinstance(doc.get("exclusions"), list) else []
    exclusion_ids = []
    for item in exclusions:
        if not isinstance(item, dict):
            errors.append({"code": "INVALID_EXCLUSION_RECORD", "side": label, "detail": "exclusion is not an object"})
            continue
        exclusion_ids.append(item.get("exclusion_id"))
        for key in ("exclusion_id", "repository", "path", "reason_code", "reason", "rationale", "evidence"):
            if not item.get(key):
                errors.append({"code": "EXCLUSION_EVIDENCE_INCOMPLETE", "side": label, "detail": f"{item.get('path')}:{key}"})
        evidence = item.get("evidence")
        if not isinstance(evidence, list) or not evidence or any(
            not isinstance(ev, dict) or not isinstance(ev.get("reference"), str) or not ev["reference"]
            for ev in (evidence if isinstance(evidence, list) else [])
        ):
            errors.append({"code": "EXCLUSION_EVIDENCE_UNVERIFIABLE", "side": label, "detail": str(item.get("path"))})
    for exclusion_id, count in Counter(exclusion_ids).items():
        if not isinstance(exclusion_id, str) or not re.fullmatch(r"[0-9a-f]{64}", exclusion_id):
            errors.append({"code": "INVALID_EXCLUSION_ID", "side": label, "detail": str(exclusion_id)})
        elif count > 1:
            errors.append({"code": "DUPLICATE_EXCLUSION_ID", "side": label, "detail": f"{exclusion_id}:{count}"})
    return errors


def reconcile(source: dict, catalog: dict, expected_deltas: dict | None = None) -> dict:
    errors = _errors_for_inventory("source", source) + _errors_for_inventory("catalog", catalog)
    if source.get("repository") != catalog.get("repository"):
        errors.append({"code": "REPOSITORY_MISMATCH", "side": "pair", "detail": f"{source.get('repository')} != {catalog.get('repository')}"})

    def unique_map(items: list, key: str) -> dict:
        result = {}
        for item in items:
            if isinstance(item, dict) and item.get(key) not in result:
                result[item.get(key)] = item
        return result

    source_files = unique_map(source.get("files", []), "path")
    catalog_files = unique_map(catalog.get("files", []), "path")
    actual_file_deltas = {}
    for path in sorted(set(source_files) | set(catalog_files)):
        left, right = source_files.get(path), catalog_files.get(path)
        if left is None or right is None or left.get("blob_sha") != right.get("blob_sha"):
            actual_file_deltas[path] = {
                "change_type": "added" if left is None else ("deleted" if right is None else "modified"),
                "source_blob_sha": left.get("blob_sha") if left else None,
                "catalog_blob_sha": right.get("blob_sha") if right else None,
            }

    expected_by_path = {}
    if expected_deltas is None:
        for path, delta in sorted(actual_file_deltas.items()):
            errors.append({"code": "UNEXPLAINED_FILE_DELTA", "side": "pair", "detail": f"{path}:{delta['change_type']}"})
    else:
        if expected_deltas.get("repository") != source.get("repository") or expected_deltas.get("repository") != catalog.get("repository"):
            errors.append({"code": "EXPECTED_DELTA_REPOSITORY_MISMATCH", "side": "pair", "detail": str(expected_deltas.get("repository"))})
        if expected_deltas.get("source_commit_sha") != source.get("observed_commit_sha") or expected_deltas.get("catalog_commit_sha") != catalog.get("observed_commit_sha"):
            errors.append({"code": "EXPECTED_DELTA_COMMIT_MISMATCH", "side": "pair", "detail": f"{expected_deltas.get('source_commit_sha')}...{expected_deltas.get('catalog_commit_sha')}"})
        if expected_deltas.get("status") != "REVIEWED_EXPECTED_CHANGE_SET" or expected_deltas.get("merge_approval") != "NOT_GRANTED":
            errors.append({"code": "EXPECTED_DELTA_GOVERNANCE_INVALID", "side": "pair", "detail": "expected change set must remain reviewed evidence, not merge approval"})
        contract_evidence = expected_deltas.get("evidence")
        if not isinstance(contract_evidence, list) or not contract_evidence or any(not isinstance(item, str) or not item for item in contract_evidence):
            errors.append({"code": "EXPECTED_DELTA_EVIDENCE_MISSING", "side": "pair", "detail": "contract evidence must be non-empty"})
        entries = expected_deltas.get("expected_deltas")
        if not isinstance(entries, list):
            errors.append({"code": "EXPECTED_DELTA_LIST_INVALID", "side": "pair", "detail": "expected_deltas must be an array"})
            entries = []
        for entry in entries:
            if not isinstance(entry, dict):
                errors.append({"code": "EXPECTED_DELTA_RECORD_INVALID", "side": "pair", "detail": "entry must be an object"})
                continue
            path = entry.get("path")
            if not isinstance(path, str) or not path:
                errors.append({"code": "EXPECTED_DELTA_PATH_INVALID", "side": "pair", "detail": str(path)})
                continue
            if path in expected_by_path:
                errors.append({"code": "EXPECTED_DELTA_PATH_DUPLICATE", "side": "pair", "detail": path})
                continue
            expected_by_path[path] = entry
            actual = actual_file_deltas.get(path)
            if actual is None:
                errors.append({"code": "STALE_EXPECTED_FILE_DELTA", "side": "pair", "detail": path})
                continue
            for field in ("change_type", "source_blob_sha", "catalog_blob_sha"):
                if entry.get(field) != actual.get(field):
                    errors.append({"code": "EXPECTED_FILE_DELTA_BINDING_MISMATCH", "side": "pair", "detail": f"{path}:{field}:{entry.get(field)!r}!={actual.get(field)!r}"})
            if not isinstance(entry.get("rationale"), str) or not entry["rationale"].strip():
                errors.append({"code": "EXPECTED_FILE_DELTA_RATIONALE_MISSING", "side": "pair", "detail": path})
            evidence = entry.get("evidence")
            if not isinstance(evidence, list) or not evidence or any(not isinstance(item, str) or not item for item in evidence):
                errors.append({"code": "EXPECTED_FILE_DELTA_EVIDENCE_MISSING", "side": "pair", "detail": path})
        for path, delta in sorted(actual_file_deltas.items()):
            if path not in expected_by_path:
                errors.append({"code": "UNEXPLAINED_FILE_DELTA", "side": "pair", "detail": f"{path}:{delta['change_type']}"})

    # Exclusion lists are reconciled independently of file changes. Stable IDs
    # are only identity keys; every evidence-bearing field must also match.
    source_exclusions = unique_map(source.get("exclusions", []), "exclusion_id")
    catalog_exclusions = unique_map(catalog.get("exclusions", []), "exclusion_id")
    for exclusion_id in sorted(set(source_exclusions) - set(catalog_exclusions)):
        errors.append({"code": "SOURCE_EXCLUSION_MISSING_FROM_CATALOG", "side": "source_to_catalog", "detail": str(exclusion_id)})
    for exclusion_id in sorted(set(catalog_exclusions) - set(source_exclusions)):
        errors.append({"code": "CATALOG_EXCLUSION_MISSING_FROM_SOURCE", "side": "catalog_to_source", "detail": str(exclusion_id)})
    exclusion_fields = ("repository", "path", "reason_code", "category", "reason", "rationale", "source_blob_sha", "evidence")
    for exclusion_id in sorted(set(source_exclusions) & set(catalog_exclusions)):
        left, right = source_exclusions[exclusion_id], catalog_exclusions[exclusion_id]
        for field in exclusion_fields:
            if left.get(field) != right.get(field):
                errors.append({"code": "EXCLUSION_BINDING_MISMATCH", "side": "pair", "detail": f"{exclusion_id}:{field}"})

    source_symbols = unique_map(source.get("symbols", []), "symbol_id")
    catalog_symbols = unique_map(catalog.get("symbols", []), "symbol_id")
    for symbol_id in sorted(set(source_symbols) - set(catalog_symbols)):
        errors.append({"code": "SOURCE_DECLARATION_MISSING_FROM_CATALOG", "side": "source_to_catalog", "detail": symbol_id})
    for symbol_id in sorted(set(catalog_symbols) - set(source_symbols)):
        errors.append({"code": "CATALOG_ENTRY_WITHOUT_SOURCE_DECLARATION", "side": "catalog_to_source", "detail": symbol_id})
    fields = ("path", "source_blob_sha", "signature", "qualified_name", "symbol_kind", "source_start_line", "source_end_line")
    for symbol_id in sorted(set(source_symbols) & set(catalog_symbols)):
        left, right = source_symbols[symbol_id], catalog_symbols[symbol_id]
        for field in fields:
            if left.get(field) != right.get(field):
                errors.append({"code": "SYMBOL_BINDING_MISMATCH", "side": "pair", "detail": f"{symbol_id}:{field}:{left.get(field)!r}!={right.get(field)!r}"})

    errors.sort(key=lambda e: (e["side"], e["code"], e["detail"]))
    return {
        "schema_version": "1.0.0",
        "repository": source.get("repository"),
        "source_commit_sha": source.get("observed_commit_sha"),
        "catalog_commit_sha": catalog.get("observed_commit_sha"),
        "status": "RECONCILED" if not errors else "BLOCKED",
        "counts": {
            "source_files": len(source.get("files", [])) if isinstance(source.get("files"), list) else None,
            "catalog_files": len(catalog.get("files", [])) if isinstance(catalog.get("files"), list) else None,
            "source_symbols": len(source.get("symbols", [])) if isinstance(source.get("symbols"), list) else None,
            "catalog_symbols": len(catalog.get("symbols", [])) if isinstance(catalog.get("symbols"), list) else None,
            "blocking_findings": len(errors),
        },
        "findings": errors,
        "limitations": [
            "RECONCILED is emitted only when this validator finds no blockers; it is not VERIFIED.",
            "Parser coverage claims are checked against explicit metadata; parser implementation correctness still requires independent tests and review.",
            "Both inventories must be generated independently from the exact source and canonical catalog trees; this validator cannot prove that provenance by itself.",
            "The filesystem census must be cross-checked against a complete Git tree manifest; this validator compares only supplied inventory artifacts.",
            "Exclusion rationale correctness still requires review of each evidence item.",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source_inventory")
    parser.add_argument("catalog_inventory")
    parser.add_argument("--expected-deltas", default=None, help="reviewed exact commit-to-commit file delta contract")
    parser.add_argument("--output", default="-")
    args = parser.parse_args()
    source = json.loads(Path(args.source_inventory).read_text(encoding="utf-8"))
    catalog = json.loads(Path(args.catalog_inventory).read_text(encoding="utf-8"))
    expected_deltas = json.loads(Path(args.expected_deltas).read_text(encoding="utf-8")) if args.expected_deltas else None
    report = reconcile(source, catalog, expected_deltas)
    rendered = json.dumps(report, ensure_ascii=False, sort_keys=True, indent=2) + "\n"
    if args.output == "-":
        print(rendered, end="")
    else:
        target = Path(args.output)
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(rendered, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
