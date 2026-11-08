---
spec_id: ATC-9005
title: "Royalty Specification"
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

# Royalty Specification (ATC-9005)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Berechnung und Fluss von Royalties (Basispunkte, Rundungsrichtung).

## 2. Scope (gilt für)

- Berechnung (bps)
- Rundungsregeln
- Verteilung

## 3. Normative Anforderungen (MUST)

- **REQ-A95-001:** royalty = floor(sale_amount * royalty_bps / 10000) — Rundung immer zulasten des Empfängers garantiert (kein Overshoot des Sale-Betrags) — *Nachweis: unit+vector*
- **REQ-A95-002:** Summe aller Abzüge (royalty + fee) ≤ sale_amount; Verletzung ⇒ Settlement-Reject (Overflow-/Underflow-safe u128) — *Nachweis: unit+negative*
- **REQ-A95-003:** Royalty-Adresse ist fix am Asset gebunden (kein post-mint Ändern) — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- sale_amount = royalty + fee + seller_proceeds — exakt, ohne Rundungslücke

## 6. Conformance-Tests (Mindestkategorien)

- royalty_math.json (Grenz-/bps-Edge-Fälle)
- rounding_direction.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (royalty manipulation)
