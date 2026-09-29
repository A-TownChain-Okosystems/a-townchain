---
spec_id: WAL-SIGN-001
title: "Transaction Signing Specification (Kanonischer Signaturalgorithmus)"
version: 0.3.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Integration und CI-Evidence PENDING
repository: atc-wallet
layer: L5-Wallet
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: ['ATC-CRYPTO-001']
---

# Transaction Signing Specification (Kanonischer Signaturalgorithmus) (WAL-SIGN-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst mit initialem Rust Trusted Core auf
> dem P0-Fix-Branch. SDK/Node-Integration, Conformance-Evidence, Exact-SHA-CI und
> Security-Review bleiben offen. Es wird kein fertiger Protokollzustand behauptet.

## 1. Zweck

Festlegung des KANONISCHEN ATC-Signaturalgorithmus für Transaktionen — löst den ed25519-vs-secp256k1-Widerspruch (F-062): secp256k1 ist kanonisch für Transaktionen; Ed25519 gilt ausschließlich für purpose-bound Identity-Layer.

## 2. Scope (gilt für)

- Algorithmus (secp256k1, ECDSA)
- Deterministische Nonce (RFC 6979)
- Low-S-Normalisierung
- Canonical transaction preimage
- Trust Boundary

## 3. Normative Anforderungen (MUST)

- **REQ-WSIG-001:** ATC-kanonisch für Transaktionssignaturen: ECDSA secp256k1, deterministic nonce nach RFC 6979 (kein Zufall im Signing) — *Nachweis: unit+vector*
- **REQ-WSIG-002:** Low-S-Pflicht: s > n/2 ⇒ normalisiert auf n - s; High-S-Signaturen sind ungültig (Anti-Malleability) — *Nachweis: negative+vector*
- **REQ-WSIG-003:** Domain-Separation: `ATC-TX-DOMAIN-V2` ist das erste Feld der kanonischen V2-Signing-Bytes. Danach folgen exakt `chain_id` (u64 BE), `tx_type` (u8), Sender/Recipient mit u32-Längenpräfix, `amount` (u128 BE), `gas_price` (u128 BE), `gas_limit` (u64 BE), `nonce` (u64 BE), `timestamp` (u64 BE), Payload mit u32-Längenpräfix und `poh_hash` (32 Bytes). Die exakten Bytes werden mit SHA-256 gehasht und als ECDSA/secp256k1-Prehash signiert — *Nachweis: vector+negative*
- **REQ-WSIG-004:** Signierung ausschließlich im Rust Trusted Core (WAL-TB-001); Python ist niemals Teil der Signing Boundary — *Nachweis: architecture+negative*
- **REQ-WSIG-005:** Wirtschaftliche Transaktionswerte verwenden die kanonische `u128`-Darstellung; reine Zähler/Ressourcenlimits bleiben `u64` gemäß Monetary Contract — *Nachweis: type+vector*

### 3.1 Canonical cryptographic contract

- Curve: secp256k1
- Signature: ECDSA
- Hash: SHA-256 over the exact V2 signing bytes
- Nonce: RFC 6979 deterministic
- Signature encoding: fixed 64-byte `r || s`
- Low-S: mandatory; high-S signatures MUST be rejected
- Public-key encoding: compressed SEC1, 33 bytes
- Protocol chain ID: numeric `658467`
- Economic fields: `u128`
- Counter/resource fields: `u64`
- `network_id`, `genesis_id`, `protocol_version`, and `vm_version` are validated runtime context and are not silently inserted into V2 signing bytes
- Legacy `ATC-TX-DOMAIN` and `atc-tx.v1` signing are forbidden

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Gleiche Tx + gleicher Key ⇒ bit-identische Signatur (RFC 6979)
- Rust und SDK müssen für denselben Transaction Input byte-identische V2-Signing-Bytes erzeugen.
- Keine Transaction-Key-Wiederverwendung für Node/Service/Agent/Consensus Identity.

## 6. Conformance-Tests (Mindestkategorien)

- sign_vectors.json (RFC-6979-Testvektoren)
- high_s_rejection.json
- determinism.json
- canonical_u128_transaction.json
- rust_typescript_preimage_equality.json
- legacy_domain_rejection.json
- cross_purpose_key_rejection.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [x] Initiale Rust Trusted-Core Implementierung auf P0-Fix-Branch
- [x] Initialer Rust canonical u128 transaction vector
- [x] Initiale SDK preimage alignment auf u128
- [ ] SDK/Node Integration mit gemeinsamem Contract
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1-2 ed25519-dalek vs. secp256k1 — kanonische Festlegung)
- ATC-CRYPTO-001 (Domain-Separation)
- `vectors/ATC-TX-V2-U128-001.json`
