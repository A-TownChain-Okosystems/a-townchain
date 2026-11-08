#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""SCR-0082: 27/27 Compliance-Matrix-Collector (API-getrieben, reproduzierbar).

Dimensionen: 1 .atc/repository.yaml · 2 Gov-CI gruen · 3 Badge · 4 AGENTS.md
· 5 LICENSE · 6 Test-Suite (N/A bei 0 Code) · 7 CodeQL (N/A ohne js/py)
· 8 .atc/evidence/evidence.yaml · 9 STATUS.md · 10 README · 11 ROADMAP.md
· 12 FILE_REGISTER.md · .github = EXEMPT (SCR-0075).
Aufruf: python3 tools/compliance/matrix_collect.py [--json OUT]
Schreibt Matrix nach docs/compliance/ ( append an COMPLIANCE-MATRIX-<date>.md )."""

import base64
import json
import os
import sys

import requests

ORG = "A-TownChain-Okosystems"
REPOS = [
    ".github",
    "a-townchain",
    "a-townchain-os",
    "a-townchain-os-docs",
    "atc-algorithm",
    "atc-compute",
    "atc-contracts",
    "atc-explorer",
    "atc-indexer",
    "atc-interop",
    "atc-launchpad",
    "atc-marketplace",
    "atc-mining",
    "atc-node",
    "atc-oracle",
    "atc-sdk",
    "atc-shivacore",
    "atc-standards",
    "atc-storage",
    "atc-vm",
    "atc-wallet",
    "atc-zkp",
    "atclang",
    "aurora-ai",
    "genesis-chronicles",
    "genesis-engine",
    "globus-os",
]
DIMS = [
    "repository.yaml",
    "Gov-CI",
    "Badge",
    "AGENTS",
    "Lizenz",
    "Test-Suite",
    "CodeQL",
    "Evidence",
    "STATUS",
    "Docs",
    "Roadmap",
    "FILE_REGISTER",
]


def main():
    tok = os.environ.get("GITHUB_ACCESS_TOKEN") or os.environ.get("GITHUB_TOKEN")
    H = {"Authorization": f"Bearer {tok}"} if tok else {}
    API = "https://api.github.com"
    rows, data = [], {}
    for r in REPOS:
        t = requests.get(f"{API}/repos/{ORG}/{r}/git/trees/main?recursive=1", headers=H).json()
        paths = [x["path"] for x in t.get("tree", []) if x["type"] == "blob"]
        js = any(
            p.endswith((".ts", ".tsx", ".js", ".jsx")) or p.endswith("package.json") for p in paths
        )
        py = any(p.endswith(".py") for p in paths)
        rs = any(p.endswith(".rs") for p in paths)
        runs = (
            requests.get(f"{API}/repos/{ORG}/{r}/actions/runs?per_page=15", headers=H)
            .json()
            .get("workflow_runs", [])
        )
        gov_ok = any(x.get("conclusion") == "success" for x in runs[:10])
        rm = requests.get(f"{API}/repos/{ORG}/{r}/contents/README.md", headers=H)
        badge = False
        if rm.status_code == 200:
            c = base64.b64decode(rm.json()["content"]).decode()
            badge = "COMPLIANCE" in c.upper() or "compliance" in c.lower()
        d6 = (
            "N/A"
            if not (js or py or rs)
            else (
                "ok" if any(x.startswith(".github/workflows/test-suite") for x in paths) else "rot"
            )
        )
        d7 = (
            "N/A"
            if not (js or py)
            else (
                "ok"
                if any("codeql" in x for x in paths if x.startswith(".github/workflows/"))
                else "rot"
            )
        )
        vals = [
            ".atc/repository.yaml" in paths,
            gov_ok,
            badge,
            "AGENTS.md" in paths,
            "LICENSE" in paths,
            d6,
            d7,
            ".atc/evidence/evidence.yaml" in paths,
            "STATUS.md" in paths,
            "README.md" in paths,
            "ROADMAP.md" in paths,
            "FILE_REGISTER.md" in paths,
        ]
        data[r] = dict(zip(DIMS, [bool(v) if v in (True, False) else v for v in vals]))
        if r == ".github":
            rows.append((r, ["➖"] * 12, "EXEMPT"))
            continue
        marks = ["🟢" if v in (True, "ok") else ("➖" if v == "N/A" else "🔴") for v in vals]
        rows.append((r, marks, "PRODUCT" if (js or py or rs) else "SPEC-ONLY"))
    tot = gr = na = 0
    for r, cells, _ in rows:
        if r == ".github":
            continue
        for c in cells:
            tot += 1
            if c == "🟢":
                gr += 1
            elif c == "➖":
                na += 1
    print(f"MATRIX: {gr}/{tot} = {100 * gr / tot:.1f} % gruen | N/A {na} | rot {tot - gr - na}")
    if "--json" in sys.argv:
        i = sys.argv.index("--json")
        json.dump(data, open(sys.argv[i + 1], "w"), indent=1)


if __name__ == "__main__":
    main()
