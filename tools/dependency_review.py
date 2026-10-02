#!/usr/bin/env python3
"""Standalone dependency graph + PR review gate.

Builds a repository-local dependency inventory from manifests/lockfiles and
GitHub Actions workflow references. For pull requests it compares the base
commit with HEAD and queries OSV for every newly introduced/changed exact
dependency version. The gate fails closed on malformed manifests or OSV
transport errors.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import urllib.error
import urllib.request
from pathlib import Path

import tomllib

ROOT = Path(__file__).resolve().parents[1]
MANIFEST_NAMES = {
    "Cargo.toml",
    "package.json",
    "requirements.txt",
    "requirements-dev.txt",
    "requirements-test.txt",
    "pyproject.toml",
}


def run(*args: str, input: str | None = None) -> str:
    p = subprocess.run(args, cwd=ROOT, text=True, input=input, capture_output=True)
    if p.returncode:
        raise RuntimeError(f"{' '.join(args)}: {p.stderr.strip()}")
    return p.stdout


def files_at(ref: str) -> list[str]:
    return run("git", "ls-tree", "-r", "--name-only", ref).splitlines()


def read_at(ref: str, path: str) -> str | None:
    if ref == "HEAD":
        p = ROOT / path
        return p.read_text(encoding="utf-8") if p.is_file() else None
    try:
        return run("git", "show", f"{ref}:{path}")
    except RuntimeError:
        return None


def add(graph: dict[str, dict], ecosystem: str, name: str, version: str, source: str):
    name, version = name.strip(), version.strip()
    if not name:
        return
    key = f"{ecosystem}:{name}@{version}"
    graph.setdefault(
        key,
        {"ecosystem": ecosystem, "name": name, "version": version, "sources": []},
    )
    graph[key]["sources"].append(source)


def cargo_graph(ref: str, paths: list[str], graph: dict):
    for path in paths:
        if not path.endswith("Cargo.toml"):
            continue
        raw = read_at(ref, path)
        if raw is None:
            continue
        try:
            doc = tomllib.loads(raw)
        except tomllib.TOMLDecodeError as e:
            raise RuntimeError(f"invalid TOML {path}: {e}")
        for section in ("dependencies", "dev-dependencies", "build-dependencies"):
            for name, spec in doc.get(section, {}).items():
                if isinstance(spec, str):
                    version = spec
                elif isinstance(spec, dict):
                    version = str(spec.get("version", "workspace"))
                    if spec.get("path") and "version" not in spec:
                        version = f"path:{spec['path']}"
                else:
                    version = "unresolved"
                add(graph, "cargo", name, version, path)
    # Exact resolved Rust packages from every Cargo.lock.
    for path in paths:
        if not path.endswith("Cargo.lock"):
            continue
        raw = read_at(ref, path)
        if raw is None:
            continue
        try:
            doc = tomllib.loads(raw)
        except tomllib.TOMLDecodeError as e:
            raise RuntimeError(f"invalid TOML {path}: {e}")
        for pkg in doc.get("package", []):
            add(
                graph,
                "cargo-lock",
                pkg.get("name", ""),
                str(pkg.get("version", "")),
                path,
            )


def npm_graph(ref: str, paths: list[str], graph: dict):
    for path in paths:
        if not path.endswith("package.json"):
            continue
        raw = read_at(ref, path)
        if raw is None:
            continue
        try:
            doc = json.loads(raw)
        except json.JSONDecodeError as e:
            raise RuntimeError(f"invalid JSON {path}: {e}")
        for section in (
            "dependencies",
            "devDependencies",
            "peerDependencies",
            "optionalDependencies",
        ):
            for name, version in doc.get(section, {}).items():
                add(graph, "npm", name, str(version), path)
    for path in paths:
        if not path.endswith("package-lock.json"):
            continue
        raw = read_at(ref, path)
        if raw is None:
            continue
        try:
            doc = json.loads(raw)
        except json.JSONDecodeError as e:
            raise RuntimeError(f"invalid JSON {path}: {e}")
        for key, pkg in doc.get("packages", {}).items():
            if not key or not isinstance(pkg, dict) or "version" not in pkg:
                continue
            name = key.rsplit("node_modules/", 1)[-1]
            add(graph, "npm-lock", name, str(pkg["version"]), path)


def python_graph(ref: str, paths: list[str], graph: dict):
    for path in paths:
        base = Path(path).name
        if base.startswith("requirements") and base.endswith(".txt"):
            raw = read_at(ref, path) or ""
            for line in raw.splitlines():
                line = line.strip()
                if not line or line.startswith("#") or line.startswith("-"):
                    continue
                m = re.match(r"([A-Za-z0-9_.-]+)\s*(==|~=|>=|<=|>|<)?\s*([^;\s]+)?", line)
                if m:
                    add(
                        graph,
                        "pypi",
                        m.group(1),
                        (m.group(3) if m.group(2) else "unresolved") or "unresolved",
                        path,
                    )


def action_graph(ref: str, paths: list[str], graph: dict):
    for path in paths:
        if not (path.startswith(".github/workflows/") and path.endswith((".yml", ".yaml"))):
            continue
        raw = read_at(ref, path) or ""
        for owner, repo, version in re.findall(
            r"\buses:\s*([A-Za-z0-9_.-]+)/([A-Za-z0-9_.-]+)@([^\s#]+)", raw
        ):
            add(graph, "github-action", f"{owner}/{repo}", version, path)


def snapshot(ref: str) -> dict:
    paths = files_at(ref)
    graph = {}
    cargo_graph(ref, paths, graph)
    npm_graph(ref, paths, graph)
    python_graph(ref, paths, graph)
    action_graph(ref, paths, graph)
    return {
        "schema": "ATC-DEP-001",
        "ref": ref,
        "dependencies": sorted(
            graph.values(), key=lambda x: (x["ecosystem"], x["name"], x["version"])
        ),
    }


def osv_query(dep: dict) -> list[dict]:
    eco_map = {
        "cargo-lock": "crates.io",
        "cargo": "crates.io",
        "npm-lock": "npm",
        "npm": "npm",
        "pypi": "PyPI",
    }
    ecosystem = eco_map.get(dep["ecosystem"])
    version = dep["version"]
    if (
        not ecosystem
        or version in {"unresolved", "workspace"}
        or version.startswith("path:")
        or any(c in version for c in "^~*<>=| ")
    ):
        return []
    payload = json.dumps(
        {"package": {"name": dep["name"], "ecosystem": ecosystem}, "version": version}
    ).encode()
    req = urllib.request.Request(
        "https://api.osv.dev/v1/query",
        data=payload,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=20) as r:
            return json.loads(r.read().decode()).get("vulns", []) or []
    except (urllib.error.URLError, TimeoutError) as e:
        raise RuntimeError(f"OSV query failed for {ecosystem}:{dep['name']}@{version}: {e}")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--output", default="dependency-graph.json")
    ap.add_argument("--base")
    ap.add_argument("--head", default="HEAD")
    args = ap.parse_args()

    head = snapshot(args.head)
    Path(args.output).write_text(
        json.dumps(head, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    if not args.base:
        print(f"DEPENDENCY GRAPH: PASS ({len(head['dependencies'])} entries)")
        return 0

    base = snapshot(args.base)
    b = {f"{d['ecosystem']}:{d['name']}@{d['version']}": d for d in base["dependencies"]}
    h = {f"{d['ecosystem']}:{d['name']}@{d['version']}": d for d in head["dependencies"]}
    introduced = [h[k] for k in sorted(set(h) - set(b))]
    removed = sorted(set(b) - set(h))
    changed_names = sorted({(d["ecosystem"], d["name"]) for d in introduced})
    findings = []
    for dep in introduced:
        for vuln in osv_query(dep):
            findings.append(
                {
                    "dependency": f"{dep['ecosystem']}:{dep['name']}@{dep['version']}",
                    "id": vuln.get("id"),
                    "summary": vuln.get("summary"),
                    "aliases": vuln.get("aliases", []),
                }
            )
    report = {
        "schema": "ATC-DEP-001-REVIEW",
        "base": args.base,
        "head": args.head,
        "introduced": introduced,
        "removed": removed,
        "changed_dependency_names": [{"ecosystem": e, "name": n} for e, n in changed_names],
        "vulnerabilities": findings,
    }
    Path(args.output).write_text(
        json.dumps({"graph": head, "review": report}, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    print(f"DEPENDENCY GRAPH: PASS ({len(head['dependencies'])} entries)")
    print(
        f"DEPENDENCY REVIEW: {len(introduced)} introduced/changed exact entries; {len(findings)} vulnerabilities"
    )
    if findings:
        for f in findings:
            print(f"VULNERABILITY: {f['id']} {f['dependency']} {f.get('summary') or ''}")
        return 1
    print("DEPENDENCY REVIEW: PASS")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as e:
        print(f"DEPENDENCY REVIEW: BLOCKED: {e}", file=sys.stderr)
        raise SystemExit(2)
