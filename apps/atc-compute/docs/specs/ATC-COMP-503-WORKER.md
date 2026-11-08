---
spec_id: ATC-COMP-503
title: "Worker Protocol Specification"
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

# Worker Protocol Specification (ATC-COMP-503)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Registration, Identity, Heartbeat und Tasks des Compute-Workers.

## 2. Scope (gilt für)

- Worker-Registration & Identity (DID-basiert)
- Heartbeat/Liveness
- Task-Annahme & -Ablauf

## 3. Normative Anforderungen (MUST)

- **REQ-WORK-001:** Worker registrieren sich mit Pubkey + Runtime-Manifest (unterstützte Workload-Klassen, Versionen, Ressourcen-Deckel); Manifest-Hash geht in den Registry-Hash ein — *Nachweis: unit+negative*
- **REQ-WORK-002:** Heartbeat-Intervall und Missed-Heartbeat-Schwelle sind genesis-locked; nach Unterschreitung gilt der Worker als DOWN und seine Jobs werden rescheduled — *Nachweis: integration*
- **REQ-WORK-003:** Ein Worker darf einen Job nur einmal annehmen (idempotent; Doppelannahme ⇒ Protocol-Violation) — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Worker ohne gültiges Manifest sind ineligible

## 6. Conformance-Tests (Mindestkategorien)

- worker_registration.json
- heartbeat_timeout.json (T11)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (T04)
