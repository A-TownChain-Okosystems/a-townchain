---
standard:
  id: ATC-STD-SC-008
  title: "ATC-STD-SC-008 — Smart Contract Event Standard"
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

# ATC-STD-SC-008 — Smart Contract Event Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Standardisierte Events als Grundlage fuer AuditTrail, LogChain, DefenderGPT, MinerWatcherGPT, Governance-Monitoring, Explorer, Compliance und Forensics.

## Scope

Gilt fuer alle Contracts; Events sind Teil der oeffentlichen Interface-Dokumentation (ATC-STD-204).

## 1. Ereignis-Pflicht (REQ-SC-023)

Jeder zustandsrelevante Vorgang MUSS ein Event emittieren (Initialisierung, Konfigurationsaenderung, Emergency-State, Werte-Transfers, Rollenaenderungen): z. B. `ContractInitialized(bytes32 indexed contractId, uint256 version)`, `ConfigurationChanged(bytes32 indexed parameter, bytes32 oldValue, bytes32 newValue)`, `EmergencyStateChanged(bool paused)`.

## 2. Indizierung (REQ-SC-024)

Schluesselparameter (IDs, Adressen, Gruende) SOLLEN indexed sein, damit Off-Chain-Konsumenten (Explorer, AuditTrail, GPT-Watcher) effizient filtern koennen.

## 3. Konsumenten (REQ-SC-025)

Events BILDEN die Grundlage fuer: AuditTrail, LogChain, DefenderGPT, MinerWatcherGPT, Governance Monitoring, Explorer, Compliance, Forensics — aenderungen am Event-Set sind daher Interface-aenderungen (ATC-STD-204).




## 4. Indizierung von Hauptparametern (REQ-SC-079)

Sicherheits- und abwicklungsrelevante Events MUESSEN mindestens zwei und maximal drei indexed-Parameter zur effizienten Off-Chain-Filterung besitzen.

## 5. Event-Emission bei State-Änderungen (REQ-SC-080)

Jede schreibende Funktion, die Zustandsvariablen oder Kontostände verändert, MUSS unmittelbar vor dem Return ein zugehöriges Event emittieren.

## 6. Standardisierte Event-Nomenklatur (REQ-SC-081)

Events MUESSEN im PascalCase-Format benannt werden und klare Handlungszustände (z. B. TokensStaked, GovernanceVoteCast) beschreiben.

## 7. Off-Chain Telemetrie-Kompatibilität (REQ-SC-082)

Event-Payloads MUESSEN so strukturiert sein, dass DefenderGPT und MinerWatcherGPT die Daten ohne zusätzliche Dekodierungsschritte verarbeiten können.

## 8. Metriken & Akzeptanzkriterien

- Event Coverage: 100% aller zustandsverändernden Funktionen MUESSEN ein Event auslösen.
- Indizierungs-Quote: MUSS mindestens 2 indexed Attribute bei komplexen Events enthalten.
- Parsing-Kompatibilität: MUSS 0 Schema-Fehler bei DefenderGPT Ingestion aufweisen.
- Nomenklatur-Konformität: MUSS 100% PascalCase Naming Standard einhalten.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-023 | Ereignis-Pflicht (§1) | MUSS |
| REQ-SC-024 | Indizierung (§2) | SOLLTE |
| REQ-SC-025 | Event = Interface (§3) | MUSS |
| REQ-SC-079 | Indizierung von Hauptparametern | MUSS |
| REQ-SC-080 | Event-Emission bei State-Änderungen | MUSS |
| REQ-SC-081 | Standardisierte Event-Nomenklatur | MUSS |
| REQ-SC-082 | Off-Chain Telemetrie-Kompatibilität | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

Events sind die primäre Datenquelle für Off-Chain-Sicherheits-Monitoring, Audit-Trails und Anomaly Detection. Lückenhafte Event-Emission beeinträchtigt das Notfall-Monitoring des Gesamtsystems.

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
