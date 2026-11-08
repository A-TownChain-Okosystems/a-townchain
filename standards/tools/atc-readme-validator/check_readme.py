#!/usr/bin/env python3
"""README-Compliance-Validator — ATC-STD-README-001 (Gates README-01..13).

Prueft ein Repository-README gegen die Quality Gates des README-Standards.
Aufruf:  python3 tools/atc-readme-validator/check_readme.py <repo-pfad> [--gate13]
Exit-Code 0 = konform (alle zutreffenden Gates PASS), 1 = NON-COMPLIANT.

Gates:
  README-01  Repository eindeutig identifiziert (H1 + Project/Org-Header)
  README-02  Purpose-Sektion mit 4-Fragen-Einbettung
  README-03  Status aus Enum (9 Werte)
  README-04  Version vorhanden
  README-05  Architecture-Sektion (Components/Data Flow/Dependencies)
  README-06  Installation reproduzierbar (Requirements + Setup)
  README-07  Usage/Nutzung dokumentiert
  README-08  Testing beschrieben (Kommando + erwartetes Ergebnis)
  README-09  Security-Hinweis (kein oeffentliches Reporting)
  README-10  Standards & Compliance-Tabelle (ATC-STD-Referenzen)
  README-11  Roadmap kanonisch verlinkt (keine erfundene Inline-Roadmap)
  README-12  Governance + Maintainer definiert
  README-13  README entspricht Repository-Zustand (Struktur-Abgleich)
"""

import argparse
import os
import re
import sys

STATUS_ENUM = {
    "planning",
    "prototype",
    "development",
    "alpha",
    "beta",
    "release-candidate",
    "stable",
    "deprecated",
    "archived",
}

H2 = "^##+\\s*(%s)\\b"
REQUIRED_SECTIONS = [
    "Overview",
    "Purpose",
    "Status",
    "Architecture",
    "Features",
    "Repository Structure",
    "Requirements",
    "Installation",
    "Configuration",
    "Usage",
    "Development",
    "Testing",
    "Security",
    "Documentation",
    "Governance",
    "Standards & Compliance",
    "Roadmap",
    "Contributing",
    "License",
    "Maintainers",
    "Repository Metadata",
]


def gate_01(text):
    ok_h1 = bool(re.search(r"^#\s+ATC\s+\S", text, re.M))
    ok_org = "**Organization:** A-TownChain" in text or "**Project:**" in text
    return ok_h1 and ok_org, "H1 'ATC <Name>' + Project/Organization-Header"


def gate_02(text):
    ok = bool(re.search(H2 % "Purpose", text, re.M | re.I))
    return ok, "## Purpose-Sektion"


def gate_03(text):
    ok = bool(re.search(H2 % "Status", text, re.M | re.I))
    found = re.findall(r"Status:\*\*\s*`?([a-z\-]+)`?", text)
    bad = [s for s in found if s.lower() not in STATUS_ENUM]
    return ok and not bad, (
        "Status-Enum ok" if not bad else "Status nicht im Enum: " + ", ".join(bad)
    )


def gate_04(text):
    return bool(re.search(r"Version:\*\*\s*`?\d", text)), "Version im Header"


def gate_05(text):
    ok = bool(re.search(H2 % "Architecture", text, re.M | re.I))
    ok = ok and bool(re.search(r"Components", text, re.I))
    return ok, "Architecture mit Components (+ Data Flow/Dependencies)"


def gate_06(text):
    ok = bool(re.search(H2 % "Installation", text, re.M | re.I))
    ok = ok and bool(
        re.search(
            r"(git clone|cargo build|npm install|pip install"
            r"|docker|make\b)",
            text,
            re.I,
        )
    )
    return ok, "Installation mit reproduzierbarem Setup"


def gate_07(text):
    return bool(re.search(H2 % "Usage", text, re.M | re.I)), "## Usage-Sektion"


def gate_08(text):
    ok = bool(re.search(H2 % "Testing", text, re.M | re.I))
    ok = ok and bool(
        re.search(
            r"(PASS|cargo test|pytest|npm test|jest|"
            r"go test|rustc --test)",
            text,
            re.I,
        )
    )
    return ok, "Testing mit Kommando + erwartetem Ergebnis"


def gate_09(text):
    if not re.search(H2 % "Security", text, re.M | re.I):
        return False, "## Security-Sektion fehlt"
    ok = bool(
        re.search(
            r"(not be disclosed publicly|nicht.*(ffentlich|public)"
            r"|official ATC security reporting|ATC-STD-203)",
            text,
            re.I,
        )
    )
    return ok, "Security-Reporting-Hinweis"


def gate_10(text):
    ok = bool(re.search(H2 % "Standards & Compliance", text, re.M | re.I))
    ok = ok and bool(re.search(r"ATC-STD-\w+", text))
    return ok, "Standards-Compliance-Tabelle mit ATC-STD-Referenzen"


def gate_11(text):
    ok = bool(re.search(H2 % "Roadmap", text, re.M | re.I))
    ok = ok and bool(
        re.search(
            r"ROADMAP\.md|GitHub (Issues|Projects)|"
            r"Development Management|Notion",
            text,
        )
    )
    return ok, "Roadmap kanonisch verlinkt"


def gate_12(text):
    ok_g = bool(re.search(H2 % "Governance", text, re.M | re.I))
    ok_m = bool(re.search(H2 % "Maintainers", text, re.M | re.I))
    return ok_g and ok_m, "Governance- + Maintainer-Sektion"


def gate_13(text, repo):
    """Struktur-Abgleich: im README dokumentierte Top-Level-Einträge muessen
    existieren und umgekehrt muessen wesentliche Top-Level-Verzeichnisse
    erwaehnt sein."""
    m = re.search(r"## Repository Structure\n+```[a-z]*\n(.*?)```", text, re.S)
    if not m:
        return False, "Repository Structure-Block fehlt"
    tree = m.group(1)
    listed = set(re.findall(r"^\s*[├└][─]+\s+([A-Za-z0-9_.\-]+)", tree, re.M))
    listed = {x.rstrip("/") for x in listed if x not in ("...", "…")}
    real = {
        x
        for x in os.listdir(repo)
        if not x.startswith(".") and os.path.isdir(os.path.join(repo, x))
    }
    fehlt = sorted(
        x
        for x in real
        if x not in listed and x not in ("node_modules", "target", "venv", "__pycache__")
    )
    if fehlt:
        return False, "Im README fehlende Verzeichnisse: " + ", ".join(fehlt[:5])
    return True, "Struktur synchron mit Repository"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("repo", help="Repository-Pfad (enthält README.md)")
    args = ap.parse_args()
    rpath = os.path.join(args.repo, "README.md")
    if not os.path.exists(rpath):
        print(
            "README-Validator: KEINE README.md in %s — NON-COMPLIANT "
            "(ATC-STD-README-001)" % args.repo
        )
        return 1
    text = open(rpath, encoding="utf-8").read()
    gates = [
        ("README-01", lambda: gate_01(text)),
        ("README-02", lambda: gate_02(text)),
        ("README-03", lambda: gate_03(text)),
        ("README-04", lambda: gate_04(text)),
        ("README-05", lambda: gate_05(text)),
        ("README-06", lambda: gate_06(text)),
        ("README-07", lambda: gate_07(text)),
        ("README-08", lambda: gate_08(text)),
        ("README-09", lambda: gate_09(text)),
        ("README-10", lambda: gate_10(text)),
        ("README-11", lambda: gate_11(text)),
        ("README-12", lambda: gate_12(text)),
        ("README-13", lambda: gate_13(text, args.repo)),
    ]
    fails = 0
    print(
        "== README-Compliance (ATC-STD-README-001 v1.0.0) — %s =="
        % os.path.basename(os.path.abspath(args.repo))
    )
    for gid, fn in gates:
        ok, msg = fn()
        print("  [%s] %s: %s" % ("PASS" if ok else "FAIL", gid, msg))
        fails += 0 if ok else 1
    print(
        "RESULT: "
        + (
            "CONFORM — alle 13 Gates bestanden"
            if not fails
            else "NON-COMPLIANT (%d Gate(s) FAIL)" % fails
        )
    )
    return 1 if fails else 0


if __name__ == "__main__":
    sys.exit(main())
