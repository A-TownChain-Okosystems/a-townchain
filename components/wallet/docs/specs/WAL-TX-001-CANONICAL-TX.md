---
spec_id: WAL-TX-001
title: "Canonical Transaction Specification"
version: 0.2.0-DRAFT
status: SPEC-DRAFT — V2-Binding an die implementierte L1-TX-Konformität; Spec-Freeze ausstehend
repository: atc-wallet
layer: L5-Wallet
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Canonical Transaction Specification (WAL-TX-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Das verbindliche Transaktionsobjekt des Wallets inkl. kanonischer Serialisierung (Signatur-Grundlage).

## 2. Scope (gilt für)

- Felder (chain_id, nonce, sender, recipient, value, fee, payload, signature)
- Feldreihenfolge & Encoding
- Kanonalität

## 3. Normative Anforderungen (MUST)

- **REQ-WTX-001:** L1-TX-Felder sind fixiert: `chain_id(u64=658467)`, `tx_type(u8)`, `sender_did(bytes)`, `recipient_did(optional)`, `amount(u128)`, `gas_price(u64)`, `gas_limit(u64)`, `nonce(u64)`, `timestamp(u64)`, `payload(bytes)`, `poh_hash(32 bytes)`, Signature = 64-Byte ECDSA/secp256k1 Compact — *Nachweis: unit+vector*
- **REQ-WTX-002:** Signing-Domain ist exakt `ATC-TX-DOMAIN-V2`; Legacy `ATC-TX-DOMAIN` und `atc-tx.v1` sind ungültig. Integer-Encoding im L1-TX-Signing-Preimage ist Big-Endian mit fester Breite; `amount` ist exakt 16 Byte (u128). Bytes/String-Felder verwenden ein u32-Big-Endian-Längenpräfix — *Nachweis: vector+negative*
- **REQ-WTX-003:** Die kanonische Signatur ist ECDSA/secp256k1 mit deterministischem RFC6979-Signing und Low-S; High-S-Signaturen werden verworfen — *Nachweis: unit+negative*
- **REQ-WTX-004:** Signing-Digest = SHA-256(canonical signing preimage); identische Transaktionen müssen byte-identische Preimages und Signatur-Digests erzeugen — *Nachweis: unit+vector*

## 4. Datenmodelle & Schnittstellen

Die ausführbare Referenz ist `components/wallet/src/tx.rs`; die TypeScript-Bindung ist `components/sdk/typescript/chain-identity.ts`. Beide müssen denselben Byte-Stream erzeugen. Der Referenzvektor für die Testtransaktion ist im SDK-Conformance-Test gebunden.

## 5. Invarianten

- Identische Transaktion ⇒ identischer Hash auf allen Implementierungen

## 6. Conformance-Tests (Mindestkategorien)

- Canonical V2 preimage golden vector Rust↔TypeScript
- u128 boundary: `0` and `2^128-1`
- Legacy-domain rejection
- High-S rejection
- TX mutation rejection (payload/recipient/amount)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [x] Implementierung (Rust) mit je-Anforderung-Nachweis — Exact-SHA CI
- [x] TypeScript V2 binding — Exact-SHA CI
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1-3 Replay-Struktur)
