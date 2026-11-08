---
spec_id: ATC-MIN-004
title: "Reward & Double-Claim-Schutz"
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

# Reward & Double-Claim-Schutz (ATC-MIN-004)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Reward über Konsens-Verifikation (SC-018), nie Miner-Autorität.

## 2. Scope
- rewards, SC-018

## 3. Normative Anforderungen (MUST)
- **REQ-001:** CLAIM_ID-Automat: UNKNOWN→ACCEPT, PENDING→REJECT_DUPLICATE, ACCEPTED→REJECT_DUPLICATE, INVALID→REJECT **[Nachweis: unit+negative]**
- **REQ-002:** Kanonische Claim-ID: H(block_hash, miner_identity, proof_hash, nonce, chain_id, height) **[Nachweis: vector]**
- **REQ-003:** Settlement: Konsens verifiziert → Block-Akzeptanz → kanonischer Reward → Claim **[Nachweis: design]**

## 4. Invarianten
- Kein Claim zweimal akzeptiert

## 5. Conformance-Tests (Mindestkategorien)
- Duplicate → reject
- Replay → reject
- Atomic transition

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit MIN-P1-006/007 · ATC-STD-SC-018

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
