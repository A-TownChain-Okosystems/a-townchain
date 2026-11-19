#!/usr/bin/env python3
"""Live-Erhebung des Organisationsbestands fuer das Master-Backlog.

Schreibt backlog/verification.yaml. Kein Token in Ausgaben: der Wert wird nur
als Umgebungsvariable gelesen und nie geloggt.
"""

import json
import os
import sys
import urllib.error
import urllib.parse
import urllib.request
from datetime import datetime, timezone

ORG = "A-TownChain-Okosystems"
OUT = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "verification.yaml")


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


def count(q):
    _, d = api("https://api.github.com/search/issues?per_page=1&q=" + urllib.parse.quote(q))
    return d["total_count"] if d else None


def main():
    _, repos = api(f"https://api.github.com/orgs/{ORG}/repos?per_page=100&type=all")
    active = [r for r in repos if not r["archived"]]
    archived = [r for r in repos if r["archived"]]

    protected, unprotected = 0, []
    for r in active:
        st, _ = api(f"https://api.github.com/repos/{ORG}/{r['name']}/branches/main/protection")
        if st == 200:
            protected += 1
        else:
            unprotected.append(r["name"])

    heads = {}
    for r in active:
        _, c = api(f"https://api.github.com/repos/{ORG}/{r['name']}/commits/main")
        if c:
            heads[r["name"]] = {
                "sha": c["sha"],
                "committed": c["commit"]["committer"]["date"],
            }

    epoch = os.environ.get("SOURCE_DATE_EPOCH")
    if epoch and epoch.isdigit():
        now = datetime.fromtimestamp(int(epoch), timezone.utc).isoformat(timespec="seconds")
    else:
        now = datetime.now(timezone.utc).isoformat(timespec="seconds")
    lines = [
        "# GENERIERT von backlog/tools/verify_org.py — nicht manuell bearbeiten.",
        f'queried_at: "{now}"',
        "organisation: " + ORG,
        "repositories:",
        f"  total: {len(repos)}",
        f"  active: {len(active)}",
        f"  archived: {len(archived)}",
        "  archived_names:",
    ]
    lines += [f"    - {n}" for n in sorted(r["name"] for r in archived)]
    lines += [
        "pull_requests_open: " + str(count(f"org:{ORG} is:pr is:open")),
        "issues_open: " + str(count(f"org:{ORG} is:issue is:open")),
        "issues_open_p0: " + str(count(f"org:{ORG} is:issue is:open label:P0")),
        "issues_open_p1: " + str(count(f"org:{ORG} is:issue is:open label:P1")),
        "branch_protection:",
        f"  protected: {protected}",
        f"  unprotected_count: {len(unprotected)}",
        "  unprotected:",
    ]
    lines += [f"    - {n}" for n in sorted(unprotected)]
    lines.append("main_heads:")
    for name in sorted(heads):
        h = heads[name]
        lines.append(f'  {name}: {{sha: "{h["sha"]}", committed: "{h["committed"]}"}}')
    open(OUT, "w", encoding="utf-8").write("\n".join(lines) + "\n")
    print(
        f"verification.yaml geschrieben: {len(active)} aktiv, {len(archived)} archiviert, "
        f"{protected} geschuetzt, {len(unprotected)} ungeschuetzt"
    )


if __name__ == "__main__":
    main()
