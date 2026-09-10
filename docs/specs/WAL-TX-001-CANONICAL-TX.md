---
spec_id: WAL-TX-001
title: "Canonical Transaction Specification"
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

- **REQ-WTX-001:** Tx-Felder sind fixiert: chain_id(u64=658467 für Mainnet), nonce(u64), sender(addr), recipient(addr), value(u128 micro-ATC), fee(u128), payload(len-bounded bytes), signature(65 Byte: r,s,recid) — *Nachweis: unit+vector*
- **REQ-WTX-002:** Kanonische Serialisierung: Little-Endian-Integer, fixe Feldreihenfolge, Längenpräfix für Bytes (u32); jede Abweichung ⇒ ungültige Signatur (verhindert malleability) — *Nachweis: vector+negative*
- **REQ-WTX-003:** payload-Obergrenze (genesis-locked) wird beim Erzeugen erzwungen; darüber ⇒ Tx-Erzeugung verweigert — *Nachweis: negative*
- **REQ-WTX-004:** tx_hash = SHA-256(canonical(tx_unsigned)) — Basis für Signatur und Nonce-Tracking — *Nachweis: unit*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Identische Transaktion ⇒ identischer Hash auf allen Implementierungen

## 6. Conformance-Tests (Mindestkategorien)

- tx_serialization.json
- payload_limit.json
- malleability_rejection.json (modified payload/recipient/amount ⇒ Verify-FAIL, WAL-VERIFY-001)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1-3 Replay-Struktur)
