---
spec_id: ATC-INTEROP-005
title: "Asset-Conservation-Invariante"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementation PENDING
repository: atc-interop
layer: L5
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0072
depends: [ATC-STD-000, ATC-STD-PROTOCOL-001]
---

# Asset-Conservation-Invariante (ATC-INTEROP-005)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Mathematische Asset-Erhaltung der Bridge.

## 2. Scope
- contracts, accounting

## 3. Normative Anforderungen (MUST)
- **REQ-001:** total_locked(source,asset) − total_released(source,asset) = total_minted(target,asset) − total_burned(target,asset) **[Nachweis: property+unit]**
- **REQ-002:** Abweichung → Fail-Closed (keine weiteren Transfers) **[Nachweis: negative]**

## 4. Invarianten
- Locked−Released ≡ Minted−Burned, jederzeit, je Asset

## 5. Conformance-Tests (Mindestkategorien)
- Property-Test-Sequenzen
- Deviation → freeze

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit INTEROP P1-8

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
