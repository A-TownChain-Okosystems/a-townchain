---
standard:
  id: ATC-STD-MAINT-006
  title: "ATC-STD-MAINT-006 — Infrastructure Maintenance Standard"
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
  applies_to: "CI/CD, Runner, Build- und Release-Infrastruktur der Organisation"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-006 — Infrastructure Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Pflege der Build-/Release-Infrastruktur als M1-Disziplin (Runner, Workflow-Governance, Release-Pipelines).

## §1 Pruefbereiche

CI/CD-Workflows (Versionen der Actions, governance-konform, .github als Org-SSOT) · Runner-Gesundheit
(Warteschlangen, Backlogs) · Build-System (reproduzierbare Builds) · Release-Infrastruktur (Tags, Artefakte).

## §2 Regeln

1. Workflow-Aenderungen folgen der Org-Workflow-Governance (.github, ATC-STD-CI-Familie).
2. Runner-Backlogs und Pipeline-Ausfaelle werden als M1 priorisiert.
3. Reproduzierbarkeit: Builds sind deterministisch reproduzierbar (R10-Prinzip).

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-036 | Workflow-Aenderungen folgen der Org-Governance und versionierten Actions | MUST |
| REQ-MAINT-037 | Infrastruktur-Ausfaelle werden als M1 priorisiert | MUST |
| REQ-MAINT-038 | Builds sind reproduzierbar | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** Org-weite Governance-CI, Drift-Checks und Runner-Ueberwachung (manuell) bestehen; ein Runner-Health-Monitoring ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
