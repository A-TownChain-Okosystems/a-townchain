---
spec_id: ATC-INTEROP-003
title: "Proof- & Finality-Verifikation"
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

# Proof- & Finality-Verifikation (ATC-INTEROP-003)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Je Source-Chain ein Finality-Adapter.

## 2. Scope
- proofs, adapters

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Je Quelle (ATC deterministisch, Ethereum probabilistisch, IBC Consensus, Solana Commitment) ein Adapter mit dokumentierter Semantik **[Nachweis: design]**
- **REQ-002:** Proof-Format, Serialisierung, Hashing, Signatur-Verifikation, Quorum spezifiziert und getestet **[Nachweis: unit+vector]**
- **REQ-003:** Generic Header → accepted ist verboten **[Nachweis: negative]**

## 4. Invarianten
- Unsichere Finality → REJECT

## 5. Conformance-Tests (Mindestkategorien)
- Wrong chain/epoch → reject
- Quorum fehlt → reject

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit INTEROP P0-1/P0-4

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
