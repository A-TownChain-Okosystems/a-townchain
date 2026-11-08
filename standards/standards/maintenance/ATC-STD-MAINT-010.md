---
standard:
  id: ATC-STD-MAINT-010
  title: "ATC-STD-MAINT-010 — AI Maintenance Standard"
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
  applies_to: "aurora-ai und AI-Komponenten (Modelle, Registry, Inference Runtime, Evaluation)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-010 — AI Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Pflege von Modellen, Registry, Inference-Runtime und Evaluation der Aurora-AI-Plattform.

## §1 Pruefbereiche

Modelle (Versionierung, Evaluation vor Deployment) · AI-Registry (Agent-Identitaet, Modellverzeichnis) ·
Inference Runtime (Ressourcen, Stabilitaet) · Evaluation (Regression der Modellqualitaet).

## §2 Regeln

1. Modell-Updates sind adaptive Maintenance: Evaluation-Gate vor Deployment (Qualitaets-Regression pruefen).
2. Die Aurora-Policy-Kette (KI erkennt → Policy entscheidet → System fuehrt aus) bleibt unberuehrt: keine Maintenance-Aenderung erweitert AI-Systemrechte.
3. Inference-Ausfaelle sind M1; Policy- oder Permission-Bypass ist M2 (Verweis MAINT-005).

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-049 | Modell-Updates durchlaufen ein Evaluation-Gate vor Deployment | MUST |
| REQ-MAINT-050 | Maintenance erweitert niemals AI-Systemrechte (Policy-Kette unberuehrt) | MUST |
| REQ-MAINT-051 | Evaluation-Regressionen werden als Finding gefuehrt | SHOULD |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** aurora-ai (Core, Agents, Memory, Runtime, AI-Studio) existiert mit CI-Evidence; ein Evaluation-Gate fuer Modell-Updates ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
