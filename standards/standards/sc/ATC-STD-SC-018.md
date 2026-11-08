---
standard:
  id: ATC-STD-SC-018
  title: "ATC-STD-SC-018 — Mining Contract Standard"
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

# ATC-STD-SC-018 — Mining Contract Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Anforderungen fuer Mining-Reward-Contracts: Emissionsplan, Schwierigkeits-/Block-Reward-Regeln, Claim-Mechanismen und Events fuer Emissions- und Reward-Verteilung (Verzahnung MinerWatcherGPT).

## Scope

Gilt fuer alle SC-MINING-Contracts (atc-mining, ATC-Kernkonsensus PoW/PoS).

## 1. Emissionsplan (REQ-SC-050)

Mining-Contracts MUESSEN einen dokumentierten Emissionsplan haben (Invariant: minedSupply <= maxSupply; halbierungs-/epochenbasierte Regeln explizit).

## 2. Reward-Verteilung (REQ-SC-051)

Claim- und Verteilungsmechanismen MUessen gegen Replay und Double-Claim gesichert sein (SC-003).

## 3. Monitoring (REQ-SC-052)

Block-Reward- und Claim-Events sind Event-pflichtig (SC-008) fuer MinerWatcherGPT-Monitoring.




## 4. Halving- & Emissionsplan-Treue (REQ-SC-119)

Mining-Rewards MUESSEN exakt gemäß dem vordefinierten mathematischen Halving-Plan nach Blockhöhen angepasst werden.

## 5. Kryptografischer Nachweis der Arbeitsleistung (REQ-SC-120)

Mining-Claim-Anforderungen MUESSEN gültige kryptografische Proofs oder signierte Zuweisungen der Mining-Knoten prüfen.

## 6. Anti-Frontrunning bei Reward-Claims (REQ-SC-121)

Claims MUESSEN an den spezifischen Empfänger gebunden sein, sodass Dritte Transaktionen nicht im Mempool abfangen können.

## 7. MinerWatcherGPT-Telemetrie-Emission (REQ-SC-122)

Mining-Contracts MUESSEN spezialisierte Telemetrie-Events für das automatisierte Monitoring durch MinerWatcherGPT emittieren.

## 8. Metriken & Akzeptanzkriterien

- Emissionsplan-Abweichung: MUSS exakt 0% Abweichung vom mathematischen Halving-Graph aufweisen.
- Proof Verification Pass-Rate: 100% ungültiger Mining-Proofs MUESSEN fehlschlagen.
- Frontrunning Protection: MUSS 100% der verfälschten Claim-Transaktionen von Fremd-Absendern blockieren.
- Telemetrie-Event-Gültigkeit: MUSS 100% Schema-Konformität für MinerWatcherGPT erfüllen.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-050 | Emissionsplan (§1) | MUSS |
| REQ-SC-051 | Double-Claim-Schutz (§2) | MUSS |
| REQ-SC-052 | Monitoring-Events (§3) | MUSS |
| REQ-SC-119 | Halving- & Emissionsplan-Treue | MUSS |
| REQ-SC-120 | Kryptografischer Nachweis der Arbeitsleistung | MUSS |
| REQ-SC-121 | Anti-Frontrunning bei Reward-Claims | MUSS |
| REQ-SC-122 | MinerWatcherGPT-Telemetrie-Emission | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

Mining-Verträge regeln den primären Influx neuer Tokens. Falsche Emissionsberechnungen oder Sybil-Mining bedrohen die Ökonomie und MUESSEN durch automatisierte Telemetrie überwacht werden.

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

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), ATC-STD-SC-004 (Testing)
- **INFORMATIVE:** SCR-0041 (Change Request), ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/, ATC-STD-BUG-001..005 (Bug Standards)
