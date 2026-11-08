---
standard:
  id: ATC-STD-V2S-021
  title: "Incident Standard"
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

# ATC-STD-V2S-021 — Incident Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-021 regelt das Incident Management für ungeplante Unterbrechungen, Leistungseinbußen
oder Sicherheitsvorfälle im ATC Vision-to-Software Lifecycle. Der Standard definiert
Severity-Stufen, Reaktions-SLAs, Eskalationspfade, verpflichtende Blameless Postmortems
sowie die Kopplung an das Finding- und Blockierungsmodell gemäß ATC-STD-BUG-001.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** Alle ungeplanten Störungen, Ausfälle, Sicherheitsvorfälle und Qualitätsabweichungen
in produktiven ATC-Systemen, Protokollen, Smart Contracts und KI-Agenten.
**Nicht-Gilt:** Geplante Wartungsfenster mit Vorankündigung und Fehler in lokalen Testumgebungen.

## §1 Gegenstand & Verortung

Incident Management ist eine kritische Betriebsfunktion im V2S-Lifecycle (ATC-STD-V2S-000 §13).
Es wird durch Alarme des Monitorings (V2S-020) oder externe Meldungen ausgelöst, stellt die
Service-Verfügbarkeit schnellstmöglich wieder her und speist die gewonnenen Erkenntnisse
über Postmortems direkt in das Feedback-System (V2S-022) und die Bug-Verwaltung (ATC-STD-BUG-001) ein.

Der Incident Lifecycle gliedert sich in:
`DETECTION → TRIAGE → CONTAINMENT → RESOLUTION → POSTMORTEM → FINDING`

## §2 Kernregeln

1. **KR-1 (Severity-Klassifizierung):** Jeder gemeldete Incident MUSS unverzüglich in eine der vier Schweregrade eingestuft werden: SEV-1 (Kritisch/Totalausfall), SEV-2 (Hoch/Schwere Beeinträchtigung), SEV-3 (Mittel/Teilfunktion betroffen), SEV-4 (Niedrig/Geringe Wirkung).
2. **KR-2 (Triage SLA):** Die Erst-Bestätigung und Triage MUSS innerhalb strikter SLAs erfolgen: SEV-1 < 15 Minuten, SEV-2 < 30 Minuten, SEV-3 < 4 Stunden, SEV-4 < 24 Stunden.
3. **KR-3 (Eindämmung vor Ursachenanalyse):** Während eines aktiven Incidents MUSS die schnellstmögliche Wiederherstellung des Betriebs (Containment / Workaround / Rollback) höchste Priorität vor der detaillierten Ursachenanalyse haben.
4. **KR-4 (Blameless Postmortem-Pflicht):** Für jeden SEV-1 und SEV-2 Incident MUSS innerhalb von 72 Stunden nach Behebung ein schuldfreies Postmortem mit Root Cause Analysis (RCA), Zeitstrahl und korrektiven Action Items erstellt werden.
5. **KR-5 (Kopplung an ATC-STD-BUG-001):** Aus Incidents resultierende technische Ursachen MÜSSEN als Findings (F-001..F-999) nach ATC-STD-BUG-001 erfasst und mit der Severity-Zuordnung S0..S4 verknüpft werden.
6. **KR-6 (P0-P3 Blockierung):** Ungelöste Ursachen aus SEV-1 und SEV-2 Incidents werden als P0/P1-Blocker klassifiziert. Sie MÜSSEN weitere Releases blockieren, bis der korrektive Patch erfolgreich deployed und verifiziert ist.

## §3 Artefakte & Nachweispflichten

- **Incident Record:** Maschinenlesbare Erfassung von Incident-ID, Zeitstempel, Betroffenen Diensten, Severity und Incident Commander.
- **Incident Communications Log:** Dokumentation der Benachrichtigungen an Stakeholder und Status-Page-Updates.
- **Postmortem Report:** Schuldfreier Bericht über Ursachen, Auswirkungen, Sofortmaßnahmen und präventive Aufgaben.
- **Finding Record (F-NNN):** Registrierter Fehlerbericht nach ATC-STD-BUG-001 zur Nachverfolgung in der Traceability-Matrix.

## §4 Lifecycle-State-Bezug

Der Standard pflegt und aktualisiert den maschinenlesbaren Lifecycle-State
(`.atc/lifecycle-state.yaml`) gemäß ATC-STD-V2S-000 §16:

- Sets `lifecycle.incident_state`: `none` | `active` | `contained` | `resolved`
- Sets `lifecycle.active_incident_severity`: `SEV-1` | `SEV-2` | `SEV-3` | `SEV-4`
- Tracks `lifecycle.open_p0_p1_findings_count`: Anzahl offener P0/P1-Blocker

## §5 Schnittstellen

- **Upstream:** ATC-STD-V2S-020 (Monitoring Standard), ATC-STD-V2S-019 (Operations Standard).
- **Downstream:** ATC-STD-V2S-022 (Feedback Standard), ATC-STD-V2S-023 (Improvement Standard).
- **Andock-Standards:** ATC-STD-BUG-001 (Finding Severities S0..S4), ATC-STD-UPDATE-001 (Emergency Hotfixes).

## Metriken & Akzeptanzkriterien

- **M1 (Mean Time to Detect, MTTD):** Automatische Erkennung von SEV-1 Incidents in MTTD < 5 Minuten.
- **M2 (Mean Time to Acknowledge, MTTA):** Triage-Bestätigung bei SEV-1 in MTTA < 15 Minuten.
- **M3 (Mean Time to Restore, MTTR):** Wiederherstellung des regulären Betriebs bei SEV-1 in MTTR < 60 Minuten.
- **M4 (Postmortem Completion Rate):** 100 % aller SEV-1 und SEV-2 Incidents besitzen innerhalb von 72 Stunden ein freigegebenes Postmortem.

## Compliance & Verifikation

Die Einhaltung wird durch automatisierte SLA-Überwachung im Incident-Tooling, atc-repo-audit
und Audits der Bug-Register verifiziert (S-01..S-25). Verletzte SLAs oder fehlende Postmortems
erzeugen Findings (F-001..F-999) nach ATC-STD-BUG-001.

## Security Considerations

Bei sicherheitsrelevanten Incidents MÜSSEN Vertraulichkeitsstufen beachtet werden.
Detaillierte Angriffsvektoren DÜRFEN erst nach erfolgreicher Behebung und Patch-Distribution
in öffentlichen Postmortems offenbart werden. Der Zugriff auf Incident-Kanäle MUSS
auditierbar protokolliert werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-UPDATE-001
**INFORMATIVE:** SCR-0042
