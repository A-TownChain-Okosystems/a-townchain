---
standard:
  id: ATC-STD-SC-016
  title: "ATC-STD-SC-016 — Oracle Contract Standard"
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

# ATC-STD-SC-016 — Oracle Contract Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Oracle-basierte Contracts definieren provider, update_frequency, max_staleness, deviation_threshold, fallback und emergency_behavior; veraltete oder manipulierte Daten werden nicht akzeptiert.

## Scope

Gilt fuer alle SC-ORACLE-Contracts und oracle-konsumierende Contracts (SC-DEFI).

## 1. Konfigurationspflicht (REQ-SC-045)

Oracle-konsumierende Contracts MUESSEN definieren: oracle.provider, update_frequency, max_staleness, deviation_threshold, fallback, emergency_behavior.

## 2. Datenvaliditaet (REQ-SC-046)

Ein Contract DARF KEINE veralteten (staleness > max_staleness) oder offensichtlich manipulierten (deviation > threshold) Daten akzeptieren; im Zweifel Fallback/Emergency-Verhalten.




## 3. Staleness-Zeitfenster-Prüfung (REQ-SC-111)

Datenpunkte von Orakeln MUESSEN verworfen werden, wenn ihr Zeitstempel älter als max_staleness (z. B. 3600 Sekunden) ist.

## 4. Prozentuale Abweichungs-Grenzwerte (REQ-SC-112)

Preisaktualisierungen MUESSEN auf ungewöhnliche Sprünge außerhalb des deviation_threshold geprüft und bei Extremwerten gesperrt werden.

## 5. Fallback-Orakel-Umschaltung (REQ-SC-113)

Bei Ausfall oder Ungültigkeit des Primär-Orakels MUSS der Contract automatisch auf ein Sekundär-Orakel umschalten.

## 6. Circuit Breaker bei Orakel-Ausfall (REQ-SC-114)

Fallen sowohl Primär- als auch Sekundär-Orakel aus, MUSS der Contract abhängige Funktionen geordnet pausieren.

## 7. Metriken & Akzeptanzkriterien

- Staleness Rejection: 100% veralteter Datenpunkte MUESSEN zum Revert führen.
- Deviation Threshold Check: MUSS Preisabweichungen > 10% ohne Mehrfach-Bestätigung abfangen.
- Umschalt-Latenz auf Fallback: MUSS in unter 1 Block nach Orakel-Fehler erfolgen.
- Uptime der Preisfeeds: MUSS 99,9% Betriebsbereitschaft im Testnet/Mainnet erreichen.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-045 | Konfiguration (§1) | MUSS |
| REQ-SC-046 | Datenvaliditaet (§2) | MUSS |
| REQ-SC-111 | Staleness-Zeitfenster-Prüfung | MUSS |
| REQ-SC-112 | Prozentuale Abweichungs-Grenzwerte | MUSS |
| REQ-SC-113 | Fallback-Orakel-Umschaltung | MUSS |
| REQ-SC-114 | Circuit Breaker bei Orakel-Ausfall | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

Falsche oder veraltete Orakel-Daten führen zu falschen Liquidationen und Fehlabwicklungen. Validierung von Timestamp, Invariants und Fallback-Feeds ist zwingend vorgeschrieben.

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

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), ATC-STD-SC-003 (Security)
- **INFORMATIVE:** SCR-0041 (Change Request), ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/, ATC-STD-BUG-001..005 (Bug Standards)
