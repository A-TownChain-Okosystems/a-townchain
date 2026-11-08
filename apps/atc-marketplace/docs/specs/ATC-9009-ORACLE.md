---
spec_id: ATC-9009
title: "Oracle Interface Specification"
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

# Oracle Interface Specification (ATC-9009)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Schnittstelle und Sicherheitsregeln für Preis-/External-Daten im Marketplace.

## 2. Scope (gilt für)

- Oracle-Interface (get_price mit Metadaten)
- Staleness & Heartbeat
- Deviation & Manipulations-Resistenz
- Fail-Closed

## 3. Normative Anforderungen (MUST)

- **REQ-A99-001:** get_price liefert {value, source_id, height, timestamp_slot, signature}; verbrauchende Contracts prüfen Signatur und Height-Frische (staleness_bound genesis-locked) — *Nachweis: unit+negative*
- **REQ-A99-002:** Deviation-Limit: Preis weicht über Schwelle vom Median der Quoren ab ⇒ Ablehnung (kein einzelner Feed entscheidet) — *Nachweis: adversarial*
- **REQ-A99-003:** Heartbeat: fehlende Updates > heartbeat_max ⇒ Oracle gilt DOWN; Nutzung ⇒ Reject (FAIL-CLOSED, niemals letzter Wert gerettet) — *Nachweis: negative+adversarial*
- **REQ-A99-004:** Manipulations-Resistenz: Quoren-Diversifizierung dokumentiert (kein Single-Oracle-Design für Wertflüsse) — *Nachweis: architecture*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Kein Fail-Open: DOWN oder STALE heißt Reject, nie Default-Wert

## 6. Conformance-Tests (Mindestkategorien)

- oracle_stale.json
- oracle_deviation.json
- oracle_down ⇒ Reject
- manipulation_simulation.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (Oracle: stale price detection, fail-closed)
