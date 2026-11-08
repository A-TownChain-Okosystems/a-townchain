#!/usr/bin/env python3
"""CI-independent file-format and implementation-language audit.

This is a source-tree policy checker, not a style preference engine.  It
flags files whose format/language conflicts with the repository architecture
or with an explicitly declared canonical implementation boundary.  It does
not rewrite files automatically: language migration is an architectural
change and must be evidenced by tests and integration checks.
"""
from __future__ import annotations

import argparse
from collections import Counter
from pathlib import Path

IGNORED_DIRS = {
    ".git", ".venv", "node_modules", "target", "dist", "build",
    "__pycache__", ".pytest_cache", ".mypy_cache", "vendor",
}

# Canonical implementation boundaries established by the ATC architecture.
# Values are allowed primary implementation languages, not exhaustive lists.
CANONICAL_LANGUAGES = {
    "atc-standards": {"python", "yaml", "markdown"},
    "atc-engineering": {"python", "bash", "rust", "yaml", "markdown"},
    "atclang": {"rust", "python", "atclang", "yaml", "markdown"},
    "atc-vm": {"rust", "yaml", "markdown"},
    "atc-node": {"rust", "yaml", "markdown"},
    "atc-shivacore": {"rust", "yaml", "markdown"},
    "globus-os": {"rust", "typescript", "javascript", "yaml", "markdown"},
    "a-townchain": {"rust", "python", "yaml", "markdown"},
    "atc-contracts": {"rust", "atclang", "yaml", "markdown"},
    "atc-zkp": {"rust", "yaml", "markdown"},
    "atc-storage": {"rust", "yaml", "markdown"},
    "atc-indexer": {"rust", "typescript", "yaml", "markdown"},
    "atc-oracle": {"rust", "python", "yaml", "markdown"},
    "atc-interop": {"rust", "yaml", "markdown"},
    "atc-algorithm": {"rust", "yaml", "markdown"},
    "atc-mining": {"rust", "yaml", "markdown"},
}

EXT_LANGUAGE = {
    ".rs": "rust", ".py": "python", ".sh": "bash", ".bash": "bash",
    ".ts": "typescript", ".tsx": "typescript", ".js": "javascript",
    ".jsx": "javascript", ".yaml": "yaml", ".yml": "yaml",
    ".md": "markdown", ".json": "json", ".toml": "toml",
    ".atc": "atclang", ".atvm": "atclang", ".sol": "solidity",
    ".go": "go", ".c": "c", ".h": "c", ".cpp": "cpp", ".hpp": "cpp",
}

FORMAT_POLICY = {
    "requirements": {"yaml", "toml", "json", "markdown"},
    "workflow": {"yaml"},
    "governance": {"yaml", "markdown"},
    "lock": {"toml", "json", "yaml"},
}


def classify(path: Path) -> str | None:
    return EXT_LANGUAGE.get(path.suffix.lower())


def policy_for(path: Path) -> str | None:
    p = str(path).lower()
    if "/.github/workflows/" in p:
        return "workflow"
    if "/.atc/" in p or path.name.lower() in {"repository.yaml", "ownership.yaml", "lifecycle.yaml", "compliance.yaml"}:
        return "governance"
    if "requirements" in path.name.lower() or "dependencies" in path.name.lower():
        return "requirements"
    if path.name.lower() in {"cargo.lock", "package-lock.json", "pnpm-lock.yaml", "yarn.lock"}:
        return "lock"
    return None


def audit(repo: Path) -> tuple[list[str], Counter[str], Counter[str]]:
    findings: list[str] = []
    languages: Counter[str] = Counter()
    formats: Counter[str] = Counter()
    repo_name = repo.name
    allowed = CANONICAL_LANGUAGES.get(repo_name)

    for path in repo.rglob("*"):
        if not path.is_file() or any(part in IGNORED_DIRS for part in path.parts):
            continue
        lang = classify(path)
        if lang:
            languages[lang] += 1
            formats[path.suffix.lower()] += 1
        policy = policy_for(path)
        if policy:
            allowed_formats = FORMAT_POLICY[policy]
            if lang and lang not in allowed_formats:
                findings.append(
                    f"F-FORMAT {path.relative_to(repo)}: {lang} is not an allowed "
                    f"{policy} format ({', '.join(sorted(allowed_formats))})"
                )

        # Canonical-language drift is a finding only for implementation files;
        # docs/config are intentionally exempt.
        if allowed and lang in {"python", "javascript", "typescript", "go", "c", "cpp", "solidity"} and lang not in allowed:
            findings.append(
                f"F-LANGUAGE {path.relative_to(repo)}: {lang} is outside the "
                f"declared canonical language set ({', '.join(sorted(allowed))}); "
                "review whether this is legacy, tooling, or a justified boundary"
            )
    return findings, languages, formats


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path, help="directory containing repository directories")
    args = parser.parse_args()
    root = args.root
    if not root.is_dir():
        parser.error(f"not a directory: {root}")

    total = 0
    failed = False
    for repo in sorted(p for p in root.iterdir() if p.is_dir() and p.name not in IGNORED_DIRS):
        findings, languages, formats = audit(repo)
        print(f"== {repo.name} ==")
        print("languages:", ", ".join(f"{k}={v}" for k, v in sorted(languages.items())) or "none")
        print("formats:", ", ".join(f"{k}={v}" for k, v in sorted(formats.items())) or "none")
        for finding in findings:
            print("FINDING", finding)
        total += len(findings)
        failed |= bool(findings)

    print(f"TOTAL_FINDINGS={total}")
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
