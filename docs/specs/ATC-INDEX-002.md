---
spec_id: ATC-INDEX-002
title: "Reorg- & Finality-Handling"
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

# Reorg- & Finality-Handling (ATC-INDEX-002)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Explizites Reorg-Modell.

## 2. Scope
- canonical chain

## 3. Normative Anforderungen (MUST)
- **REQ-001:** REORG DETECTED → ROLLBACK/COMPENSATE → REINDEX → CANONICAL RESTORED **[Nachweis: design+integration]**
- **REQ-002:** FINALIZED-Blöcke roll-back-geschützt **[Nachweis: negative]**
- **REQ-003:** indexer_state: last_seen_height/hash, last_finalized, schema_version, indexer_version **[Nachweis: unit]**

## 4. Invarianten
- Nach Reorg ist die kanonische Kette vollständig wiederhergestellt

## 5. Conformance-Tests (Mindestkategorien)
- Reorg-Simulation
- Finalized-Protection
- Restart-Recovery

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit INDEX P0-6

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
