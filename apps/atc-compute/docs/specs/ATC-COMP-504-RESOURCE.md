---
spec_id: ATC-COMP-504
title: "Resource Accounting Specification"
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

# Resource Accounting Specification (ATC-COMP-504)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Verbindliche Messung und Abrechnung verbrauchter Ressourcen je Job.

## 2. Scope (gilt für)

- Messgrößen (CPU-Zeit, Speicher-Peak, I/O, Walltime)
- Accounting-Record & Abrechnung
- Nachweis-Pflicht

## 3. Normative Anforderungen (MUST)

- **REQ-RES-001:** Accounting-Record je Job: cpu_ms, mem_peak_mib, io_bytes, walltime_slots, gemessen vom Runtime-Sandbox-Monitor; Werte sind Teil der Attestation (ATC-COMP-505) — *Nachweis: unit+integration*
- **REQ-RES-002:** Abrechnung nutzt ausschließlich Integer (Micro-Units) mit checked ops; Rundungsrichtung: zulasten des Workers (konservativ) — *Nachweis: unit+vector*
- **REQ-RES-003:** Ressourcen-Grenzen werden von der Sandbox erzwungen (Hard-Limits), nicht nur deklariert — *Nachweis: adversarial*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Abgerechnet ≤ Budget (Job-Objekt); Überschreitung ist ein FAILED-, kein Abrechnungsfall

## 6. Conformance-Tests (Mindestkategorien)

- resource_accounting.json (T05)
- budget_enforcement.json (T06)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (T05/T06)
