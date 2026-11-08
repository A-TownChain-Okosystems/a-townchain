---
standard:
  id: ATC-STD-V2S-018
  title: "Deployment Standard"
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

# ATC-STD-V2S-018 — Deployment Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-018 regelt den Auslieferungsprozess (Deployment) von qualifizierten
Release Candidates in Zielumgebungen (Staging, Production) im ATC Vision-to-Software
Lifecycle. Der Standard beschreibt die harten Kriterien für Gate V2S-G9, die
verbindliche Deployment-Pipeline, den automatisierten Health Check und die
ausnahmslose Rollback-Pflicht im Fehlerfall.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** Alle Software-Auslieferungen, Service-Deployments, Smart-Contract-
Deployments und KI-Agenten-Instanziierungen in der ATC-Infrastruktur.
**Nicht-Gilt:** Lokale Entwickler-Builds, ephemeral Test-Runner und reine
Dokumentations-Publikationen.

## §1 Gegenstand & Verortung

Deployment ist Station 14 im V2S-Lifecycle (ATC-STD-V2S-000 §1). Es verbindet die
Release-Erstellung (V2S-017 / Gate V2S-G8) mit dem Produktivbetrieb (V2S-019 /
Operations). Es transformiert ein immutable Release Artifact in einen laufenden
Systemdienst.

Das verbindliche Ablaufmuster lautet:
`CURRENT → DEPLOY → HEALTH CHECK → SUCCESS / FAILURE → ROLLBACK`

- **CURRENT:** Der aktuell aktive, stabile Produktionszustand (vorn-1).
- **DEPLOY:** Das Einspielen und Starten des neuen Release-Builds (vorn).
- **HEALTH CHECK:** Die automatisierte Verifizierung aller Vital-Parameter.
- **SUCCESS:** Umschaltung des Produktiv-Traffics und Status-Setzung auf active.
- **FAILURE:** Sofortiger Abbruch und automatisierter Rollback auf CURRENT.

## §2 Kernregeln

1. **KR-1 (Gate V2S-G9 Freigabe):** Kein Deployment in Produktivumgebungen DARF ohne erfolgreiches Bestehen des Quality Gates V2S-G9 (Betriebs- und Auslieferungsreife) gestartet werden (MUST NOT).
2. **KR-2 (Ablauf-Pipeline):** Jedes Deployment MUSS exakt der Pipeline `CURRENT → DEPLOY → HEALTH CHECK → SUCCESS / FAILURE → ROLLBACK` folgen. Ein Überspringen der Phase HEALTH CHECK ist VERBOTEN (MUST NOT).
3. **KR-3 (Automatisierter Health Check):** Ein Deployment MUSS innerhalb eines definierten Zeitfensters (default: 300 Sekunden) automatische Health Checks (HTTP Liveness/Readiness, RPC Ping, Service-Status) erfolgreich bestehen.
4. **KR-4 (Rollback-Pflicht):** Schlägt der Health Check fehl oder treten während des Deployments P0/P1-Fehler auf, MUSS das System unverzüglich und vollautomatisch auf den vorherigen Zustand CURRENT zurückgerollt werden (MUST).
5. **KR-5 (Unveränderbare Artefakte):** Für das Deployment MÜSSEN exakt die unveränderlichen Artefakte (Container Image Digests, Binary Checksums, Smart Contract Bytecode) verwendet werden, die in Gate V2S-G8 freigegeben wurden.
6. **KR-6 (Protokollierung & Traceability):** Jedes Deployment MUSS mit Zeitstempel, ausführender Instanz, Git Commit Hash, Artifact Digest und Ergebnis im Deployment-Log protokolliert werden.

## §3 Artefakte & Nachweispflichten

- **Deployment-Plan:** Dokumentiert Zielumgebung, Strategie (Blue/Green, Canary, Rolling), Parameter und beteiligte Services.
- **Rollback-Plan:** Technisch erprobter und automatisierter Pfad zur Wiederherstellung des Zustands CURRENT.
- **Deployment-Log:** Maschinenlesbares Protokoll aller Pipeline-Schritte inklusive Health-Check-Ergebnissen.
- **Gate-Record V2S-G9:** Nachweis der Freigabe für die Auslieferung gemäß ATC-STD-V2S-000 §9.

## §4 Lifecycle-State-Bezug

Der Standard pflegt und aktualisiert den maschinenlesbaren Lifecycle-State
(`.atc/lifecycle-state.yaml`) gemäß ATC-STD-V2S-000 §16:

- Sets `lifecycle.deployment`: `in_progress` | `complete` | `failed` | `rolled_back`
- Sets `lifecycle.last_deployment_timestamp`: ISO-8601 Zeitstempel
- Sets `lifecycle.active_version`: Aktuelle Versionsnummer bei SUCCESS
- Updates `lifecycle.maturity`: `M8` (OPERATING) nach erfolgreichem Deployment

## §5 Schnittstellen

- **Upstream:** ATC-STD-V2S-017 (Release Standard), Gate V2S-G8 (Release Ready), ATC-STD-203 (Release Management).
- **Downstream:** ATC-STD-V2S-019 (Operations Standard), Gate V2S-G9 (Operational Readiness), ATC-STD-V2S-020 (Monitoring Standard).
- **Andock-Standards:** ATC-STD-UPDATE-001 (Emergency Updates), ATC-STD-NET-006 (Netzwerk- und Protokoll-Upgrades).

## Metriken & Akzeptanzkriterien

- **M1 (Deployment Success Rate):** ≥ 98 % aller Produktiv-Deployments schließen ohne manuellen Eingriff erfolgreich ab.
- **M2 (Mean Time to Rollback, MTTRb):** Automatische Rollbacks dauern im Fehlerfall < 120 Sekunden.
- **M3 (Health Check Coverage):** 100 % aller deployten Services besitzen mindestens einen aktiven Liveness- und Readiness-Probe.
- **M4 (Traceability):** 100 % der Produktiv-Deployments sind lückenlos auf ein V2S-G8 freigegebenes Release-Artefakt rückverfolgbar.

## Compliance & Verifikation

Die Einhaltung dieses Standards wird automatisiert über CI/CD-Pipeline-Audits,
atc-repo-audit und den Lifecycle-State-Validator geprüft (S-01..S-25).
Verstöße gegen MUST-Regeln (z.B. manuelles Übergehen des Health Checks oder
fehlender Rollback-Plan) führen zu Findings (F-001..F-999) nach ATC-STD-BUG-001.

## Security Considerations

Deployments stellen kritische Angriffsflächen dar. Alle Deployment-Pipelines MÜSSEN
über getrennte Service-Accounts mit Least-Privilege-Rechten verfügen. Secrets,
Private Keys und API-Tokens DÜRFEN NICHT im Deployment-Log oder in Artefakten im
Klartext erscheinen, sondern MÜSSEN über sichere Key-Management-Systeme injiziert
werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-203, ATC-STD-UPDATE-001, ATC-STD-NET-006
**INFORMATIVE:** SCR-0042
