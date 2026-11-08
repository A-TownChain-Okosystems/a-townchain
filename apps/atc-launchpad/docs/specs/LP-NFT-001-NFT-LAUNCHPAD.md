---
spec_id: LP-NFT-001
title: "NFT Launchpad Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-launchpad
layer: L5-Launchpad
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# NFT Launchpad Specification (LP-NFT-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

NFT-spezifische Sale-Parameter (Collections, Per-Wallet-Limits, Reveal).

## 2. Scope (gilt für)

- Collection-Parameter
- Mint-Autorisierung & Per-Wallet-Limit
- Reveal-Commitment

## 3. Normative Anforderungen (MUST)

- **REQ-LPN-001:** Collection: {collection_id, max_supply, mint_price, per_wallet_limit, metadata_commitment_root} — Parameter nach Sale-Start immutable — *Nachweis: unit+negative*
- **REQ-LPN-002:** Mint erfordert Autorisierung (Whitelist/Public gemäß Sale-State) und erzwingt per_wallet_limit (checked, Overflow-safe) — *Nachweis: unit+adversarial*
- **REQ-LPN-003:** Reveal: Metadata-Commitment (Merkle-Root) vor Sale fixiert; Reveal ist reine Off-Chain-Verifikation gegen den Commitment (kein post-hoc Austausch) — *Nachweis: property+negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Minted ≤ max_supply — in jedem Zwischenzustand

## 6. Conformance-Tests (Mindestkategorien)

- nft_mint_limits.json
- per_wallet_bypass ⇒ Reject
- reveal_tampering ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (M5 NFT Launchpad)
