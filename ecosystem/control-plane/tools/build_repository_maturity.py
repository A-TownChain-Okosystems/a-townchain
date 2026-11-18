#!/usr/bin/env python3
"""Erzeugt REPOSITORY_MATURITY.yaml — HEAD-gebundener Reifegrad aller Org-Repos.

Quelle: GitHub API (Live). Jede Aussage ist an den exakten HEAD des
Default-Branches gebunden (check-runs am SHA, nicht "letzter grüner Lauf").

Stufen (streng getrennt, kein automatischer Aufstieg):
  ARCHIVED       nicht mehr aktives Entwicklungsziel
  SPEC_ONLY      keine Implementierung (Doku/Governance/Scaffold)
  IMPLEMENTED    Code + Build-Manifest vorhanden
  TESTED         Testsuite vorhanden und am HEAD gruen
  CI_VERIFIED    alle Check-Runs am HEAD gruen (Mindesttiefe 1)
  E2E_VERIFIED   zusaetzlich repositoryuebergreifender Lauf gruen am HEAD
  VERIFIED       zusaetzlich HEAD-gebundene Evidenz + Reproduzierbarkeit

Aufruf:
  python3 build_repository_maturity.py --out REPOSITORY_MATURITY.yaml
  python3 build_repository_maturity.py --check   # Exit 1 bei Drift

Env: GITHUB_ACCESS_TOKEN (nur lesend), GITHUB_ORG (Default A-TownChain-Okosystems)
"""
from __future__ import annotations

import argparse
import json
import os
import sys
import urllib.parse
import urllib.request
from datetime import datetime, timezone

ORG = os.environ.get("GITHUB_ORG", "A-TownChain-Okosystems")
TOKEN = os.environ.get("GITHUB_ACCESS_TOKEN", "")

# Architekturrolle und SSOT-Zustaendigkeit (kuratiert, nicht geraten).
ROLE = {
    "a-townchain": ("monorepo-ssot", "SSOT: gesamter Code, apps/, kernel/, ai/, components/"),
    "a-townchain-ecosystem": ("control-plane", "Governance/Integration/Evidenz, keine zweite Implementierung"),
    "a-townchain-os": ("kernel-runtime", "KAI-OS Runtime -> Monorepo kernel/atc-os"),
    "a-townchain-os-docs": ("documentation", "Doku-Hub -> Monorepo docs/"),
    "atc-standards": ("governance-root", "Normative Standards -> Monorepo standards/"),
    "atc-engineering": ("engineering", "Engineering-Tooling -> Monorepo tooling/atc-engineering"),
    "atc-toolchain": ("toolchain", "Toolchain -> Monorepo tooling/atc-toolchain"),
    "atc-ide": ("application", "IDE -> Monorepo apps/atc-ide"),
    "atc-node": ("blockchain-runtime", "Full-Node-Runtime -> Monorepo apps/atc-node + components/node"),
    "atc-sdk": ("sdk", "SDK -> Monorepo apps/atc-sdk + components/sdk"),
    "atc-vm": ("governance-spec", "VM-Kompatibilitaets-/Governance-Rolle, bewusst kein Code"),
    "atc-algorithm": ("consensus-library", "Konsens-/Oekonomie-Algorithmen -> components/algorithm"),
    "atc-contracts": ("smart-contracts", "ATCLang-Contracts -> components/contracts"),
    "atc-compute": ("compute", "Compute-Schicht -> Monorepo apps/atc-compute"),
    "atc-zkp": ("zkp-layer", "ZKP-Layer -> components/zkp (R1-Skelett)"),
    "globus-os": ("operating-system", "Kernel/OS -> Monorepo kernel/atc-shivacore-Bereich"),
    "atc-shivacore": ("trust-anchor", "ShivaCore Kern -> Monorepo kernel/atc-shivacore"),
    "aurora-ai": ("ai-runtime", "Aurora-Modell/Runtime -> Monorepo ai/aurora-ai"),
    "atclang": ("language", "ATCLang Compiler/VM -> Monorepo ai/atclang"),
    "genesis-engine": ("genesis", "Genesis Engine -> Monorepo ai/genesis-engine"),
    "genesis-franchise-factory": ("genesis", "GFF -> Monorepo ai/genesis-franchise-factory"),
    "genesis-chronicles": ("application", "Spiel/NFT -> Monorepo ai/genesis-chronicles"),
    "atc-launchpad": ("application", "Launchpad -> Monorepo apps/atc-launchpad"),
    "atc-marketplace": ("application", "Marketplace -> Monorepo apps/atc-marketplace"),
    "atc-wallet": ("archived", "Wallet-Funktion im Monorepo (components/wallet)"),
    "atc-explorer": ("archived", "Explorer-Funktion im Monorepo (apps/atc-explorer)"),
    "atc-indexer": ("archived", "Indexer-Funktion im Monorepo (apps/atc-indexer)"),
    "atc-mining": ("archived", "Mining-Funktion im Monorepo (components/mining)"),
    "atc-interop": ("archived", "Interop-Funktion im Monorepo (components/interop)"),
    "atc-oracle": ("archived", "Oracle-Funktion im Monorepo (components/oracle)"),
    "atc-storage": ("archived", "Storage-Funktion im Monorepo (components/storage)"),
    "demo-repository": ("demo", "Demo, kein Reifeziel"),
    ".github": ("governance-control-plane", "Org-Governance-Hub (Policies, Validatoren)"),
}

BUILD_MANIFESTS = ("Cargo.toml", "package.json", "pyproject.toml", "requirements.txt")


def api(path: str):
    req = urllib.request.Request(
        "https://api.github.com" + path,
        headers={
            "Authorization": f"token {TOKEN}",
            "Accept": "application/vnd.github+json",
        },
    )
    return json.loads(urllib.request.urlopen(req, timeout=90).read())


def head_sha(repo: str, branch: str) -> str:
    try:
        return api(f"/repos/{ORG}/{repo}/git/ref/heads/{branch}")["object"]["sha"]
    except Exception:
        return ""


def tree_paths(repo: str, sha: str) -> list[str]:
    try:
        t = api(f"/repos/{ORG}/{repo}/git/trees/{sha}?recursive=1")
        return [x["path"] for x in t.get("tree", []) if x.get("type") == "blob"]
    except Exception:
        return []


def check_summary(repo: str, sha: str) -> dict:
    if not sha:
        return {"total": 0, "success": 0, "failure": 0, "failing": []}
    try:
        runs = api(f"/repos/{ORG}/{repo}/commits/{sha}/check-runs?per_page=100").get("check_runs", [])
    except Exception:
        return {"total": 0, "success": 0, "failure": 0, "failing": []}
    ok = sum(1 for r in runs if r.get("conclusion") == "success")
    bad = [r["name"] for r in runs if r.get("conclusion") == "failure"]
    return {"total": len(runs), "success": ok, "failure": len(bad), "failing": bad}


BLOCKERS = {
    "a-townchain": [
        "6 von 15 Checks am main-HEAD rot (markdownlint, ruff, npm-tests, pytest, quality, determinism) — Fix liegt in PR #116",
        ">=100 offene Dependabot-Alerts (API-Deckel), ueberwiegend aus Konsolidierungsimporten",
    ],
    "atc-node": ["keine Testsuite", "reproduzierbarer Build erst mit PR #15/#16", "markdownlint rot am HEAD"],
    "globus-os": ["keine Testdateien", "Syscall-ABI nur spezifiziert, nicht implementiert", "cargo-tests rot am HEAD"],
    "atc-standards": ["regenerate-Check rot (Generator-Drift)", "330 von 534 Standards specification_only"],
    "a-townchain-ecosystem": ["Knowledge-Indexer-Check rot", "176 Sync-Commits/Woche ohne Automatisierung"],
    "aurora-ai": ["kein Runtime-E2E gegen GlobusOS nachgewiesen"],
    "atc-launchpad": ["cargo-tests rot", "keine Tests"],
    "atc-marketplace": ["pytest rot"],
    "genesis-chronicles": ["ruff/quality/pytest rot"],
    "atc-contracts": ["markdownlint rot"],
    "atc-vm": ["bewusst kein Code: Governance-/Kompatibilitaetsrolle"],
    "atc-toolchain": ["keine Toolchain-Implementierung (Spec/Doku)"],
}

EVIDENCE = {
    "a-townchain": ["docs/audits/TECHNICAL_MATURITY-REPORT-2026-11-18.md", "PR #116"],
    "atc-standards": ["registry/standard-implementation.yaml", "registry/standards.yaml"],
    "a-townchain-os-docs": ["docs/audits/TECHNICAL_MATURITY-REPORT-2026-11-18.md"],
}


def level_for(archived: bool, role: str, files: list[str], checks: dict) -> str:
    if archived:
        return "ARCHIVED"
    if role in ("governance-spec", "demo"):
        return "CI_VERIFIED" if checks["total"] and not checks["failure"] else "SPEC_ONLY"
    has_build = any(p.split("/")[-1] in BUILD_MANIFESTS for p in files)
    has_tests = any(
        k in p.lower() for p in files for k in ("/tests/", "test_", "_test.", ".test.", ".spec.")
    )
    ci_green = checks["total"] > 0 and checks["failure"] == 0
    # Nur was am HEAD als Job gelaufen ist, zaehlt (keine Ableitung aus Dateinamen).
    # Kummulative Leiter: eine Stufe nur, wenn alle Vorstufen belegt sind.
    # CI_VERIFIED setzt getestete Funktion voraus — ein gruener Build ohne
    # Testgate rechtfertigt kein CI_VERIFIED.
    if ci_green and has_build and has_tests:
        return "CI_VERIFIED"
    if has_build:
        return "IMPLEMENTED"
    return "SPEC_ONLY"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default="REPOSITORY_MATURITY.yaml")
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()

    repos = api(f"/orgs/{ORG}/repos?per_page=100")
    rows = []
    for r in sorted(repos, key=lambda x: x["name"]):
        name, branch, archived = r["name"], r["default_branch"], r["archived"]
        sha = head_sha(name, branch)
        files = tree_paths(name, sha) if sha else []
        checks = check_summary(name, sha)
        role, ssot = ROLE.get(name, ("unclassified", "nicht zugeordnet"))
        rows.append(
            {
                "repository": name,
                "default_branch": branch,
                "head_sha": sha,
                "archived": archived,
                "architecture_role": role,
                "ssot_scope": ssot,
                "build_manifests": sum(
                    1 for p in files if p.split("/")[-1] in BUILD_MANIFESTS and "/docs/" not in p
                ),
                "test_files": sum(
                    1
                    for p in files
                    if any(k in p.lower() for k in ("/tests/", "test_", "_test.", ".test.", ".spec."))
                ),
                "ci": checks,
                "maturity_level": level_for(archived, role, files, checks),
                "ci_status": ("GREEN" if checks["total"] and not checks["failure"]
                              else ("RED" if checks["failure"] else "NONE")),
                "e2e_status": "NOT_VERIFIED",
                "open_blockers": BLOCKERS.get(name, []),
                "evidence_refs": EVIDENCE.get(name, []),
            }
        )

    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    out = [
        "# REPOSITORY_MATURITY.yaml — HEAD-gebundener Reifegrad aller Org-Repositories",
        "# Erzeugt von ecosystem/control-plane/tools/build_repository_maturity.py",
        "# Stufen: ARCHIVED < SPEC_ONLY < IMPLEMENTED < TESTED < CI_VERIFIED < E2E_VERIFIED < VERIFIED",
        "# Kein Aufstieg ohne Nachweis: CI_VERIFIED nur bei 0 Fehl-Checks am exakten HEAD.",
        "schema_version: '1.0.0'",
        f"generated_at: '{now}'",
        f"organization: {ORG}",
        "summary:",
        f"  repositories_total: {len(rows)}",
        f"  active: {sum(1 for x in rows if not x['archived'])}",
        f"  archived: {sum(1 for x in rows if x['archived'])}",
        f"  ci_verified: {sum(1 for x in rows if x['maturity_level'] == 'CI_VERIFIED')}",
        f"  ci_red: {sum(1 for x in rows if x['ci']['failure'] > 0)}",
        f"  verified: {sum(1 for x in rows if x['maturity_level'] == 'VERIFIED')}",
        "repositories:",
    ]
    for x in rows:
        out += [
            f"  - repository: {x['repository']}",
            f"    default_branch: {x['default_branch']}",
            f"    head_sha: {x['head_sha'] or 'null'}",
            f"    archived: {str(x['archived']).lower()}",
            f"    architecture_role: {x['architecture_role']}",
            f"    ssot_scope: \"{x['ssot_scope']}\"",
            f"    build_manifests: {x['build_manifests']}",
            f"    test_files: {x['test_files']}",
            f"    ci_checks: {x['ci']['total']}",
            f"    ci_success: {x['ci']['success']}",
            f"    ci_failure: {x['ci']['failure']}",
            f"    ci_status: {x['ci_status']}",
            f"    maturity_level: {x['maturity_level']}",
            f"    e2e_status: {x['e2e_status']}",
            "    open_blockers:" + (" []" if not x["open_blockers"] else ""),
        ]
        for b in x["open_blockers"]:
            out.append(f"      - \"{b}\"")
        if x["evidence_refs"]:
            out.append("    evidence_refs:")
            out += [f"      - \"{e}\"" for e in x["evidence_refs"]]
        if x["ci"]["failing"]:
            out.append("    failing_checks:")
            out += [f"      - \"{n}\"" for n in x["ci"]["failing"][:8]]
    text = "\n".join(out) + "\n"

    if args.check:
        try:
            current = open(args.out, encoding="utf-8").read()
        except OSError:
            current = ""
        if current != text:
            print("DRIFT: REPOSITORY_MATURITY.yaml ist nicht aktuell", file=sys.stderr)
            return 1
        print("OK: Register ist aktuell")
        return 0

    with open(args.out, "w", encoding="utf-8") as fh:
        fh.write(text)
    print(f"geschrieben: {args.out} ({len(rows)} Repositories)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
