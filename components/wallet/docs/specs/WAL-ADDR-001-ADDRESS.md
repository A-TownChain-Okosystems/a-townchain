---
spec_id: WAL-ADDR-001
title: "Address Encoding Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-wallet
layer: L5-Wallet
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Address Encoding Specification (WAL-ADDR-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Eindeutige, prüfsummierte ATC-Adressdarstellung.

## 2. Scope (gilt für)

- Payload (Pubkey-Hash)
- Checksum
- Base58-/human-readable Encoding

## 3. Normative Anforderungen (MUST)

- **REQ-WA-001:** address_payload = RIPEMD160(SHA-256(pubkey_compressed)) (20 Byte) — kanonisch, keine Alternativ-Hashes — *Nachweis: unit+vector*
- **REQ-WA-002:** Checksum: erste 4 Byte von SHA-256(SHA-256(version || payload)); version byte genesis-locked (ATC-NETWORK-ID-001) — *Nachweis: vector+negative*
- **REQ-WA-003:** Human-Encoding: Base58 (Bitcoin-Alphabet) von (version||payload||checksum); Netzen (mainnet/testnet) unterscheiden sich im version byte — *Nachweis: unit+negative*
- **REQ-WA-004:** Ungültige Checksummen, falsche Länge oder ungültige Zeichen ⇒ Decode-Fehler (kein silent accept) — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- 1:1-Abbildung Adresse↔PubKey-Hash im Gültigkeitsbereich

## 6. Conformance-Tests (Mindestkategorien)

- address_vectors.json
- invalid_address.json (T: invalid address)
- checksum_corruption.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- a-townchain Chain-ID/Netz-Spezifikation
