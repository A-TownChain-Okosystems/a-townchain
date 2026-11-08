---
spec_id: ATC-9001
title: "Asset Registry Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-marketplace
layer: L5-Marketplace
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Asset Registry Specification (ATC-9001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Die verbindliche Registry für Asset-Existenz und -Eigentum.

## 2. Scope (gilt für)

- Registry-Operationen (register, lookup)
- Ownership-Nachweis
- Fehlerfälle

## 3. Normative Anforderungen (MUST)

- **REQ-A91-001:** Registry-Einträge sind append-only (History bleibt lesbar); Ownership ist das outcome der Transfer-Kette, nicht ein editierbares Feld — *Nachweis: unit+property*
- **REQ-A91-002:** Lookup per asset_id und per owner (Index) — beide deterministisch — *Nachweis: unit*
- **REQ-A91-003:** Unbekannte Asset-ID ⇒ eindeutiger Fehlercode (kein Null-Objekt) — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- (in diesem Grundgerüst noch offen)

## 6. Conformance-Tests (Mindestkategorien)

- registry_vectors.json
- unknown_asset ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- ATC-9000 (Basisobjekt)
