---
spec_id: ATC-CONSENSUS-ENC
title: "Canonical Encoding Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-algorithm
layer: L3-Konsens
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Canonical Encoding Specification (ATC-CONSENSUS-ENC)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Eindeutige Byte-Darstellung aller Konsensobjekte — Basis für Hashing, Signaturen und deterministische Vergleiche.

## 2. Scope (gilt für)

- Block-/Header-Felder
- Tick- & Attestation-Felder
- Feldreihenfolge
- Varint-Regeln

## 3. Normative Anforderungen (MUST)

- **REQ-ENC-001:** Feldreihenfolge ist fixiert je Objekttyp; jede Änderung = MAJOR (COMPAT-001) — *Nachweis: unit+vector*
- **REQ-ENC-002:** Integer: Little-Endian, feste Breite (u32/u64/u128) — keine Varints im Konsenspfad (Vereinfachung der Verifikation) — *Nachweis: vector*
- **REQ-ENC-003:** Hashes: 32-Byte-Rohwerte; Hex-Darstellung ausschließlich lowercase für Debug/JSON-RPC, niemals für Hashing — *Nachweis: negative*
- **REQ-ENC-004:** Serialize⇒Deserialize ist verlustfrei und eindeutig; unbekannte Felder ⇒ Decode-Fehler (kein permissives Parsing im Konsenspfad) — *Nachweis: unit+negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- hash(X) ist definiert als SHA-256(canonical_encode(X)) — jede Abweichung macht Signatur/Attestation ungültig

## 6. Conformance-Tests (Mindestkategorien)

- encode_roundtrip.json
- decode_unknown_field ⇒ Reject
- hex_case_sensitivity.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P0-04 kanonisches Encoding)
