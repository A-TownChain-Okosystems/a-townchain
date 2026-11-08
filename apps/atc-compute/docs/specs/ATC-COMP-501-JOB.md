---
spec_id: ATC-COMP-501
title: "Compute Job Specification (Objektmodell & Serialisierung)"
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

# Compute Job Specification (Objektmodell & Serialisierung) (ATC-COMP-501)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Das verbindliche Job-Objekt: Eingabe, Hash, Runtime-Referenz, Ressourcen-Budget, Ergebnis-Referenz.

## 2. Scope (gilt für)

- Job-Objekt & -Zustände (QUEUED→SCHEDULED→RUNNING→DONE/FAILED/REJECTED)
- Job-Hash & Determinismus
- Ressourcen-Budget je Job

## 3. Normative Anforderungen (MUST)

- **REQ-JOB-001:** job_hash = SHA-256(canonical(job)) umfasst: workload_class, input_ref (Hash), runtime_version, model/env-Hash (falls AI), resource_budget, chain_id — *Nachweis: unit+vector*
- **REQ-JOB-002:** Zustandsübergänge sind eine fixierte endliche Maschine; illegale Übergänge (z. B. RUNNING→QUEUED) sind verboten — *Nachweis: unit+negative*
- **REQ-JOB-003:** resource_budget ist Obergrenze (CPU-Slots, Speicher-MiB, Walltime in PoH-Slots, ggf. Gas-Äquivalent); Überschreitung ⇒ FAILED mit Teil-Ergebnis-Verwurf — *Nachweis: unit+negative*
- **REQ-JOB-004:** Cancellation ist nur von Job-Owner oder Policy möglich und idempotent — *Nachweis: unit*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Ein Job-Hash identifiziert den Job eindeutig; gleicher Hash ⇒ identisches erwartetes Ergebnis (bei Determinism-Klasse)

## 6. Conformance-Tests (Mindestkategorien)

- job_serialization.json
- job_hash_determinism.json
- state_machine_negative.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (T01/T02)
