---
spec_id: WAL-KEY-002
title: "BIP44 HD-Derivation Specification (Coin Type 658467)"
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

# BIP44 HD-Derivation Specification (Coin Type 658467) (WAL-KEY-002)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Hierarchische Schlüsselableitung nach BIP44 mit ATC-Coin-Type 658467.

## 2. Scope (gilt für)

- Pfad-Schema
- Kanonische Pfad-Serialisierung
- Index-Grenzen

## 3. Normative Anforderungen (MUST)

- **REQ-WB44-001:** Standardpfad: m/658467'/account'/change'/index (alle ableitbaren Kontenste durch Konvention; hardened auf Level 0-2) — *Nachweis: unit+vector*
- **REQ-WB44-002:** Pfad-Serialisierung ist kanonisch (m/x'/y'/z'/i); führende Nullen in Indizes sind verboten; Index-Maximum 2^31-1 (hardened) — *Nachweis: vector+negative*
- **REQ-WB44-003:** Master-Seed-Ableitung folgt BIP32 (HMAC-SHA512); Input ist der mnemonic-seed (BIP39 optional, aber dann pflichtdokumentiert) — *Nachweis: unit*
- **REQ-WB44-004:** BIP39-Mnemonik (falls genutzt): Prüfsummenvalidierung verpflichtend; ungültige Mnemonik ⇒ Reject — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Gleicher Seed + gleicher Pfad ⇒ bit-identischer abgeleiteter Key (plattformübergreifend testbar)

## 6. Conformance-Tests (Mindestkategorien)

- bip44_vectors.json (eindeutige Testvektoren inkl. 658467)
- invalid_path.json
- hardened_boundary.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (BIP44, Coin Type 658467)
