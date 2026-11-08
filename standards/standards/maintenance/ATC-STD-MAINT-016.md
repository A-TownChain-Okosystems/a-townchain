---
standard:
  id: ATC-STD-MAINT-016
  title: "ATC-STD-MAINT-016 — Compatibility Maintenance Standard"
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
  applies_to: "Alle versionierten Schnittstellen (ABI/API, Protokolle, Formate, Ketten-Kompatibilitaet)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-016 — Compatibility Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Backward-/Forward-Compatibility als Bindungsstandard; SSOT bleibt die COMPAT-Familie.

## §1 Verhaeltnis zur COMPAT-/UPDATE-Familie (Kollisionsvermeidung)

ATC-STD-COMPAT-001 (Kompatibilitaetspruefung/-Migration nach MAJOR-Updates, Pflicht-Gate UPD-G04
in UPDATE-001) bleibt SSOT. Dieser Standard bindet Compatibility an die MAINT-Familie:

1. Kompatibilitaetsbrueche werden mit M-Klasse klassifiziert (VM/Chain-Bezug: M3, sonst M1/M2).
2. Kompatibilitaetspruefungen erzeugen MAINT-Evidence (Bestandteil der Pruefkette, vgl. MAINT-009).

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-065 | Compatibility folgt COMPAT-001/UPDATE-001 als SSOT | MUST |
| REQ-MAINT-066 | Kompatibilitaetsbrueche werden mit M-Klasse klassifiziert und evidenced | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** COMPAT/UPDATE-Standards sind approved; die M-Klassen-Bindung ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
