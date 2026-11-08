---
standard:
  id: ATC-STD-MAINT-018
  title: "ATC-STD-MAINT-018 — Rollback & Recovery Standard"
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
  applies_to: "Alle Deployment-/Release-Vorgaenge des Oekosystems (Rollback, Restore, Failover)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-018 — Rollback & Recovery Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Rollback, Restore und Failover als P0-Disziplin: kein CLOSE ohne getesteten Rollback bei M2/M3.

## §1 Pruefbereiche

Rollback (Pfad vorhanden, getestet, dokumentiert) · Restore (Daten/State) · Failover
(fuer verteilte Komponenten) · A/B-Updates (OS: Version N → N+1 → Health Check → PASS: commit / FAIL: rollback).

## §2 Regeln (normativ)

1. M2/M3-Aenderungen schliessen NICHT ohne verfuegbaren UND getesteten Rollback (Evidence-Feld rollback.available + rollback.tested).
2. OS-/Plattform-Updates folgen dem A/B-Prinzip mit Health-Check-Entscheidung.
3. KPI: Rollback Success Rate (generiert aus Recovery-Tests).
4. Failover-Mechanismen werden wie Rollbacks regelmaeßig getestet (M1).
```
Version N → Update → N+1 → Health Check → PASS: commit
                                      → FAIL: rollback → N (verifiziert)
```

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-069 | M2/M3 ohne getesteten Rollback: kein CLOSE (rollback.available + rollback.tested) | MUST |
| REQ-MAINT-070 | OS-/Plattform-Updates laufen nach A/B-Prinzip mit Health-Check | MUST |
| REQ-MAINT-071 | Rollback Success Rate wird als KPI gefuehrt | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** A/B-Update-Mechanismus ist Zielarchitektur (ATC-DOC-ARC-GLOB-002 §14); ein implementierter Rollback-Rahmen ist NICHT vorhanden. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
