---
spec_id: ATC-CRYPTO-001
title: "Cryptographic Specification (kanonische Primitive & Domain-Separation)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: a-townchain
layer: L3-Chain
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

- **REQ-CRY-001:** Kanonische Zuordnung: TX-Signaturen = ECDSA secp256k1 (RFC 6979, Low-S) — löst F-062 org-weit; P2P-/DID-Layer = Ed25519; Hashes = SHA-256 (Konsens) bzw. BLAKE3 (Storage/CAS); Addressen = RIPEMD160(SHA-256(pubkey)) — *Nachweis: unit+vector*
- **REQ-CRY-002:** Domain-Separation ist Pflicht je Kontext: atc-tx.v1, atc-p2p.v1, atc-storage.key.v1, atc-compute.attestation.v1, atc-asset.transfer.v1 — Registry in dieser Spec — *Nachweis: unit+negative*
- **REQ-CRY-003:** Malleability-Verbot: High-S, nicht-kanonische Encodings, fehlende Längen-Checks ⇒ Reject (Verweise auf WAL-VERIFY-001-Tests) — *Nachweis: negative+vector*
- **REQ-CRY-004:** Key-Lifecycle & Rotation für Validator-/Node-Keys: dokumentierte Verfahren + Notfall-Rotation; PQC-Migration als MAJOR-Roadmap (COMPAT-001), aktuelle Migration ist explizit NICHT behauptet — *Nachweis: governance*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Keine Signatur außerhalb ihrer Domain ist gültig (Cross-Context-Replay unmöglich)

## 6. Conformance-Tests (Mindestkategorien)

- crypto_vectors.json (alle Primitive + Domains)
- malleability_matrix.json
- cross_domain_replay ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (8: Kryptografie: „nicht bei ECDSA stehen bleiben“ — PQC-Pfad als MAJOR-Roadmap)
- WAL-SIGN-001 (Wallet-Instanz)
