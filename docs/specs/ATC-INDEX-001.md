---
spec_id: ATC-INDEX-001
title: "Chain-Source & Ingestion"
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

# Chain-Source & Ingestion (ATC-INDEX-001)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Normative Node-Schnittstelle und Ingestion-Stufen.

## 2. Scope
- ingestion, ETL

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Endpoint/RPC/WS, Block-/Tx-/Event-Schema, Finality, Confirmation-Depth, Genesis-/Halt-Handling spezifiziert **[Nachweis: design+vector]**
- **REQ-002:** Stufen OBSERVED→VALIDATED→CANONICAL→FINALIZED; kein Direkt-INSERT bei new block **[Nachweis: design+unit]**
- **REQ-003:** Malformed/Duplicate-Handling definiert **[Nachweis: negative]**

## 4. Invarianten
- Indexer erzeugt keine alternative Chain-Truth

## 5. Conformance-Tests (Mindestkategorien)
- Duplicate → idempotent
- Parent mismatch → reject

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit INDEX P0 · Chain-ID 658467

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
