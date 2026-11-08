#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Standard-Version-Gate — P0 Governance-Version-Enforcement (Registry = SSOT).

Prueft die Versionserklaerungen aller Repositories gegen die SSOT-Registry
(registry/standards.yaml): Registry-Version = required. Abweichung = Drift.

Modi:
  --github    READMEs aller Repos (registry/repositories.yaml) via GitHub-API
              laden (GITHUB_TOKEN oder anonym)
  --local DIR Repos aus lokalem Workspace-Mirror lesen (<DIR>/<repo>/README.md)

Bewertung je Fund (pro Repo+Standard dedupliziert):
  repo_version == registry_version  -> PASS
  repo_version != registry_version  -> FAIL (MIGRATION_REQUIRED)
  ID nicht in Registry             -> WARN (Kein Eintrag = kein Standard)

Exit 0 = OK, 1 = FAIL."""

import argparse
import base64
import json
import os
import re
import sys
import urllib.request

try:
    import yaml
except ImportError:
    sys.exit("PyYAML erforderlich: pip install pyyaml")

SEMV_RE = re.compile(r"ATC-STD-([A-Z0-9][A-Z0-9\-]*[A-Z0-9])\s*\|?\s*v?(\d+\.\d+\.\d+)")


def load_registry(path):
    with open(path, encoding="utf-8") as f:
        data = yaml.safe_load(f)
    return {s["id"]: str(s.get("version", "")) for s in data.get("standards", [])}


def repo_list(path):
    with open(path, encoding="utf-8") as f:
        data = yaml.safe_load(f)
    return [r["name"] for r in data.get("repositories", []) if r.get("name")]


def fetch_readme_github(org, repo, token=None):
    req = urllib.request.Request(
        f"https://api.github.com/repos/{org}/{repo}/readme",
        headers={"Accept": "application/vnd.github+json"},
    )
    if token:
        req.add_header("Authorization", f"Bearer {token}")
    with urllib.request.urlopen(req) as r:
        return base64.b64decode(json.load(r)["content"]).decode("utf-8")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--registry", default="registry/standards.yaml")
    ap.add_argument("--repos", default="registry/repositories.yaml")
    ap.add_argument("--org", default="A-TownChain-Okosystems")
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--github", action="store_true")
    g.add_argument("--local", metavar="DIR")
    ap.add_argument("--only", help="nur diese Repos pruefen (Komma-Liste)")
    args = ap.parse_args()

    reg = load_registry(args.registry)
    repos = repo_list(args.repos)
    if args.only:
        wanted = {r.strip() for r in args.only.split(",")}
        repos = [r for r in repos if r in wanted]
    token = os.environ.get("GITHUB_TOKEN")

    fails, warns, checked = [], [], 0
    for repo in repos:
        try:
            if args.github:
                readme = fetch_readme_github(args.org, repo, token)
            else:
                with open(os.path.join(args.local, repo, "README.md"), encoding="utf-8") as f:
                    readme = f.read()
        except Exception as e:
            warns.append(f"{repo}: README nicht lesbar ({e})")
            continue
        seen = set()
        for match in SEMV_RE.finditer(readme):
            sid, ver = f"ATC-STD-{match.group(1)}", match.group(2)
            if (sid, ver) in seen:
                continue
            seen.add((sid, ver))
            if sid not in reg:
                warns.append(f"{repo}: {sid} v{ver} deklariert, aber NICHT in Registry")
                continue
            checked += 1
            if ver != reg[sid]:
                fails.append(f"{repo}: {sid} v{ver} != Registry v{reg[sid]} -> MIGRATION_REQUIRED")

    print(
        f"Standard-Version-Gate: {checked} Deklarationen geprueft, "
        f"{len(fails)} FAIL, {len(warns)} WARN"
    )
    for w in warns:
        print(f"  WARN  {w}")
    for f_ in fails:
        print(f"  FAIL  {f_}")
    if fails:
        print("Ergebnis: FAIL — Registry-Drift vorhanden (MIGRATION_REQUIRED).")
    else:
        print("Ergebnis: PASS — alle Versionserklaerungen konsistent mit Registry.")
    sys.exit(1 if fails else 0)


if __name__ == "__main__":
    main()
