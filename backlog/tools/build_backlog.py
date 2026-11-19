#!/usr/bin/env python3
"""Baut das Master-Backlog aus drei Quellen:

1. backlog/sources/register-2026-11-17.yaml — gepinnter Blocker-Snapshot (Inhalt)
2. backlog/curation.yaml                   — SSOT, Phase, Abhaengigkeiten, Zusatz-Tasks
3. backlog/verification.yaml               — Live-Erhebung des Organisationsbestands

Ausgabe: backlog/MASTER_BACKLOG.yaml und backlog/MASTER_BACKLOG.md.
Reine Ableitung: keine erfundenen Zustaende, keine Prosa-Zahlen.
"""

import os
import re
from datetime import datetime, timezone

import yaml

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PRIORITIES = ("P0", "P1", "P2")
PHASES = {
    0: "Repository- und CI-Stabilisierung",
    1: "Normative Grundlagen",
    2: "Blockchain-Basis",
    3: "ATCLang und ATC-VM",
    4: "ShivaCore und GlobusOS",
    5: "Aurora AI",
    6: "Genesis Engine und Oekosystemdienste",
    7: "Gesamtsystem-Verifikation",
}


def _timestamp():
    """Provenienz-Zeitstempel. Mit SOURCE_DATE_EPOCH reproduzierbar."""
    epoch = os.environ.get("SOURCE_DATE_EPOCH")
    if epoch and epoch.isdigit():
        return datetime.fromtimestamp(int(epoch), timezone.utc).isoformat(timespec="seconds")
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def load(name):
    with open(os.path.join(ROOT, name), encoding="utf-8") as f:
        return yaml.safe_load(f)


def split_criteria(text):
    if not text:
        return []
    parts = re.split(r";\s+|,\s+(?=[A-ZÄÖÜ])", str(text).strip())
    return [p.strip().rstrip(".") for p in parts if len(p.strip()) > 8]


def main():
    register = load("sources/register-2026-11-17.yaml")
    curation = load("curation.yaml")
    verify = load("verification.yaml")
    mapping = curation["blockers"]

    tasks = []
    for b in register["blockers"]:
        bid = b["id"]
        m = mapping.get(bid, {})
        prio = b.get("priority") or ("P" + bid.split("-")[1][1])
        criteria = split_criteria(b.get("closure_criteria"))
        tasks.append(
            {
                "id": bid,
                "title": b.get("title"),
                "priority": prio,
                "phase": m.get("phase", 0),
                "status": m.get("status_override", b.get("status", "OPEN")),
                "ssot": m.get("ssot", "UNASSIGNED"),
                "gate": b.get("gate"),
                "depends_on": m.get("depends_on", []),
                "register_blocked_by": b.get("blocked_by", []),
                "acceptance_criteria": criteria,
                "evidence_required": b.get("evidence"),
                "evidence_head": b.get("evidence_head"),
                "last_verified": str(b.get("last_verified") or ""),
                "source": "BLOCKER_REGISTER-001 v" + str(register.get("version", "")),
            }
        )

    for t in curation.get("additional", []):
        tasks.append(
            {
                "id": t["id"],
                "title": t["title"],
                "priority": t["priority"],
                "phase": t.get("phase", 0),
                "status": t.get("status", "OPEN"),
                "ssot": t["ssot"],
                "gate": t.get("gate"),
                "depends_on": t.get("depends_on", []),
                "register_blocked_by": [],
                "acceptance_criteria": t.get("acceptance_criteria", []),
                "evidence_required": t.get("evidence_required"),
                "evidence_head": None,
                "last_verified": "",
                "source": "Bestandsaufnahme 2026-11-19 (verifiziert)",
            }
        )

    tasks.sort(
        key=lambda x: (
            PRIORITIES.index(x["priority"]) if x["priority"] in PRIORITIES else 9,
            x["phase"],
            x["id"],
        )
    )

    def tally(key):
        out = {}
        for t in tasks:
            out[t[key]] = out.get(t[key], 0) + 1
        return dict(sorted(out.items(), key=lambda kv: str(kv[0])))

    ev = [t for t in tasks if t.get("evidence_head")]
    ev_full = [t for t in ev if len(str(t["evidence_head"])) == 40]

    doc = {
        "schema_version": "1.0.0",
        "document_id": "ATC-MASTER-BACKLOG-001",
        "title": "Repositoryuebergreifendes Master-Backlog",
        "generated_at": _timestamp(),
        "generated_by": "backlog/tools/build_backlog.py",
        "sources": {
            "register": {
                "repository": "A-TownChain-Okosystems/a-townchain-ecosystem",
                "path": "docs/system-readiness/BLOCKER_REGISTER.yaml",
                "ref": "PR #247 head d691e51d",
                "register_version": str(register.get("version")),
                "register_date": str(register.get("updated")),
                "pinned_copy": "backlog/sources/register-2026-11-17.yaml",
            },
            "verification": {
                "file": "backlog/verification.yaml",
                "queried_at": verify.get("queried_at"),
            },
        },
        "policy": {
            "ssot_rule": "Eine Aufgabe, ein kanonischer Eigentuemer. Zwei SSOT fuer dieselbe Funktion sind ein Finding.",
            "evidence_rule": "Kein Status ohne HEAD-gebundene Evidence: Repository, Branch, vollstaendiger Commit-SHA, Datei, Workflow-Run, Job, Step, Exit-Code.",
            "green_ci_is_not_proof": "Ein gruener CI-Lauf belegt, dass ein Job lief, nicht dass die Funktion erfuellt ist.",
            "corrections_rule": "Widersprueche zwischen Bericht und Live-Bestand werden hier gefuehrt, nicht stillschweigend uebernommen.",
        },
        "summary": {
            "tasks_total": len(tasks),
            "by_priority": tally("priority"),
            "by_status": tally("status"),
            "by_phase": tally("phase"),
            "evidence_binding": {
                "tasks_with_evidence_head": len(ev),
                "evidence_sha_full_40": len(ev_full),
                "evidence_sha_short": len(ev) - len(ev_full),
                "tasks_without_evidence": len(tasks) - len(ev),
            },
            "org_snapshot": {
                "repositories_total": verify["repositories"]["total"],
                "repositories_active": verify["repositories"]["active"],
                "repositories_archived": verify["repositories"]["archived"],
                "pull_requests_open": verify["pull_requests_open"],
                "issues_open": verify["issues_open"],
                "issues_open_p0": verify["issues_open_p0"],
                "issues_open_p1": verify["issues_open_p1"],
                "branch_protection_protected": verify["branch_protection"]["protected"],
                "branch_protection_unprotected": verify["branch_protection"]["unprotected_count"],
            },
        },
        "corrections": curation.get("corrections", []),
        "phases": [
            {
                "id": pid,
                "name": name,
                "task_ids": [t["id"] for t in tasks if t["phase"] == pid],
            }
            for pid, name in PHASES.items()
        ],
        "tasks": tasks,
    }

    with open(os.path.join(ROOT, "MASTER_BACKLOG.yaml"), "w", encoding="utf-8") as f:
        f.write("# GENERIERT von backlog/tools/build_backlog.py — nicht manuell bearbeiten.\n")
        f.write(
            "# Aenderungen an Inhalt oder Reihenfolge gehoeren in die Quellen, nicht hierher.\n"
        )
        yaml.safe_dump(doc, f, allow_unicode=True, sort_keys=False, width=100)

    # ---------- Markdown-Ansicht ----------
    md = []
    md.append("---")
    md.append("document_id: ATC-MASTER-BACKLOG-001")
    md.append('title: "Repositoryuebergreifendes Master-Backlog"')
    md.append(f'generated: "{doc["generated_at"][:10]}"')
    md.append("status: GENERATED")
    md.append("normative: false")
    md.append("generator: backlog/tools/build_backlog.py")
    md.append("---")
    md.append("")
    md.append("# Master-Backlog — A-TownChain / KAI-OS")
    md.append("")
    md.append("> **Nicht-normativ, generiert.** Quelle ist das Blocker-Register in")
    md.append("> `a-townchain-ecosystem` (PR #247, Stand 2026-11-17) plus die live")
    md.append("> geprueften Zahlen vom " + str(verify.get("queried_at"))[:10] + ". Diese Datei")
    md.append("> enthaelt keine eigene Fachwahrheit und keine handgepflegten Zaehler.")
    md.append("")
    md.append("## Bestand")
    md.append("")
    md.append("| Kennzahl | Wert |")
    md.append("|---|---|")
    snap = doc["summary"]["org_snapshot"]
    md.append(
        f"| Repositories (gesamt / aktiv / archiviert) | {snap['repositories_total']} / {snap['repositories_active']} / {snap['repositories_archived']} |"
    )
    md.append(f"| Offene Pull Requests | {snap['pull_requests_open']} |")
    md.append(
        f"| Offene Issues (davon P0 / P1) | {snap['issues_open']} ({snap['issues_open_p0']} / {snap['issues_open_p1']}) |"
    )
    md.append(
        f"| Branch Protection (geschuetzt / ungeschuetzt) | {snap['branch_protection_protected']} / {snap['branch_protection_unprotected']} |"
    )
    md.append(f"| Aufgaben im Backlog | {doc['summary']['tasks_total']} |")
    md.append("")
    md.append("## Widersprueche zwischen Bericht und Live-Bestand")
    md.append("")
    md.append("| Behauptung | Live-Ergebnis | Methode |")
    md.append("|---|---|---|")
    for c in doc["corrections"]:
        md.append(
            f"| {c['claim']} | {c['result'] if 'result' in c else c['live_result']} | {c['method']} |"
        )
    md.append("")
    eb = doc["summary"]["evidence_binding"]
    md.append("## Evidence-Bindung")
    md.append("")
    md.append("| Kennzahl | Wert |")
    md.append("|---|---|")
    md.append(f"| Aufgaben mit Evidence-Kopf | {eb['tasks_with_evidence_head']} |")
    md.append(f"| davon vollstaendiger 40-stelliger SHA | {eb['evidence_sha_full_40']} |")
    md.append(f"| davon Kurz-SHA (nicht releasefaehig) | {eb['evidence_sha_short']} |")
    md.append(f"| Aufgaben ohne Evidence | {eb['tasks_without_evidence']} |")
    md.append("")
    md.append("Ein Kurz-SHA ist ein Hinweis, kein Nachweis. Fuer Release-Gates muss der")
    md.append("vollstaendige SHA samt Workflow-Run, Job und Exit-Code vorliegen.")
    md.append("")
    md.append("## Reihenfolge")
    md.append("")
    for ph in doc["phases"]:
        ids = ", ".join(ph["task_ids"]) if ph["task_ids"] else "—"
        md.append(f"**Phase {ph['id']} — {ph['name']}** ({len(ph['task_ids'])} Aufgaben)")
        md.append("")
        md.append(ids)
        md.append("")
    md.append("## Aufgaben")
    md.append("")
    md.append("| ID | Prio | Phase | Status | SSOT | Titel |")
    md.append("|---|---|---|---|---|---|")
    for t in tasks:
        md.append(
            f"| {t['id']} | {t['priority']} | {t['phase']} | {t['status']} | {t['ssot']} | {t['title']} |"
        )
    md.append("")
    md.append("## Abhaengigkeiten")
    md.append("")
    for t in tasks:
        if t["depends_on"]:
            md.append(f"- **{t['id']}** setzt voraus: {', '.join(t['depends_on'])}")
    md.append("")
    md.append("## Evidence-Regel")
    md.append("")
    md.append("Ein Task gilt erst als geschlossen, wenn der Nachweis an einen Commit gebunden ist:")
    md.append("Repository, Branch, vollstaendiger SHA, Datei und Symbol, Workflow-Run, Job,")
    md.append("Step, Exit-Code und Log. Ein gruener CI-Lauf allein ist kein Funktionsnachweis.")
    md.append("")

    with open(os.path.join(ROOT, "MASTER_BACKLOG.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(md))

    print(f"MASTER_BACKLOG: {len(tasks)} Aufgaben, {len(doc['corrections'])} Korrekturen")
    print("  nach Prioritaet:", doc["summary"]["by_priority"])
    print("  nach Status:    ", doc["summary"]["by_status"])


if __name__ == "__main__":
    main()
