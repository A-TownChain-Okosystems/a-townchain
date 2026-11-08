#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""ATC Repository Auditor v0.2.0 — Validator fuer ATC-STD-201 v1.0.0 (AD-031).

Prueft die MUST/SHOULD-Regeln der Compliance-Matrix (V-01…V-16), erzeugt einen
Health Score (ATC-STD-203 §15) und entscheidet GATE: PASS / NO-GO.

Aufruf: python3 atc_repo_audit.py <repo-pfad> [--level R2] [--registry <repositories.yaml>]
Exit:   0 = GATE: PASS, 1 = GATE: NO-GO. Abhaengigkeiten: nur Python-stdlib.
"""

import argparse
import os
import re
import subprocess
import sys

VERSION = "0.2.0"  # SCR-0115: Ticket-Praefix erlaubt, Merges ausgenommen
LEVELS = ["R0", "R1", "R2", "R3", "R4"]
BAD_PATHS = [".env", "node_modules/", "target/debug/", "/dist/", "/*.log", "tmp/", ".DS_Store"]
CC_RE = re.compile(
    r"^(\[[^\]]+\]\s*)?(feat|fix|docs|style|refactor|test|security|perf|build|ci|chore|spec|release|audit|approve|restore|sync|review|init)(\([^)]+\))?: .+"
)  # SCR-0115: optionaler [Ticket]-Praefix ist Org-Konvention (z.B. "[S26] docs(...):")


def lvl_ge(required, level):
    return LEVELS.index(level) >= LEVELS.index(required)


class Audit:
    def __init__(self):
        self.results = []  # (regel, status, kategorie, sev, message)

    def add(self, regel, status, kategorie, message):
        self.results.append((regel, status, kategorie, message))

    def score(self):
        kat = {}
        for _, status, kategorie, _ in self.results:
            g, p = kat.get(kategorie, (0, 0))
            if status != "SKIP":
                g += 1
                if status == "PASS":
                    p += 1
                elif status == "WARN":
                    p += 0.5
            kat[kategorie] = (g, p)
        total = sum(p for g, p in kat.values()) / max(1, sum(g for g, _ in kat.values()))
        return round(total * 100)

    def gate(self, level):
        fails = [r for r in self.results if r[1] == "FAIL"]
        return "PASS" if not fails and self.score() >= 85 else "NO-GO"


def tracked_files(repo):
    out = subprocess.run(["git", "ls-files"], cwd=repo, capture_output=True, text=True).stdout
    return [ln for ln in out.splitlines() if ln.strip()]


def read(path):
    try:
        with open(path, encoding="utf-8", errors="replace") as f:
            return f.read()
    except OSError:
        return None


def yaml_has(text, keys):
    """Minimal-YAML-Check: alle Schluessel als 'key:' im Text vorhanden."""
    return all(re.search(r"^\s*%s\s*:" % re.escape(k), text, re.M) for k in keys)


def audit_repo(repo, level, registry_path=None):
    a = Audit()
    meta0 = read(os.path.join(repo, ".atc/repository.yaml")) or ""
    m0 = re.search(r"name:\s*([A-Za-z0-9_-]+)", meta0)
    name = m0.group(1) if m0 else os.path.basename(os.path.abspath(repo))
    files = tracked_files(repo)

    def present(p):
        return any(f == p or f.startswith(p) for f in files) or os.path.exists(
            os.path.join(repo, p)
        )

    # V-01 LICENSE
    a.add(
        "V-01",
        "PASS"
        if present("LICENSE") and (read(os.path.join(repo, "LICENSE")) or "").strip()
        else "FAIL",
        "Structure",
        "LICENSE fehlt/leer" if not present("LICENSE") else "LICENSE vorhanden",
    )

    # V-02/V-03 README
    readme = read(os.path.join(repo, "README.md")) or ""
    a.add(
        "V-02",
        "PASS" if readme.strip() else "FAIL",
        "Documentation",
        "README.md fehlt" if not readme.strip() else "README.md vorhanden",
    )
    sections = [
        "urpose",
        "cope",
        "rchitect",
        "eatures",
        "nstall",
        "evelop",
        "estin",
        "ecurit",
        "oadmap",
        "ersion",
        "icens",
    ]
    hit = sum(1 for s in sections if re.search(s, readme, re.I))
    if lvl_ge("R2", level):
        sev = "PASS" if hit >= 10 else "FAIL"
        a.add("V-03", sev, "Documentation", "README 12-Abschnitte: %d/11 Schluesselworte" % hit)
    else:
        a.add("V-03", "SKIP", "Documentation", "nicht anwendbar < R2")

    # V-04 .atc/repository.yaml
    meta = read(os.path.join(repo, ".atc/repository.yaml"))
    if lvl_ge("R1", level):
        if meta and yaml_has(
            meta,
            [
                "standard",
                "name",
                "classification",
                "maturity",
                "organization",
                "primary",
                "criticality",
            ],
        ):
            a.add("V-04", "PASS", "Structure", ".atc/repository.yaml gueltig")
        else:
            a.add("V-04", "FAIL", "Structure", ".atc/repository.yaml fehlt/unvollstaendig")
    else:
        a.add("V-04", "SKIP", "Structure", "nicht anwendbar R0")

    # V-05 ownership + CODEOWNERS
    if lvl_ge("R2", level):
        own = read(os.path.join(repo, ".atc/ownership.yaml"))
        co = present(".github/CODEOWNERS") or present("CODEOWNERS") or present("docs/CODEOWNERS")
        ok = bool(own and yaml_has(own, ["repository", "maintainers"])) and co
        a.add(
            "V-05",
            "PASS" if ok else "FAIL",
            "Ownership",
            "ownership.yaml %s, CODEOWNERS %s"
            % ("ok" if own else "FEHLT", "ok" if co else "FEHLT"),
        )
    else:
        a.add("V-05", "SKIP", "Ownership", "nicht anwendbar < R2")

    # V-06 lifecycle
    lc = read(os.path.join(repo, ".atc/lifecycle.yaml"))
    if lvl_ge("R2", level):
        stages = ["experimental", "development", "beta", "production", "deprecated", "archived"]
        ok = bool(lc) and any(("stage: %s" % s) in lc for s in stages)
        a.add(
            "V-06",
            "PASS" if ok else "FAIL",
            "Ownership",
            "lifecycle.yaml %s" % ("gueltig" if ok else "fehlt/ungueltig"),
        )
    else:
        a.add("V-06", "SKIP", "Ownership", "nicht anwendbar < R2")

    # V-07 compliance.yaml
    comp = read(os.path.join(repo, ".atc/compliance.yaml"))
    if lvl_ge("R2", level):
        ok = bool(comp) and yaml_has(comp, ["standard", "level", "gates"])
        a.add(
            "V-07",
            "PASS" if ok else "FAIL",
            "Security",
            "compliance.yaml %s" % ("ok" if ok else "fehlt"),
        )
    else:
        a.add("V-07", "SKIP", "Security", "nicht anwendbar < R2")

    # V-08 SECURITY.md
    sec = read(os.path.join(repo, "SECURITY.md"))
    if lvl_ge("R2", level):
        ok = bool(sec and len(sec.strip()) > 100)
        a.add(
            "V-08",
            "PASS" if ok else "FAIL",
            "Security",
            "SECURITY.md %s" % ("vorhanden" if ok else "fehlt/duenn"),
        )
    else:
        a.add("V-08", "SKIP", "Security", "nicht anwendbar < R2")

    # V-09 CHANGELOG
    ch = read(os.path.join(repo, "CHANGELOG.md"))
    if lvl_ge("R2", level):
        a.add(
            "V-09",
            "PASS" if ch and ch.strip() else "FAIL",
            "Versioning",
            "CHANGELOG.md %s" % ("ok" if ch and ch.strip() else "fehlt/leer"),
        )
    else:
        a.add("V-09", "SKIP", "Versioning", "nicht anwendbar < R2")

    # V-10 REPOSITORY_STANDARD.md
    rs = read(os.path.join(repo, "docs/REPOSITORY_STANDARD.md"))
    if lvl_ge("R1", level):
        a.add(
            "V-10",
            "PASS" if rs and rs.strip() else "FAIL",
            "Documentation",
            "docs/REPOSITORY_STANDARD.md %s" % ("ok" if rs and rs.strip() else "fehlt"),
        )
    else:
        a.add("V-10", "SKIP", "Documentation", "nicht anwendbar R0")

    # V-11 Hygiene
    bad = [
        f
        for f in files
        for b in BAD_PATHS
        if (b.startswith("/") and f.endswith(b[1:]))
        or f == b.rstrip("/")
        or f.startswith(b.rstrip("/") + "/")
    ]
    a.add(
        "V-11",
        "PASS" if not bad else "FAIL",
        "Security",
        "Hygiene ok" if not bad else "verbotene Pfade: %s" % ", ".join(sorted(set(bad))[:5]),
    )

    # V-12 tests + CI
    if lvl_ge("R1", level):
        wf_dir = os.path.join(repo, ".github/workflows")
        wf_files = [
            f for f in files if f.startswith(".github/workflows") and f.endswith((".yml", ".yaml"))
        ] or [
            f
            for f in (os.listdir(wf_dir) if os.path.isdir(wf_dir) else [])
            if f.endswith((".yml", ".yaml"))
        ]
        has_tests = (
            any(("test" in f.lower()) for f in files)
            or os.path.exists(os.path.join(repo, "tests"))
            or os.path.isdir(os.path.join(repo, "tools"))
        )
        has_ci = bool(wf_files)
        ok = has_tests and has_ci
        a.add(
            "V-12",
            "PASS" if ok else "FAIL",
            "Testing",
            "Tests %s, CI %s" % ("ok" if has_tests else "FEHLT", "ok" if has_ci else "FEHLT"),
        )
    else:
        a.add("V-12", "SKIP", "Testing", "nicht anwendbar R0")

    # V-13 ADR/decisions
    if lvl_ge("R2", level):
        has_adr = any(f.startswith("docs/decisions/") for f in files)
        refs = any(
            "DECISIONS_REGISTER" in (read(os.path.join(repo, f)) or "")
            for f in files
            if f.endswith((".md", ".yaml")) and f.count("/") <= 1
        )
        a.add(
            "V-13",
            "PASS" if (has_adr or refs) else "FAIL",
            "Documentation",
            "ADR-Pflicht %s" % ("erfuellt (lokal/zentral)" if (has_adr or refs) else "OFFEN"),
        )
    else:
        a.add("V-13", "SKIP", "Documentation", "nicht anwendbar < R2")

    # V-14 Badge
    if lvl_ge("R3", level):
        ok = "ATC COMPLIANCE" in readme
        a.add(
            "V-14",
            "PASS" if ok else "FAIL",
            "Documentation",
            "Compliance-Badge %s" % ("ok" if ok else "fehlt im README"),
        )
    elif lvl_ge("R2", level):
        a.add(
            "V-14",
            "WARN" if "ATC COMPLIANCE" not in readme else "PASS",
            "Documentation",
            "Badge empfohlen ab R2",
        )
    else:
        a.add("V-14", "SKIP", "Documentation", "nicht anwendbar < R2")

    # V-15 Registry
    reg_ok = None
    if registry_path and os.path.exists(registry_path):
        reg = read(registry_path) or ""
        m = re.search(r"-\s*\{?\s*name:\s*%s\s*[,}]" % re.escape(name), reg)
        reg_ok = bool(m)
    if reg_ok is None:
        a.add(
            "V-15",
            "WARN",
            "Documentation",
            "Registry-Check uebersprungen (kein --registry gegeben)",
        )
    else:
        a.add(
            "V-15",
            "PASS" if reg_ok else "FAIL",
            "Documentation",
            "Registry-Eintrag %s" % ("ok" if reg_ok else "fehlt fuer %s" % name),
        )

    # V-16 Conventional Commits (F-048/SCR-0039: Shallow-Clone-Guard —
    # flache Historie <= 2 Commits ist fuer die 20-Commit-Bewertung unzureichend
    # und darf kein MUST-FAIL erzeugen, nur WARN mit Reparaturhinweis.)
    if lvl_ge("R2", level):
        log = subprocess.run(
            ["git", "log", "--oneline", "-20"], cwd=repo, capture_output=True, text=True
        ).stdout.splitlines()
        subs = [ln.split(" ", 1)[-1].strip() for ln in log if ln.strip()]
        # SCR-0115: Merge-Commits unterliegen nicht dem Conventional-Commits-Format
        # und werden aus der Stichprobe ausgenommen (Spec: Merge-Commits ausgenommen).
        subs = [s for s in subs if not s.startswith("Merge ")]
        if len(subs) <= 2:
            a.add(
                "V-16",
                "WARN",
                "CI/CD",
                "Historie zu flach (%d Commit(s)) — fetch-depth: 0 im Workflow verwenden (F-048/SCR-0039)"
                % len(subs),
            )
        else:
            frac = sum(1 for s in subs if CC_RE.match(s)) / max(1, len(subs)) if subs else 0
            if frac >= 0.8:
                a.add(
                    "V-16",
                    "PASS",
                    "CI/CD",
                    "Conventional Commits: %d%% der letzten %d" % (frac * 100, len(subs)),
                )
            else:
                a.add(
                    "V-16",
                    "WARN" if frac >= 0.5 else "FAIL",
                    "CI/CD",
                    "Conventional Commits nur %d%% (<80%%)" % (frac * 100),
                )
    else:
        a.add("V-16", "SKIP", "CI/CD", "nicht anwendbar < R2")

    return a


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("repo")
    ap.add_argument(
        "--level", default=None, help="R-Level-Override (sonst aus .atc/repository.yaml)"
    )
    ap.add_argument("--registry", default=None)
    args = ap.parse_args()
    repo = os.path.abspath(args.repo)
    if not os.path.isdir(repo):
        print("FEHLER: %s ist kein Verzeichnis" % repo)
        sys.exit(2)
    level = args.level
    if not level:
        meta = read(os.path.join(repo, ".atc/repository.yaml")) or ""
        m = re.search(r"maturity:\s*(R[0-4])", meta)
        level = m.group(1) if m else "R1"
    reg = args.registry or os.path.join(
        os.path.dirname(os.path.abspath(__file__)), "../../registry/repositories.yaml"
    )
    a = audit_repo(repo, level, reg)
    meta1 = read(os.path.join(repo, ".atc/repository.yaml")) or ""
    m1 = re.search(r"name:\s*([A-Za-z0-9_-]+)", meta1)
    anzeige = m1.group(1) if m1 else os.path.basename(repo)

    print("ATC Repository Audit")
    print("=" * 60)
    print("Repository: %s   Level: %s   Standard: ATC-STD-201 v1.0.0" % (anzeige, level))
    print()
    for regel, status, kat, msg in a.results:
        print("[%s] %s (%s): %s" % (status, regel, kat, msg))
    score = a.score()
    print()
    print("RESULT")
    print("=" * 60)
    st = (
        "EXCELLENT"
        if score >= 95
        else "COMPLIANT"
        if score >= 85
        else "CONDITIONAL"
        if score >= 70
        else "NON-COMPLIANT"
        if score >= 50
        else "CRITICAL"
    )
    fails = sum(1 for r in a.results if r[1] == "FAIL")
    print("%s COMPLIANT · Score: %d/100 [%s] · MUST-FAILs: %d" % (level, score, st, fails))
    print("GATE: %s" % a.gate(level))
    sys.exit(0 if a.gate(level) == "PASS" else 1)


if __name__ == "__main__":
    main()
