---
spec_id: ATC-COMP-508
title: "Compute Security Specification"
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

# Compute Security Specification (ATC-COMP-508)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Bedrohungsmodell und Pflicht-Controls der Compute-Schicht.

## 2. Scope (gilt für)

- Bedrohungen (byzantinische Worker, Job-Injection, Ressourcen-Erschöpfung, Replay)
- Isolation & Limits
- Malicious-Input-Handling

## 3. Normative Anforderungen (MUST)

- **REQ-CSEC-001:** Job-Inputs sind schemavalidiert und größenlimitiert; nicht validierbare Inputs werden vor Scheduling REJECTED (kein Ausführungsversuch) — *Nachweis: negative+fuzz*
- **REQ-CSEC-002:** Timeouts, Cancellation und Partition-Handhabung sind spezifiziert (T14-T16); verwaiste Jobs werden nach Timeout-Grenze deterministisch beendet — *Nachweis: integration*
- **REQ-CSEC-003:** Kein Worker sieht Privates eines anderen Jobs (Isolation per Sandbox + Speicher-Nullung nach Jobende) — *Nachweis: adversarial*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- FAIL-CLOSED über der gesamten Verifikationskette

## 6. Conformance-Tests (Mindestkategorien)

- timeout.json (T14)
- cancellation.json (T15)
- network_partition.json (T16)
- fuzz: job_input_mutation

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (T10-T16)
