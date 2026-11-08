---
standard:
  id: ATC-STD-MAINT-024
  title: "ATC-STD-MAINT-024 — Cross-Ecosystem Maintenance Standard"
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
  applies_to: "Alle Wartungsaufgaben, die mehrere Repositories und/oder Oekosystem-Schichten gleichzeitig betreffen"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-024 — Cross-Ecosystem Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Koordinierte Maintenance ueber Repo- und Schichtgrenzen hinweg: eine Aenderungsnummer, konsolidierte Evidence, gemeinsam gepruefte Kompatibilitaet. Prioritaet P2.

## §1 Geltungsbereich

Cross-Repo-/Cross-Schicht-Aenderungen: Registry- und Standards-Syncs, Plattform-Updates
(z.B. ATC-VM), org-weite Workflow-Aenderungen, API-/Schema-Aenderungen mit mehreren Consumern.

## §2 Regeln (normativ)

1. Cross-Ecosystem-Aenderungen laufen unter EINER Change-Nummer ueber alle betroffenen Repos
   (konsolidierter Evidence-Record, MAINT-019).
2. Betroffenheit wird Registry-basiert bestimmt (Abhaengigkeits-/Consumer-Pruefung), nicht geraten.
3. Kompatibilitaet wird an allen Schichtgrenzen geprueft (MAINT-000 §24 Ecosystem Integration; Pruefkette MAINT-009 als
   Muster).
4. Reihenfolge: Kernel-nahe Schichten zuerst, dann aufwaerts (Hardware → ATCLang → Chain → VM →
   OS → AI), sofern die Aenderung nicht ausdruecklich anders erfordert.
5. Bestehende Cross-Repo-Muster bleiben verbindliche Referenz: Views-Sync, Version-Gate,
   Cross-Registry-Check R1-R15.

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-092 | Cross-Ecosystem-Aenderungen nutzen eine Change-Nummer und konsolidierte Evidence | MUST |
| REQ-MAINT-093 | Betroffenheit wird Registry-basiert bestimmt | MUST |
| REQ-MAINT-094 | Kompatibilitaetspruefung an allen betroffenen Schichtgrenzen | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** Cross-Repo-Syncs sind gelebte Praxis (Views-Sync 28 Repos, Version-Gate, 11-Repo-cargo-audit-Rollout); ein formales Koordinationsverfahren mit konsolidierter Evidence ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
