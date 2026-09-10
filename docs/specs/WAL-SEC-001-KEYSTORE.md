---
spec_id: WAL-SEC-001
title: "Secure Keystore Specification"
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

# Secure Keystore Specification (WAL-SEC-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Verschlüsselte Schlüsselspeicherung at-rest.

## 2. Scope (gilt für)

- Keystore-Format
- KDF & AEAD
- Key-Handling im Speicher

## 3. Normative Anforderungen (MUST)

- **REQ-WS1-001:** Keystore: AES-256-GCM (AEAD) über verschlüsseltem Key-Material; Nonce eindeutig je Save (nie wiederverwendet) — *Nachweis: unit+vector*
- **REQ-WS1-002:** KDF: Argon2id (Parameter verbindlich dokumentiert: m=64 MiB, t=3, p=1 als Mindestwerte) — *Nachweis: unit+negative*
- **REQ-WS1-003:** Falsches Passwort ⇒ Decryption-Fail, nie ein Teilergebnis; Keystore-Format hat Version-Feld (Migration nur vorwärts, dokumentiert) — *Nachweis: negative+vector*
- **REQ-WS1-004:** Private Keys im RAM: zeroize-on-drop; keine long-lived Klartext-Kopien — *Nachweis: security*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Kein Klartext-Key at-rest — niemals

## 6. Conformance-Tests (Mindestkategorien)

- keystore_roundtrip.json
- wrong_password.json
- nonce_reuse_forbidden.json (statisch prüfbar)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (WAL-SEC-001)
