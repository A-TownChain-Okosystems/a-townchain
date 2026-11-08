#!/usr/bin/env python3
"""Fail-closed validator for proposal-level ATC Engineering component evidence.

This tool validates evidence bindings; it does not establish semantic completeness,
runtime correctness, normative registration, or VERIFIED status.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
from pathlib import Path
from typing import Any

SHA_RE = re.compile(r"^[0-9a-f]{40}$")
ALLOWED_STATUSES = {
    "IMPLEMENTED", "TESTED", "CI_VERIFIED", "E2E_VERIFIED",
    "VERIFIED", "RESIDUAL", "BLOCKED", "EXCLUDED",
}


def git_blob_sha(data: bytes) -> str:
    return hashlib.sha1(b"blob " + str(len(data)).encode("ascii") + b"\0" + data).hexdigest()


def current_commit(root: Path) -> str | None:
    try:
        value = subprocess.check_output(
            ["git", "-C", str(root), "rev-parse", "HEAD"],
            text=True, stderr=subprocess.DEVNULL,
        ).strip()
    except (OSError, subprocess.CalledProcessError):
        return None
    return value if SHA_RE.fullmatch(value) else None


def validate(manifest: dict[str, Any], root: Path, expected_commit: str | None = None) -> dict[str, Any]:
    findings: list[dict[str, str]] = []
    candidates = manifest.get("candidates")
    if not isinstance(candidates, list):
        candidates = []
        findings.append({"code": "INVALID_CANDIDATES", "status": "BLOCKED",
                         "message": "manifest.candidates must be an array"})

    ids: set[str] = set()
    for index, candidate in enumerate(candidates):
        prefix = f"candidates[{index}]"
        if not isinstance(candidate, dict):
            findings.append({"code": "INVALID_CANDIDATE", "status": "BLOCKED",
                             "message": f"{prefix} must be an object"})
            continue
        component_id = candidate.get("component_id")
        if not isinstance(component_id, str) or not component_id.strip():
            findings.append({"code": "MISSING_COMPONENT_ID", "status": "BLOCKED",
                             "message": f"{prefix} has no component_id"})
        elif component_id in ids:
            findings.append({"code": "DUPLICATE_COMPONENT_ID", "status": "BLOCKED",
                             "message": f"duplicate component_id: {component_id}"})
        else:
            ids.add(component_id)

        status = candidate.get("status")
        if status not in ALLOWED_STATUSES:
            findings.append({"code": "INVALID_STATUS", "status": "BLOCKED",
                             "message": f"{prefix} has invalid status {status!r}"})
        if status == "VERIFIED":
            # The validator cannot independently infer semantic requirements.
            proof = candidate.get("verification_evidence")
            if not isinstance(proof, list) or not proof:
                findings.append({"code": "VERIFIED_WITHOUT_PROOF", "status": "BLOCKED",
                                 "message": f"{prefix} claims VERIFIED without verification_evidence"})

        evidence = candidate.get("evidence")
        if not isinstance(evidence, list) or not evidence:
            findings.append({"code": "MISSING_SOURCE_EVIDENCE", "status": "BLOCKED",
                             "message": f"{prefix} has no source evidence"})
            continue

        for eidx, item in enumerate(evidence):
            ep = f"{prefix}.evidence[{eidx}]"
            if not isinstance(item, dict):
                findings.append({"code": "INVALID_EVIDENCE", "status": "BLOCKED",
                                 "message": f"{ep} must be an object"})
                continue
            path_value = item.get("path")
            blob_value = item.get("blob_sha")
            if not isinstance(path_value, str) or not path_value.strip():
                findings.append({"code": "MISSING_SOURCE_PATH", "status": "BLOCKED",
                                 "message": f"{ep} has no source path"})
                continue
            if not isinstance(blob_value, str) or not SHA_RE.fullmatch(blob_value):
                findings.append({"code": "INVALID_BLOB_SHA", "status": "BLOCKED",
                                 "message": f"{ep} has missing or invalid Git blob SHA"})
                continue
            path = Path(path_value)
            if path.is_absolute() or ".." in path.parts:
                findings.append({"code": "UNSAFE_SOURCE_PATH", "status": "BLOCKED",
                                 "message": f"{ep} path must be repository-relative"})
                continue
            source = root / path
            if not source.is_file():
                findings.append({"code": "SOURCE_PATH_NOT_FOUND", "status": "BLOCKED",
                                 "message": f"{ep} source path does not exist: {path_value}"})
                continue
            actual_blob = git_blob_sha(source.read_bytes())
            if actual_blob != blob_value:
                findings.append({"code": "STALE_BLOB_SHA", "status": "BLOCKED",
                                 "message": f"{ep} blob SHA mismatch: expected {blob_value}, actual {actual_blob}"})
                continue
            symbols = item.get("symbols", [])
            if not isinstance(symbols, list):
                findings.append({"code": "INVALID_SYMBOL_LIST", "status": "BLOCKED",
                                 "message": f"{ep}.symbols must be an array"})
                continue
            text = source.read_text(encoding="utf-8", errors="replace")
            for symbol in symbols:
                if not isinstance(symbol, str) or not symbol.strip():
                    findings.append({"code": "INVALID_SYMBOL", "status": "BLOCKED",
                                     "message": f"{ep} contains an empty or invalid symbol"})
                elif symbol not in text:
                    findings.append({"code": "SYMBOL_NOT_FOUND", "status": "BLOCKED",
                                     "message": f"{ep} symbol not found in source: {symbol}"})

        if not isinstance(candidate.get("contract_refs"), list) or not candidate.get("contract_refs"):
            findings.append({"code": "MISSING_CONTRACT_BINDING", "status": "BLOCKED",
                             "message": f"{prefix} has no contract_refs; absence is not proof of no contract"})
        if not isinstance(candidate.get("runtime_wiring_evidence"), list) or not candidate.get("runtime_wiring_evidence"):
            findings.append({"code": "MISSING_RUNTIME_WIRING", "status": "BLOCKED",
                             "message": f"{prefix} has no runtime_wiring_evidence"})
        decision = candidate.get("decision")
        if decision == "EXCLUDED" and not candidate.get("exclusion_rationale"):
            findings.append({"code": "UNJUSTIFIED_EXCLUSION", "status": "BLOCKED",
                             "message": f"{prefix} exclusion has no exclusion_rationale"})

    observed = manifest.get("observed_commit_sha") or manifest.get("source_commit_sha")
    target = expected_commit or current_commit(root)
    if target and observed != target:
        findings.append({"code": "STALE_MANIFEST_COMMIT", "status": "BLOCKED",
                         "message": f"manifest commit {observed!r} does not match checked commit {target}"})

    return {
        "schema_version": "1.0.0",
        "status": "BLOCKED" if findings else "RESIDUAL",
        "candidate_count": len(candidates),
        "finding_count": len(findings),
        "findings": findings,
        "limitations": [
            "Passing this validator does not establish normative registration, semantic completeness, test execution, CI success, or end-to-end behavior.",
            "A clean result is RESIDUAL, never VERIFIED; verification requires claim-specific independent evidence.",
            "Unknown inventory deltas must remain unknown until a complete bidirectional comparison is executed.",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("manifest", type=Path)
    parser.add_argument("--root", type=Path, default=Path("."))
    parser.add_argument("--expected-commit")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    try:
        manifest = json.loads(args.manifest.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        print(json.dumps({"status": "BLOCKED", "error": str(exc)}, indent=2))
        return 2
    if not isinstance(manifest, dict):
        print(json.dumps({"status": "BLOCKED", "error": "manifest root must be an object"}, indent=2))
        return 2
    report = validate(manifest, args.root.resolve(), args.expected_commit)
    rendered = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(rendered, encoding="utf-8")
    print(rendered, end="")
    return 1 if report["status"] == "BLOCKED" else 0


if __name__ == "__main__":
    raise SystemExit(main())
