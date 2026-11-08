#!/usr/bin/env python3
"""Deterministic discovery scan for source symbols.

This is a candidate inventory generator, not a compiler/parser and not proof of
functional correctness. It records exact source SHA supplied by the caller and
per-file SHA-256. Every result remains DISCOVERY_ONLY until reconciled with
normative contracts and tests.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
from collections import Counter
from pathlib import Path

SKIP_DIRS = {
    ".git", ".hg", ".svn", "target", "node_modules", "vendor", "dist",
    "build", ".venv", "venv", "__pycache__", "coverage", "generated",
}
EXTENSIONS = {
    ".rs": "rust", ".py": "python", ".js": "javascript", ".jsx": "javascript",
    ".mjs": "javascript", ".cjs": "javascript", ".ts": "typescript",
    ".tsx": "typescript", ".go": "go", ".java": "java", ".cs": "csharp",
    ".c": "c", ".h": "c", ".cc": "cpp", ".hh": "cpp", ".cpp": "cpp",
    ".hpp": "cpp", ".swift": "swift", ".kt": "kotlin", ".kts": "kotlin",
    ".rb": "ruby", ".php": "php",
}
PATTERNS = {
    "rust": [
        ("function", re.compile(r"^\s*pub(?:\([^)]*\))?\s+(?:async\s+)?(?:unsafe\s+)?(?:extern\s+\"[^\"]+\"\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)),
        ("type", re.compile(r"^\s*pub(?:\([^)]*\))?\s+(?:struct|enum|trait|type|union)\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)),
        ("macro", re.compile(r"^\s*\#\[macro_export\]\s*\n\s*macro_rules!\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)),
    ],
    "python": [
        ("function", re.compile(r"^\s*(?:async\s+)?def\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)),
        ("type", re.compile(r"^\s*class\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)),
    ],
    "javascript": [
        ("function", re.compile(r"^\s*export\s+(?:default\s+)?(?:async\s+)?function\*?\s+([A-Za-z_$][\w$]*)", re.M)),
        ("type", re.compile(r"^\s*export\s+(?:default\s+)?(?:abstract\s+)?class\s+([A-Za-z_$][\w$]*)", re.M)),
        ("value", re.compile(r"^\s*export\s+(?:const|let|var)\s+([A-Za-z_$][\w$]*)", re.M)),
    ],
    "typescript": [
        ("function", re.compile(r"^\s*export\s+(?:default\s+)?(?:declare\s+)?(?:async\s+)?function\*?\s+([A-Za-z_$][\w$]*)", re.M)),
        ("type", re.compile(r"^\s*export\s+(?:default\s+)?(?:abstract\s+)?(?:class|interface|type|enum)\s+([A-Za-z_$][\w$]*)", re.M)),
        ("value", re.compile(r"^\s*export\s+(?:const|let|var)\s+([A-Za-z_$][\w$]*)", re.M)),
    ],
    "go": [
        ("function", re.compile(r"^func\s+(?:\([^\n)]*\)\s*)?([A-Z][A-Za-z0-9_]*)\s*\(", re.M)),
        ("type", re.compile(r"^type\s+([A-Z][A-Za-z0-9_]*)\s+(?:struct|interface|\[|[A-Za-z_*])", re.M)),
    ],
    "java": [
        ("type", re.compile(r"^\s*(?:(?:public|protected)\s+)?(?:(?:abstract|final|sealed|static)\s+)*(?:class|interface|enum|record)\s+([A-Za-z_$][\w$]*)", re.M)),
        ("function", re.compile(r"^\s*(?:public|protected)\s+(?:(?:static|final|abstract|synchronized|native|default)\s+)*[\w<>?,.\[\] ]+\s+([A-Za-z_$][\w$]*)\s*\(", re.M)),
    ],
    "csharp": [
        ("type", re.compile(r"^\s*public\s+(?:(?:abstract|sealed|static|partial)\s+)*(?:class|interface|struct|enum|record)\s+([A-Za-z_][\w]*)", re.M)),
        ("function", re.compile(r"^\s*public\s+(?:(?:static|async|virtual|override|abstract|sealed|extern|unsafe|new)\s+)*[\w<>?,.\[\] ]+\s+([A-Za-z_][\w]*)\s*\(", re.M)),
    ],
    "c": [
        ("function", re.compile(r"^\s*(?:extern\s+)?(?:const\s+)?[A-Za-z_][\w\s*]*\s+([A-Za-z_][\w]*)\s*\([^;{}]*\)\s*\{", re.M)),
    ],
    "cpp": [
        ("function", re.compile(r"^\s*(?:extern\s+\"C\"\s+)?(?:inline\s+|static\s+|virtual\s+|constexpr\s+|friend\s+|explicit\s+)*[\w:<>~,\s*&]+\s+([A-Za-z_][\w:]*)\s*\([^;{}]*\)\s*(?:const\s*)?(?:noexcept\s*)?\{", re.M)),
        ("type", re.compile(r"^\s*(?:class|struct|enum\s+class|enum)\s+([A-Za-z_][\w]*)", re.M)),
    ],
    "swift": [
        ("function", re.compile(r"^\s*(?:public|open)\s+(?:static\s+|class\s+|override\s+|mutating\s+|async\s+|throws\s+)*func\s+([A-Za-z_][\w]*)", re.M)),
        ("type", re.compile(r"^\s*(?:public|open)\s+(?:final\s+)?(?:class|struct|enum|protocol|actor)\s+([A-Za-z_][\w]*)", re.M)),
    ],
    "kotlin": [
        ("function", re.compile(r"^\s*(?:public\s+)?(?:suspend\s+|inline\s+|operator\s+|override\s+|open\s+|final\s+)*fun\s+(?:<[^>]+>\s*)?([A-Za-z_][\w]*)", re.M)),
        ("type", re.compile(r"^\s*(?:public\s+)?(?:data\s+|sealed\s+|abstract\s+|open\s+)?(?:class|interface|object|enum\s+class)\s+([A-Za-z_][\w]*)", re.M)),
    ],
    "ruby": [
        ("function", re.compile(r"^\s*def\s+([A-Za-z_][\w!?=]*)", re.M)),
        ("type", re.compile(r"^\s*class\s+([A-Z][A-Za-z0-9_:]*)", re.M)),
    ],
    "php": [
        ("function", re.compile(r"^\s*(?:(?:public|protected)\s+)?(?:static\s+|final\s+|abstract\s+)?function\s+&?([A-Za-z_][\w]*)", re.M)),
        ("type", re.compile(r"^\s*(?:abstract\s+|final\s+)?(?:class|interface|trait|enum)\s+([A-Za-z_][\w]*)", re.M)),
    ],
}


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def should_skip(path: Path) -> bool:
    return any(part in SKIP_DIRS for part in path.parts)


def scan(root: Path, repository: str, source_sha: str) -> dict:
    files = []
    symbols_total = 0
    language_counts: Counter[str] = Counter()
    unclassified = []
    for path in sorted(root.rglob("*")):
        if not path.is_file() or should_skip(path.relative_to(root)):
            continue
        language = EXTENSIONS.get(path.suffix.lower())
        if language is None:
            continue
        language_counts[language] += 1
        try:
            source = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError) as exc:
            unclassified.append({"path": path.relative_to(root).as_posix(), "reason": type(exc).__name__})
            continue
        found = set()
        for kind, pattern in PATTERNS.get(language, []):
            for match in pattern.finditer(source):
                name = match.group(1)
                # Exclude common test-only/private symbols from the public API candidate set.
                if language == "python" and name.startswith("_"):
                    continue
                found.add((kind, name))
        symbols = [{"kind": kind, "name": name} for kind, name in sorted(found)]
        symbols_total += len(symbols)
        files.append({
            "path": path.relative_to(root).as_posix(),
            "language": language,
            "file_sha256": sha256_file(path),
            "candidate_public_symbols": symbols,
            "candidate_symbol_count": len(symbols),
        })
    return {
        "schema_id": "ATC-SOURCE-SYMBOL-INVENTORY-001",
        "schema_version": "1.0.0",
        "repository": repository,
        "source_sha": source_sha,
        "scanner": "tools/scan_public_symbols.py",
        "scan_mode": "LEXICAL_CANDIDATE_DISCOVERY",
        "completeness_status": "RESIDUAL",
        "functional_verification": "NOT_PERFORMED",
        "limitations": [
            "Regex/lexical discovery is not a compiler AST and can miss macro-generated, dynamically registered, conditional, or indirect entrypoints.",
            "Candidate symbols are not automatically equated to catalog function contracts.",
            "Generated/vendor/build paths are excluded by policy; excluded path policy must be reviewed per repository.",
            "File hashes bind results to content, but source_sha must be supplied from the exact checked-out repository HEAD."
        ],
        "summary": {
            "source_files_scanned": len(files),
            "candidate_public_symbols": symbols_total,
            "files_by_language": dict(sorted(language_counts.items())),
            "files_with_read_errors": len(unclassified),
        },
        "files": files,
        "unclassified_files": unclassified,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", required=True, type=Path, help="Root of one checked-out repository")
    parser.add_argument("--repository", required=True, help="Canonical owner/repository name")
    parser.add_argument("--source-sha", required=True, help="Exact git HEAD SHA of the checked-out repository")
    parser.add_argument("--output", required=True, type=Path, help="Output JSON path")
    args = parser.parse_args()
    if not re.fullmatch(r"[0-9a-fA-F]{40}", args.source_sha):
        parser.error("--source-sha must be a full 40-character Git commit SHA")
    root = args.root.resolve()
    if not root.is_dir():
        parser.error("--root must be an existing directory")
    try:
        actual_sha = subprocess.run(
            ["git", "-C", str(root), "rev-parse", "HEAD"],
            check=True, capture_output=True, text=True,
        ).stdout.strip().lower()
    except (OSError, subprocess.CalledProcessError):
        parser.error("--root must be a Git checkout with a readable HEAD")
    if actual_sha != args.source_sha.lower():
        parser.error(
            f"--source-sha mismatch: supplied {args.source_sha.lower()}, "
            f"actual HEAD {actual_sha}"
        )
    result = scan(root, args.repository, actual_sha)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"repository": args.repository, "source_sha": args.source_sha.lower(), **result["summary"], "completeness_status": "RESIDUAL", "output": str(args.output)}, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
