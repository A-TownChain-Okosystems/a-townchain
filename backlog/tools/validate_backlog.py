#!/usr/bin/env python3
"""Validator fuer backlog/MASTER_BACKLOG.yaml.

Prueft, was ein maschinenlesbares Backlog tragen muss:
  V-1  jede ID genau einmal
  V-2  jede Abhaengigkeit existiert
  V-3  kein Zyklus
  V-4  Prioritaet aus P0/P1/P2
  V-5  Phase 0..7
  V-6  SSOT gesetzt und nicht UNASSIGNED
  V-7  mindestens ein pruefbares Akzeptanzkriterium
  V-8  Status aus dem erlaubten Vokabular
  V-9  Evidence-Feld vorhanden (auch wenn leer, dann offen)
  V-10 CLOSED-Tasks tragen eine Evidence-Angabe
Exit-Code 0 = PASS, 1 = FAIL.
"""

import os
import sys

import yaml

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
STATUSES = {
    "OPEN",
    "IN_PROGRESS",
    "BLOCKED",
    "TODO",
    "CLOSED",
    "FIX_IN_SOURCE_PR",
    "FIX_PENDING_OWNER_PUSH",
    "DONE",
}


def main():
    doc = yaml.safe_load(open(os.path.join(ROOT, "MASTER_BACKLOG.yaml"), encoding="utf-8"))
    tasks = doc["tasks"]
    errs = []

    ids = [t["id"] for t in tasks]
    seen = set()
    for i in ids:
        if i in seen:
            errs.append(f"V-1 doppelte ID: {i}")
        seen.add(i)

    graph = {}
    for t in tasks:
        graph[t["id"]] = list(t.get("depends_on") or [])
        if t["priority"] not in ("P0", "P1", "P2"):
            errs.append(f"V-4 {t['id']}: Prioritaet {t['priority']}")
        if t.get("phase") not in range(8):
            errs.append(f"V-5 {t['id']}: Phase {t.get('phase')}")
        if not t.get("ssot") or t["ssot"] == "UNASSIGNED":
            errs.append(f"V-6 {t['id']}: SSOT fehlt")
        if not t.get("acceptance_criteria"):
            errs.append(f"V-7 {t['id']}: kein Akzeptanzkriterium")
        if t.get("status") not in STATUSES:
            errs.append(f"V-8 {t['id']}: Status {t.get('status')}")
        if "evidence_required" not in t:
            errs.append(f"V-9 {t['id']}: Feld evidence_required fehlt")
        if t.get("status") in ("CLOSED", "DONE") and not (
            t.get("evidence_required") or t.get("evidence_head")
        ):
            errs.append(f"V-10 {t['id']}: CLOSED ohne Evidence")

    for t in tasks:
        for d in graph[t["id"]]:
            if d not in seen:
                errs.append(f"V-2 {t['id']}: Abhaengigkeit {d} existiert nicht")

    state = {}

    def walk(n, path):
        if state.get(n) == 2:
            return
        if state.get(n) == 1:
            errs.append("V-3 Zyklus: " + " -> ".join(path + [n]))
            return
        state[n] = 1
        for d in graph.get(n, []):
            if d in graph:
                walk(d, path + [n])
        state[n] = 2

    for n in graph:
        walk(n, [])

    print(f"Tasks: {len(tasks)} | Abhaengigkeiten: {sum(len(v) for v in graph.values())}")
    if errs:
        for e in errs:
            print("  FAIL", e)
        print("BACKLOG: FAIL")
        return 1
    print("BACKLOG: PASS (V-1..V-10)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
