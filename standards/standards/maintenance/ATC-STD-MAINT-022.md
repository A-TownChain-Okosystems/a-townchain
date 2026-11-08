---
standard:
  id: ATC-STD-MAINT-022
  title: "ATC-STD-MAINT-022 — End-of-Life & Retirement Standard"
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
  applies_to: "Alle Komponenten, Schnittstellen und Standards des Oekosystems (Deprecation, Sunset, Retirement)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-022 — End-of-Life & Retirement Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Geordneter Ausstieg: Deprecation → Sunset → Retirement mit koordinierten Consumern und Retirement-Evidence. Prioritaet P2.

## §1 Ausstiegsphasen (normativ)

1. **Deprecation:** Ankuendigung + Migrationspfad + Frist; Nutzung wird als Finding gefuehrt.
2. **Sunset:** keine neuen Features; nur noch sicherheitsrelevante Fixes (M2 weiter verbindlich).
3. **Retirement:** Archivierung (Zustand reversibel dokumentiert), Entfernung aus Profilen und
   Registry-Verweisen, Abschluss-Evidence.

## §2 Regeln

1. Kein stilles Entfernen (auch nicht von Standards — Sunset/Supersede statt Loeschung).
2. Vor Retirement: Consumer-Pruefung (wer nutzt die Komponente noch?) und Koordination aller
   Betroffenen (MAINT-024).
3. Archivierte Komponenten bleiben als Referenz konserviert (Zustand: archived, reversibel).
4. Retirement erzeugt einen vollstaendigen Evidence-Record (MAINT-019).

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-085 | Ausstieg nur ueber Deprecation → Sunset → Retirement (nie still) | MUST |
| REQ-MAINT-086 | Consumer werden vor Retirement geprueft und koordiniert | MUST |
| REQ-MAINT-087 | Retirement erzeugt vollstaendigen Evidence-Record | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** Einzelne Archivierungen sind durchgefuehrt (private Repos, reversibel); der formale 3-Phasen-Prozess ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
