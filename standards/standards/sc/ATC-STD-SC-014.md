---
standard:
  id: ATC-STD-SC-014
  title: "ATC-STD-SC-014 — Governance Contract Standard"
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

# ATC-STD-SC-014 — Governance Contract Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Anforderungen fuer DAO-, Voting- und Proposal-Contracts: Proposal-Lifecycle, Quorum/Threshold-Regeln, Stimmgewichtungs-Verfahren, Timelock vor Execution und vollstaendige Event-Nachvollziehbarkeit.

## Scope

Gilt fuer alle SC-GOV-Contracts; verzaehnt mit ATC-9900 (Governance/DAO-Token) und ATC-ENT-002 (Entscheidungen).

## 1. Proposal-Lifecycle (REQ-SC-039)

Proposals MUESSEN einen definierten Lifecycle haben (draft → active → passed → executed/defeated) mit dokumentierten Quorum- und Threshold-Regeln.

## 2. Timelock vor Execution (REQ-SC-040)

Verbindliche Governance-Aktionen MUESSEN einen Timelock vor Execution haben (Ausnahme: dokumentierte Emergency-Verfahren).

## 3. Nachvollziehbarkeit (REQ-SC-041)

Votes und Executions sind Event-pflichtig (SC-008); off-chain spiegelt die DEC-Record-Struktur (ATC-ENT-002) die On-Chain-Entscheidungen.




## 4. Historische Stimmgewichts-Snapshots (REQ-SC-103)

Stimmrechte MUESSEN auf historischen Block-Snapshots basieren, um Flash-Loan-Voting-Attacken wirksam zu unterbinden.

## 5. Quorum- und Schwellenwert-Validierung (REQ-SC-104)

Proposals MUESSEN On-Chain das konfigurierte Quorum und die erforderliche Mehrheit erreichen, bevor sie in die Timelock-Phase eintreten.

## 6. Verpflichtender Timelock vor Execution (REQ-SC-105)

Erfolgreich angenommene Anträge MUESSEN vor der Ausführung eine Timelock-Mora von mindestens 48 Stunden durchlaufen.

## 7. Proposal-Stornierungs-Recht (REQ-SC-106)

Ersteller oder Governance-Guards MUESSEN Anträge stornieren können, wenn die Stimmgewichts-Schwelle des Erstellers unterschritten wird.

## 8. Metriken & Akzeptanzkriterien

- Snapshot-Präzision: MUSS historische Bilanzen exakt zum Voting-Start-Block auslesen.
- Timelock-Verzögerung: MUSS mindestens 172.800 Sekunden betragen.
- Quorum-Prüfungs-Abdeckung: MUSS 100% aller Anträge vor Execution validieren.
- Flash-Loan-Voting Resistenz: MUSS 100% der Versuche abwehren, im selben Block Stimmrechte zu leihen und abzustimmen.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-039 | Proposal-Lifecycle (§1) | MUSS |
| REQ-SC-040 | Timelock (§2) | MUSS |
| REQ-SC-041 | Nachvollziehbarkeit (§3) | MUSS |
| REQ-SC-103 | Historische Stimmgewichts-Snapshots | MUSS |
| REQ-SC-104 | Quorum- und Schwellenwert-Validierung | MUSS |
| REQ-SC-105 | Verpflichtender Timelock vor Execution | MUSS |
| REQ-SC-106 | Proposal-Stornierungs-Recht | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

Governance-Verträge steuern die Parameter und Upgrades des Gesamtsystems. Schutz vor feindlicher Übernahme und Blitz-Abstimmungen ist überlebenswichtig für das A-TownChain-Netzwerk.

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

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), ATC-ENT-002 (Entscheidungen)
- **INFORMATIVE:** SCR-0041 (Change Request), ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/, ATC-STD-BUG-001..005 (Bug Standards)
