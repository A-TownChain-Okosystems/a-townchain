---
standard:
  id: ATC-STD-MAINT-020
  title: "ATC-STD-MAINT-020 — Maintenance Automation Standard"
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
  applies_to: "Alle Repositories (Scanning, Gates, Remediation, Maintenance Engine)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-020 — Maintenance Automation Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Automatisierte Detection, Gates und Remediation: Repository-Scanner → Maintenance Engine → M0 Auto / M1 Review / M2-M3 Escalate.

## §1 Zielarchitektur

```
Repository: Dependency Scanner · Vulnerability Scanner · SBOM Generator · License Scanner
   · Static Analysis · Test Suite · Performance Tests · Compatibility Tests · Repository Governance
        → Maintenance Engine
             M0 → Auto     M1 → Review     M2/M3 → Escalate
```

## §2 Automatisierungsleiter (verbindliche Richtung)

1. Stufe 1 (bestehend): Detection-Gates in CI (cargo audit, CodeQL, Drift-Checks, Tests).
2. Stufe 2 (geplant): Maintenance Engine klassifiziert Befunde automatisch vor (M0-Vorschlag),
   M2/M3 werden ESCALIERT (nie automatisch bearbeitet).
3. Stufe 3 (geplant): automatische M0-Remediation mit vollstaendigem Evidence-Record (MAINT-019).

## §3 Grenzen

1. Keine autonome M2/M3-Durchfuehrung: Eskalation an Humans (Owner/Governance), immer.
2. Automation folgt ATC-STD-AI-DEV-001..012 (Agent-Governance) und darf Gates nicht umgehen.

## §4 Systemintegritaet (Verkettung, MAINT-000 §23 GSEPF Integration)

```
ATC-STD-MAINT-* (normativ) → GSEPF (kontrollierter Prozess) → Maintenance Engine (CI/CD)
   → M0 Automate / M1 Review / M2+M3 Escalate → Evidence → Verified State
```

ATC-STD-MAINT-* definiert die normative Ebene, GSEPF setzt sie als kontrollierten Prozess um,
CI/CD bzw. Maintenance Automation erzeugen die tatsaechlichen Evidence-Records. Dadurch ist
Maintenance ein Bestandteil der Systemintegritaet — nicht bloss ein organisatorisches To-do.

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-076 | Detection-Gates in CI sind verbindlich (bestehend: cargo audit/CodeQL/Drift-Checks) | MUST |
| REQ-MAINT-077 | M2/M3-Befunde werden eskaliert, niemals autonom bearbeitet | MUST |
| REQ-MAINT-078 | Automatische M0-Remediation erzeugt vollstaendige Evidence-Records | MUST |
| REQ-MAINT-079 | Automation umgeht niemals Governance-Gates | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** Stufe 1 ist org-weit live; Maintenance Engine (Stufe 2/3) ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
