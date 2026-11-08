#!/usr/bin/env python3
"""GENERATOR: Per-Standard-Metadaten (ATC-STD-LIB-001 Phase 1, SCR-0100).

Erzeugt je Registry-Standard eine maschinenlesbare <ID>.metadata.yaml neben der
Standard-Datei. Registry ist SSOT (ATC-STD-003): dieses Tool PROJIZIERT nur —
es sammelt nichts selbst. Manuelle Aenderungen sind verboten und werden von
Pruefregel R13 (Cross-Registry-Test) als Drift abgelehnt.
"""

import os

import yaml

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
REG = os.path.join(ROOT, "registry", "standards.yaml")
DEPS = os.path.join(ROOT, "registry", "dependencies.yaml")
IMPL = os.path.join(ROOT, "registry", "standard-implementation.yaml")
STAMP = "GENERATED-BY tools/gen_views/gen_metadata.py (ATC-STD-LIB-001 sec.6 Phase 1) - NICHT MANUELL BEARBEITEN"


def main():
    reg = yaml.safe_load(open(REG, encoding="utf-8"))
    try:
        deps = (yaml.safe_load(open(DEPS, encoding="utf-8")) or {}).get("standards", {})
    except FileNotFoundError:
        deps = {}
    try:
        impl = {
            e.get("id"): e
            for e in yaml.safe_load(open(IMPL, encoding="utf-8")).get("standards", [])
        }
    except FileNotFoundError:
        impl = {}
    n = 0
    for s in reg.get("standards", []):
        f = s.get("file")
        if not f or not os.path.exists(os.path.join(ROOT, f)):
            continue
        sid = s["id"]
        data = {
            "id": sid,
            "title": s.get("title"),
            "version": str(s.get("version", "")),
            "status": s.get("status"),
            "category": s.get("category"),
            "authority": s.get("authority"),
            "owner": s.get("owner"),
            "normative": bool(s.get("normative", False)),
            "source_file": f,
            "dependencies": sorted(
                (deps.get(sid) or {}).get("depends", [])
                if isinstance(deps.get(sid), dict)
                else (deps.get(sid) or [])
            ),
            "implementation": (impl.get(sid) or {}).get(
                "implementation", {"status": "specification_only"}
            ),
            "conformance": {"required": bool(s.get("normative", False))},
        }
        out = os.path.join(ROOT, os.path.dirname(f), sid + ".metadata.yaml")
        with open(out, "w", encoding="utf-8") as fh:
            fh.write("# " + STAMP + "\n")
            fh.write(
                yaml.safe_dump(data, sort_keys=True, allow_unicode=True, default_flow_style=False)
            )
        n += 1
    print(f"OK gen_metadata: {n} Metadaten-Dateien aus Registry projiziert")


if __name__ == "__main__":
    main()
