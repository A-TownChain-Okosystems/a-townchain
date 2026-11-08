#!/usr/bin/env python3
"""Konsolidierung aller Organisations-Repos in das Monorepo a-townchain.

Ziel-Layout (Phase 1 — physische Zusammenfuehrung mit Provenienz):

  components/   bestehende Bibliotheken (bleiben)
  apps/         ausfuehrbare Produkte (node, ide, launchpad, marketplace, ...)
  kernel/       KAI-OS, ShivaCore, GlobusOS
  ai/           aurora-ai, atclang, genesis-*
  standards/    atc-standards
  tooling/      atc-toolchain, atc-engineering
  docs/         os-docs, demo
  ecosystem/    catalog, control-plane, schemas, integration (ohne Spiegel)

Das Skript kopiert Dateiinhalte (kein Git-Verlauf), schreibt fuer jedes Repo
Herkunft und Commit-SHA in MIGRATION_MANIFEST.yaml und ueberspringt
Build-Artefakte. Es ist idempotent: mehrfaches Ausfuehren ueberschreibt
denselben Zielzustand.

Aufruf:
    python3 tools/consolidate_monorepo.py --sources ../.. --target . [--dry-run]
"""

import argparse
import os
import shutil
import subprocess
import sys

EXCLUDE_DIRS = {".git", "target", "node_modules", "__pycache__", ".venv", "dist", "build"}
EXCLUDE_FILES = {".DS_Store"}

LAYOUT = {
    # apps/ — ausfuehrbare Produkte
    "atc-node": "apps/atc-node",
    "atc-ide": "apps/atc-ide",
    "atc-launchpad": "apps/atc-launchpad",
    "atc-marketplace": "apps/atc-marketplace",
    "atc-explorer": "apps/atc-explorer",
    "atc-indexer": "apps/atc-indexer",
    "atc-compute": "apps/atc-compute",
    "atc-storage": "apps/atc-storage",
    "atc-sdk": "apps/atc-sdk",
    "atc-interop": "apps/atc-interop",
    "atc-contracts": "apps/atc-contracts",
    "atc-mining": "apps/atc-mining",
    # kernel/ — Betriebssystem- und Kernel-Ebene
    "a-townchain-os": "kernel/kai-os",
    "atc-shivacore": "kernel/atc-shivacore",
    "globus-os": "kernel/globus-os",
    # ai/
    "aurora-ai": "ai/aurora-ai",
    "atclang": "ai/atclang",
    "genesis-engine": "ai/genesis-engine",
    "genesis-chronicles": "ai/genesis-chronicles",
    "genesis-franchise-factory": "ai/genesis-franchise-factory",
    # standards/ und tooling/
    "atc-standards": "standards",
    "atc-toolchain": "tooling/atc-toolchain",
    "atc-engineering": "tooling/atc-engineering",
    # docs/
    "a-townchain-os-docs": "docs/os-docs",
    "demo-repository": "docs/demo",
}

# Repos, die bereits als components/* im Ziel liegen oder sich selbst sind.
SKIP = {"a-townchain"}

# ecosystem: nur die produktiven Teile, nicht die 26 Spiegel unter components/
ECOSYSTEM_KEEP = {"catalog", "control-plane", "schemas", "integration", "tools", "docs", "tests"}


def head_sha(repo_dir):
    try:
        return subprocess.check_output(
            ["git", "-C", repo_dir, "rev-parse", "HEAD"], text=True
        ).strip()
    except Exception:
        return "unknown"


def copy_tree(src, dst, dry_run, counter):
    for root, dirs, files in os.walk(src):
        dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS]
        rel = os.path.relpath(root, src)
        target_dir = os.path.join(dst, rel) if rel != "." else dst
        if not dry_run:
            os.makedirs(target_dir, exist_ok=True)
        for name in files:
            if name in EXCLUDE_FILES:
                continue
            s = os.path.join(root, name)
            t = os.path.join(target_dir, name)
            counter[0] += 1
            if not dry_run:
                shutil.copy2(s, t)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--sources", required=True, help="Verzeichnis mit den geklonten Repos")
    ap.add_argument("--target", required=True, help="Ziel-Monorepo (Arbeitskopie)")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    src_root = os.path.abspath(args.sources)
    dst_root = os.path.abspath(args.target)
    manifest = []
    counter = [0]
    missing = []

    for repo, rel_target in sorted(LAYOUT.items()):
        src = os.path.join(src_root, repo)
        if not os.path.isdir(src):
            missing.append(repo)
            continue
        dst = os.path.join(dst_root, rel_target)
        before = counter[0]
        copy_tree(src, dst, args.dry_run, counter)
        manifest.append(
            {
                "source_repo": f"A-TownChain-Okosystems/{repo}",
                "source_commit": head_sha(src),
                "target_path": rel_target,
                "files": counter[0] - before,
            }
        )

    # ecosystem ohne Spiegel
    eco = os.path.join(src_root, "a-townchain-ecosystem")
    if os.path.isdir(eco):
        before = counter[0]
        for name in sorted(ECOSYSTEM_KEEP):
            src = os.path.join(eco, name)
            if os.path.isdir(src):
                copy_tree(src, os.path.join(dst_root, "ecosystem", name), args.dry_run, counter)
        manifest.append(
            {
                "source_repo": "A-TownChain-Okosystems/a-townchain-ecosystem",
                "source_commit": head_sha(eco),
                "target_path": "ecosystem/ (ohne components/-Spiegel)",
                "files": counter[0] - before,
            }
        )

    print(f"{'DRY-RUN' if args.dry_run else 'KOPIERT'}: {counter[0]} Dateien aus "
          f"{len(manifest)} Repos -> {dst_root}")
    for m in manifest:
        print(f"  {m['source_repo']:45s} -> {m['target_path']:32s} {m['files']:5d} Dateien")
    if missing:
        print("  FEHLEND (nicht geklont):", ", ".join(missing))

    if not args.dry_run:
        path = os.path.join(dst_root, "MIGRATION_MANIFEST_MONOREPO.json")
        import json

        with open(path, "w") as fh:
            json.dump({"consolidated_at": "2026-11-08", "entries": manifest}, fh, indent=2)
        print("  Manifest:", path)
    return 0


if __name__ == "__main__":
    sys.exit(main())
