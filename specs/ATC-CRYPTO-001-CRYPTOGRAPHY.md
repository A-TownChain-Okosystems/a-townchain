---
spec_id: ATC-CRYPTO-001
title: "Cryptographic Specification (kanonische Primitive & Domain-Separation)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: a-townchain
layer: L2-Blockchain-Core
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Cryptographic Specification (kanonische Primitive & Domain-Separation) (ATC-CRYPTO-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Die verbindliche Krypto-Referenz der Chain: was, wie und mit welcher Trennung signiert/gehasht wird — inkl. PQC-Migrationspfad.

## 2. Scope (gilt für)

- Primitive (SHA-256, BLAKE3, secp256k1, Ed25519, RIPEMD160)
- Domain-Separation-Registry
- Malleability-Regeln
- PQC-Migrationspfad

## 3. Normative Anforderungen (MUST)

- **REQ-CRY-001:** Kanonische Zuordnung: L1-TX-Signaturen = ECDSA secp256k1 (RFC 6979, Low-S); P2P-/DID-Layer = Ed25519; Hashes = SHA-256 (Konsens/TX-Digest) bzw. BLAKE3 (Storage/CAS); Adressen = RIPEMD160(SHA-256(pubkey)) — *Nachweis: unit+vector*
- **REQ-CRY-002:** L1-TX-Signaturen verwenden ausschließlich den Domain-Tag `ATC-TX-DOMAIN-V2`. Die Legacy-Domains `ATC-TX-DOMAIN` und `atc-tx.v1` sind für L1-TX-Signaturen verboten. Andere Protokollkontexte führen eigene, getrennte Domain-Tags — *Nachweis: unit+negative*
- **REQ-CRY-003:** L1-TX-Signaturen verwenden 64 Byte kompakte ECDSA-`r||s`-Kodierung auf secp256k1; Public Keys werden als kanonische 33-Byte-komprimierte SEC1-Schlüssel übertragen. High-S, nicht-kanonische Encodings, falsche Längen und ungültige Schlüssel ⇒ Reject — *Nachweis: negative+vector*
- **REQ-CRY-004:** Key-Lifecycle & Rotation für Validator-/Node-Keys: dokumentierte Verfahren + Notfall-Rotation; PQC-Migration als MAJOR-Roadmap (COMPAT-001), aktuelle Migration ist explizit NICHT behauptet — *Nachweis: governance*

## 4. Kanonischer L1-TX-Vertrag

Die Signing-Preimage ist bytegenau:

`ATC-TX-DOMAIN-V2 || chain_id(u64 BE) || tx_type(u8) || sender_did(bytes) || recipient_did(optional) || amount(u128 BE) || gas_price(u64 BE) || gas_limit(u64 BE) || nonce(u64 BE) || timestamp(u64 BE) || payload(bytes) || poh_hash(32 bytes)`

Längenpräfixe für variable Bytefelder sind jeweils `u32 BE`. Optionales `recipient_did` wird mit einem Presence-Byte `0/1` kodiert. Der Digest ist SHA-256 der vollständigen Preimage. Die ECDSA-Signatur wird über diesen 32-Byte-Digest mit RFC 6979 erzeugt und als kompakte 64-Byte-`r||s`-Form serialisiert.

Die Transaktions-ID verwendet unabhängig von der Signatur:

`SHA-256(ATC-TX-ID-V2 || canonical_tx_fields_without_signature)`

Die Signatur selbst ist **kein Bestandteil der TX-ID**.

## 5. Invarianten

- Keine L1-TX-Signatur außerhalb `ATC-TX-DOMAIN-V2` ist gültig.
- Gleiche kanonische TX + gleicher Key ⇒ bit-identische Signatur.
- Die TX-ID ist unabhängig von der Signatur.
- Rust und TypeScript müssen für identische Eingaben byteidentische Preimages und identische TX-IDs erzeugen.

## 6. Conformance-Tests (Mindestkategorien)

- canonical_tx_v2_vectors
- tx_id_v2_vectors
- u128_boundaries
- high_s_rejection
- malformed_signature_length
- compressed_pubkey_validation
- legacy_domain_rejection
- cross_domain_replay ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Cross-language Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (8: Kryptografie: „nicht bei ECDSA stehen bleiben“ — PQC-Pfad als MAJOR-Roadmap)
- WAL-SIGN-001 (Wallet-Instanz)
