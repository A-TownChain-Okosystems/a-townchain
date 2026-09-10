---
spec_id: ATC-ORACLE-005
title: "Feed-Lifecycle"
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

# Feed-Lifecycle (ATC-ORACLE-005)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Zustandsautomat je Feed.

## 2. Scope
- Feed-Management

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Zustände PROPOSED→ACTIVE→DEGRADED→STALE→QUARANTINED→DISABLED→RETIRED; nur definierte Übergänge **[Nachweis: design+negative]**
- **REQ-002:** Zustandswechsel auditiert (Zeit, Auslöser, Grund) **[Nachweis: audit]**

## 4. Invarianten
- RETIERT liefert nie Daten; QUARANTINED liefert nie in die Aggregation

## 5. Conformance-Tests (Mindestkategorien)
- Illegaler Übergang → reject
- Recovery DEGRADED→ACTIVE

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit P1-ORACLE-005

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
