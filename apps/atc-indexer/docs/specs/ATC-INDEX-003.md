---
spec_id: ATC-INDEX-003
title: "Datenmodell & Idempotenz"
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

# Datenmodell & Idempotenz (ATC-INDEX-003)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Versioniertes Datenmodell, eindeutige Schlüssel.

## 2. Scope
- storage

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Kerntabellen blocks/transactions/receipts/events/logs/contracts/accounts/state_changes/validators/consensus_events/indexer_state mit schema_version **[Nachweis: design]**
- **REQ-002:** Eindeutige Schlüssel: (chain_id, block_hash), (chain_id, height, canonical), (tx_hash), (tx_hash, index); Doppel-Lieferung nie doppelter Zustand **[Nachweis: unit+property]**

## 4. Invarianten
- Indexing ist idempotent

## 5. Conformance-Tests (Mindestkategorien)
- Doppelte Block-Lieferung
- Tx-/Event-Duplikat

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit INDEX P1-7/9

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
