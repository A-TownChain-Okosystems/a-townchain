#!/usr/bin/env python3
"""meta_data_audit.py — Metadaten-Vollstaendigkeits-Audit (SCR-0050, Owner-Weisung 08.09.).

Prueft JEDE Standard-Datei gegen schemas/standard.schema.yaml (ATC-STD-000 §8):
  MD-A  YAML-Frontmatter vorhanden
  MD-B  10 Pflichtfelder (id, title, version, status, category, authority,
        owner, created, updated, normative) vorhanden
  MD-C  Wertvalidierung (SemVer, Status-Enum, ISO-Datum, Boolean)
  MD-D  Frontmatter-ID == Dateiname == Registry-ID
  MD-E  Kategorie-Konsistenz gegen Registry-Kategorien
  MD-F  Erweiterte Felder (applies_to, effective_date, review_date, license,
        supersedes, superseded_by) — Coverage-KPI
"""

import collections
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
REQUIRED = [
    "id",
    "title",
    "version",
    "status",
    "category",
    "authority",
    "owner",
    "created",
    "updated",
    "normative",
]
EXTENDED = ["applies_to", "effective_date", "review_date", "license", "supersedes", "superseded_by"]
STATUS_ENUM = {
    "idea",
    "proposed",
    "draft",
    "review",
    "candidate",
    "approved",
    "stable",
    "deprecated",
    "retired",
}


def parse_fm(c):
    m = re.match(r"^---\s*\n(.*?)\n-{3,}\s*\n", c, re.S)  # Fence: --- oder ---- (Repo-Konvention)
    if not m:
        return None
    body = m.group(1)
    # Verschachtelung: Felder koennen unter 'standard:' liegen
    lines = body.splitlines()
    out_lines, nested = [], False
    for ln in lines:
        if re.match(r"^standard:\s*$", ln):
            nested = True
            continue
        if nested and ln.strip():
            out_lines.append(ln)
        elif not nested and not ln.startswith(" "):
            out_lines.append(ln)
    body = "\n".join(out_lines)
    fm = {}
    for line in body.splitlines():
        m2 = re.match(r"^\s*(\w[\w-]*):\s*(.*)$", line)
        if m2:
            fm[m2.group(1)] = m2.group(2).strip().strip('"')
    return fm


def main():
    reg_ids = set(
        re.findall(
            r"- \{id: (ATC-[A-Z0-9-]+),",
            open(os.path.join(ROOT, "registry", "standards.yaml")).read(),
        )
    )
    reg_cats = set(
        re.findall(
            r"category: ([\w-]+)", open(os.path.join(ROOT, "registry", "standards.yaml")).read()
        )
    )
    findings = []
    stats = collections.Counter()
    per_field = collections.Counter()
    total = 0
    for fam in sorted(os.listdir(os.path.join(ROOT, "standards"))):
        fdir = os.path.join(ROOT, "standards", fam)
        if not os.path.isdir(fdir):
            continue
        for fn in sorted(os.listdir(fdir)):
            if not fn.endswith(".md"):
                continue
            if fn == "INDEX.md":
                continue  # generierter Familien-Navigationsindex (MAINT-000 §25), kein Standard
            total += 1
            path = os.path.join(fdir, fn)
            sid_file = fn.replace(".md", "")
            c = open(path, encoding="utf-8").read()
            fm = parse_fm(c)
            if fm is None:
                stats["MD-A kein Frontmatter"] += 1
                findings.append(f"MD-A {fn}: kein YAML-Frontmatter (alle 10 Pflichtfelder fehlen)")
                per_field["ohne_frontmatter"] += 1
                continue
            stats["mit_frontmatter"] += 1
            for f in REQUIRED:
                if f in fm:
                    per_field[f] += 1
                else:
                    per_field[f + "_FEHLT"] += 1
                    findings.append(f"MD-B {fn}: Pflichtfeld '{f}' fehlt")
            for f in EXTENDED:
                if f in fm:
                    per_field["ext_" + f] += 1
            # Wertvalidierung
            if fm.get("version") and not re.match(r"^\d+\.\d+\.\d+$", fm["version"]):
                findings.append(f"MD-C {fn}: Version '{fm['version']}' kein SemVer")
            if fm.get("status") and fm["status"] not in STATUS_ENUM:
                findings.append(f"MD-C {fn}: Status '{fm['status']}' ausserhalb Schema-Enum")
            if fm.get("normative") and fm["normative"] not in ("true", "false"):
                findings.append(f"MD-C {fn}: normative '{fm['normative']}' kein Boolean")
            for df in ("created", "updated"):
                if fm.get(df) and not re.match(r"^\d{4}-\d{2}-\d{2}$", fm[df]):
                    findings.append(f"MD-C {fn}: {df} '{fm[df]}' kein ISO-Datum")
            # ID-Konsistenz
            if fm.get("id"):
                if fm["id"] != sid_file:
                    findings.append(f"MD-D {fn}: Frontmatter-ID '{fm['id']}' != Dateiname")
                if fm["id"] not in reg_ids:
                    findings.append(f"MD-D {fn}: '{fm['id']}' nicht in Registry")
            if fm.get("category") and fm["category"] not in reg_cats:
                findings.append(
                    f"MD-E {fn}: Kategorie '{fm['category']}' nicht in Registry-Kategorien"
                )
    print(f"Metadaten-Audit ueber {total} Standard-Dateien\n")
    print(
        f"Frontmatter: {stats['mit_frontmatter']}/{total} · ohne: {stats['MD-A kein Frontmatter']}"
    )
    print("\nPflichtfeld-Abdeckung (bei Dateien MIT Frontmatter):")
    nf = stats["mit_frontmatter"] or 1
    for f in REQUIRED:
        ok = per_field[f]
        print(f"  {f:<12} {ok:>4}/{nf}")
    print("\nErweiterte Felder (Coverage-KPI):")
    for f in EXTENDED:
        print(f"  {f:<14} {per_field['ext_' + f]:>4}/{total}")
    print(f"\nFunde gesamt: {len(findings)}")
    for x in findings[:30]:
        print(" ", x)
    if len(findings) > 30:
        print(f"  ... +{len(findings) - 30} weitere")
    # CI-Gate: Funde schlagen den Workflow fehl (Governance-CI, SCR-0051)
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
