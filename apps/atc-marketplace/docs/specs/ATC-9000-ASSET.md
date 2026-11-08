---
spec_id: ATC-9000
title: "ATC-9000 Asset Standard (NFT/Fungible Basisnorm)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-marketplace
layer: L5-Marketplace
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# ATC-9000 Asset Standard (NFT/Fungible Basisnorm) (ATC-9000)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Verbindliche Basisklasse aller ATC-Assets: einheitliche IDs, Metadaten-Bindung und Lifecycle, gegen die atc-contracts, atc-wallet, atc-indexer und atc-explorer implementieren.

## 2. Scope (gilt für)

- Asset-Kernobjekt
- Übergreifende Lifecycle-Regeln
- Rollout-Kompatibilität

## 3. Normative Anforderungen (MUST)

- **REQ-A9K-001:** Asset-Objekt: {asset_id, asset_type (NFT|FUNGIBLE), owner_did, metadata_cid (ATC-STOR-002), royalty_bps, immutable_flag, created_height} — kanonische Serialisierung verpflichtend — *Nachweis: unit+vector*
- **REQ-A9K-002:** asset_id ist eindeutig und unveränderlich; Ownership-Änderungen erfolgen ausschließlich über ATC-9004-Transfer mit Protocol-Evidence — *Nachweis: unit+negative*
- **REQ-A9K-003:** Jede zustandsändernde Operation ist deterministisch, Integer-only (checked), mit eindeutigem Fehlercode — *Nachweis: unit+property*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Metadata-CID und immutable_flag sind nach Prägung unveränderbar (Mutation ⇒ eigene Asset-Version, nie In-Place)

## 6. Conformance-Tests (Mindestkategorien)

- asset_vectors.json
- immutability_violation ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (ATC-9000-Familie)
- atc-contracts (Implementierung)
