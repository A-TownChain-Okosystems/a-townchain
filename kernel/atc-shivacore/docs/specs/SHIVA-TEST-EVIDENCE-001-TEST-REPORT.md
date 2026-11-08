---
spec_id: SHIVA-TEST-EVIDENCE-001
title: "Canonical Test Report Specification (test-report.json)"
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

# Canonical Test Report Specification (test-report.json) (SHIVA-TEST-EVIDENCE-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Eine einzige, kanonische Test-Evidenz-Quelle — README/STATUS/ROADMAP referenzieren sie, statt Handzahlen zu pflegen (löst F-061: 423 vs. 674 vs. 703).

## 2. Scope (gilt für)

- Artefakt artifacts/test-report.json (CI-generiert)
- Verweis-Pflicht in README/STATUS/ROADMAP
- Zähl-Disziplin je Suite

## 3. Normative Anforderungen (MUST)

- **REQ-STE-001:** test-report.json: {repository, commit, toolchain (rustc-Version), kernel_tests, service_space_tests, integration, total, passed, failed, skipped, status, workflow_run_id, generated_at} — vom CI erzeugt, nicht von Hand — *Nachweis: ci+unit*
- **REQ-STE-002:** README/STATUS/ROADMAP dürfen Testzahlen ausschließlich aus dem Bericht referenzieren (Zitat mit Commit-Bezug); abweichende Handzahlen sind Governance-Verstoß (Validator-prüfbar) — *Nachweis: governance*
- **REQ-STE-003:** Suite-Partitionierung ist fixiert (kernel / service_space / integration) — Summen müssen konsistent sein; inkonsistente Summen ⇒ CI-FAIL — *Nachweis: unit*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Genau eine Testzahl-Quelle je Commit — keine Duplikatzählung

## 6. Conformance-Tests (Mindestkategorien)

- report_schema.json
- sum_consistency.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1-001 TEST-EVIDENCE-001)
- F-061 (registry/findings.yaml)
