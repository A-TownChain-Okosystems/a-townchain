---
spec_id: ATC-MIN-002
title: "Consens-Grenze (Rule vs. Execution)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementation PENDING
repository: atc-mining
layer: L5
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0072
depends: [ATC-STD-000, ATC-STD-PROTOCOL-001]
---

# Consens-Grenze (Rule vs. Execution) (ATC-MIN-002)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
atc-mining führt aus, definiert nie Regeln.

## 2. Scope
- Gesamt-Repository

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Verboten zu definieren: Block-Validity, Difficulty-Consensus, Reward-Consensus, Emission, Header-Semantik, Chain-Selection, Fork-Regeln **[Nachweis: audit+design]**
- **REQ-002:** Erlaubt: Hashing, Nonce-Suche, Job-Verteilung, Hardware, Shares/Proofs, Telemetrie **[Nachweis: design]**

## 4. Invarianten
- Keine Mining-Entität autorisiert eigenen Reward oder Block-Validity

## 5. Conformance-Tests (Mindestkategorien)
- Regel-Override im Review → BLOCK

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit MIN-P0-003

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
