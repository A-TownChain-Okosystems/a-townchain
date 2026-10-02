---
spec_id: WAL-SIGN-001
title: "Transaction Signing Specification (Kanonischer Signaturalgorithmus)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
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

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Festlegung des KANONISCHEN ATC-Signaturalgorithmus für L1-Transaktionen: secp256k1/ECDSA mit RFC 6979 und Low-S.

## 2. Scope (gilt für)

- secp256k1 / ECDSA
- RFC 6979 deterministic nonce
- Low-S-Normalisierung und High-S-Rejection
- 64-Byte-kompakte `r||s`-Signatur
- 33-Byte-komprimierter SEC1-Public-Key
- ATC-TX-DOMAIN-V2
- Rust Trusted Core / Cross-language conformance

## 3. Normative Anforderungen (MUST)

- **REQ-WSIG-001:** ATC-kanonisch für L1-Transaktionssignaturen: ECDSA secp256k1, deterministic nonce nach RFC 6979 (kein Zufall im Signing) — *Nachweis: unit+vector*
- **REQ-WSIG-002:** Low-S-Pflicht: `s > n/2` ⇒ normalisieren auf `n - s`; High-S-Signaturen sind ungültig — *Nachweis: negative+vector*
- **REQ-WSIG-003:** Signing-Domain ist ausschließlich `ATC-TX-DOMAIN-V2`. `ATC-TX-DOMAIN` und `atc-tx.v1` sind Legacy/forbidden und dürfen für L1-TX-Signaturen nicht akzeptiert werden — *Nachweis: unit+negative*
- **REQ-WSIG-004:** Signing-Preimage und TX-ID sind bytegenau zum Cross-language Vertrag in ATC-CRYPTO-001; `amount` ist fixed-width u128 BE; variable byte fields verwenden u32 BE Längenpräfixe — *Nachweis: vector*
- **REQ-WSIG-005:** Signierung erfolgt ausschließlich im Rust Trusted Core. Python ist niemals Teil der Signing Boundary — *Nachweis: architecture+negative*
- **REQ-WSIG-006:** Die TX-ID ist `SHA-256(ATC-TX-ID-V2 || canonical_tx_fields_without_signature)`; die Signatur darf die TX-ID nicht verändern — *Nachweis: unit+vector*

## 4. Datenmodelle & Schnittstellen

Signing-Input, Wire-Encoding und TX-ID müssen exakt dem kanonischen L1-TX-Vertrag entsprechen. Signatur = 64 Byte `r||s`, Public Key = 33 Byte compressed SEC1.

## 5. Invarianten

- Gleiche Tx + gleicher Key ⇒ bit-identische Signatur.
- Gleiche kanonische Tx mit unterschiedlichen gültigen Signaturen ⇒ identische TX-ID.
- Jede Mutation eines signierten Feldes ⇒ Signaturprüfung FAIL.

## 6. Conformance-Tests (Mindestkategorien)

- sign_vectors.json
- tx_id_vectors.json
- high_s_rejection.json
- malformed_signature_length.json
- legacy_domain_rejection.json
- determinism.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Cross-language Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1-2 ed25519-dalek vs. secp256k1 — kanonische Festlegung)
- ATC-CRYPTO-001 (Domain-Separation)
