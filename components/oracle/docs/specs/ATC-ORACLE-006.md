---
spec_id: ATC-ORACLE-006
title: "Fallback, Emergency & Fail-Closed"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementation PENDING
repository: atc-oracle
layer: L5
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0072
depends: [ATC-STD-000, ATC-STD-PROTOCOL-001]
---

# Fallback, Emergency & Fail-Closed (ATC-ORACLE-006)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Unsicherheit führt nie zur Ausführung.

## 2. Scope
- F/E-Policy, Gesamtsystem

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Fail-Closed-REJECT-Liste: invalid, unknown source, quorum fehlt, stale, clock violation, malformed **[Nachweis: negative]**
- **REQ-002:** Emergency Pause mit dokumentierter Recovery-Prozedur; Fallback-Quellen im selben Trust Model **[Nachweis: design+config]**

## 4. Invarianten
- Im Emergency-Zustand werden keine Results geliefert

## 5. Conformance-Tests (Mindestkategorien)
- All stale → no result
- Fallback activation
- Pause + Recovery

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit P0-ORACLE-003

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
