#!/usr/bin/env python3
"""GFF Spec-Gate (CI): Vollstaendigkeit + Integritaet der kanonischen .atc-Specs.

Exit 1 bei jedem Versto (Evidence-First: dieses Gate ist CI-verbindlich).
"""

from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from gff.spec_loader import SpecValidationError, load_specs  # noqa: E402

CORE_AD = 20
FACTORY_ADS = set(range(21, 44))  # AD-21..AD-43 = 23 Factories (11 v1 + 12 v2)
DAO_CORE_FILES = ("factory.atc", "routes.atc")
CONTRACT_FILES = ("contracts/registry.atc", "contracts/revenue.atc")


def main() -> int:
    spec_dir = Path(__file__).resolve().parents[1] / "specs"
    errors: list[str] = []

    specs = {}
    try:
        specs = load_specs(spec_dir)
    except SpecValidationError as e:
        errors.append(str(e))

    # 1) Kern- und Factory-ADs vorhanden
    if CORE_AD not in specs:
        errors.append(f"AD-20 (GFF Core) fehlt in {spec_dir}")
    found_factories = {ad for ad in specs if ad in FACTORY_ADS}
    missing = FACTORY_ADS - found_factories
    extra = set(specs) - FACTORY_ADS - {CORE_AD}
    if missing:
        errors.append(f"Fehlende Factory-Specs: {sorted(f'AD-{a}' for a in missing)}")
    if extra:
        errors.append(f"Unerwartete AD-IDs: {sorted(f'AD-{a}' for a in extra)}")

    # 2) Substanz: jede Spec deklariert struct + pub fn
    for ad in sorted(specs):
        s = specs[ad]
        if not s.has_implementation_surface:
            errors.append(f"AD-{ad:02d} ({s.file}): keine struct/pub fn-Oberflaeche")

    # 3) DAO-Kern + Contracts vorhanden
    for rel in DAO_CORE_FILES + CONTRACT_FILES:
        if not (spec_dir / rel).exists():
            errors.append(f"Kern-/Contract-Datei fehlt: {rel}")

    # 4) Plattform-Trennung: KEINE .atc-Datei darf Genesis Chronicles importieren
    import re as _re

    forbidden = _re.compile(r'import\s+"?[^"\n]*chronicles', _re.IGNORECASE)
    copyright_re = _re.compile(r"Copyright \(c\) 2026")
    for p in sorted(spec_dir.rglob("*.atc")):
        raw = p.read_text(encoding="utf-8")
        if forbidden.search(raw):
            errors.append(f"{p.name}: verbotene Abhaengigkeit 'chronicles' (Plattform-Trennung)")
        if not copyright_re.search(raw):
            errors.append(f"{p.name}: Copyright-Header fehlt")
        # DAO-Kern/Contracts: bewusst KEIN AD-Header erzwungen (keine AD-Factory-Specs)

    print(
        f"Spec-Gate: {len(specs)} AD-Specs geprueft "
        f"({len(found_factories)} Factories AD-21..43, Core AD-20 "
        f"+ DAO-Kern {len(DAO_CORE_FILES)} + Contracts {len(CONTRACT_FILES)})"
    )
    if errors:
        for e in errors:
            print(f"  FAIL: {e}")
        print(f"RESULT: SPEC-GATE FAILED ({len(errors)} Verstoesse)")
        return 1
    print("RESULT: SPEC-GATE PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
