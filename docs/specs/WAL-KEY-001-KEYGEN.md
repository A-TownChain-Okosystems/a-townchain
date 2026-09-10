---
spec_id: WAL-KEY-001
title: "Key Generation Specification"
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

# Key Generation Specification (WAL-KEY-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Sichere, CSPRNG-basierte Schlüsselerzeugung des Trusted Core (Rust).

## 2. Scope (gilt für)

- Entropy-Quelle
- Private-Key-Format (secp256k1)
- Public-Key-Ableitung

## 3. Normative Anforderungen (MUST)

- **REQ-WK-001:** Entropy ≥ 256 Bit aus CSPRNG (OS-Entropy-Quelle + Mischung); keine benutzerseitigen Passwörter als Entropie-Ersatz — *Nachweis: unit+security*
- **REQ-WK-002:** Private Key: 32-Byte secp256k1-Skalar im Bereich [1, n-1]; außerhalb generierte Werte werden verworfen und neu erzeugt (kein Mod-Redukt) — *Nachweis: unit+negative*
- **REQ-WK-003:** Public Key: secp256k1 (compressed, 33 Byte) — kanonisch, keine unkomprimierte Form im Wire-Format — *Nachweis: vector*
- **REQ-WK-004:** Schlüsselerzeugung NUR im Rust Trusted Core; Python/UI hat keinen Zugriff auf Private Keys (Trust Boundary WAL-TB-001) — *Nachweis: architecture+negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Privater Schlüssel verlässt den Trusted Core niemals im Klartext

## 6. Conformance-Tests (Mindestkategorien)

- keygen_vectors.json (RFC-6979-kompatible Determinismus-Tests)
- weak_key_rejection.json
- entropy_source_mock (T08)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (WAL-KEY-001)
- ATC-CRYPTO-001 (kanonische Primitive)
