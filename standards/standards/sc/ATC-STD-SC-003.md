---
standard:
  id: ATC-STD-SC-003
  title: "ATC-STD-SC-003 — Smart Contract Security Standard"
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

# ATC-STD-SC-003 — Smart Contract Security Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Mindest-Security-Katalog fuer Production-Contracts: Reentrancy (CEI), Integer Overflow/Underflow, Access Control, Flash-Loan-/Oracle-/Price-Manipulation, Front-Running/MEV, Signature Replay, Delegatecall-Risiken, Timestamp-Manipulation, DoS, unchecked External Calls, Autorisierungsfehler, Upgradeability-/Storage-Collision-Risiken, Dependency-Schwachstellen.

## Scope

Gilt fuer alle Contracts vor SC-G10 (Mainnet); verstaerkt die allgemeinen Security-Standards (ATC-STD-203) um Contract-spezifische Pruefungen.

## 1. Pruefkatalog (REQ-SC-008)

Jeder Production-Contract MUSS mindestens geprueft werden auf: Reentrancy (Checks-Effects-Interactions), Integer Overflow/Underflow, Access Control, Flash-Loan-Manipulation, Oracle Manipulation, Price Manipulation, Front-Running, MEV, Signature Replay, Delegatecall-Risiken, Timestamp Manipulation, Denial of Service, Unchecked External Calls, Incorrect Authorization, Upgradeability Risks, Storage Collision, Dependency Vulnerabilities.

## 2. Checks-Effects-Interactions (REQ-SC-009)

State-Aenderungen MUESSEN vor externen Interaktionen erfolgen (CEI-Pattern); Reentrancy-Guards SOLLLEN bei werttransferierenden Funktionen eingesetzt werden.

## 3. Dependency-Pruefung (REQ-SC-010)

Contract-Abhaengigkeiten MUESSEN im Dependency Graph (ATC-STD-204) registriert und auf bekannte Schwachstellen geprueft sein (SC-G6).




## 4. Reentrancy-Guard-Verpflichtung (REQ-SC-063)

Funktionen mit externen Wertzustellungen oder variablen Interaktionen MUESSEN zwingend durch Reentrancy-Guards (Checks-Effects-Interactions Pattern) abgesichert werden.

## 5. Arithmetischer Überlaufschutz (REQ-SC-064)

Alle mathematischen Operationen MUESSEN durch native Compiler-Überlaufprüfungen oder zertifizierte Arithmetik-Bibliotheken abgesichert sein, um Overflow/Underflow-Lücken auszuschließen.

## 6. Timelock für Sicherheits-Parameter (REQ-SC-065)

Änderungen an sicherheitskritischen Systemparametern MUESSEN eine Verzögerungszeit (Timelock) von mindestens 48 Stunden durchlaufen.

## 7. Notfall-Stopp-Mechanismus (REQ-SC-066)

Verträge mit hohen Vermögenswerten MUESSEN ein Notfall-Pausierungs-Interface (Circuit Breaker) besitzen, um bei erkannten Angriffen Auszahlungen sofort zu stoppen.

## 8. Metriken & Akzeptanzkriterien

- Slither/Mythril Static Analysis: MUSS 0 High/Critical Findings aufweisen.
- Testabdeckung sicherheitskritischer Pfade: MUSS 100% Branch Coverage erreichen.
- Reentrancy-Schutz-Rate: 100% aller zustandsverändernden externen Funktionen MUESSEN abgesichert sein.
- Notfall-Reaktionszeit: Notfall-Pausierung MUSS innerhalb von 1 Block wirksam werden.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-008 | Pruefkatalog komplett (§1) | MUSS |
| REQ-SC-009 | CEI-Pattern (§2) | MUSS |
| REQ-SC-010 | Dependency-Check (§3) | MUSS |
| REQ-SC-063 | Reentrancy-Guard-Verpflichtung | MUSS |
| REQ-SC-064 | Arithmetischer Überlaufschutz | MUSS |
| REQ-SC-065 | Timelock für Sicherheits-Parameter | MUSS |
| REQ-SC-066 | Notfall-Stopp-Mechanismus | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

Dieser Standard ist Höchst-Sicherheitsrelevant (S1/S2); Ausnahmen sind strikt untersagt. Ergänzende Vorgaben aus ATC-STD-203 und ATC-STD-204 MUESSEN bei allen Sicherheitsaudits zwingend angewendet werden.

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
