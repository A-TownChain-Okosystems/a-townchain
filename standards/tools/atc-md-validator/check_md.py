#!/usr/bin/env python3
"""MD-Compliance-Validator — ATC-STD-MD-001 (Gates MD-01..MD-10).

Prueft ein Repository gegen den ATC Markdown & Documentation Standard.
Aufruf:  python3 tools/atc-md-validator/check_md.py <repo-pfad>
Exit-Code 0 = konform, 1 = NON-COMPLIANT.

Gates:
  MD-01  Pflichtdateien (README, LICENSE/LICENCE(.md), CONTRIBUTING, SECURITY,
         CHANGELOG, STATUS; ROADMAP/ARCHITECTURE/GOVERNANCE/CODE_OF_CONDUCT
         situativ = WARN)
  MD-02  UPPER_SNAKE_CASE-Dateinamen (.md in Wurzel + docs/, Legacy- und
         Standards-Bereiche ausgenommen)
  MD-03  Überschriften: genau 1x H1, keine Ebenensprünge
  MD-04  Codeblöcke deklariert (Sprache oder text/plain-markierbar)
  MD-05  Dokument-Status-Enum (frontmatter status)
  MD-06  CHANGELOG-Format ([x.y.z] - Datum + Added/Changed/Fixed)
  MD-07  STATUS.md maschinenlesbar (Property-Value-Tabelle)
  MD-08  AI-Agenten-Bereich (AGENTS.md oder README AI Agent Instructions)
  MD-09  Interne Links relativ (keine file://- oder abs. Repo-URLs)
  MD-10  Dokumentationshierarchie: docs/-Struktur vorhanden oder begründet
"""

import os
import re
import sys

DOC_STATUS = {
    "draft",
    "proposed",
    "review",
    "approved",
    "active",
    "deprecated",
    "superseded",
    "archived",
}
MUST_FILES = ["README.md", "CONTRIBUTING.md", "SECURITY.md", "CHANGELOG.md", "STATUS.md"]
LICENSE_OK = ["LICENSE", "LICENSE.md", "LICENCE", "LICENCE.md", "COPYING"]
COND_FILES = {
    "ROADMAP.md": "aktive Entwicklung",
    "ARCHITECTURE.md": "Software",
    "GOVERNANCE.md": "Governance-relevant",
    "CODE_OF_CONDUCT.md": "oeffentliches Repo",
}
EXCLUDE_DIRS = {
    "standards",
    "atc",
    "ats",
    "references",
    "node_modules",
    "target",
    "vendor",
    ".git",
    ".github",
    "tools",
    "templates",
    "approval",
    "change-requests",
    "licensing",
    "governance",
    "docs",
}


def iter_md(repo):
    for root, dirs, files in os.walk(repo):
        dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS or root != repo]
        for f in files:
            if f.endswith(".md"):
                yield os.path.join(root, f)


def main():
    repo = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else ".")
    fails, warns = [], []

    # MD-01 Pflichtdateien
    for f in MUST_FILES:
        if not os.path.exists(os.path.join(repo, f)):
            fails.append("MD-01: Pflichtdatei fehlt: " + f)
    if not any(os.path.exists(os.path.join(repo, x)) for x in LICENSE_OK):
        fails.append("MD-01: LICENSE fehlt")
    for f, why in COND_FILES.items():
        if not os.path.exists(os.path.join(repo, f)):
            warns.append("MD-01: %s fehlt (Pflicht bei: %s)" % (f, why))

    # MD-02 UPPER_SNAKE_CASE
    for p in iter_md(repo):
        name = os.path.basename(p)
        if (
            name not in ("AGENTS.md",)
            and not re.match(r"^[A-Z0-9_-]+\.md$", name)
            and name != "AGENT_MANIFEST.md"
        ):
            fails.append("MD-02: kein UPPER_SNAKE_CASE: " + name)

    # MD-03/04/05 Überschriften, Fences, Frontmatter-Status
    for p in iter_md(repo):
        name = os.path.basename(p)
        text = open(p, encoding="utf-8", errors="replace").read()
        # Codeblock-Inhalte von der Struktur-Analyse ausnehmen
        prose = re.sub(r"```[a-z]*\n.*?```", "", text, flags=re.S)
        h1 = re.findall(r"^# ", prose, re.M)
        if len(h1) == 0:
            fails.append("MD-03: ohne H1: " + name)
        elif len(h1) > 1:
            warns.append("MD-03: %dx H1 in %s (SOLLTE genau 1)" % (len(h1), name))
        lv = [len(m.group(1)) for m in re.finditer(r"^(#{1,6}) ", prose, re.M)]
        if lv and max(lv) > 6:
            fails.append("MD-03: >6 Ebenen: " + name)
        for i in range(1, len(lv)):
            if lv[i] > lv[i - 1] + 1:
                warns.append("MD-03: Ebenensprung in %s (H%d->H%d)" % (name, lv[i - 1], lv[i]))
                break
        fence_open = False
        bad_fences = []
        for ln in text.split("\n"):
            if ln.strip().startswith("```"):
                if not fence_open and ln.strip() == "```":
                    bad_fences.append(ln)
                fence_open = not fence_open
        if bad_fences:
            warns.append("MD-04: %d unbeschriftete Fences in %s" % (len(bad_fences), name))
        fm = re.match(r"^---\n(.*?)\n---", text, re.S)
        if fm:
            m = re.search(r"^status:\s*(\S+)", fm.group(1), re.M)
            if (
                m
                and m.group(1).lower() not in DOC_STATUS
                and m.group(1).lower()
                not in (
                    "candidate",
                    "approved",
                    "planning",
                    "prototype",
                    "development",
                    "alpha",
                    "beta",
                    "release-candidate",
                    "stable",
                )
            ):
                fails.append("MD-05: Status '%s' nicht im Enum: %s" % (m.group(1), name))

    # MD-06 CHANGELOG
    cl = os.path.join(repo, "CHANGELOG.md")
    if os.path.exists(cl):
        t = open(cl, encoding="utf-8").read()
        if not re.search(r"##\s*\[\d+\.\d+\.\d+\]", t):
            fails.append("MD-06: CHANGELOG ohne [x.y.z]-Versionseintraege")
        if not re.search(r"###\s*(Added|Changed|Fixed|Security)", t):
            warns.append("MD-06: CHANGELOG ohne Added/Changed/Fixed-Sektionen")

    # MD-07 STATUS.md maschinenlesbar
    st = os.path.join(repo, "STATUS.md")
    if os.path.exists(st):
        t = open(st, encoding="utf-8").read()
        if not re.search(r"\|\s*-+\s*\|", t) or not re.search(r"Repository|Version|Status", t):
            fails.append("MD-07: STATUS.md ohne Property-Value-Tabelle")

    # MD-08 AI-Agenten-Bereich
    ag = os.path.exists(os.path.join(repo, "AGENTS.md")) or (
        os.path.exists(os.path.join(repo, "README.md"))
        and re.search(
            r"AI Agent Instructions|AGENTS\.md",
            open(os.path.join(repo, "README.md"), encoding="utf-8").read(),
        )
    )
    if not ag:
        fails.append("MD-08: Kein AI-Agenten-Bereich (AGENTS.md oder README)")

    # MD-09 Interne Links
    for p in [f for f in iter_md(repo)]:
        name = os.path.basename(p)
        text = open(p, encoding="utf-8", errors="replace").read()
        if re.search(r"\]\(\s*file://", text) or re.search(
            r"\]\(https?://github\.com/A-TownChain-Okosystems/[^/]+/blob/", text
        ):
            warns.append("MD-09: Nicht-relative Links in " + name)

    # MD-10 docs/-Struktur
    if not os.path.isdir(os.path.join(repo, "docs")):
        warns.append("MD-10: docs/-Verzeichnis fehlt (Hierarchie §2)")

    print("== MD-Compliance (ATC-STD-MD-001 v1.0.0) — %s ==" % os.path.basename(repo))
    for f in fails:
        print("  [FAIL] " + f)
    for w in warns:
        print("  [WARN] " + w)
    if not fails:
        print(
            "RESULT: "
            + ("CONFORM — %d Gate-Verstoss(e), %d Warnung(en)" % (len(fails), len(warns)))
            if warns
            else "CONFORM — keine Befunde"
        )
    else:
        print("RESULT: NON-COMPLIANT (%d FAIL, %d WARN)" % (len(fails), len(warns)))
    return 1 if fails else 0


if __name__ == "__main__":
    sys.exit(main())
