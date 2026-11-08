---
spec_id: ATC-COMP-502
title: "Compute Scheduler Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-compute
layer: L5-Compute
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Compute Scheduler Specification (ATC-COMP-502)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Deterministische, faire Job-Zuweisung an Worker — ohne Timing-Zufall.

## 2. Scope (gilt für)

- Worker-Selection je Job
- Fairness-/Prioritätsklassen
- Scheduler-Determinismus

## 3. Normative Anforderungen (MUST)

- **REQ-SCHED-001:** Worker-Selection ist deterministisch aus (job_hash, worker_registry_hash, epoch_seed) abgeleitet; reine Zufallsauswahl ist verboten — *Nachweis: unit+property*
- **REQ-SCHED-002:** Fairness: Round-Robin-Granularität über Priority-Klassen (HIGH/DEFAULT/LOW); Klassenwechsel nur am Epochenrand — *Nachweis: unit*
- **REQ-SCHED-003:** Scheduler-Zustand ist replay-bar: aus Job-Queue + Registry-Hash folgt die identische Schedule-Sequenz — *Nachweis: property*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Gleiche Queue + gleiche Registry ⇒ identische Zuweisung (T03)

## 6. Conformance-Tests (Mindestkategorien)

- scheduler_determinism.json
- fairness_property.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (T03)
