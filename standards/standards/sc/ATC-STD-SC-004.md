---
standard:
  id: ATC-STD-SC-004
  title: "ATC-STD-SC-004 — Smart Contract Testing Standard"
  version: "1.1.0"
  status: approved
  category: sc
  authority: A-TownChain-Okosystems
  owner: Michael (Owner-Entwurf) / ATC-AI-ARCH-001 (Formalfassung)
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle Smart Contracts des A-TownChain-Oekosystems"
  supersedes: []
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  superseded_by: null
---

# ATC-STD-SC-004 — Smart Contract Testing Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Kein Production-Deployment ohne automatisierte Tests: Unit-, Integration-, Fuzz-, Invariant-, Negative-, Security-, Upgrade- und Gas-Tests sowie Testnet-Tests — mit expliziten Invarianten (z. B. totalSupply <= maxSupply).

## Scope

Gilt fuer alle Contracts vor SC-G3/G4/G8; Teststrategie je Contract dokumentiert.

## 1. Testarten (REQ-SC-011)

Mindestens MUSS getestet werden: Unit Tests, Integration Tests, Fuzz Tests, Invariant Tests, Negative Tests, Security Tests, Upgrade Tests, Gas Tests, Testnet Tests.

## 2. Invarianten (REQ-SC-012)

Kerninvarianten MUESSEN explizit als Invariant-Tests codiert sein, z. B. `totalSupply <= maxSupply`, `lockedBalance <= userBalance`, `bridgeMinted <= bridgeDeposited`.

## 3. Deployment-Blocker (REQ-SC-013)

Ohne bestandene SC-G3 (Unit) und SC-G4 (Fuzz/Invariant) ist ein Deployment UNZULAESSIG.




## 4. Mindest-Testabdeckung (REQ-SC-067)

Alle Smart Contracts MUESSEN eine Testabdeckung von mindestens 95% Statement Coverage und 90% Branch Coverage vorweisen.

## 5. Invarianten- und Fuzzing-Tests (REQ-SC-068)

Sicherheitsrelevante Funktionen MUESSEN automatisierte Fuzzing- und Invarianten-Tests mit mindestens 100.000 Testdurchläufen ohne Fehlschlag bestehen.

## 6. Negativ- und Revert-Pfad-Testing (REQ-SC-069)

Jede zugriffsgeschützte oder bedingungsabhängige Funktion MUSS explizit auf alle negativen Ausführungspfade und Revert-Bedingungen getestet werden.

## 7. Gas-Regressionstests (REQ-SC-070)

In der CI/CD-Pipeline MUSS ein automatisierter Gas-Snapshot-Test durchgeführt werden, der unerwartete Gas-Drifts über 5% blockiert.

## 8. Metriken & Akzeptanzkriterien

- Code Coverage: MUSS >= 95% Line Coverage und >= 90% Branch Coverage betragen.
- Fuzzing Runs: MUSS mindestens 100.000 Fuzz-Iterationen ohne Invariantenverletzung durchlaufen.
- Revert-Test-Abdeckung: 100% aller definierten Require/Revert-Statements MUESSEN durch Negativtests abgedeckt sein.
- Gas-Drift-Toleranz: MUSS unter 5% Abweichung gegenüber der Baseline liegen.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-011 | Testarten (§1) | MUSS |
| REQ-SC-012 | Invariant-Tests (§2) | MUSS |
| REQ-SC-013 | Deployment-Blocker (§3) | MUSS |
| REQ-SC-067 | Mindest-Testabdeckung | MUSS |
| REQ-SC-068 | Invarianten- und Fuzzing-Tests | MUSS |
| REQ-SC-069 | Negativ- und Revert-Pfad-Testing | MUSS |
| REQ-SC-070 | Gas-Regressionstests | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

Umfassende automatisierte Tests minimieren das Risiko von unentschlossenen Fehlerzuständen. Das Bestehen aller Testkategorien ist eine notwendige Voraussetzung für die Freigabe in Gate SC-G4.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Entwurf Michael, Formalfassung ATC-AI-ARCH-001) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## References

- **NORMATIVE:** ATC-STD-000 (Verfassung), ATC-STD-201/202, ATC-STD-203
  (Security), ATC-STD-204 (Dependency & Interface), ATC-STD-SC-001
  (Familien-Hauptstandard)
- **INFORMATIVE:** SCR-0041 (Change Request), ATC-AAS-001..025 (Agenten-Standards), AI-DEV-001..012,
  contracts/registry/ (Contract Registry), ATC-STD-BUG-001..005 (Bug Standards)
