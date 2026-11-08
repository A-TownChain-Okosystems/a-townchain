#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
ATC Registry Records Generator (ATC-STD-002 §6) — generiert normalisierte
per-Standard-Records registry/standards/<ID>.yaml aus der Allokations-SSOT
registry/standards.yaml.

Datenmodell je Record (family_id != standard_id, ATC-STD-002 §3):
  family_id / family / standard_id / title / status / version / category /
  file / mandate

Praefix-Familien (ERR-, ZKP-, ...) und Legacy-Serien: family_id: PREFIX/LEGACY
(grandfathered, ATC-STD-002 §3). Generierte Dateien: NICHT manuell pflegen.
"""

import os
import re

import yaml

BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
FAMS = {
    0: "GOVERNANCE / META",
    1: "ARCHITECTURE",
    2: "REPOSITORY / GIT",
    3: "SOFTWARE ENGINEERING",
    4: "SECURITY",
    5: "DATA / STORAGE",
    6: "BLOCKCHAIN / PROTOCOL",
    7: "AI / AGENTS",
    8: "OS / RUNTIME",
    9: "INTEGRATION / INTEROPERABILITY",
}


def main():
    src = os.path.join(BASE, "registry", "standards.yaml")
    out_dir = os.path.join(BASE, "registry", "standards")
    os.makedirs(out_dir, exist_ok=True)
    data = yaml.safe_load(open(src, encoding="utf-8"))
    n = 0
    for s in data["standards"]:
        sid = s["id"]
        m = re.match(r"^ATC-STD-(\d{3})$", sid)
        if m:
            fam_h = int(m.group(1)[0])
            family_id, family = f"{fam_h}00", FAMS[fam_h]
        elif sid.startswith("ATC-STD-"):
            family_id, family = "PREFIX", sid.replace("ATC-STD-", "").split("-")[0]
        else:
            family_id, family = "LEGACY", "LEGACY-SERIES"
        rec = {
            "registry_version": 1,  # Record-Format-Version, unabhaengig von der Standard-Version (MAINT-000 §22.1)
            "family_id": family_id,
            "family": family,
            "standard_id": sid,
            "title": s.get("title", ""),
            "status": s.get("status", ""),
            "version": s.get("version", ""),
            "category": s.get("category", ""),
            "file": s.get("file", ""),
            "mandate": s.get("mandate", ""),
        }
        fn = os.path.join(out_dir, f"{sid}.yaml")
        with open(fn, "w", encoding="utf-8") as f:
            f.write(
                "# GENERIERT aus registry/standards.yaml (Allokations-SSOT) — nicht manuell pflegen\n"
            )
            f.write("# ATC-STD-002 §3/§6: family_id != standard_id\n")
            yaml.safe_dump(rec, f, allow_unicode=True, sort_keys=True, default_flow_style=False)
        n += 1
    print(f"✓ {n} Standard-Records generiert -> registry/standards/")


if __name__ == "__main__":
    main()
