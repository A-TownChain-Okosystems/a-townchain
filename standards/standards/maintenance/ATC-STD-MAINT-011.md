---
standard:
  id: ATC-STD-MAINT-011
  title: "ATC-STD-MAINT-011 — Repository Maintenance Standard"
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
  applies_to: "Alle Repositories (Git, Branches, Issues, Actions, CODEOWNERS)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-011 — Repository Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Familien-Bindung der Repository-Pflege (Git, Branches, Issues, Actions, CODEOWNERS) an M-Klassifikation und MAINT-Lifecycle; SSOT bleibt ATC-STD-REPO-MAINT-001.

## §1 Verhaeltnis zu ATC-STD-REPO-MAINT-001 (Kollisionsvermeidung)

ATC-STD-REPO-MAINT-001 (SCR-0043) bleibt SSOT des Repository-Pflegezyklus (16 Stationen, P0-P3,
Pflicht-Checkliste). Dieser Standard normiert NUR die Einbindung in die MAINT-Familie:

1. Repository-Pflegeaufgaben werden zusaetzlich mit M0-M3 klassifiziert (MAINT-001).
2. Repository-Maintenance folgt dem 13-Phasen-Lifecycle (MAINT-002) mit Evidence-Record (MAINT-019).
3. Git-Hygiene, Branch-/Issue-Pflege und CODEOWNERS folgen REPO-MAINT-001 — keine Doppelnormierung.

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-052 | Repository-Pflege folgt REPO-MAINT-001 als SSOT (keine Doppelnormierung) | MUST |
| REQ-MAINT-053 | Repository-Maintenance-Items tragen M-Klassen und MAINT-Evidence | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** REPO-MAINT-001 ist approved; die M-Klassen-Bindung ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
