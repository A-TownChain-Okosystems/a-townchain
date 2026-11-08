---
spec_id: ATC-INDEX-004
title: "API-Contract"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementation PENDING
repository: atc-indexer
layer: L5
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0072
depends: [ATC-STD-000, ATC-STD-PROTOCOL-001]
---

# API-Contract (ATC-INDEX-004)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Formaler Query-API-Contract.

## 2. Scope
- API

## 3. Normative Anforderungen (MUST)
- **REQ-001:** GET /blocks/{height|hash}, /transactions/{hash}, /events, /accounts/{address}, /validators, /metrics — mit Pagination, Cursor, Ordering, Finality-Semantik, Fehlercodes, Schema-Version **[Nachweis: design+integration]**
- **REQ-002:** Canonical-only-Queries und finality-annotierte Antworten Pflicht **[Nachweis: unit]**

## 4. Invarianten
- Responses konsistent mit kanonischem Stand

## 5. Conformance-Tests (Mindestkategorien)
- Pagination
- Missing data → 404+code

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit INDEX P2-15/16

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
