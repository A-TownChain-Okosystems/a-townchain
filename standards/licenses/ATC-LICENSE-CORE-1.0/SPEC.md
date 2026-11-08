# ATC-LICENSE-CORE-1.0 — Dach-Spezifikation des ATC-Lizenzsystems

**ATC-LIC-CORE-000 · Level: OPEN · Status: ACTIVE · SCR-0037 · 08.09.2026**

## Architektur

License Core (Rechte · Pflichten · Einschränkungen · Durchsetzung) → License Types
(9 registrierte Typen, License-Registry) → SPDX/Metadata (maschinenlesbar, Manifest-
Schema ATC-LICENSE-MANIFEST-1.0) → Compliance Engine (Scanner, PASS/BLOCK) →
Audit System (AUD-Records, MAJOR-Gates).

## Ebenen-Modell (kein Pseudo-Open-Source)

OPEN (ATC-OSS — MUSS OSD-kompatibel sein) · RESTRICTED (PROTOCOL, SOURCE,
COMMERCIAL, ASSET, AI, DATA, EXPERIMENTAL) · PROPRIETARY (PROPRIETARY).

## Bindung an Projekte

Jedes ATC-Projekt deklariert seinen ATC-Lizenztyp über ATC-LICENSE.yaml (Manifest-
Schema); Repos behalten ihre SPDX-Basisschicht (aktuell Apache-2.0, SCR-0036).
Registry, Manifeste und Lizenztexte MUSSEN versionssynchron sein (LICENSE-009).

## Normative Bindung

ATC-STD-LICENSE-001..009 (FAM-44) · License-Registry licenses/LICENSE-REGISTRY.yaml
(SSOT) · Durchsetzung via Compliance Engine (LICENSE-006) und Audit (LICENSE-007).

*Copyright (c) 2026 Michael Wroblewski · Keine Rechtsberatung; externe Prüfung ausstehend.*
