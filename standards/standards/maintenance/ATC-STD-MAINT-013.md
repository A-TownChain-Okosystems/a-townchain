---
standard:
  id: ATC-STD-MAINT-013
  title: "ATC-STD-MAINT-013 — Documentation Maintenance Standard"
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
  applies_to: "Alle Repositories (README, Wiki, API-/Architekturdokumentation, Docs)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-013 — Documentation Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Pflege von README, Wiki, API- und Architekturdokumentation als M0-Disziplin mit Drift-Checks.

## §1 Pruefbereiche

README (Claims vs. Evidence — README-Drift-Check bestehend) · Wiki/Docs (Aktualitaet) ·
API-Dokumentation (Sync mit Implementierung) · Architekturdokumentation (Sync mit Registry).

## §2 Regeln

1. README-Claims sind gegen CI-Evidence pruefbar (Evidence-First; bestehender Drift-Check).
2. Dokumentationskorrekturen sind M0; falsche Claims (CLAIMED ohne Evidence) sind Findings (M1).
3. Generierbare Views werden generiert, nicht handgepflegt (REQ-IMP-006).

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-057 | README-Claims bleiben gegen CI-Evidence pruefbar | MUST |
| REQ-MAINT-058 | Doku-Aktualisierung erfolgt im selben PR wie die zugehoerige Aenderung | MUST |
| REQ-MAINT-059 | Generierbare Doku-Views werden generiert | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** README-/Views-Drift-Checks sind bestehend und live; Wiki-/API-Sync-Gates sind NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
