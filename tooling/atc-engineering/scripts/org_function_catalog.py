#!/usr/bin/env python3
"""Deterministic, offline first-pass source catalog with explicit limitations."""
from __future__ import annotations

import argparse
import ast
import hashlib
import json
import os
import re
import subprocess
from datetime import datetime, timezone
from pathlib import Path

SCHEMA_VERSION = "1.1.0"
SKIP_DIRS = {".git", "target", "node_modules", ".venv", "venv", "dist", "build", "__pycache__"}
EXTENSIONS = {
    ".py": "python", ".rs": "rust", ".js": "javascript", ".jsx": "javascript",
    ".ts": "typescript", ".tsx": "typescript", ".go": "go", ".java": "java",
    ".cs": "csharp", ".c": "c", ".h": "c", ".cc": "cpp", ".cpp": "cpp",
    ".hpp": "cpp", ".sh": "shell", ".bash": "shell", ".yml": "yaml",
    ".yaml": "yaml", ".toml": "toml", ".json": "json", ".md": "markdown",
}
PATTERNS = {
    "rust": re.compile(r"^\s*(?:(?:pub(?:\([^)]*\))?)\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^>]*>)?\s*\(", re.M),
    "javascript": re.compile(r"^\s*(?:(?:export|async|static|public|private)\s+)*(?:function\s+([\w$]+)|(?:const|let|var)\s+([\w$]+)\s*=\s*(?:async\s*)?(?:function|\([^\n]*\)\s*=>|[\w$]+\s*=>))", re.M),
    "typescript": re.compile(r"^\s*(?:(?:export|declare|async|public|private|static)\s+)*function\s+([\w$]+)\s*(?:<[^>]*>)?\s*\(", re.M),
    "go": re.compile(r"^\s*func\s+(?:\([^)]*\)\s*)?([A-Za-z_][A-Za-z0-9_]*)\s*\(", re.M),
    "java": re.compile(r"^\s*(?:(?:public|protected|private|static|final|synchronized|abstract)\s+)+[\w<>?,.\[\]]+\s+([A-Za-z_$][\w$]*)\s*\(", re.M),
    "csharp": re.compile(r"^\s*(?:(?:public|protected|private|static|async|virtual|override|sealed)\s+)+[\w<>?,.\[\]]+\s+([A-Za-z_]\w*)\s*\(", re.M),
    "c": re.compile(r"^\s*(?:[\w*]+\s+)+([A-Za-z_]\w*)\s*\([^;{}]*\)\s*\{", re.M),
    "cpp": re.compile(r"^\s*(?:[\w:*&<>]+\s+)+([A-Za-z_]\w*)\s*\([^;{}]*\)\s*(?:const\s*)?\{", re.M),
    "shell": re.compile(r"^\s*(?:function\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*\(\s*\)\s*\{", re.M),
}

def git_sha(root: Path) -> str:
    env_sha = os.environ.get("ATC_CATALOG_SOURCE_SHA", "") or os.environ.get("GITHUB_SHA", "")
    if re.fullmatch(r"[0-9a-f]{40,64}", env_sha):
        return env_sha
    try:
        value = subprocess.check_output(["git", "-C", str(root), "rev-parse", "HEAD"], text=True, stderr=subprocess.DEVNULL).strip()
        if re.fullmatch(r"[0-9a-f]{40,64}", value):
            return value
    except (OSError, subprocess.CalledProcessError):
        pass
    return "0" * 40

def blob_sha(data: bytes) -> str:
    return hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()

def stable_symbol_id(repo: str, rel: str, kind: str, qualified_name: str, signature: str = "") -> str:
    """Identity excludes line numbers and includes signature to distinguish overloads."""
    identity = json.dumps([repo, rel, kind, qualified_name, signature.strip()], ensure_ascii=False, separators=(",", ":"))
    return hashlib.sha256(identity.encode("utf-8")).hexdigest()

def classify(path: Path) -> str:
    parts = {part.lower() for part in path.parts}
    if parts & {"archive", "archived", "legacy"}:
        return "archived"
    if parts & {"vendor", "third_party", "third-party"}:
        return "vendored"
    if "generated" in parts or path.name.endswith(".generated.ts"):
        return "generated"
    if parts & {"tests", "test", "fixtures", "__tests__"} or path.name.startswith("test_") or ".test." in path.name or ".spec." in path.name:
        return "test"
    if path.suffix.lower() in {".md", ".rst", ".adoc"} or path.name.lower().startswith("readme"):
        return "documentation"
    if path.suffix.lower() in {".yml", ".yaml", ".toml", ".json", ".ini", ".cfg"}:
        return "configuration"
    return "first_party"

def exclusion_record(repo: str, path: str, reason_code: str, reason: str, *,
                     category: str, source_blob_sha: str | None = None,
                     evidence_ref: str) -> dict:
    """Create a stable, source/policy-bound exclusion record; never infer semantic meaning."""
    identity = json.dumps([repo, path, reason_code], ensure_ascii=False, separators=(",", ":"))
    return {
        "exclusion_id": hashlib.sha256(identity.encode("utf-8")).hexdigest(),
        "repository": repo,
        "path": path,
        "reason_code": reason_code,
        "category": category,
        "reason": reason,
        "rationale": reason,
        "source_blob_sha": source_blob_sha,
        "evidence": [{"kind": "source_or_policy_reference", "reference": evidence_ref}],
    }


def symbol_record(repo: str, rel: str, language: str, kind: str, name: str, start: int, end: int,
                  signature: str, visibility: str, source_sha: str, commit: str, limitations: list[str]) -> dict:
    return {
        "schema_version": SCHEMA_VERSION, "symbol_id": stable_symbol_id(repo, rel, kind, name, signature),
        "repository": repo, "path": rel, "language": language, "symbol_kind": kind,
        "qualified_name": name, "signature": signature, "visibility": visibility, "summary": "",
        "parameters": [], "return_type": None, "errors": [], "side_effects": [],
        "security_boundary": None, "dependencies": [], "callers": [], "callees": [], "tests": [],
        "doc_url_or_path": None, "source_start_line": start, "source_end_line": max(start, end),
        "source_blob_sha": source_sha, "observed_commit_sha": commit, "status": "DETECTED",
        "canonical_owner": repo, "component_id": None, "contract_refs": [],
        "evidence": [f"{rel}:{start}-{max(start, end)}"], "limitations": limitations,
    }

def extract_python(repo: str, rel: str, text: str, sha: str, commit: str) -> list[dict]:
    try:
        tree = ast.parse(text, filename=rel)
    except SyntaxError as exc:
        raise ValueError(f"python parse error line {exc.lineno}: {exc.msg}") from exc
    lines, records = text.splitlines(), []
    def walk(body, prefix=""):
        for node in body:
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                name = f"{prefix}.{node.name}" if prefix else node.name
                start, end = node.lineno, getattr(node, "end_lineno", node.lineno) or node.lineno
                signature = lines[start - 1].strip() if 0 < start <= len(lines) else node.name
                records.append(symbol_record(repo, rel, "python", "method" if prefix else "function", name, start, end, signature,
                    "private" if node.name.startswith("_") else "public", sha, commit, []))
            elif isinstance(node, ast.ClassDef):
                name = f"{prefix}.{node.name}" if prefix else node.name
                start, end = node.lineno, getattr(node, "end_lineno", node.lineno) or node.lineno
                signature = lines[start - 1].strip() if 0 < start <= len(lines) else node.name
                records.append(symbol_record(repo, rel, "python", "class", name, start, end, signature,
                    "private" if node.name.startswith("_") else "public", sha, commit, []))
                walk(node.body, name)
    walk(tree.body)
    return records

def extract_heuristic(repo: str, rel: str, language: str, text: str, sha: str, commit: str) -> list[dict]:
    pattern = PATTERNS.get(language)
    if not pattern:
        return []
    lines, out = text.splitlines(), []
    for match in pattern.finditer(text):
        name_group = next((index for index, value in enumerate(match.groups(), start=1) if value), None)
        if name_group is None:
            continue
        name = match.group(name_group)
        # The pattern's leading \\s* can consume blank lines. Anchor the source
        # binding to the captured declaration name, not match.start(), or a
        # preceding blank line can produce an empty/wrong signature.
        declaration_pos = match.start(name_group)
        start = text.count("\n", 0, declaration_pos) + 1
        signature = lines[start - 1].strip() if start <= len(lines) else name
        vis = "public" if re.search(r"^\s*(?:pub\b|export\b|public\b)", signature) else "unknown"
        out.append(symbol_record(repo, rel, language, "function", name, start, start, signature, vis, sha, commit,
            ["heuristic declaration match; body range, overloads, nested scopes, visibility, and signature may be incomplete"]))
    return out

def validate_document_format(language: str, text: str) -> None:
    """Parse non-executable repository formats with format-specific parsers."""
    if language == "json":
        json.loads(text)
    elif language == "toml":
        import tomllib
        tomllib.loads(text)
    elif language == "yaml":
        import yaml
        list(yaml.compose_all(text))
    elif language == "markdown":
        from markdown_it import MarkdownIt
        MarkdownIt("commonmark").parse(text)
    else:
        raise ValueError(f"no document parser for {language}")


def extract_tree_sitter(repo: str, rel: str, language: str, text: str, sha: str, commit: str) -> list[dict]:
    """Extract Rust/Bash function declarations from concrete syntax trees."""
    from tree_sitter import Language, Parser

    if language == "rust":
        import tree_sitter_rust
        grammar = Language(tree_sitter_rust.language())
    elif language == "shell":
        import tree_sitter_bash
        grammar = Language(tree_sitter_bash.language())
    else:
        raise ValueError(f"no tree-sitter grammar for {language}")
    parser = Parser(grammar)
    tree = parser.parse(text.encode("utf-8"))
    root = tree.root_node
    if root.has_error:
        raise ValueError(f"{parser_language} syntax tree contains ERROR or missing nodes")

    source = text.encode("utf-8")
    records = []
    declaration_types = {"rust": {"function_item", "function_signature_item"},
                         "shell": {"function_definition"}}
    stack = [(root, "")]
    while stack:
        node, scope = stack.pop()
        # Macro token trees are opaque Rust syntax, not source declarations. Do
        # not descend into them: large nested token trees can stress native
        # grammar traversal and macro-generated declarations are not source-bound.
        if language == "rust" and node.type in {"macro_invocation", "macro_definition", "macro_rules_definition", "token_tree"}:
            continue
        if node.type in declaration_types[language]:
            name_node = node.child_by_field_name("name")
            if name_node is None and language == "shell":
                name_node = next((child for child in node.named_children if child.type in {"word", "identifier"}), None)
            if name_node is None:
                raise ValueError(f"parser found {node.type} without a name at line {node.start_point.row + 1}")
            name = source[name_node.start_byte:name_node.end_byte].decode("utf-8").strip()
            qualified = f"{scope}::{name}" if scope else name
            body = node.child_by_field_name("body")
            signature_end = body.start_byte if body is not None else node.end_byte
            signature = source[node.start_byte:signature_end].decode("utf-8").strip().rstrip("{").strip()
            if not signature:
                raise ValueError(f"empty parser-derived signature for {qualified} at line {node.start_point.row + 1}")
            start_line = node.start_point.row + 1
            end_line = node.end_point.row + 1
            visibility = "public" if re.search(r"^\\s*pub\\b", signature) else ("private" if language == "shell" else "unknown")
            records.append(symbol_record(repo, rel, language, "function", qualified, start_line, end_line,
                                         signature, visibility, sha, commit, []))
        child_scope = scope
        if language == "rust" and node.type in {"mod_item", "trait_item", "impl_item"}:
            scope_node = node.child_by_field_name("name")
            if scope_node is None:
                scope_node = node.child_by_field_name("type")
            if scope_node is not None:
                scope_name = source[scope_node.start_byte:scope_node.end_byte].decode("utf-8").strip()
                child_scope = f"{scope}::{scope_name}" if scope else scope_name
        for child in reversed(node.named_children):
            stack.append((child, child_scope))
    return records


def explicit_unknown_classification(rel: str) -> tuple[str, str] | None:
    """Allowlist metadata files with no executable symbol semantics, per path."""
    name = Path(rel).name.lower()
    if name == ".gitignore":
        return ("repository_metadata", "Git ignore patterns configure repository tooling and contain no executable declarations")
    if name == "cargo.lock":
        return ("dependency_lockfile", "Cargo.lock is a generated dependency-resolution lockfile, not a source declaration file")
    if name == "license":
        return ("legal_text", "LICENSE contains legal terms and is not an executable source format")
    if name == "codeowners":
        return ("ownership_metadata", "CODEOWNERS maps paths to reviewers and is not an executable source format")
    return None


def scan(root: Path, repo: str) -> dict:
    root, commit = root.resolve(), git_sha(root)
    files, symbols, limitations, exclusions = [], [], [], []

    def record_symlink(rel_path: str, symlink_path: Path) -> None:
        target_bytes = os.fsencode(os.readlink(symlink_path))
        sha = blob_sha(target_bytes)
        reason = "symlink target is inventoried as a Git blob but not followed or parsed"
        files.append({"path": rel_path, "language": "unknown", "classification": "symlink",
                      "blob_sha": sha, "status": "SKIPPED", "reason": reason})
        exclusions.append(exclusion_record(
            repo, rel_path, "SYMLINK_FILE_NOT_FOLLOWED", reason,
            category="safety_boundary", source_blob_sha=sha, evidence_ref=f"{rel_path}#{sha}",
        ))
    # Prune excluded build/cache directories explicitly and record each exclusion.
    # This records policy boundaries without scanning potentially large generated trees.
    for current_dir, dirnames, filenames in os.walk(root, topdown=True, followlinks=False):
        current = Path(current_dir)
        retained_dirs = []
        for dirname in sorted(dirnames):
            candidate = current / dirname
            rel_dir = candidate.relative_to(root).as_posix()
            if dirname in SKIP_DIRS:
                exclusions.append(exclusion_record(
                    repo, rel_dir, "EXCLUDED_DIRECTORY",
                    f"directory name '{dirname}' is excluded by scanner policy",
                    category="scanner_policy", evidence_ref=f"SKIP_DIRS:{dirname}",
                ))
                continue
            if candidate.is_symlink():
                record_symlink(rel_dir, candidate)
                continue
            retained_dirs.append(dirname)
        dirnames[:] = retained_dirs
        for filename in sorted(filenames):
            path = current / filename
            rel = path.relative_to(root).as_posix()
            if path.is_symlink():
                record_symlink(rel, path)
                continue
            if not path.is_file():
                continue
            # Git worktrees represent repository metadata as a root-level .git file.
            # It is not a tracked source file and must not enter the source census.
            if ".git" in Path(rel).parts:
                continue
            try:
                data = path.read_bytes()
            except (OSError, ValueError) as exc:
                limitations.append(f"file read failure: {rel}: {type(exc).__name__}")
                exclusions.append(exclusion_record(
                    repo, rel, "READ_ERROR", type(exc).__name__, category="read_failure",
                    evidence_ref=f"filesystem-read:{rel}",
                ))
                continue
            language, classification, sha = EXTENSIONS.get(path.suffix.lower(), "unknown"), classify(Path(rel)), blob_sha(data)
            entry = {"path": rel, "language": language, "classification": classification, "blob_sha": sha, "status": "ANALYSED"}
            files.append(entry)
            if classification in {"vendored", "generated", "archived"}:
                entry.update(status="SKIPPED", reason=f"classification={classification}; excluded from symbol extraction")
                exclusions.append(exclusion_record(
                    repo, rel, classification.upper(), entry["reason"],
                    category=classification, source_blob_sha=sha, evidence_ref=f"{rel}#{sha}",
                ))
                continue
            try:
                content = data.decode("utf-8")
                if language == "python":
                    symbols.extend(extract_python(repo, rel, content, sha, commit))
                elif language in {"rust", "shell"}:
                    symbols.extend(extract_tree_sitter(repo, rel, language, content, sha, commit))
                elif language in {"json", "markdown", "toml", "yaml"}:
                    validate_document_format(language, content)
                elif language == "unknown":
                    explicit = explicit_unknown_classification(rel)
                    if explicit is None:
                        entry.update(status="UNSUPPORTED", reason="unknown file type has no authoritative parser or path-specific exclusion policy")
                        exclusions.append(exclusion_record(
                            repo, rel, "UNCLASSIFIED_UNKNOWN_FORMAT", entry["reason"],
                            category="unsupported_format", source_blob_sha=sha, evidence_ref=f"{rel}#{sha}",
                        ))
                    else:
                        category, reason = explicit
                        entry.update(status="SKIPPED", classification=category, reason=reason)
                        exclusions.append(exclusion_record(
                            repo, rel, category.upper(), reason, category=category,
                            source_blob_sha=sha, evidence_ref=f"{rel}#{sha}",
                        ))
                else:
                    entry.update(status="UNSUPPORTED", reason=f"no authoritative parser configured for language={language}")
                    exclusions.append(exclusion_record(
                        repo, rel, "UNSUPPORTED_FORMAT", entry["reason"],
                        category="unsupported_format", source_blob_sha=sha, evidence_ref=f"{rel}#{sha}",
                    ))
            except UnicodeDecodeError:
                entry.update(status="UNSUPPORTED", reason="not UTF-8 text; file indexed only")
                exclusions.append(exclusion_record(
                    repo, rel, "NON_UTF8", entry["reason"],
                    category="encoding_failure", source_blob_sha=sha, evidence_ref=f"{rel}#{sha}",
                ))
            except Exception as exc:
                entry.update(status="PARSE_ERROR", reason=f"{type(exc).__name__}: {exc}")
                exclusions.append(exclusion_record(
                    repo, rel, "PARSE_ERROR", str(exc),
                    category="parse_failure", source_blob_sha=sha, evidence_ref=f"{rel}#{sha}",
                ))
                limitations.append(f"{rel}: {exc}")
    # If heuristic parsers collapse distinct declarations onto the same identity, assign
    # deterministic occurrence suffixes in source order. This avoids silent ID collisions.
    by_id = {}
    for symbol in symbols:
        by_id.setdefault(symbol["symbol_id"], []).append(symbol)
    for base_id, group in by_id.items():
        if len(group) > 1:
            for ordinal, symbol in enumerate(sorted(group, key=lambda item: (item["path"], item["source_start_line"], item["qualified_name"])), 1):
                symbol["symbol_id"] = hashlib.sha256(f"{base_id}:occurrence:{ordinal}".encode("utf-8")).hexdigest()
            limitations.append(f"duplicate parser identity disambiguated by source order: {base_id}")
    if commit == "0" * 40:
        limitations.append("repository commit SHA unavailable; output cannot be VERIFIED")
    if exclusions:
        limitations.append("one or more files or directories are explicitly excluded, unsupported, or have parse/read errors; catalog is not complete")

    # Publish parser coverage by language instead of letting a successful scan imply
    # complete extraction. Regex-based languages are explicitly heuristic; formats
    # without an extractor remain visible as unsupported rather than silently omitted.
    symbols_by_path = {}
    for symbol in symbols:
        symbols_by_path.setdefault(symbol["path"], 0)
        symbols_by_path[symbol["path"]] += 1
    parser_coverage = {}
    modes = {
        "json": "stdlib_json",
        "markdown": "commonmark_ast",
        "python": "python_ast",
        "rust": "tree_sitter_rust_cst",
        "shell": "tree_sitter_bash_cst",
        "toml": "stdlib_tomllib",
        "unknown": "explicit_unknown_classifier",
        "yaml": "pyyaml_ast",
    }
    for language in sorted(set(item["language"] for item in files) | set(modes)):
        language_files = [item for item in files if item["language"] == language]
        mode = modes.get(language, "none")
        analysed = sum(item["status"] == "ANALYSED" for item in language_files)
        unsupported = sum(item["status"] == "UNSUPPORTED" for item in language_files)
        parse_errors = sum(item["status"] == "PARSE_ERROR" for item in language_files)
        skipped = sum(item["status"] in {"SKIPPED", "EXCLUDED"} for item in language_files)
        symbol_count = sum(symbols_by_path.get(item["path"], 0) for item in language_files)
        if mode == "none":
            coverage_status = "UNSUPPORTED"
        elif unsupported or parse_errors or analysed + skipped != len(language_files):
            coverage_status = "PARTIAL"
        else:
            # This records parser execution and per-file classification only. It
            # does not by itself establish semantic catalog completeness.
            coverage_status = "AUTHORITATIVE_COMPLETE"
        parser_coverage[language] = {
            "parser_mode": mode,
            "files_seen": len(language_files),
            "files_excluded_by_policy": skipped,
            "files_analysed": analysed,
            "unsupported_files": unsupported,
            "parse_errors": parse_errors,
            "symbols_detected": symbol_count,
            "status": coverage_status,
        }
    return {"schema_version": SCHEMA_VERSION, "repository": repo, "observed_commit_sha": commit,
            "generated_at": datetime.now(timezone.utc).isoformat(), "status": "DETECTED",
            "files": files, "symbols": sorted(symbols, key=lambda s: (s["path"], s["source_start_line"], s["qualified_name"])),
            "parser_coverage": parser_coverage,
            "exclusions": exclusions, "limitations": limitations}

def reconcile(current: dict, baseline: dict) -> dict:
    """Compare symbol identities and source bindings; never upgrades lifecycle status."""
    if current.get("repository") != baseline.get("repository"):
        raise ValueError("baseline repository does not match current repository")
    old = {s["symbol_id"]: s for s in baseline.get("symbols", [])}
    new = {s["symbol_id"]: s for s in current.get("symbols", [])}
    added = sorted(new.keys() - old.keys())
    removed = sorted(old.keys() - new.keys())
    changed = sorted(k for k in old.keys() & new.keys()
                     if old[k].get("source_blob_sha") != new[k].get("source_blob_sha")
                     or old[k].get("signature") != new[k].get("signature"))
    return {"schema_version": "1.0.0", "repository": current["repository"],
            "baseline_commit_sha": baseline.get("observed_commit_sha"),
            "current_commit_sha": current.get("observed_commit_sha"),
            "status": "RESIDUAL" if (added or removed or changed or current.get("exclusions")) else "DETECTED",
            "added_symbol_ids": added, "removed_symbol_ids": removed,
            "changed_symbol_ids": changed, "current_exclusions": current.get("exclusions", []),
            "limitations": ["heuristic parser coverage is incomplete; reconciliation is not proof of complete source coverage"]}

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", default=".", help="repository checkout to inventory")
    parser.add_argument("--repository", default=None, help="canonical owner/repository label")
    parser.add_argument("--output", default="-", help="JSON output path, or - for stdout")
    parser.add_argument("--baseline", default=None, help="prior catalog JSON for symbol reconciliation")
    parser.add_argument("--reconciliation-output", default=None, help="write reconciliation JSON to this path")
    args = parser.parse_args()
    root = Path(args.root)
    if not root.is_dir():
        parser.error(f"not a directory: {root}")
    catalog = scan(root, args.repository or root.name)
    rendered = json.dumps(catalog, ensure_ascii=False, sort_keys=True, indent=2) + "\n"
    if args.output == "-":
        print(rendered, end="")
    else:
        target = Path(args.output); target.parent.mkdir(parents=True, exist_ok=True); target.write_text(rendered, encoding="utf-8")
    if args.baseline:
        baseline = json.loads(Path(args.baseline).read_text(encoding="utf-8"))
        report = reconcile(catalog, baseline)
        report_text = json.dumps(report, ensure_ascii=False, sort_keys=True, indent=2) + "\n"
        if args.reconciliation_output:
            target = Path(args.reconciliation_output); target.parent.mkdir(parents=True, exist_ok=True); target.write_text(report_text, encoding="utf-8")
        else:
            print(report_text, file=__import__("sys").stderr)
    print(f"CATALOG: repository={catalog['repository']} commit={catalog['observed_commit_sha']} files={len(catalog['files'])} symbols={len(catalog['symbols'])} exclusions={len(catalog['exclusions'])}", file=__import__("sys").stderr)
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
