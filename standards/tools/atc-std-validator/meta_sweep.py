#!/usr/bin/env python3
"""meta_sweep.py — Meta-Compliance: Standards ueber Standards (Owner-Weisung 08.09.).

Wendet die uebergreifenden Standards auf JEDE Standard-Datei an:
  MS-1  REQ-ID-Format (Naming-Schema: REQ-[A-Z]+-[0-9]{3})
  MS-2  Implementierungsstatus-Sektion (Pivot-Doktrin: SPECIFIED..ENFORCED)
  MS-3  Security-Considerations-Sektion (ATC-STD-000 §38)
  MS-4  Changelog-Sektion
  MS-5  References-Sektion
  MS-6  Referenz-Aufloesung: genannte ATC-STD-IDs existieren in der Registry
  MS-7  Kategorie-Konsistenz: Frontmatter-Kategorie ist in categories.yaml
KPI je Familie und Gesamt; Funde = Fix-Liste. Legacy-Files: KPI, neue Familien: Pflicht.
"""

import collections
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


def registry_ids():
    ids = set()
    cats = set()
    for line in open(os.path.join(ROOT, "registry", "standards.yaml"), encoding="utf-8"):
        m = re.search(r"id: (ATC-[A-Z0-9-]+),", line)
        if m:
            ids.add(m.group(1))
        m2 = re.search(r"category: (\w+)", line)
        if m2:
            cats.add(m2.group(1))
    return ids, cats


def main():
    ids, cats = registry_ids()
    per_family = collections.defaultdict(lambda: collections.Counter())
    findings = []
    total = 0
    std_dir = os.path.join(ROOT, "standards")
    for fam in sorted(os.listdir(std_dir)):
        fdir = os.path.join(std_dir, fam)
        if not os.path.isdir(fdir):
            continue
        for fn in sorted(os.listdir(fdir)):
            if not fn.endswith(".md"):
                continue
            total += 1
            path = os.path.join(fdir, fn)
            c = open(path, encoding="utf-8").read()
            f = per_family[fam]
            f["total"] += 1
            # MS-1 REQ-Format
            reqs = re.findall(r"REQ-[A-Za-z]+-\d+", c)
            bad = [r for r in reqs if not re.match(r"^REQ-[A-Z]+-\d{3}$", r)]
            f["ms1_req_ok" if not bad else "ms1_req_bad"] += 1
            if bad:
                findings.append(f"MS-1 {fn}: REQ-Formatfehler {sorted(set(bad))[:3]}")
            # MS-2 Implementierungsstatus
            if re.search(r"Implementierungsstatus", c, re.I):
                f["ms2_implstatus"] += 1
            else:
                findings.append(f"MS-2 {fn}: keine Implementierungsstatus-Sektion")
            # MS-3 Security Considerations
            if re.search(r"Security Considerations", c, re.I):
                f["ms3_sec"] += 1
            else:
                findings.append(f"MS-3 {fn}: keine Security-Considerations-Sektion")
            # MS-4 Changelog
            if re.search(r"## Changelog|Changelog", c):
                f["ms4_changelog"] += 1
            else:
                findings.append(f"MS-4 {fn}: keine Changelog-Sektion")
            # MS-5 References
            if re.search(r"## References", c):
                f["ms5_refs"] += 1
            else:
                findings.append(f"MS-5 {fn}: keine References-Sektion")
            # MS-6 Referenz-Aufloesung (nur References-Sektion)
            m = re.search(r"## References\s*\n(.*?)(\n## |\Z)", c, re.S)
            if m:
                for ref in set(re.findall(r"ATC-STD-[A-Z0-9-]+", m.group(1))):
                    if re.match(r"ATC-STD-[A-Z]+-\d+-999$", ref):
                        continue  # Range-Notation (Familien-Range), keine Standard-ID
                    if ref not in ids:
                        findings.append(f"MS-6 {fn}: Referenz {ref} nicht in Registry")
            # MS-7 Kategorie
            m = re.search(r"category: (\w+)", c)
            if m and m.group(1) in cats:
                f["ms7_cat_ok"] += 1
            elif m:
                findings.append(f"MS-7 {fn}: Kategorie '{m.group(1)}' nicht in categories.yaml")

    print(f"Meta-Sweep ueber {total} Standard-Dateien ({len(per_family)} Familien)\n")
    print(
        f"{'Familie':<22} {'Dateien':>7} {'REQ':>4} {'ImplSt':>6} {'Sec':>4} {'Chlog':>5} {'Refs':>5} {'Kat':>4}"
    )
    for fam, f in sorted(per_family.items()):
        t = f["total"]
        print(
            f"{fam:<22} {t:>7} {f['ms1_req_ok']:>4} {f['ms2_implstatus']:>6} {f['ms3_sec']:>4} {f['ms4_changelog']:>5} {f['ms5_refs']:>5} {f['ms7_cat_ok']:>4}"
        )
    print(f"\nFunde gesamt: {len(findings)}")
    for x in findings:
        print(" ", x)

    # KPI-Zusammenfassung
    allc = sum((f for f in per_family.values()), collections.Counter())
    print(
        f"\nKPI: Implementierungsstatus {allc['ms2_implstatus']}/{total} · Security {allc['ms3_sec']}/{total}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
