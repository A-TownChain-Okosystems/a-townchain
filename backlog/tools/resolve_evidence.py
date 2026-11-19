#!/usr/bin/env python3
"""Loest die Kurz-SHAs aus dem Blocker-Register zu vollstaendigen Commit-SHAs auf.

Das Register speichert evidence_head als 9-stelligen Praefix. Ein Praefix ist
kein Nachweis. Dieses Skript fragt die GitHub-API je Kandidaten-Repository ab
und schreibt backlog/evidence.yaml mit dem vollstaendigen SHA, dem Repository
und dem Commit-Datum. Nicht aufloesbare Eintraege bleiben ausdruecklich offen.
"""

import json
import os
import sys
import urllib.error
import urllib.request
from datetime import datetime, timezone

import yaml

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ORG = "A-TownChain-Okosystems"

# Reihenfolge der Kandidaten: erst das SSOT-Repository des Tasks, dann die Haeuser,
# in denen das Register Evidence belegt.
FALLBACK = [
    "a-townchain",
    "atc-standards",
    "a-townchain-ecosystem",
    ".github",
    "globus-os",
    "atclang",
    "atc-shivacore",
    "aurora-ai",
    "genesis-engine",
    "atc-vm",
    "atc-algorithm",
    "atc-contracts",
    "atc-zkp",
]


def api(url):
    tok = os.environ.get("GITHUB_ACCESS_TOKEN")
    if not tok:
        sys.exit("GITHUB_ACCESS_TOKEN fehlt (Wert wird nicht ausgegeben)")
    req = urllib.request.Request(
        url,
        headers={
            "Authorization": f"Bearer {tok}",
            "Accept": "application/vnd.github+json",
            "User-Agent": "atc-backlog",
        },
    )
    try:
        with urllib.request.urlopen(req, timeout=30) as f:
            return f.status, json.load(f)
    except urllib.error.HTTPError as e:
        return e.code, None


def candidate_repos(task):
    out = []
    ssot = (task.get("ssot") or "").split()[0].strip()
    if ssot in FALLBACK:
        out.append(ssot)
    for r in FALLBACK:
        if r not in out:
            out.append(r)
    return out


def main():
    doc = yaml.safe_load(open(os.path.join(ROOT, "MASTER_BACKLOG.yaml"), encoding="utf-8"))
    results = {}
    resolved = unresolved = 0

    for t in doc["tasks"]:
        head = t.get("evidence_head")
        if not head:
            results[t["id"]] = {"resolved": False, "reason": "kein evidence_head im Register"}
            unresolved += 1
            continue
        head = str(head)
        if len(head) == 40:
            results[t["id"]] = {"resolved": True, "sha": head, "reason": "bereits vollstaendig"}
            resolved += 1
            continue
        found = None
        for repo in candidate_repos(t):
            st, c = api(f"https://api.github.com/repos/{ORG}/{repo}/commits/{head}")
            if st == 200 and c:
                found = {
                    "resolved": True,
                    "repo": repo,
                    "sha": c["sha"],
                    "committed": c["commit"]["committer"]["date"],
                    "resolved_from": head,
                }
                break
        if found:
            results[t["id"]] = found
            resolved += 1
        else:
            results[t["id"]] = {
                "resolved": False,
                "resolved_from": head,
                "reason": "Praefix in keinem Kandidaten-Repository gefunden",
            }
            unresolved += 1

    epoch = os.environ.get("SOURCE_DATE_EPOCH")
    if epoch and epoch.isdigit():
        now = datetime.fromtimestamp(int(epoch), timezone.utc).isoformat(timespec="seconds")
    else:
        now = datetime.now(timezone.utc).isoformat(timespec="seconds")

    out = {
        "generated_at": now,
        "organisation": ORG,
        "resolved": resolved,
        "unresolved": unresolved,
        "tasks": results,
    }
    with open(os.path.join(ROOT, "evidence.yaml"), "w", encoding="utf-8") as f:
        f.write("# GENERIERT von backlog/tools/resolve_evidence.py — nicht manuell bearbeiten.\n")
        yaml.safe_dump(out, f, allow_unicode=True, sort_keys=False, width=100)
    print(f"evidence.yaml: {resolved} aufgeloest, {unresolved} offen")


if __name__ == "__main__":
    main()
