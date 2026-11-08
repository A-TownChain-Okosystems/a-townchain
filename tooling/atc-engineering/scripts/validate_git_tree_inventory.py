#!/usr/bin/env python3
"""Fail-closed reconciliation of inventory JSON against complete Git-tree manifests."""
from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path


SHA40 = re.compile(r"^[0-9a-f]{40}$")
BLOB_MODES = {"100644", "100755", "120000"}
TREE_MODES = {"040000", "40000"}


def _parents(path: str) -> set[str]:
    parts = path.split("/")
    return {"/".join(parts[:index]) for index in range(1, len(parts))}


def validate_side(side: str, manifest: dict, inventory: dict) -> list[dict]:
    findings: list[dict] = []
    commit = manifest.get("commit_sha")
    if not isinstance(commit, str) or not SHA40.fullmatch(commit):
        findings.append({"code": "INVALID_TREE_COMMIT_SHA", "side": side, "path": "", "detail": str(commit)})
    if inventory.get("observed_commit_sha") != commit:
        findings.append({"code": "TREE_INVENTORY_COMMIT_MISMATCH", "side": side, "path": "", "detail": f"{commit}!={inventory.get('observed_commit_sha')}"})
    entries = manifest.get("entries")
    if not isinstance(entries, list):
        findings.append({"code": "INVALID_TREE_ENTRIES", "side": side, "path": "", "detail": "entries must be an array"})
        entries = []
    if manifest.get("entry_count") != len(entries):
        findings.append({"code": "TREE_ENTRY_COUNT_MISMATCH", "side": side, "path": "", "detail": f"declared={manifest.get('entry_count')};actual={len(entries)}"})

    tree_by_path: dict[str, dict] = {}
    for entry in entries:
        if not isinstance(entry, dict):
            findings.append({"code": "INVALID_TREE_ENTRY", "side": side, "path": "", "detail": "entry must be an object"})
            continue
        path, mode, kind, sha = entry.get("path"), entry.get("mode"), entry.get("type"), entry.get("sha")
        if not isinstance(path, str) or not path or path.startswith("/") or ".." in path.split("/"):
            findings.append({"code": "INVALID_TREE_PATH", "side": side, "path": str(path), "detail": "path must be a safe repository-relative path"})
            continue
        if path in tree_by_path:
            findings.append({"code": "DUPLICATE_TREE_PATH", "side": side, "path": path, "detail": "duplicate manifest path"})
        else:
            tree_by_path[path] = entry
        if not isinstance(sha, str) or not SHA40.fullmatch(sha):
            findings.append({"code": "INVALID_TREE_OBJECT_SHA", "side": side, "path": path, "detail": str(sha)})
        valid_pair = ((kind == "tree" and mode in TREE_MODES)
                      or (kind == "blob" and mode in BLOB_MODES)
                      or (kind == "commit" and mode == "160000"))
        if not valid_pair:
            findings.append({"code": "INVALID_TREE_MODE_TYPE", "side": side, "path": path, "detail": f"mode={mode};type={kind}"})

    files = inventory.get("files")
    if not isinstance(files, list):
        findings.append({"code": "INVALID_INVENTORY_FILES", "side": side, "path": "", "detail": "files must be an array"})
        files = []
    file_by_path: dict[str, dict] = {}
    for item in files:
        if not isinstance(item, dict) or not isinstance(item.get("path"), str) or not item["path"]:
            findings.append({"code": "INVALID_INVENTORY_FILE", "side": side, "path": "", "detail": "file record requires a path"})
            continue
        path = item["path"]
        if path in file_by_path:
            findings.append({"code": "DUPLICATE_INVENTORY_PATH", "side": side, "path": path, "detail": "duplicate inventory path"})
        else:
            file_by_path[path] = item
        blob = item.get("blob_sha")
        if not isinstance(blob, str) or not SHA40.fullmatch(blob):
            findings.append({"code": "INVALID_INVENTORY_BLOB_SHA", "side": side, "path": path, "detail": str(blob)})

    tree_blobs = {path: entry for path, entry in tree_by_path.items() if entry.get("type") == "blob"}
    submodules = {path: entry for path, entry in tree_by_path.items() if entry.get("type") == "commit" and entry.get("mode") == "160000"}
    tree_paths = set(tree_blobs)
    inventory_paths = set(file_by_path)
    for path in sorted(tree_paths - inventory_paths):
        findings.append({"code": "GIT_TREE_FILE_MISSING_FROM_INVENTORY", "side": side, "path": path, "detail": tree_blobs[path].get("sha", "")})
    for path in sorted(inventory_paths - tree_paths):
        findings.append({"code": "INVENTORY_FILE_NOT_IN_GIT_TREE", "side": side, "path": path, "detail": str(file_by_path[path].get("blob_sha"))})
    for path in sorted(tree_paths & inventory_paths):
        expected, observed = tree_blobs[path].get("sha"), file_by_path[path].get("blob_sha")
        if expected != observed:
            findings.append({"code": "GIT_BLOB_SHA_MISMATCH", "side": side, "path": path, "detail": f"tree={expected};inventory={observed}"})

    tree_dirs = {path for path, entry in tree_by_path.items() if entry.get("type") == "tree"}
    leaf_paths = tree_paths | set(submodules)
    expected_dirs = set().union(*(_parents(path) for path in leaf_paths)) if leaf_paths else set()
    for path in sorted(expected_dirs - tree_dirs):
        findings.append({"code": "GIT_DIRECTORY_MISSING_FROM_TREE_MANIFEST", "side": side, "path": path, "detail": "parent directory of a tracked leaf is absent"})
    for path in sorted(tree_dirs - expected_dirs):
        findings.append({"code": "ORPHAN_GIT_TREE_DIRECTORY", "side": side, "path": path, "detail": "directory has no tracked leaf descendant"})
    # Gitlinks are intentionally not source files. Their path, mode 160000, object
    # type commit, and pinned commit SHA remain explicit in the tree manifest.
    return findings


def reconcile(source_manifest: dict, source_inventory: dict, catalog_manifest: dict, catalog_inventory: dict) -> dict:
    findings = validate_side("source", source_manifest, source_inventory)
    findings.extend(validate_side("catalog", catalog_manifest, catalog_inventory))
    findings.sort(key=lambda item: (item["side"], item["code"], item["path"], item["detail"]))
    source_entries = source_manifest.get("entries", []) if isinstance(source_manifest.get("entries"), list) else []
    catalog_entries = catalog_manifest.get("entries", []) if isinstance(catalog_manifest.get("entries"), list) else []
    source_submodules = [entry for entry in source_entries if isinstance(entry, dict) and entry.get("type") == "commit" and entry.get("mode") == "160000"]
    catalog_submodules = [entry for entry in catalog_entries if isinstance(entry, dict) and entry.get("type") == "commit" and entry.get("mode") == "160000"]
    return {
        "schema_version": "1.0.0",
        "repository": source_inventory.get("repository"),
        "source_commit_sha": source_manifest.get("commit_sha"),
        "catalog_commit_sha": catalog_manifest.get("commit_sha"),
        "status": "PASS" if not findings else "BLOCKED",
        "counts": {
            "blocking_findings": len(findings),
            "source_tree_entries": len(source_entries),
            "catalog_tree_entries": len(catalog_entries),
            "source_tree_blobs": sum(1 for entry in source_entries if isinstance(entry, dict) and entry.get("type") == "blob"),
            "catalog_tree_blobs": sum(1 for entry in catalog_entries if isinstance(entry, dict) and entry.get("type") == "blob"),
            "source_submodules": len(source_submodules),
            "catalog_submodules": len(catalog_submodules),
            "source_inventory_files": len(source_inventory.get("files", [])) if isinstance(source_inventory.get("files"), list) else None,
            "catalog_inventory_files": len(catalog_inventory.get("files", [])) if isinstance(catalog_inventory.get("files"), list) else None,
        },
        "findings": findings,
        "limitations": [
            "PASS proves path, object type, Git mode, object SHA, directory presence, and submodule pointer agreement for the supplied exact commits.",
            "Gitlinks (mode 160000, type commit) are recorded as submodule pointers and are not treated as blob files.",
            "A PASS does not prove parser semantics, symbol completeness, exclusion rationale correctness, or release readiness.",
        ],
    }


def main() -> int:
    from git_tree_manifest import build_manifest
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-manifest", required=True)
    parser.add_argument("--source-inventory", required=True)
    parser.add_argument("--catalog-manifest", required=True)
    parser.add_argument("--catalog-inventory", required=True)
    parser.add_argument("--root", default=".", help="Git checkout containing both pinned commits")
    parser.add_argument("--output", default="-")
    args = parser.parse_args()
    source_manifest = json.loads(Path(args.source_manifest).read_text(encoding="utf-8"))
    catalog_manifest = json.loads(Path(args.catalog_manifest).read_text(encoding="utf-8"))
    source_inventory = json.loads(Path(args.source_inventory).read_text(encoding="utf-8"))
    catalog_inventory = json.loads(Path(args.catalog_inventory).read_text(encoding="utf-8"))
    report = reconcile(source_manifest, source_inventory, catalog_manifest, catalog_inventory)
    # Rebuild each manifest directly from Git, independently of the supplied
    # artifact, so an empty or truncated manifest cannot pass by self-consistency.
    for side, manifest in (("source", source_manifest), ("catalog", catalog_manifest)):
        try:
            actual = build_manifest(Path(args.root), manifest.get("commit_sha", ""))
            if actual.get("entries") != manifest.get("entries") or actual.get("entry_count") != manifest.get("entry_count"):
                report["findings"].append({"code": "GIT_TREE_MANIFEST_NOT_AUTHENTICATED", "side": side, "path": "", "detail": "manifest differs from a fresh git ls-tree enumeration"})
        except Exception as exc:
            report["findings"].append({"code": "GIT_TREE_MANIFEST_REGENERATION_FAILED", "side": side, "path": "", "detail": f"{type(exc).__name__}: {exc}"})
    report["findings"].sort(key=lambda item: (item["side"], item["code"], item["path"], item["detail"]))
    report["counts"]["blocking_findings"] = len(report["findings"])
    report["status"] = "PASS" if not report["findings"] else "BLOCKED"
    rendered = json.dumps(report, ensure_ascii=True, sort_keys=True, indent=2) + "\n"
    if args.output == "-":
        print(rendered, end="")
    else:
        target = Path(args.output)
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(rendered, encoding="utf-8")
    print(json.dumps({"status": report["status"], "counts": report["counts"]}, sort_keys=True))
    return 0 if report["status"] == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
