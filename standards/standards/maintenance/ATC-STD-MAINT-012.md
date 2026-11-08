---
standard:
  id: ATC-STD-MAINT-012
  title: "ATC-STD-MAINT-012 — Standards Maintenance Standard"
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
  applies_to: "atc-standards (Standards, Registry, Schemas, Conformance)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-012 — Standards Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Pflege von Standards, Registry, Schemas und Conformance im atc-standards-Repository.

## §1 Pruefbereiche

Standards (Aktualitaet, Sunset, §9-Status) · Registry (Integritaet, keine Waisen/Duplikate) ·
Schemas (Versionierung) · Conformance (Profile, Drift-Checks).

## §2 Regeln

1. Standards-Aenderungen laufen ausschliesslich via SCR + Views-Regeneration (bestehende Gates).
2. Registry-Integritaet ist CI-erzwungen (R1-R15, Cross-Registry-Check) — Maintenance hier ist
   die Pflege dieser Gates selbst.
3. Veraltete Standards werden via Sunset/Supersede gefuehrt, nie still entfernt.

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-054 | Standards-Aenderungen nur via SCR inkl. Views-Regeneration | MUST |
| REQ-MAINT-055 | Registry-Integritaets-Gates werden instand gehalten (R1-R15) | MUST |
| REQ-MAINT-056 | Auslaufende Standards folgen Sunset/Supersede, nie stiller Entfernung | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** Die beschriebenen Gates (SCR, Drift-Checks, Cross-Registry R1-R15) sind bestehend und live. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
