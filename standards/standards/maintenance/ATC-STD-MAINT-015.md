---
standard:
  id: ATC-STD-MAINT-015
  title: "ATC-STD-MAINT-015 — Reliability Maintenance Standard"
  version: "1.0.0"
  status: approved
  category: maint
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-14"
  updated: "2026-09-14"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-14"
  review_date: null
  applies_to: "Alle produktiven Komponenten (Monitoring, Backup, Recovery, Regressionstests)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-015 — Reliability Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Pflege von Monitoring, Backup, Recovery und Regressionstests.

## §1 Pruefbereiche

Monitoring (lokale Observability, Health) · Backup (Plan, Takt, Verification) · Recovery
(getestete Wiederherstellung; Recovery-Tests sind M1) · Regressionstests (Suite pflegen).

## §2 Regeln

1. Recovery-Tests sind planbare M1-Aufgaben (KPI: Recovery Readiness).
2. Monitoring ist lokal (keine ungepruefte Telemetrie nach aussen — ATC-DOC-ARC-GLOB-002 §15).
3. Backups sind verifiziert (Restore-Probe), nicht nur erzeugt.

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-062 | Recovery wird regelmaeßig getestet (Recovery Readiness KPI) | MUST |
| REQ-MAINT-063 | Monitoring bleibt lokal (keine ungepruefte ausgehende Telemetrie) | MUST |
| REQ-MAINT-064 | Backups werden per Restore-Probe verifiziert | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** Test-Suiten und lokale Logs bestehen; ein Monitoring-/Backup-Regelbetrieb ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
