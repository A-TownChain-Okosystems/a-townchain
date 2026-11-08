---
spec_id: ATC-AI-S2-001
title: "S2 Security Controls Specification"
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

# S2 Security Controls Specification (ATC-AI-S2-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Verbindliche Controls der S2-Klassifizierung (security-sensitive) mit Review-Kadenz und Nachweisen.

## 2. Scope (gilt für)

- Pflicht-Controls (Dependency, Secrets, SAST, SBOM, Audit-Log, Threat Model, Security-Regression)
- Evidence je Control
- Definition S-Klassen

## 3. Normative Anforderungen (MUST)

- **REQ-S2C-001:** S-Klassen-Definition normativ: S0 informational | S1 low | S2 security-sensitive | S3 critical (financial/kernel-nah) | S4 system/kernel critical — Zuordnung je Repo via .atc — *Nachweis: governance*
- **REQ-S2C-002:** S2-Pflicht-Controls mit je Nachweis: dependency scanning (CI), secret scanning, SAST, SBOM je Release, Audit-Logging (ATC-AI-CAP-001), Threat Model (aktuell), Security-Regression-Suite — *Nachweis: process+ci*
- **REQ-S2C-003:** Control-Evidence ist Registry-artig geführt (Status je Control, Run-Referenzen); „Security clear“ ohne Evidence ist verboten (F-073-Muster) — *Nachweis: governance*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Kein Release ohne vollständige S2-Control-Evidence

## 6. Conformance-Tests (Mindestkategorien)

- s2_control_matrix.json (je Control ein Test/Nachweis)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 09.09. (P1-9: S2 braucht Definition)
- aurora-ai SECURITY.md (S2-Klassifizierung)
