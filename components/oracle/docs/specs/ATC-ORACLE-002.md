---
spec_id: ATC-ORACLE-002
title: "Aggregation & Median-Konsens"
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

# Aggregation & Median-Konsens (ATC-ORACLE-002)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Deterministische Multi-Source-Aggregation.

## 2. Scope
- Aggregation, Feeds

## 3. Normative Anforderungen (MUST)
- **REQ-001:** min_source_count und max_deviation sind config-gelockte Parameter **[Nachweis: config]**
- **REQ-002:** Pipeline: invalid→reject, stale→reject, outlier→quarantine, Quorum-Check, Median mit deterministischer u64-Fixpoint-Rundung **[Nachweis: unit+vector]**
- **REQ-003:** Weighted Median optional; Gewichte registriert und versioniert **[Nachweis: config]**

## 4. Invarianten
- Quorum unterschritten → kein Resultat (fail-closed)
- Median byte-identisch reproduzierbar

## 5. Conformance-Tests (Mindestkategorien)
- Median-Vector
- Outlier-Quarantäne
- Overflow/Rounding

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit P1-ORACLE-002

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
