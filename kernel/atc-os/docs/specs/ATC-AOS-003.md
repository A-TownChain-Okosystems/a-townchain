---
spec_id: ATC-AOS-003
title: "Version- & Release-Date-Policy"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementation PENDING
repository: a-townchain-os
layer: L7
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0072
depends: [ATC-STD-000, ATC-STD-PROTOCOL-001]
---

# Version- & Release-Date-Policy (ATC-AOS-003)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Eine kanonische Versionsquelle; keine starr datierten Mainnet-Claims.

## 2. Scope
- Release-Governance

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Kanonische Version: .atc/repository.yaml version-Feld (SCR-0072); README/STATUS/CHANGELOG/Tag leiten ab **[Nachweis: design+audit]**
- **REQ-002:** Kein festes Launch-Datum in Metadaten; Release evidence-gated (F-069 Owner-Entscheidung ausstehend) **[Nachweis: governance]**
- **REQ-003:** About/Banner aus kanonischer Quelle abgeleitet **[Nachweis: audit]**

## 4. Invarianten
- Nur eine Versionsaussage je Zeitpunkt, aus einer Quelle

## 5. Conformance-Tests (Mindestkategorien)
- Version-Drift-Check im Governance-CI

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit AOS P0-002/003

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
