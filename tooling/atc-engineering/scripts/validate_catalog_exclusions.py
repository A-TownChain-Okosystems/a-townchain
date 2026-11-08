#!/usr/bin/env python3
"""Fail-closed consistency audit for source-catalog exclusion evidence.

This validates evidence bindings inside one generated inventory. It does not
prove that the scanner found every file or that an exclusion is semantically
appropriate; those remain independent review obligations.
"""
from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path


FILE_EXCLUSION_CODES = {
    "ARCHIVED", "VENDORED", "GENERATED", "UNSUPPORTED_FORMAT",
    "NON_UTF8", "PARSE_ERROR", "READ_ERROR", "SYMLINK_FILE_NOT_FOLLOWED", "UNCLASSIFIED_UNKNOWN_FORMAT", "REPOSITORY_METADATA", "DEPENDENCY_LOCKFILE", "LEGAL_TEXT", "OWNERSHIP_METADATA",
}
DIRECTORY_EXCLUSION_CODES = {"EXCLUDED_DIRECTORY", "SYMLINK_NOT_FOLLOWED"}


def audit_inventory(doc: dict) -> dict:
    errors: list[dict[str, str]] = []
    files = doc.get("files")
    exclusions = doc.get("exclusions")
    if not isinstance(files, list):
        files = []
        errors.append({"code": "INVALID_FILES", "path": "", "detail": "files must be an array"})
    if not isinstance(exclusions, list):
        exclusions = []
        errors.append({"code": "INVALID_EXCLUSIONS", "path": "", "detail": "exclusions must be an array"})

    file_by_path: dict[str, dict] = {}
    for item in files:
        if not isinstance(item, dict) or not isinstance(item.get("path"), str) or not item["path"]:
            errors.append({"code": "INVALID_FILE_RECORD", "path": "", "detail": "file record requires a non-empty path"})
            continue
        path = item["path"]
        if path in file_by_path:
            errors.append({"code": "DUPLICATE_FILE_PATH", "path": path, "detail": "file path occurs more than once"})
        file_by_path[path] = item

    exclusion_by_id: dict[str, dict] = {}
    exclusions_by_path: dict[str, list[dict]] = {}
    for item in exclusions:
        if not isinstance(item, dict):
            errors.append({"code": "INVALID_EXCLUSION_RECORD", "path": "", "detail": "exclusion must be an object"})
            continue
        path, eid = item.get("path", ""), item.get("exclusion_id", "")
        if not isinstance(eid, str) or not re.fullmatch(r"[0-9a-f]{64}", eid):
            errors.append({"code": "INVALID_EXCLUSION_ID", "path": str(path), "detail": "expected 64 lowercase hex characters"})
        elif eid in exclusion_by_id:
            errors.append({"code": "DUPLICATE_EXCLUSION_ID", "path": str(path), "detail": eid})
        else:
            exclusion_by_id[eid] = item
        if isinstance(path, str) and path:
            exclusions_by_path.setdefault(path, []).append(item)
        else:
            errors.append({"code": "INVALID_EXCLUSION_PATH", "path": "", "detail": "path must be a non-empty string"})
        required = ("repository", "reason_code", "category", "reason", "rationale", "evidence")
        for key in required:
            if not item.get(key):
                errors.append({"code": "MISSING_EXCLUSION_FIELD", "path": str(path), "detail": key})
        evidence = item.get("evidence")
        if not isinstance(evidence, list) or not evidence:
            errors.append({"code": "MISSING_EXCLUSION_EVIDENCE", "path": str(path), "detail": "evidence must be non-empty"})
        elif any(not isinstance(ev, dict) or not isinstance(ev.get("reference"), str) or not ev["reference"]
                 for ev in evidence):
            errors.append({"code": "INVALID_EXCLUSION_EVIDENCE", "path": str(path), "detail": "each evidence item requires a reference"})

        reason_code = item.get("reason_code")
        if reason_code in FILE_EXCLUSION_CODES:
            source_sha = item.get("source_blob_sha")
            source_file = file_by_path.get(path)
            if not isinstance(source_sha, str) or not re.fullmatch(r"[0-9a-f]{40}", source_sha):
                errors.append({"code": "MISSING_SOURCE_BLOB_BINDING", "path": str(path), "detail": str(reason_code)})
            if source_file is None:
                errors.append({"code": "EXCLUDED_FILE_NOT_IN_INVENTORY", "path": str(path), "detail": str(reason_code)})
            elif source_sha != source_file.get("blob_sha"):
                errors.append({"code": "EXCLUSION_BLOB_MISMATCH", "path": str(path), "detail": f"exclusion={source_sha}; inventory={source_file.get('blob_sha')}"})
            expected_ref = f"{path}#{source_sha}"
            refs = [ev.get("reference") for ev in evidence if isinstance(ev, dict)]
            if expected_ref not in refs:
                errors.append({"code": "MISSING_PATH_BLOB_EVIDENCE", "path": str(path), "detail": expected_ref})
        elif reason_code in DIRECTORY_EXCLUSION_CODES:
            if item.get("source_blob_sha") is not None:
                errors.append({"code": "DIRECTORY_EXCLUSION_HAS_FILE_BLOB", "path": str(path), "detail": "directory/symlink policy exclusions must not claim a file blob"})
        else:
            errors.append({"code": "UNKNOWN_EXCLUSION_REASON", "path": str(path), "detail": str(reason_code)})

    # Every non-analysed inventoried file must have a path-bound exclusion record.
    for path, item in file_by_path.items():
        if item.get("status") in {"SKIPPED", "UNSUPPORTED", "PARSE_ERROR"}:
            matches = exclusions_by_path.get(path, [])
            if not matches:
                errors.append({"code": "UNEXPLAINED_FILE_STATUS", "path": path, "detail": str(item.get("status"))})
            elif not any(x.get("source_blob_sha") == item.get("blob_sha") for x in matches):
                errors.append({"code": "FILE_STATUS_NOT_BLOB_BOUND", "path": path, "detail": str(item.get("status"))})

    reason_counts = Counter(str(item.get("reason_code", "<missing>")) for item in exclusions if isinstance(item, dict))
    return {
        "schema_version": "1.0.0",
        "repository": doc.get("repository"),
        "observed_commit_sha": doc.get("observed_commit_sha"),
        "status": "PASS" if not errors else "BLOCKED",
        "counts": {"files": len(file_by_path), "exclusions": len(exclusions), "errors": len(errors)},
        "reason_code_counts": dict(sorted(reason_counts.items())),
        "errors": sorted(errors, key=lambda e: (e["path"], e["code"], e["detail"])),
        "limitations": [
            "PASS proves only internal evidence consistency for this inventory.",
            "PASS does not prove the filesystem census is exhaustive.",
            "PASS does not prove exclusion rationale is semantically correct or approved.",
            "Directory exclusions bind to scanner policy, not to a source blob.",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("inventory", help="generated source-catalog inventory JSON")
    parser.add_argument("--output", default="-", help="report path or - for stdout")
    args = parser.parse_args()
    doc = json.loads(Path(args.inventory).read_text(encoding="utf-8"))
    report = audit_inventory(doc)
    rendered = json.dumps(report, ensure_ascii=False, sort_keys=True, indent=2) + "\n"
    if args.output == "-":
        print(rendered, end="")
    else:
        target = Path(args.output)
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(rendered, encoding="utf-8")
    return 0 if report["status"] == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
