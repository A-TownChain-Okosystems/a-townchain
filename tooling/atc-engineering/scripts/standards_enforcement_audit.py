#!/usr/bin/env python3
"""Offline ATC standards enforcement audit.

Audits a checked-out organization workspace against the normative repository
baseline in ATC-STD-201/202/203 and records machine-readable findings.
No network access is performed.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path

ORG = "A-TownChain-Okosystems"
CLASSIFICATION = {
    "P0": "security/critical correctness or governance bypass",
    "P1": "correctness/security/integration blocking readiness",
    "P2": "completeness/quality/documentation or non-critical integration",
    "P3": "hygiene/informational improvement",
}


def read(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        return ""


def path_exists(repo: Path, rel: str) -> bool:
    return (repo / rel).exists()


def extract_scalar(text: str, key: str) -> str | None:
    m = re.search(rf"^\s*{re.escape(key)}:\s*[\"']?([^\"'\n#]+?)[\"']?\s*$", text, re.M)
    return m.group(1).strip() if m else None


def finding(repo: str, severity: str, category: str, family: str, tags: list[str],
            path: str, description: str, remediation: str) -> dict:
    return {
        "repository": repo,
        "severity": severity,
        "class": CLASSIFICATION[severity],
        "category": category,
        "family": family,
        "tags": sorted(set(tags)),
        "path": path,
        "description": description,
        "remediation": remediation,
    }


def audit_repo(repo: Path) -> list[dict]:
    name = repo.name
    findings: list[dict] = []
    metadata = read(repo / ".atc/repository.yaml")
    compliance = read(repo / ".atc/compliance.yaml")
    maturity = extract_scalar(metadata, "maturity") or extract_scalar(compliance, "level") or "R0"

    if metadata:
        declared_name = extract_scalar(metadata, "name")
        if declared_name and declared_name != name:
            findings.append(finding(name, "P1", "consistency", "repository-metadata",
                                    ["metadata", "name", "ATC-STD-201"], ".atc/repository.yaml",
                                    f"repository.name={declared_name!r} differs from directory/repository name {name!r}.",
                                    "Set repository.name to the exact GitHub repository name."))
    elif maturity != "R0":
        findings.append(finding(name, "P1", "completeness", "repository-metadata",
                                ["metadata", "ATC-STD-201", maturity], ".atc/repository.yaml",
                                f"ATC-STD-201 M-04 requires .atc/repository.yaml for {maturity}.",
                                "Add a valid .atc/repository.yaml matching the repository and registry."))

    required = ["LICENSE", "README.md"]
    if maturity in {"R2", "R3", "R4"}:
        required += ["SECURITY.md", "CHANGELOG.md", ".atc/ownership.yaml",
                     ".atc/lifecycle.yaml", ".atc/compliance.yaml"]
    if maturity in {"R1", "R2", "R3", "R4"}:
        required += [".atc/repository.yaml", "docs/REPOSITORY_STANDARD.md"]
        if not path_exists(repo, ".github/workflows"):
            findings.append(finding(name, "P1", "ci", "governance/ci",
                                    ["M-13", "ATC-STD-201", "CI"], ".github/workflows",
                                    "R1+ repository has no .github/workflows directory; ATC-STD-201 M-13 requires running CI.",
                                    "Install a governance workflow plus product-specific validation where applicable."))
        if not path_exists(repo, "tests") and not any(repo.glob("**/*test*")):
            findings.append(finding(name, "P1", "testing", "governance/ci",
                                    ["M-13", "ATC-STD-201", maturity, "tests"], "tests",
                                    "No tests directory or test-named file found for an R1+ repository.",
                                    "Add executable tests appropriate to the repository's language and maturity."))
    if maturity in {"R2", "R3", "R4"}:
        required += []
        if not path_exists(repo, "docs/decisions") and not path_exists(repo, "DECISIONS_REGISTER.md"):
            findings.append(finding(name, "P1", "completeness", "governance/decisions",
                                    ["M-14", "ATC-STD-201", maturity, "ADR"], "docs/decisions",
                                    "R2+ repository has neither docs/decisions/ nor a central DECISIONS_REGISTER reference.",
                                    "Add ADR storage or an explicit, valid central decisions-register reference."))

    for rel in required:
        if not path_exists(repo, rel):
            sev = "P1" if rel in {"LICENSE", "README.md", ".atc/repository.yaml", ".atc/compliance.yaml"} else "P2"
            findings.append(finding(name, sev, "completeness", "repository-governance",
                                    ["ATC-STD-201", maturity, rel.replace("/", "-")], rel,
                                    f"Required artifact {rel} is missing for maturity {maturity}.",
                                    f"Add and validate {rel}; do not mark the repository compliant until verified."))

    if maturity in {"R3", "R4"}:
        readme = read(repo / "README.md")
        for keyword in ("Purpose", "Scope", "Architecture", "Testing", "Security", "Roadmap", "Versioning", "License"):
            if keyword.lower() not in readme.lower():
                findings.append(finding(name, "P2", "documentation", "repository-documentation",
                                        ["M-03", "ATC-STD-201", maturity, "README"], "README.md",
                                        f"README does not contain the required R3/R4 documentation concept: {keyword}.",
                                        "Complete the normative README sections or document an explicit justified exception."))
        if "ATC COMPLIANCE:" not in readme:
            findings.append(finding(name, "P1", "governance", "compliance-badge",
                                    ["M-15", "ATC-STD-201", maturity, "badge"], "README.md",
                                    "R3/R4 README is missing the mandatory ATC Compliance badge.",
                                    "Add a machine-detectable compliance badge whose gate values are evidence-backed."))

    manifest = read(repo / "AGENT_MANIFEST.md")
    if not manifest and name != "demo-repository":
        findings.append(finding(name, "P1", "governance", "agent-governance",
                                ["AGENT_MANIFEST", "ATC-AI", "governance"], "AGENT_MANIFEST.md",
                                "No root AGENT_MANIFEST.md found in an official product repository.",
                                "Install the current generated agent manifest from the standards SSOT."))
    if manifest:
        m = re.search(r"(\d+)\s+Standards", manifest)
        if m and int(m.group(1)) < 505:
            findings.append(finding(name, "P2", "consistency", "agent-governance/registry-sync",
                                    ["stale-manifest", "505-standards", "SSOT-sync"], "AGENT_MANIFEST.md",
                                    f"Manifest advertises {m.group(1)} standards while the current registry matrix is 505.",
                                    "Regenerate AGENT_MANIFEST.md from the current standards registry and verify its digest."))

    secret_patterns = [
        r"BEGIN (?:RSA|EC|OPENSSH|PRIVATE) KEY",
        r"\bAKIA[0-9A-Z]{16}\b",
        r"\bghp_[A-Za-z0-9]{20,}\b",
        r"\bgithub_pat_[A-Za-z0-9_]{20,}\b",
    ]
    for pattern in secret_patterns:
        for p in repo.rglob("*"):
            if not p.is_file() or ".git" in p.parts or "archive" in p.parts:
                continue
            if re.search(pattern, read(p)):
                findings.append(finding(name, "P0", "security", "credential-exposure",
                                        ["M-12", "secret", "ATC-STD-203"], str(p.relative_to(repo)),
                                        f"Credential/private-key pattern matched: {pattern}.",
                                        "Remove the credential from history as required, rotate it, and add a regression guard."))
                break

    workflow_dir = repo / ".github/workflows"
    if workflow_dir.is_dir():
        for wf in workflow_dir.glob("*.y*ml"):
            text = read(wf)
            if re.search(r"pull_request_target\s*:", text):
                findings.append(finding(name, "P1", "security", "ci-execution-boundary",
                                        ["pull_request_target", "CI", "security"], str(wf.relative_to(repo)),
                                        "pull_request_target workflow requires an explicit privilege review.",
                                        "Minimize permissions, never execute untrusted PR code with write credentials, and document the trust boundary."))
            if re.search(r"uses:\s*[^\s@]+@(main|master|latest)\s*$", text, re.M):
                findings.append(finding(name, "P1", "security", "ci-supply-chain",
                                        ["mutable-action-ref", "CI", "supply-chain"], str(wf.relative_to(repo)),
                                        "Workflow references a mutable action ref (main/master/latest).",
                                        "Pin third-party actions to immutable release SHAs."))
            if "permissions:" not in text:
                findings.append(finding(name, "P2", "security", "ci-permissions",
                                        ["permissions", "least-privilege", "CI"], str(wf.relative_to(repo)),
                                        "Workflow has no explicit permissions block.",
                                        "Declare least-privilege workflow/job permissions explicitly."))

    for p in repo.rglob("*"):
        if not p.is_file() or ".git" in p.parts or "archive" in p.parts or "target" in p.parts:
            continue
        if p.suffix not in {".rs", ".py", ".js", ".ts", ".tsx", ".sh", ".go", ".c", ".h"}:
            continue
        if re.search(r"\b(unimplemented!\(\)|NotImplementedError|TODO\(\)|todo!\(\))", read(p)):
            findings.append(finding(name, "P1", "completeness", "stub/placeholder",
                                    ["stub", "TODO", "implementation", "ATC-STD-BUG"], str(p.relative_to(repo)),
                                    "Active source contains an unimplemented/TODO placeholder that may represent an incomplete function.",
                                    "Classify it explicitly as planned/non-canonical or implement it with tests and re-audit."))

    hashes: dict[str, list[str]] = {}
    for p in repo.rglob("*"):
        if not p.is_file() or ".git" in p.parts or "archive" in p.parts or "target" in p.parts or "node_modules" in p.parts:
            continue
        try:
            digest = hashlib.sha256(p.read_bytes()).hexdigest()
        except OSError:
            continue
        hashes.setdefault(digest, []).append(str(p.relative_to(repo)))
    for digest, paths in hashes.items():
        if len(paths) > 1:
            findings.append(finding(name, "P3", "quality", "duplication",
                                    ["duplicate-content", "hygiene"], ";".join(paths),
                                    f"Exact duplicate content group ({len(paths)} files), sha256={digest[:16]}.",
                                    "Classify intentional copies or consolidate duplicate implementation/documentation."))
    return findings


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path, help="directory containing checked-out repositories")
    parser.add_argument("--json", dest="json_path", type=Path, default=None)
    args = parser.parse_args()
    if not args.root.is_dir():
        print(f"error: not a directory: {args.root}", file=sys.stderr)
        return 2
    repos = sorted(p for p in args.root.iterdir() if (p / ".git").is_dir())
    all_findings = [f for repo in repos for f in audit_repo(repo)]
    report = {
        "audit": "ATC-STD-ENFORCEMENT-OFFLINE-001",
        "organization": ORG,
        "repositories_scanned": len(repos),
        "blocking_findings": sum(1 for f in all_findings if f["severity"] in {"P0", "P1"}),
        "findings": all_findings,
    }
    out = args.json_path or (args.root / "STANDARDS_ENFORCEMENT_AUDIT.json")
    out.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({k: report[k] for k in ("audit", "repositories_scanned", "blocking_findings")}, ensure_ascii=False))
    return 1 if report["blocking_findings"] else 0


if __name__ == "__main__":
    raise SystemExit(main())
