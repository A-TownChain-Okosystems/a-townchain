---
standard:
  id: ATC-STD-V2S-020
  title: "Monitoring Standard"
  version: "1.0.0"
  status: candidate
  category: v2s
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle ATC-Projekte gemäß ATC-STD-V2S-000 Scope"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-V2S-020 — Monitoring Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-020 regelt die kontinuierliche Überwachung (Monitoring), Protokollierung
(Logging), Metrik-Erhebung und Alarmierung (Alerting) von Software, Smart Contracts und
KI-Agenten im ATC Vision-to-Software Lifecycle. Der Standard etabliert Service Level
Indicators (SLIs), Service Level Objectives (SLOs) und Error Budgets als Steuerungselemente.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** Alle laufenden Anwendungen, Microservices, Blockchain-Knoten, KI-Agenten
und Infrastruktur-Komponenten im produktiven Betrieb.
**Nicht-Gilt:** Lokale Entwicklungsumgebungen und isolierte Einmal-Skripte ohne Laufzeitdienst.

## §1 Gegenstand & Verortung

Monitoring ist Station 16 im V2S-Lifecycle (ATC-STD-V2S-000 §1). Es baut direkt auf dem
operativen Betrieb (V2S-019 / Operations) auf und bildet das technische Fundament für
das Incident Management (V2S-021) sowie für die systematische Erfassung von System-Feedback
(V2S-022 / Feedback Standard).

Die drei Säulen des Observability-Frameworks sind:
1. **Logging:** Strukturierte Aufzeichnung von Systemereignissen mit Traceability.
2. **Metrics:** Zeitreihendaten zu Performance, Fehlerraten und Systemzuständen.
3. **Alerting:** Zielgerichtete, stufenbasierte Alarmierung bei Grenzwertverletzungen.

## §2 Kernregeln

1. **KR-1 (Strukturierte Logs & Correlation IDs):** Alle Systemkomponenten MÜSSEN Logs im strukturierten JSON-Format erzeugen. Jeder Anwendungs-Call MUSS eine eindeutige Correlation/Trace-ID durch alle Service-Ebenen mitführen.
2. **KR-2 (Four Golden Signals):** Das Metrik-Monitoring MUSS zwingend die vier Golden Signals erheben: Latency (Antwortzeiten), Traffic (Anfragevolumen), Errors (Fehlerrate) und Saturation (Auslastung).
3. **KR-3 (SLO/SLI Pflicht):** Für jeden produktiven Dienst MÜSSEN quantifizierbare SLIs (z.B. erfolgreiche HTTP-Anfragen %) und dazugehörige SLOs (z.B. 99,9 % über 30 Tage) verbindlich definiert und überwacht werden.
4. **KR-4 (Aktionsorientiertes Alerting):** Alarme MÜSSEN nach Schweregraden klassifiziert werden (Paging vs. Ticket). Jeder Pager-Alarm MUSS zwingend auf ein konkretes Runbook verweisen, das Entstörschritte beschreibt.
5. **KR-5 (Schutz sensibler Daten):** Log- und Metrik-Pipelines MÜSSEN vertrauliche Daten (Private Keys, Passwörter, PII, Tokens) vor der Übertragung automatisch filtern oder unkenntlich machen.
6. **KR-6 (Error Budget Governance):** Der Verbrauch des Error Budgets MUSS kontinuierlich berechnet werden. Bei 100 % Error-Budget-Erschöpfung MÜSSEN Feature-Releases automatisch gesperrt werden, bis die Stabilität wiederhergestellt ist.

## §3 Artefakte & Nachweispflichten

- **SLO/SLI-Spezifikation:** Dokumentation aller definierten Indikatoren, Zielwerte und Berechnungsmethoden per Dienst.
- **Monitoring-as-Code Konfiguration:** Versionierte Regelsätze für Prometheus, OpenTelemetry, Grafana und Pager-Services.
- **Alert-Katalog:** Verzeichnis aller Alarme inklusive Schwellenwerten, Schweregraden und Verweisen auf Betriebshandbücher.
- **Error-Budget-Bericht:** Monatlicher Nachweis über SLO-Einhaltung und verbliebene Fehlerbudgets.

## §4 Lifecycle-State-Bezug

Der Standard pflegt und aktualisiert den maschinenlesbaren Lifecycle-State
(`.atc/lifecycle-state.yaml`) gemäß ATC-STD-V2S-000 §16:

- Sets `lifecycle.monitoring`: `active` | `degraded` | `unhealthy`
- Sets `lifecycle.slo_compliance_rate`: Prozentwert (z.B. 99.95 %)
- Tracks `lifecycle.error_budget_remaining`: Prozentwert des verbleibenden Budgets

## §5 Schnittstellen

- **Upstream:** ATC-STD-V2S-019 (Operations Standard).
- **Downstream:** ATC-STD-V2S-021 (Incident Standard), ATC-STD-V2S-022 (Feedback Standard).
- **Andock-Standards:** ATC-STD-BUG-001 (Finding Severity), ATC-STD-314 (Security Telemetry).

## Metriken & Akzeptanzkriterien

- **M1 (Log Structuring Compliance):** 100 % aller Anwendungslogs halten das strukturierte JSON-Schema und Correlation-ID-Vorgaben ein.
- **M2 (SLO Observability Rate):** 100 % der kritischen Dienste besitzen aktive SLI/SLO-Messungen.
- **M3 (False Positive Alert Rate):** Weniger als 5 % aller ausgelösten Alarme als False Positive eingestuft.
- **M4 (Alert Runbook Coverage):** 100 % aller Paging-Alarme verfügen über einen direkten Link zu einem gültigen Runbook.

## Compliance & Verifikation

Die Einhaltung wird automatisiert durch Pipeline-Tests der Monitoring-as-Code-Repos,
atc-repo-audit und Log-Schema-Validierung geprüft (S-01..S-25). Unstrukturierte Logs oder
Alarme ohne Runbook führen zu Findings (F-001..F-999) nach ATC-STD-BUG-001.

## Security Considerations

Telemetrie-Pipelines übertragen hochsensitive Systemzustände. Alle Übertragungswege
MÜSSEN mittels TLS 1.3 verschlüsselt werden. Der Zugriff auf Monitoring-Dashboards
und Log-Management-Systeme MUSS strikt über RBAC und MFA geschützt sein.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-314, ATC-STD-BUG-001
**INFORMATIVE:** SCR-0042
