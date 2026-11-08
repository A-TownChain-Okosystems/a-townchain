#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
ATC-STD-TUD-001 Validator — Technology Registry Evidence-Gates.

TUD-1: Registry parsebar, IDs eindeutig
TUD-2: Klassifikation aus {UNIQUE,NOVEL,DIFFERENTIATED,PENDING-EVIDENCE} + Tier {S,A}
TUD-3: PENDING-EVIDENCE erfordert fehlende_evidence; UNIQUE/NOVEL erfordern
       vollstaendige Evidence-Kette (Prior Art + Implementation mind.)
TUD-4: review_date nicht ueberschritten (Fail-Closed -> PENDING-EVIDENCE)
TUD-5: Repo-Referenzen im Org-Scope vorhanden (wenn GITHUB_ACCESS_TOKEN gesetzt)
TUD-6: DIFFERENTIATED erfordert Implementierungs-Evidence oder R-Skelett-Note

Exit: 0 = PASS, 1 = FAIL. Teil von validate_all.py (CI-gebunden).
"""

import datetime
import pathlib
import sys

import yaml

REGISTRY = pathlib.Path(__file__).resolve().parents[2] / "registry" / "technology-registry.yaml"
VALID_CLASSES = {"UNIQUE", "NOVEL", "DIFFERENTIATED", "PENDING-EVIDENCE"}
VALID_TIERS = {"S", "A"}
EVIDENCE_KEYS = [
    "prior_art_analysis",
    "implementation_evidence",
    "benchmark_test_evidence",
    "security_review",
    "patent_ip_assessment",
]


def main():
    fails = []
    today = datetime.date.today().isoformat()
    d = yaml.safe_load(REGISTRY.read_text(encoding="utf-8"))
    entries = [d["sovereign_stack"]] + d["technologies"]
    ids = [e["id"] for e in entries]

    # TUD-1
    if len(ids) != len(set(ids)):
        fails.append("TUD-1: doppelte IDs")

    for e in entries:
        eid = e["id"]
        cls = e.get("classification", "")
        tier = e.get("tier", "")
        # TUD-2
        if cls not in VALID_CLASSES:
            fails.append(f"TUD-2 {eid}: Klassifikation '{cls}' ungueltig")
        if tier not in VALID_TIERS:
            fails.append(f"TUD-2 {eid}: Tier '{tier}' ungueltig")
        # TUD-3
        if cls == "PENDING-EVIDENCE":
            if not e.get("fehlende_evidence"):
                fails.append(f"TUD-3 {eid}: PENDING ohne fehlende_evidence-Liste")
        ev = e.get("evidence", {})
        if cls in ("UNIQUE", "NOVEL"):
            for k in EVIDENCE_KEYS:
                if not ev.get(k) or str(ev.get(k)).upper().startswith("TODO"):
                    fails.append(f"TUD-3 {eid}: UNIQUE/NOVEL benoetigt {k} (TODO/leer)")
        # TUD-4
        rd = e.get("review_date", "")
        if rd and rd < today:
            fails.append(
                f"TUD-4 {eid}: review_date {rd} abgelaufen -> PENDING-EVIDENCE (Fail-Closed)"
            )
        # TUD-6
        if cls == "DIFFERENTIATED":
            impl = str(ev.get("implementation_evidence", ""))
            if (not impl or impl.upper().startswith("TODO")) and "fehlende_evidence" not in e:
                fails.append(f"TUD-6 {eid}: DIFFERENTIATED ohne Implementierungs-Evidence")

    # TUD-5 (optional, nur mit Token)
    import json
    import os
    import urllib.request

    token = os.environ.get("GITHUB_ACCESS_TOKEN")
    if token:
        org = "A-TownChain-Okosystems"
        req = urllib.request.Request(
            f"https://api.github.com/orgs/{org}/repos?per_page=100",
            headers={"Authorization": f"token {token}"},
        )
        org_repos = {r["name"] for r in json.loads(urllib.request.urlopen(req).read())}
        for e in entries:
            repos = e.get("repo", [])
            repos = repos if isinstance(repos, list) else [repos]
            for r in repos:
                if r and r not in org_repos:
                    fails.append(f"TUD-5 {e['id']}: Repo '{r}' nicht im Org-Scope")

    if fails:
        for f in fails:
            print(f"FAIL {f}")
        print(f"RESULT: FAIL ({len(fails)} Findings)")
        sys.exit(1)
    print(f"Technology Registry: {len(entries)} Eintraege — TUD-1..TUD-6 PASS")
    print("RESULT: ALL COMPLIANT")
    sys.exit(0)


if __name__ == "__main__":
    main()
