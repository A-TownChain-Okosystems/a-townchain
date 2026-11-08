---
spec_id: ATC-AI-TB-001
title: "Kernel Trust Boundary Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: aurora-ai
layer: L2-AI
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Kernel Trust Boundary Specification (ATC-AI-TB-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Die normative Grenze zwischen Aurora (KI) und ShivaCore (Kernel): „Aurora may request. Policy decides. Kernel executes.“

## 2. Scope (gilt für)

- Intent → Policy → Capability → Kernel
- Ausschluss aus dem TCB
- Audit-Pflicht

## 3. Normative Anforderungen (MUST)

- **REQ-ATB-001:** Alle KI-Anfragen laufen über Intent-Objekte → Policy-Prüfung (ATC-AI-CAP-001) → Kernel-Ausführung; direkte Kernel-Aufrufe aus AI-Komponenten sind verboten (statisch prüfbar) — *Nachweis: architecture+negative*
- **REQ-ATB-002:** Die AI-Pipeline (Model, Inference, Memory) ist außerhalb des TCB — Annahme: kompromittiert; der Kernel bleibt sicher — *Nachweis: architecture+adversarial*
- **REQ-ATB-003:** Jeder Capability-Request und jedes Enforcement ist audit-geprotokolliert (audit_id, ATC-AI-CAP-001) — *Nachweis: unit*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Kein Pfad existiert, in dem KI ohne Policy/Kernel-Gate Wirkung entfaltet

## 6. Conformance-Tests (Mindestkategorien)

- boundary_static_check.json
- audit_log_completeness.json
- compromised_ai_simulation.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 09.09. (P1-7 Trust Boundary)
- GEN-AI-001 (gleiches Muster in Genesis)
