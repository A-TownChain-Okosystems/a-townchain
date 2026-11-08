---
standard:
  id: ATC-STD-V2S-019
  title: "Operations Standard"
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

# ATC-STD-V2S-019 — Operations Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-019 definiert den operativen Betrieb (Operations) von Software, Smart
Contracts und KI-Agenten im ATC Vision-to-Software Lifecycle. Der Standard konkretisiert
die 11 Betriebspflichten aus Master §13 und stellt den dauerhaften, sicheren, hochverfügbaren
und wartbaren Zustand im Produktivbetrieb sicher.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** Alle produktiv betriebenen Services, Protokoll-Knoten, Datenbanken, KI-
Agenten und Anwendungsdienste im ATC-Ökosystem.
**Nicht-Gilt:** Ephemer eingerichtete Testumgebungen, Staging-Systeme außerhalb des
Dauerbetriebs und nicht-produktive Prototypen.

## §1 Gegenstand & Verortung

Operations bildet Station 15 im V2S-Lifecycle (ATC-STD-V2S-000 §1). Es schließt direkt
an die Auslieferung (V2S-018 / Deployment) an und stellt die operative Laufzeitumgebung
für Monitoring (V2S-020), Incident Management (V2S-021) und kontinuierliches Feedback
(V2S-022) bereit.

Gemäß Master ATC-STD-V2S-000 §13 gliedert sich der Betrieb in 11 funktionale
Betriebspflichten:
1. Monitoring, 2. Logging, 3. Metrics, 4. Alerting, 5. Backup, 6. Recovery,
7. Incident Management, 8. Security Monitoring, 9. Performance Monitoring,
10. Capacity Monitoring, 11. Version Management.

## §2 Kernregeln

1. **KR-1 (11 Betriebspflichten):** Jedes produktiv geschaltete System MUSS die 11 Betriebspflichten nach ATC-STD-V2S-000 §13 nachweisbar abdecken (MUST).
2. **KR-2 (Betriebshandbuch & Runbook):** Für jeden Service MUSS ein maschinen- und menschenlesbares Betriebshandbuch (Runbook) existieren, das Standardbetrieb, Wartungsfenster, Notfallprozeduren und Kontaktpfade regelt.
3. **KR-3 (Backup & Recovery Drills):** Datensicherungen MÜSSEN automatisiert nach definierten Intervallen (mindestens täglich) erstellt und mindestens einmal pro Quartal im Rahmen eines Recovery Drills auf Wiederherstellbarkeit geprüft werden.
4. **KR-4 (Sicherheits-Patching):** Kritische Sicherheitsupdates (P0/P1) MÜSSEN innerhalb verbindlicher SLAs (P0: < 24 Stunden, P1: < 72 Stunden) in allen Produktivumgebungen eingespielt werden.
5. **KR-5 (Kapazitäts-Schwellenwerte):** Sobald die Ressourcenauslastung (CPU, RAM, Disk, I/O) dauerhaft 80 % überschreitet, MUSS das Capacity Monitoring Warnschwellen auslösen und Auto-Scaling oder Kapazitätserweiterungen initiieren.
6. **KR-6 (Laufzeit-Invarianz & Version-Sync):** Im Betrieb MUSS kontinuierlich verifiziert werden, dass ausschließlich autorisierte, unveränderte Release-Versionen ausgeführt werden. Nicht autorisierte Abweichungen MÜSSEN das System sofort sperren.

## §3 Artefakte & Nachweispflichten

- **Betriebshandbuch (Runbook):** Vollständige Dokumentation von Architektur, Konfiguration, Start-/Stopp-Prozeduren und Maintenance-Fenstern.
- **Backup- & Recovery-Protokoll:** Nachweise über durchgeführte Datensicherungen und Quartals-Recovery-Tests.
- **Maintenance- & Patch-Log:** Lückenlose Historie aller eingespielten Betriebssystem-, Abhängigkeits- und Anwendungs-Updates.
- **Kapazitätsplan:** Dokumentierte Prognose der Ressourcenentwicklung und Skalierungsanforderungen.

## §4 Lifecycle-State-Bezug

Der Standard pflegt und aktualisiert den maschinenlesbaren Lifecycle-State
(`.atc/lifecycle-state.yaml`) gemäß ATC-STD-V2S-000 §16:

- Sets `lifecycle.operations`: `active` | `maintenance` | `degraded` | `suspended`
- Sets `lifecycle.last_maintenance_date`: ISO-8601 Datum
- Verifiziert `lifecycle.maturity`: `M8` (OPERATING) für alle produktiven Dienste

## §5 Schnittstellen

- **Upstream:** ATC-STD-V2S-018 (Deployment Standard), Gate V2S-G9 (Operational Readiness).
- **Downstream:** ATC-STD-V2S-020 (Monitoring Standard), ATC-STD-V2S-021 (Incident Standard), ATC-STD-V2S-022 (Feedback Standard).
- **Andock-Standards:** ATC-STD-UPDATE-001 (Emergency Patching), ATC-STD-314 (Security Operations).

## Metriken & Akzeptanzkriterien

- **M1 (Service Availability / Uptime):** Die Uptime kritischer Produktivdienste beträgt ≥ 99,9 % pro Kalendermonat.
- **M2 (Recovery Point Objective, RPO):** Der maximale Datenverlust bei Wiederherstellung beträgt RPO < 15 Minuten.
- **M3 (Recovery Time Objective, RTO):** Die maximale Ausfallzeit bis zur vollständigen Service-Wiederherstellung beträgt RTO < 60 Minuten.
- **M4 (Patch SLA Compliance):** 100 % aller P0/P1-Sicherheits-Patches werden innerhalb der definierten Fristen eingespielt.

## Compliance & Verifikation

Die Einhaltung wird durch regelmäßige Betriebs-Audits, atc-repo-audit und automatisierte
Sicherheitschecks geprüft (S-01..S-25). Nicht eingehaltene SLAs, fehlende Runbooks
oder nicht nachgewiesene Recovery-Tests erzeugen Findings (F-001..F-999) nach ATC-STD-BUG-001.

## Security Considerations

Der operative Betrieb verarbeitet sensible Produktionsdaten. Betriebszugänge (SSH, RPC,
K8s API, Admin-Konsolen) MÜSSEN über Multi-Faktor-Authentifizierung (MFA) und Role-Based
Access Control (RBAC) abgesichert werden. Sämtliche Zugriffe und administrativen Aktionen
MÜSSEN in unveränderlichen Audit-Logs protokolliert werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-UPDATE-001, ATC-STD-314
**INFORMATIVE:** SCR-0042
