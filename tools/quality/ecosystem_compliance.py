#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Ecosystem Compliance (Ebene 3) — organisationsweite Pruefung im Monorepo.

Ebene 1 (Code Lint) prueft Syntax und Formatierung einer Datei.
Ebene 2 (Repository Lint) prueft Struktur, Pflichtdateien und Governance eines
Repositories (siehe standards/tools/atc-repo-audit/atc_repo_audit.py).
Ebene 3 prueft hier die Zustaendigkeiten zwischen den Komponenten: gibt es je
Faehigkeit genau eine kanonische Implementierung, sind die Registry-Aussagen
wahr, zeigen Referenzen noch auf existierende Ziele.

Aufruf: python3 tools/quality/ecosystem_compliance.py [--json] [--root PFAD]
Exit:   0 = alle Pruefungen PASS/WARN, 1 = mindestens ein BLOCKED.
Abhaengigkeiten: Python-stdlib plus PyYAML (im Repo bereits vorausgesetzt).
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

VERSION = "1.0.0"
REGISTRY = "standards/registry/repositories.yaml"
OWNERSHIP = ".atc/ownership.yaml"
# Importierte Fremdbestaende: nicht Teil der eigenen Zustaendigkeit (SCR-0127).
SKIP_DIRS = {".git", "docs/os-docs", "node_modules", "target", ".venv", ".org-github-policy"}
TEXT_SUFFIXES = {".md", ".yml", ".yaml", ".toml", ".json", ".py", ".rs", ".ts", ".tsx", ".sh"}


def load_yaml(path: Path):
    import yaml

    with path.open(encoding="utf-8") as handle:
        return yaml.safe_load(handle)


def skipped(rel: Path) -> bool:
    parts = rel.as_posix()
    return any(parts == s or parts.startswith(s + "/") for s in SKIP_DIRS)


def tracked_files(root: Path, suffixes=None):
    """Getrackte Dateien relativ zum Repo-Root (ohne Fremdbestaende)."""
    try:
        out = subprocess.run(
            ["git", "ls-files", "-z"], cwd=root, capture_output=True, check=True
        ).stdout
        names = [Path(p) for p in out.decode("utf-8", "replace").split("\0") if p]
    except (OSError, subprocess.CalledProcessError):
        names = [p.relative_to(root) for p in root.rglob("*") if p.is_file()]
    for rel in names:
        if skipped(rel):
            continue
        if suffixes is None or rel.suffix in suffixes:
            yield rel


ROOTS = ("components", "apps", "tooling", "ecosystem", "ai", "kernel", "modules", "standards")
# Repos, deren Inhalt im Monorepo unter einem anderen Pfad liegt (SCR-0127-Konsolidierung).
ALIASES = {
    "a-townchain": ".",
    "a-townchain-os": ".",
    "a-townchain-ecosystem": "ecosystem",
    "a-townchain-os-docs": "docs",
}


def resolve_repo_path(root: Path, name: str):
    """Findet den kanonischen Pfad eines (ggf. aufgeloesten) Repos im Monorepo."""
    alias = ALIASES.get(name)
    if alias is not None:
        return root / alias
    for candidate in (name, name.removeprefix("atc-")):
        for base in ROOTS:
            for path in (root / base / candidate, root / base / name):
                if path.is_dir():
                    return path
    return None


class Report:
    def __init__(self):
        self.rows = []  # (check, status, message)

    def add(self, check: str, status: str, message: str):
        self.rows.append((check, status, message))

    def blocked(self) -> bool:
        return any(status == "BLOCKED" for _, status, _ in self.rows)


def check_registry(root: Path, rep: Report):
    """E-01: Registry ist lesbar, zaehlt korrekt und ist eindeutig."""
    path = root / REGISTRY
    if not path.is_file():
        rep.add("E-01 Registry-Integritaet", "BLOCKED", f"{REGISTRY} fehlt")
        return []
    data = load_yaml(path) or {}
    repos = data.get("repositories") or []
    names = [r.get("name") for r in repos]
    declared = data.get("count")
    if declared is not None and declared != len(repos):
        rep.add(
            "E-01 Registry-Integritaet",
            "BLOCKED",
            f"count={declared}, tatsaechlich {len(repos)} Eintraege",
        )
    elif len(names) != len(set(names)):
        dupes = sorted({n for n in names if names.count(n) > 1})
        rep.add("E-01 Registry-Integritaet", "BLOCKED", f"doppelte Repo-Namen: {dupes}")
    else:
        rep.add("E-01 Registry-Integritaet", "PASS", f"{len(repos)} Repositories, count stimmt")
    missing_id = [r.get("name") for r in repos if not r.get("id")]
    if missing_id:
        rep.add("E-01 Registry-Integritaet", "WARN", f"ohne ID: {missing_id[:5]}")
    return repos


def check_canonical_locations(root: Path, repos, rep: Report):
    """E-02: Jede migrierte Komponente hat genau eine kanonische Quelle."""
    component_dirs = sorted(p.name for p in (root / "components").iterdir() if p.is_dir())
    by_name = {r.get("name"): r for r in repos}
    unresolved = []
    for name in component_dirs:
        entry = by_name.get(f"atc-{name}") or by_name.get(name)
        if entry is None:
            unresolved.append(name)
    if unresolved:
        rep.add(
            "E-02 Kanonischer Ort",
            "BLOCKED",
            f"components/ ohne Registry-Eintrag: {unresolved}",
        )
    else:
        rep.add(
            "E-02 Kanonischer Ort",
            "PASS",
            f"{len(component_dirs)} Komponenten in components/ sind registriert",
        )
    frozen = [r.get("name") for r in repos if r.get("status") in {"frozen", "archived"}]
    canonical = [r.get("name") for r in repos if r.get("canonical") is True]
    rep.add(
        "E-02 Kanonischer Ort",
        "PASS" if canonical else "WARN",
        f"{len(frozen)} eingefrorene/archivierte Repos, {len(canonical)} kanonische Eintraege",
    )


def check_duplicate_implementations(root: Path, rep: Report):
    """E-03: Dieselbe Faehigkeit darf nicht zweimal gebaut werden."""
    components = {p.name for p in (root / "components").iterdir() if p.is_dir()}
    apps = {p.name.removeprefix("atc-") for p in (root / "apps").iterdir() if p.is_dir()}
    overlap = sorted(components & apps)
    if not overlap:
        rep.add("E-03 Doppel-Implementierung", "PASS", "keine Namensueberschneidung")
        return
    # Ueberschneidung ist zulaessig, wenn genau eine Seite gebaut wird. Wir pruefen,
    # ob die App-Variante im Workspace steht; steht sie nicht drin, ist sie stillgelegt.
    cargo = (root / "Cargo.toml").read_text(encoding="utf-8", errors="replace")
    built = [n for n in overlap if f'"{n}"' in cargo or f"apps/{n}" in cargo]
    if built:
        rep.add(
            "E-03 Doppel-Implementierung",
            "BLOCKED",
            f"parallel gebaut: {built}",
        )
    else:
        rep.add(
            "E-03 Doppel-Implementierung",
            "WARN",
            f"{len(overlap)} Faehigkeiten liegen in components/ und apps/ (apps stillgelegt): "
            f"{overlap[:6]}",
        )


def check_stale_references(root: Path, repos, rep: Report):
    """E-04: Keine Referenz auf ein aufgeloestes Repository in eigenem Bestand."""
    gone = {
        r["name"]
        for r in repos
        if r.get("status") in {"archived"} and r.get("name") not in {"a-townchain", ".github"}
    }
    if not gone:
        rep.add("E-04 Veraltete Referenzen", "PASS", "keine archivierten Ziele")
        return
    pattern = re.compile(
        r"github\.com/A-TownChain-Okosystems/(" + "|".join(sorted(map(re.escape, gone))) + r")\b"
    )
    hits = {}
    for rel in tracked_files(root, TEXT_SUFFIXES):
        try:
            text = (root / rel).read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for match in set(pattern.findall(text)):
            hits.setdefault(match, []).append(rel.as_posix())
    if not hits:
        rep.add("E-04 Veraltete Referenzen", "PASS", f"0 Treffer auf {len(gone)} geloeste Repos")
        return
    sample = {k: v[:2] for k, v in sorted(hits.items())}
    # Referenzen in Dokumentation sind Historie, solange die Datei den kanonischen
    # Ort nennt. Referenzen in ausfuehrbarem Code sind Defekte.
    code_hits = [p for ps in hits.values() for p in ps if Path(p).suffix != ".md"]
    status = "BLOCKED" if code_hits else "WARN"
    rep.add("E-04 Veraltete Referenzen", status, json.dumps(sample, ensure_ascii=False)[:300])


def check_ssot_mapping(root: Path, repos, rep: Report):
    """E-05: Jede Faehigkeit nennt genau eine kanonische Quelle."""
    caps = (load_yaml(root / REGISTRY) or {}).get("capabilities") or {}
    if not caps:
        rep.add("E-05 SSOT-Zuordnung", "WARN", "keine capabilities in der Registry")
        return
    live = {r.get("name") for r in repos if r.get("status") not in {"archived", "frozen"}}
    broken = []
    for cap, spec in caps.items():
        target = (spec or {}).get("canonical_repo")
        if not target:
            broken.append(f"{cap}: kein canonical_repo")
            continue
        if resolve_repo_path(root, target) is None and target not in live:
            broken.append(f"{cap}: {target} existiert nicht")
    if broken:
        rep.add("E-05 SSOT-Zuordnung", "BLOCKED", "; ".join(broken[:6]))
    else:
        rep.add("E-05 SSOT-Zuordnung", "PASS", f"{len(caps)} Faehigkeiten eindeutig zugeordnet")
    own = root / OWNERSHIP
    rep.add(
        "E-05 SSOT-Zuordnung",
        "PASS" if own.is_file() else "BLOCKED",
        f"{OWNERSHIP} {'vorhanden' if own.is_file() else 'fehlt'}",
    )


def check_workspace_contracts(root: Path, rep: Report):
    """E-06: Workspace-Mitglieder existieren, Pflichtmetadaten sind gesetzt."""
    cargo_path = root / "Cargo.toml"
    if not cargo_path.is_file():
        rep.add("E-06 Workspace-Vertrag", "BLOCKED", "Cargo.toml fehlt")
        return
    text = cargo_path.read_text(encoding="utf-8", errors="replace")
    members = re.findall(r'"([^"]+)"', text.split("members", 1)[-1].split("]", 1)[0])
    missing = [m for m in members if not (root / m).is_dir()]
    if missing:
        rep.add("E-06 Workspace-Vertrag", "BLOCKED", f"Mitglieder ohne Verzeichnis: {missing}")
    else:
        rep.add("E-06 Workspace-Vertrag", "PASS", f"{len(members)} Workspace-Mitglieder vorhanden")
    meta = root / ".atc/repository.yaml"
    rep.add(
        "E-06 Workspace-Vertrag",
        "PASS" if meta.is_file() else "BLOCKED",
        f".atc/repository.yaml {'vorhanden' if meta.is_file() else 'fehlt'}",
    )


CHECKS = (
    check_registry,
    check_canonical_locations,
    check_duplicate_implementations,
    check_stale_references,
    check_ssot_mapping,
    check_workspace_contracts,
)


def run(root: Path) -> Report:
    rep = Report()
    repos = check_registry(root, rep)
    for check in CHECKS[1:]:
        if check in {check_canonical_locations, check_stale_references, check_ssot_mapping}:
            check(root, repos, rep)
        else:
            check(root, rep)
    return rep


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description="Ecosystem Compliance (Ebene 3)")
    parser.add_argument("--root", default=".", help="Repository-Wurzel")
    parser.add_argument("--json", action="store_true", help="JSON-Ausgabe")
    args = parser.parse_args(argv)

    root = Path(args.root).resolve()
    rep = run(root)
    if args.json:
        print(json.dumps({"version": VERSION, "checks": rep.rows}, ensure_ascii=False, indent=2))
    else:
        print(f"Ecosystem Compliance v{VERSION} — {root.name}")
        for check, status, message in rep.rows:
            print(f"  [{status:7}] {check}: {message}")
        print("GATE:", "BLOCKED" if rep.blocked() else "PASS")
    return 1 if rep.blocked() else 0


if __name__ == "__main__":
    sys.exit(main())
