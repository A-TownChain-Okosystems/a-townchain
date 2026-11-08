---
standard:
  id: ATC-STD-MAINT-003
  title: "ATC-STD-MAINT-003 — Code Maintenance Standard"
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
  applies_to: "Quellcode aller Repositories (Refactoring, Technical Debt, Deprecations)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-003 — Code Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Refactoring, Abbau technischer Schulden und geordnete Deprecations als planbare M0/M1-Disziplin.

## §1 Pruefbereiche

Refactoring (Verhaltensinvarianz via Tests) · Technical Debt (Registro und Priorisierung im
Maintenance-Backlog) · Deprecations (Ankuendigung, Migrationshinweis, Entfernung im geplanten Zyklus,
niemals still).

## §2 Einordnung

Code-Qualitaet folgt ATC-STD-ENG-001 und registry/code-quality-matrix.yaml; dieser Standard bindet
die Pflege an M-Klassifikation und Lifecycle.

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-025 | Refactorings sind verhaltensinvariant und testgesichert | MUST |
| REQ-MAINT-026 | Technical Debt wird im Maintenance-Backlog gefuehrt (KPI Technical Debt) | MUST |
| REQ-MAINT-027 | Deprecations werden ankuendigt, dokumentiert und planbar entfernt | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** Code-Qualitaets-Gates (ruff, clippy) bestehen; ein Technical-Debt-Register je Repository ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
