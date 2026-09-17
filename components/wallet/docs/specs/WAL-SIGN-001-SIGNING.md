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

Festlegung des KANONISCHEN ATC-Signaturalgorithmus für Transaktionen — löst den ed25519-vs-secp256k1-Widerspruch (F-062): secp256k1 ist kanonisch für Transaktionen; Ed25519 gilt ausschließlich für den P2P-/DID-Layer (ShivaCore K6b).

## 2. Scope (gilt für)

- Algorithmus (secp256k1, ECDSA)
- Deterministische Nonce (RFC 6979)
- Low-S-Normalisierung
- Trust Boundary

## 3. Normative Anforderungen (MUST)

- **REQ-WSIG-001:** ATC-kanonisch für Transaktionssignaturen: ECDSA secp256k1, deterministic nonce nach RFC 6979 (kein Zufall im Signing) — *Nachweis: unit+vector*
- **REQ-WSIG-002:** Low-S-Pflicht: s > n/2 ⇒ normalisiert auf n - s; High-S-Signaturen sind ungültig (Anti-Malleability) — *Nachweis: negative+vector*
- **REQ-WSIG-003:** Domain-Separation: sign(tx_hash) mit Präfix „atc-tx.v1“ (Hash-Präfix vor dem Hashing); verhindert Kreuz-Protokoll-Replay — *Nachweis: unit+negative*
- **REQ-WSIG-004:** Signierung ausschließlich im Rust Trusted Core (WAL-TB-001); Python ist niemals Teil der Signing Boundary — *Nachweis: architecture+negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Gleiche Tx + gleicher Key ⇒ bit-identische Signatur (RFC 6979)

## 6. Conformance-Tests (Mindestkategorien)

- sign_vectors.json (RFC-6979-Testvektoren)
- high_s_rejection.json
- determinism.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1-2 ed25519-dalek vs. secp256k1 — kanonische Festlegung)
- ATC-CRYPTO-001 (Domain-Separation)
