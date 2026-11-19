#!/usr/bin/env python3
"""PR-Triage: vollstaendiger Bestand, Entscheidung je PR, Abhaengigkeiten, Konflikte, Evidence.

Kein Merge, kein Close, keine Aenderung an GitHub. Reine Klassifikation.
Ausgabe: backlog/pr-triage/PR_*.yaml
"""

import json
import os
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from concurrent.futures import ThreadPoolExecutor

import yaml

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(ROOT, "pr-triage")
ORG = "A-TownChain-Okosystems"
DECISIONS = ["KEEP", "UPDATE", "MERGE_CANDIDATE", "CLOSE", "BLOCKED", "DUPLICATE"]

FROZEN = {
    "atc-vm": "a-townchain/components/vm",
    "atc-algorithm": "a-townchain/components/algorithm",
    "atc-wallet": "a-townchain/components/wallet",
    "atc-explorer": "a-townchain/components/explorer",
    "atc-indexer": "a-townchain/components/indexer",
    "atc-mining": "a-townchain/components/mining",
    "atc-interop": "a-townchain/components/interop",
    "atc-oracle": "a-townchain/components/oracle",
    "atc-storage": "a-townchain/components/storage",
}

NAMED = {
    ("a-townchain-ecosystem", 247): "Blocker-Register 0.3.7",
    ("a-townchain-ecosystem", 248): "Cargo-Workspace-Korrektur",
    ("a-townchain-ecosystem", 243): "Validator-Schluessel fail-closed",
    ("a-townchain", 118): "Devnet Ein-Befehl-Start (G1)",
    ("a-townchain", 116): "SSOT-Qualitaetskorrektur",
    ("globus-os", 117): "Typisierte Kernel-Rechte",
    ("atc-standards", 121): "Hybrid Governance",
    ("atc-standards", 123): "Strenge Task-Validierung",
    ("atclang", 40): "Evidenzkorrektur",
    ("atc-vm", 16): "Ownership-Dokumentation",
}


def tok():
    t = os.environ.get("GITHUB_ACCESS_TOKEN")
    if not t:
        sys.exit("GITHUB_ACCESS_TOKEN fehlt (Wert wird nicht ausgegeben)")
    return t


def api(path):
    url = path if path.startswith("http") else "https://api.github.com" + path
    req = urllib.request.Request(
        url,
        headers={
            "Authorization": "Bearer " + tok(),
            "Accept": "application/vnd.github+json",
            "User-Agent": "atc-triage",
        },
    )
    for attempt in range(4):
        try:
            with urllib.request.urlopen(req, timeout=40) as f:
                return json.load(f)
        except urllib.error.HTTPError as e:
            if e.code in (403, 429) and attempt < 3:
                time.sleep(20 * (attempt + 1))
                continue
            return {"__err": e.code}
        except Exception:
            if attempt < 3:
                time.sleep(5)
                continue
            return {"__err": "net"}
    return {"__err": "exhausted"}


def search_prs():
    out, page = [], 1
    while True:
        q = urllib.parse.quote(f"org:{ORG} is:pr is:open")
        d = api(f"/search/issues?per_page=100&page={page}&q={q}")
        items = d.get("items", []) if isinstance(d, dict) else []
        out += items
        if len(items) < 100:
            break
        page += 1
    return out


def detail(it):
    repo_full = it["repository_url"].split("/repos/")[-1]
    repo = repo_full.split("/")[-1]
    num = it["number"]
    pr = api(f"/repos/{repo_full}/pulls/{num}")
    if "__err" in pr:
        return {"repo": repo, "number": num, "error": pr["__err"]}
    files = api(f"/repos/{repo_full}/pulls/{num}/files?per_page=100")
    names = [f["filename"] for f in files] if isinstance(files, list) else []
    sha = pr["head"]["sha"]
    checks = api(f"/repos/{repo_full}/commits/{sha}/check-runs?per_page=100")
    runs = checks.get("check_runs", []) if isinstance(checks, dict) else []
    concl = {}
    for r in runs:
        c = r.get("conclusion") or r.get("status") or "pending"
        concl[c] = concl.get(c, 0) + 1
    return {
        "repo": repo,
        "number": num,
        "title": pr["title"],
        "author": pr["user"]["login"],
        "draft": pr["draft"],
        "created": pr["created_at"],
        "updated": pr["updated_at"],
        "sha": sha,
        "base": pr["base"]["ref"],
        "mergeable": pr.get("mergeable"),
        "mergeable_state": pr.get("mergeable_state"),
        "additions": pr["additions"],
        "deletions": pr["deletions"],
        "changed_files": pr["changed_files"],
        "commits": pr["commits"],
        "review_comments": pr.get("review_comments", 0),
        "labels": [lbl["name"] for lbl in pr["labels"]],
        "files": names,
        "checks": concl,
        "checks_total": len(runs),
    }


def verdict(p):
    if p["repo"] in FROZEN:
        return "BLOCKED", f"SSOT-Freeze: kanonischer Ort ist {FROZEN[p['repo']]}"
    failing = (
        p["checks"].get("failure", 0)
        + p["checks"].get("cancelled", 0)
        + p["checks"].get("timed_out", 0)
    )
    green = p["checks"].get("success", 0)
    pending = (
        p["checks"].get("in_progress", 0)
        + p["checks"].get("queued", 0)
        + p["checks"].get("pending", 0)
    )
    if p["mergeable_state"] == "dirty":
        return "BLOCKED", "Merge-Konflikt mit der Basis"
    if failing:
        return "BLOCKED", f"{failing} fehlgeschlagene Checks"
    if p["draft"]:
        return "UPDATE", "Draft, nicht abnahmebereit"
    if pending:
        return "UPDATE", f"{pending} Checks laufen noch"
    if p["mergeable"] and green and not pending:
        return "MERGE_CANDIDATE", f"{green} Checks gruen, mergebar"
    return "KEEP", "kein blockierender Befund, aber kein gruener Nachweis"


def link_backlog(p, tasks):
    text = (p["title"] + " " + " ".join(p["files"])).lower()
    hits = []
    for t in tasks:
        ssot = (t.get("ssot") or "").lower()
        if not ssot:
            continue
        key = ssot.split()[0]
        if key and key in text:
            hits.append(t["id"])
    return sorted(set(hits))[:4]


def main():
    tasks = yaml.safe_load(open(os.path.join(ROOT, "MASTER_BACKLOG.yaml"), encoding="utf-8"))[
        "tasks"
    ]
    listing = search_prs()
    if not listing:
        sys.exit("Suche lieferte 0 PRs — Abbruch, keine leeren Artefakte geschrieben")
    with ThreadPoolExecutor(max_workers=8) as ex:
        prs = [r for r in ex.map(detail, listing) if "error" not in r]

    for p in prs:
        d, why = verdict(p)
        p["decision"] = d
        p["reason"] = why
        p["backlog_links"] = link_backlog(p, tasks)
        p["named_in_backlog"] = NAMED.get((p["repo"], p["number"]))

    # Duplikate: gleiche Datei, gleiche Aenderungsrichtung, aelterer PR
    def words(t):
        return set(re.findall(r"[a-z0-9]{4,}", t.lower()))

    for a in prs:
        if a["decision"] in ("DUPLICATE",):
            continue
        for b in prs:
            if a is b or a["repo"] != b["repo"]:
                continue
            shared = set(a["files"]) & set(b["files"])
            if not shared or not a["files"]:
                continue
            sim = len(words(a["title"]) & words(b["title"])) / max(1, len(words(a["title"])))
            if sim >= 0.5 and a["updated"] < b["updated"]:
                a["decision"] = "DUPLICATE"
                a["reason"] = (
                    f"ueberlappt mit #{b['number']} ({len(shared)} gemeinsame Dateien, Titelaehnlichkeit {sim:.2f})"
                )
                a["duplicate_of"] = b["number"]
                break

    prs.sort(key=lambda p: (p["repo"], p["number"]))
    counts = {}
    for p in prs:
        counts[p["decision"]] = counts.get(p["decision"], 0) + 1

    def dump(name, doc):
        with open(os.path.join(OUT, name), "w", encoding="utf-8") as f:
            f.write("# GENERIERT von backlog/tools/pr_triage.py — nicht manuell bearbeiten.\n")
            yaml.safe_dump(doc, f, allow_unicode=True, sort_keys=False, width=110)

    dump(
        "PR_INVENTORY.yaml",
        {
            "schema_version": "1.0.0",
            "organisation": ORG,
            "source": "GitHub Search + REST, is:pr is:open",
            "total": len(prs),
            "repositories": sorted({p["repo"] for p in prs}),
            "pull_requests": [
                {
                    k: p[k]
                    for k in (
                        "repo",
                        "number",
                        "title",
                        "author",
                        "draft",
                        "created",
                        "updated",
                        "sha",
                        "base",
                        "mergeable_state",
                        "changed_files",
                        "additions",
                        "deletions",
                        "commits",
                        "labels",
                    )
                }
                for p in prs
            ],
        },
    )
    dump(
        "PR_TRIAGE.yaml",
        {
            "schema_version": "1.0.0",
            "total": len(prs),
            "counts": counts,
            "rules": [
                "Merge-Konflikt -> BLOCKED",
                "fehlgeschlagene Checks -> BLOCKED",
                "Draft -> UPDATE",
                "laufende Checks -> UPDATE",
                "mergebar + alle Checks gruen -> MERGE_CANDIDATE",
                "gleiche Datei + Titelaehnlichkeit >= 0.5 + aelterer Stand -> DUPLICATE",
                "sonst KEEP",
            ],
            "decisions": [
                {
                    k: p.get(k)
                    for k in (
                        "repo",
                        "number",
                        "decision",
                        "reason",
                        "draft",
                        "mergeable_state",
                        "checks_total",
                        "backlog_links",
                        "named_in_backlog",
                        "duplicate_of",
                    )
                }
                for p in prs
            ],
        },
    )
    dump(
        "PR_DEPENDENCIES.yaml",
        {
            "schema_version": "1.0.0",
            "note": "Dateiueberlappung innerhalb eines Repositories: wer zuerst, blockiert den anderen.",
            "edges": [
                {
                    "repo": a["repo"],
                    "before": b["number"],
                    "after": a["number"],
                    "shared_files": len(set(a["files"]) & set(b["files"])),
                }
                for a in prs
                for b in prs
                if a is not b
                and a["repo"] == b["repo"]
                and a["files"]
                and set(a["files"]) & set(b["files"])
                and a["updated"] > b["updated"]
            ][:400],
        },
    )
    conflicts = []
    for repo in sorted({p["repo"] for p in prs}):
        rs = [p for p in prs if p["repo"] == repo]
        for i, a in enumerate(rs):
            for b in rs[i + 1 :]:
                shared = set(a["files"]) & set(b["files"])
                if shared:
                    conflicts.append(
                        {
                            "repo": repo,
                            "prs": [a["number"], b["number"]],
                            "shared_files": sorted(shared)[:8],
                            "count": len(shared),
                        }
                    )
    dump(
        "PR_SSOT_CONFLICTS.yaml",
        {
            "schema_version": "1.0.0",
            "note": "Zwei offene PRs, die dieselbe Datei aendern, sind ein SSOT-Konflikt bis einer geschlossen ist.",
            "conflicts": sorted(conflicts, key=lambda c: -c["count"])[:200],
        },
    )
    dump(
        "PR_EVIDENCE.yaml",
        {
            "schema_version": "1.0.0",
            "note": "Head-SHA und Check-Lage je PR. Ein gruener Check belegt den Lauf, nicht die Funktion.",
            "evidence": [
                {
                    k: p.get(k)
                    for k in (
                        "repo",
                        "number",
                        "sha",
                        "checks",
                        "checks_total",
                        "commits",
                        "review_comments",
                    )
                }
                for p in prs
            ],
        },
    )
    md = [
        "---",
        "document_id: ATC-PR-TRIAGE-001",
        'title: "PR-Triage — Bestand und Einzelentscheidung"',
        "status: GENERATED",
        "normative: false",
        "generator: backlog/tools/pr_triage.py",
        "---",
        "",
        "# PR-Triage",
        "",
        "> Nicht-normativ, generiert. Keine Aktion an GitHub ausgefuehrt: keine Merges,",
        "> keine Schliessungen. Die Entscheidung ist ein Vorschlag, keine Vollmacht.",
        "",
        "## Bestand",
        "",
        "| Kennzahl | Wert |",
        "|---|---|",
        f"| Offene PRs | {len(prs)} |",
        f"| Repositories mit offenen PRs | {len({p['repo'] for p in prs})} |",
    ]
    for k in DECISIONS:
        md.append(f"| {k} | {counts.get(k, 0)} |")
    md += [
        "",
        "## Die zehn im Backlog benannten PRs",
        "",
        "| PR | Repository | Entscheidung | Befund |",
        "|---|---|---|---|",
    ]
    for p in prs:
        if p["named_in_backlog"]:
            md.append(f"| #{p['number']} | {p['repo']} | {p['decision']} | {p['reason']} |")
    md += [
        "",
        "## Mergekandidaten",
        "",
        "| PR | Repository | Checks | Titel |",
        "|---|---|---|---|",
    ]
    for p in prs:
        if p["decision"] == "MERGE_CANDIDATE":
            md.append(
                f"| #{p['number']} | {p['repo']} | {p['checks'].get('success', 0)} gruen | {p['title'][:60]} |"
            )
    md += [
        "",
        "## Groesste Dateikonflikte",
        "",
        "| Repository | PRs | Gemeinsame Dateien |",
        "|---|---|---|",
    ]
    for c in sorted(conflicts, key=lambda c: -c["count"])[:10]:
        md.append(f"| {c['repo']} | #{c['prs'][0]} / #{c['prs'][1]} | {c['count']} |")
    md += [
        "",
        "## Regel",
        "",
        "Jede Entscheidung ist ein Vorschlag mit Begruendung. Ein Merge oder Close",
        "erfordert eine bewusste Handlung am jeweiligen PR.",
        "",
    ]
    open(os.path.join(OUT, "PR_TRIAGE.md"), "w", encoding="utf-8").write("\n".join(md))

    print(f"PRs: {len(prs)} | {counts}")
    named = [p for p in prs if p["named_in_backlog"]]
    print("benannte PRs live gefunden:", len(named), "von", len(NAMED))
    missing = [
        f"{r}#{n}" for (r, n) in NAMED if not any(p["repo"] == r and p["number"] == n for p in prs)
    ]
    print("nicht offen oder fehlend:", missing)


if __name__ == "__main__":
    main()
