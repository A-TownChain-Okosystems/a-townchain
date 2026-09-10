# -*- coding: utf-8 -*-
"""SCR-0081 P0-07: Consensus-Boundary-Check (F-105 Code-Gate, Report-Modus).

KANONISCHE REGEL (registry/repositories.yaml, SCR-0080/0081):
  Konsens-Implementierung liegt KANONISCH bei atc-algorithm (capability: consensus).
  a-townchain orchestriert NUR — keine zweite Konsens-Implementierung.

Modus:
  python3 tools/consensus_boundary_check.py            # Report (exit 0)
  python3 tools/consensus_boundary_check.py --strict   # Gate (exit 1 bei Fund)

Status: Report-Modus aktiv bis der Legacy-Konsensbestand (ShivaConsensus/PoH/PoS/PoW)
nach atc-algorithm refactored ist (F-105). Strict-Modus wird danach Pflicht."""
import os, re, sys, subprocess

INDICATORS = re.compile(
    r"(consensus|/poh/|/pos/|/pow/|validator[_/]|shivaconsensus|finality)", re.I)
CODE = re.compile(r".*\.(py|rs|ts|tsx)$")
SKIP = re.compile(r"(archive|node_modules|\.git|docs?/|spec|draft)", re.I)

def scan(root):
    hits = []
    for dirpath, dirs, files in os.walk(root):
        rel = os.path.relpath(dirpath, root)
        if SKIP.search(rel):
            dirs[:] = []
            continue
        if INDICATORS.search(rel + "/"):
            for f in files:
                if CODE.match(f):
                    hits.append(os.path.join(rel, f))
    return hits

def main():
    strict = "--strict" in sys.argv
    root = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    hits = scan(root)
    print("CONSENSUS-BOUNDARY-CHECK (F-105, SCR-0081)")
    print("  Kanonisch: atc-algorithm | a-townchain = orchestration only")
    if not hits:
        print("  OK: keine Konsens-Implementierung im a-townchain-Baum")
        return 0
    print(f"  LEGACY-KONSENS-BESTAND ({len(hits)} Dateien) — Refactor nach atc-algorithm offen (F-105):")
    for h in hits[:25]:
        print("   -", h)
    if len(hits) > 25:
        print(f"   ... +{len(hits)-25} weitere")
    if strict:
        print("  STRICT-GATE: FAIL — zweite Konsens-Implementierung verboten")
        return 1
    print("  REPORT-Modus: kein Gate-Fehler (strict nach F-105-Refactor)")
    return 0

if __name__ == "__main__":
    sys.exit(main())
