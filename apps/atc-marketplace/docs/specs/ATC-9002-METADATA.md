---
spec_id: ATC-9002
title: "NFT Metadata Specification"
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

# NFT Metadata Specification (ATC-9002)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Metadaten-Standard für NFTs (JSON-Schema, Storage-Referenz).

## 2. Scope (gilt für)

- Schema-Pflichtfelder
- Größenlimits
- Off-chain-Referenz

## 3. Normative Anforderungen (MUST)

- **REQ-A92-001:** Metadaten sind JSON nach Schema: name, description, image_cid, attributes[]; Zirkelreferenzen verboten (Schema-validierung verpflichtend) — *Nachweis: unit+negative*
- **REQ-A92-002:** Gesamtgröße ≤ Limit (genesis-locked); größerer Content nur über Storage-Chunk-Verweis (ATC-STOR-004) — *Nachweis: negative*
- **REQ-A92-003:** Royalty-Änderung nach Prägung verboten (royalty_bps ist immutable; Manipulation ⇒ Registry-Reject) — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- (in diesem Grundgerüst noch offen)

## 6. Conformance-Tests (Mindestkategorien)

- metadata_schema.json
- oversize_metadata ⇒ Reject
- royalty_mutation ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (MKT-P1-003: royalty manipulation)
