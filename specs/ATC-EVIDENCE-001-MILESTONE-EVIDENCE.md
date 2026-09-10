---
spec_id: ATC-EVIDENCE-001
title: "Milestone Evidence Package Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: a-townchain
layer: L3-Chain
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Milestone Evidence Package Specification (ATC-EVIDENCE-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Maschinenlesbare Evidence-Pakete je Meilenstein (M4 „Blockchain läuft“ als erster Anwendungsfall) — gegen unbelegte Milestone-Claims.

## 2. Scope (gilt für)

- Paket-Layout (evidence/M<N>/)
- Pflicht-Artefakte (Logs, Genesis, Chain-State, Blöcke, Proofs, SHA256SUMS)
- Claim-Kette

## 3. Normative Anforderungen (MUST)

- **REQ-EVM-001:** Je Meilenstein: evidence/M<N>/ mit node-logs, genesis.json, chain-state.json, transaction.json, block-00000N.json, consensus-proof.json, atvm-execution.json, SHA256SUMS — alles Commit-gebunden — *Nachweis: process+artifact*
- **REQ-EVM-002:** Claim-Kette: jeder Milestone-Status (MILESTONE-001) verweist auf Evidence-Paket + CI-Run; ohne Paket kein ACCEPTED (No status without evidence) — *Nachweis: governance*
- **REQ-EVM-003:** Paket-Integrität: SHA256SUMS verifizierbar; Verletzung ⇒ Evidence ungültig — *Nachweis: unit*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Kein Milestone-ACCEPTED ohne verifizierbares Evidence-Paket

## 6. Conformance-Tests (Mindestkategorien)

- evidence_package_schema.json
- sums_verification.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (6: M4 Evidence Package)
- ATC-STD-MILESTONE-001 (Registry)
