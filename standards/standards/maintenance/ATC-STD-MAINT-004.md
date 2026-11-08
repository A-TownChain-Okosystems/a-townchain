---
standard:
  id: ATC-STD-MAINT-004
  title: "ATC-STD-MAINT-004 — Dependency Maintenance Standard"
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
  applies_to: "Alle Abhaengigkeiten aller Repositories (Dependencies, Lockfiles, SBOM, Updates)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-004 — Dependency Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Verbindliche Pflege von Abhaengigkeiten: Lockfile-SSOT, SBOM, Update-Pfade und CVE-Reaktion. P0-Governance-Standard.

## §1 Pruefbereiche

Dependencies (aktuell, keine ungenutzten, keine verwurzelten Forks) · Lockfiles (ein Lockfile je
Manifest, committet; Workspace-Root fuer Workspace-Member) · SBOM (fuer Releases) · Updates
(planbare Zyklen, M0) · CVE-Reaktion (M2, darf Release-Zyklen vorlagern).

## §2 Bestehende Referenz-Implementierung (Evidence)

Org-weites cargo audit (RustSec) als CI-Gate in allen Rust-Repos (push, PR, Wochenplan);
zwei-phasige Workspace-Logik; zero RustSec-Befunde zum Aufbauzeitpunkt.

## §3 Regeln

1. Lockfile ist SSOT der Abhaengigkeitsaufloesung; Commits ohne Lockfile-Update bei Manifest-Aenderung sind unzulaessig.
2. Jedes Release erzeugt SBOM-Nachweis.
3. Kompromittierte Abhaengigkeit = M2 (Security Review, vorlagerbar).
4. KPI: Dependency Freshness, Patch Latency (generiert).

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-028 | Lockfile-SSOT je Manifest; Manifest-Aenderung ohne Lockfile-Update unzulaessig | MUST |
| REQ-MAINT-029 | Releases erzeugen SBOM-Nachweis | MUST |
| REQ-MAINT-030 | Kompromittierte Abhaengigkeit wird als M2 klassifiziert und priorisiert | MUST |
| REQ-MAINT-031 | Dependency Freshness und Patch Latency werden als generierte KPIs gefuehrt | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** cargo audit ist org-weit live; SBOM-Erzeugung je Release und ein allgemeines Lockfile-Gate sind NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
