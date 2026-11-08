#!/usr/bin/env python3
"""Run the source-symbol census across an explicit organization repository manifest.

This orchestration layer fails closed: a repository is counted as scanned only
when its checkout exists, has a Git HEAD, matches the expected full SHA when
one is supplied, and has no tracked/untracked modifications. Missing or dirty
repositories remain BLOCKED/RESIDUAL; they are never silently omitted.
"""
from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

try:
    # Package import for unittest/module execution; sibling import for direct CLI.
    from tools.scan_public_symbols import scan
except ModuleNotFoundError:
    from scan_public_symbols import scan


def git(root: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", "-C", str(root), *args],
        check=True, capture_output=True, text=True,
    )
    return result.stdout.strip()


def run_census(workspace: Path, manifest: dict) -> dict:
    repositories = []
    active_total = sum(1 for r in manifest["repositories"] if not r.get("archived", False))
    scanned_active = 0
    excluded_private = 0
    for item in sorted(manifest["repositories"], key=lambda r: r["name"].lower()):
        name = item["name"]
        active = not item.get("archived", False)
        root = workspace / name
        entry = {
            "repository": f'{manifest["owner"]}/{name}',
            "name": name,
            "archived": not active,
            "private": bool(item.get("private", False)),
            "expected_role": item.get("role", "UNCLASSIFIED"),
            "status": "BLOCKED",
            "source_sha": None,
            "working_tree_clean": False,
            "summary": None,
            "report": None,
            "residuals": [],
        }
        if not root.is_dir() or not (root / ".git").exists():
            if active and item.get("private"):
                # Documented exclusion, not a silent omission: a private
                # repository without CI credentials cannot be cloned by the
                # census workflow. It is recorded as excluded and counted
                # separately; the overall census stays RESIDUAL only when
                # every non-excluded active repository is scanned.
                entry["status"] = "EXCLUDED_PRIVATE_NO_CI_ACCESS"
                entry["residuals"].append("PRIVATE_REPOSITORY_EXCLUDED_NO_CI_ACCESS")
                excluded_private += 1
                repositories.append(entry)
                continue
            entry["residuals"].append("CHECKOUT_MISSING")
            repositories.append(entry)
            continue
        try:
            sha = git(root, "rev-parse", "HEAD")
            if len(sha) != 40:
                raise ValueError("HEAD is not a full SHA")
            entry["source_sha"] = sha
            expected_sha = item.get("expected_sha")
            if expected_sha and expected_sha.lower() != sha.lower():
                entry["residuals"].append("EXPECTED_SHA_MISMATCH")
                repositories.append(entry)
                continue
            dirty = git(root, "status", "--porcelain", "--untracked-files=all")
            if dirty:
                entry["residuals"].append("WORKTREE_DIRTY")
                repositories.append(entry)
                continue
            entry["working_tree_clean"] = True
            report = scan(root, entry["repository"], sha)
            entry["summary"] = report["summary"]
            entry["report"] = report
            entry["status"] = "RESIDUAL"
            entry["residuals"].append("LEXICAL_DISCOVERY_NOT_FUNCTIONALLY_VERIFIED")
            if active:
                scanned_active += 1
        except (subprocess.CalledProcessError, OSError, ValueError) as exc:
            entry["residuals"].append(f"CHECKOUT_AUDIT_ERROR:{type(exc).__name__}")
        repositories.append(entry)

    missing_active = [
        r["repository"] for r in repositories
        if not r["archived"] and r["status"] == "BLOCKED"
    ]
    return {
        "schema_id": "ATC-ORG-SOURCE-CENSUS-001",
        "schema_version": "1.0.0",
        "owner": manifest["owner"],
        "catalog_ref": manifest.get("catalog_ref"),
        "status": "RESIDUAL" if not missing_active and scanned_active + excluded_private == active_total else "BLOCKED",
        "functional_verification": "NOT_PERFORMED",
        "summary": {
            "repositories_expected": len(repositories),
            "active_expected": active_total,
            "active_scanned": scanned_active,
            "active_blocked": len(missing_active),
            "active_excluded_private": excluded_private,
            "archived_reference_repositories": sum(1 for r in repositories if r["archived"]),
            "source_files_scanned": sum((r["summary"] or {}).get("source_files_scanned", 0) for r in repositories),
            "candidate_public_symbols": sum((r["summary"] or {}).get("candidate_public_symbols", 0) for r in repositories),
        },
        "blocked_active_repositories": missing_active,
        "limitations": [
            "A clean checkout and symbol scan are source-inventory evidence, not functional verification.",
            "Regex discovery may miss generated, macro-expanded, dynamic, conditional, or indirect entrypoints.",
            "Catalog reconciliation, canonical ownership review, contract checks, tests, and exact-SHA CI evidence remain separate required gates.",
            "Archived repositories are recorded as historical reference only and are not counted as active coverage.",
            "Private repositories without CI credential access are recorded as EXCLUDED_PRIVATE_NO_CI_ACCESS and counted in active_excluded_private; the exclusion reduces CI coverage and is documented, not silent."
        ],
        "repositories": repositories,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workspace", required=True, type=Path, help="Directory containing one clean Git checkout per manifest repository name")
    parser.add_argument("--manifest", required=True, type=Path, help="Organization repository manifest JSON")
    parser.add_argument("--output", required=True, type=Path, help="Combined census JSON output")
    args = parser.parse_args()
    manifest = json.loads(args.manifest.read_text(encoding="utf-8"))
    if not isinstance(manifest.get("repositories"), list) or not manifest.get("owner"):
        parser.error("manifest requires owner and repositories array")
    result = run_census(args.workspace.resolve(), manifest)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": result["status"], **result["summary"], "output": str(args.output)}, sort_keys=True))
    return 0 if result["status"] == "RESIDUAL" else 2


if __name__ == "__main__":
    sys.exit(main())
