---
spec_id: SHIVA-DEP-001
title: "Kernel Dependency Policy Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-shivacore
layer: L1-Kernel
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Kernel Dependency Policy Specification (SHIVA-DEP-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Governance für Dependency-Änderungen am S4-Kernel: keine automatische Merge-Policy.

## 2. Scope (gilt für)

- Kategorisierung (Kernel-nah vs. Peripherie)
- Review-Gates je Änderung
- Freeze-Phasen

## 3. Normative Anforderungen (MUST)

- **REQ-SDE-001:** Kernel-nahe Dependencies (u. a. ed25519-dalek, rand, spin, uart_16550) sind Change-kontrolliert: API-Kompatibilität + no_std + Target-Kompatibilität + Security + Determinismus + volle Testsuite + Boot-Test — jedes Gate mit Nachweis; kein SemVer-Auto-Merge — *Nachweis: governance+adversarial*
- **REQ-SDE-002:** Jede Änderung dokumentiert Impact auf Boot und Determinismus (Boot-Log-Evidence) — Merge nur mit kompletter Gate-Evidence — *Nachweis: process*
- **REQ-SDE-003:** Während Kernel-Härtungsphasen (Roadmap M3) gilt ein Dependency-Freeze außerhalb sicherheitsgetriebener Updates — *Nachweis: governance*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Keine unauditierte Dependency erreicht den Kernel-Build

## 6. Conformance-Tests (Mindestkategorien)

- dependency_gate_checklist.json
- boot_regression.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1-002: Microkernel ≠ SemVer-Routine; 8 offene Dependabot-PRs)
