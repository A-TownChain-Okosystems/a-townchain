---
standard:
  id: ATC-STD-MAINT-009
  title: "ATC-STD-MAINT-009 — VM & Runtime Maintenance Standard"
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
  applies_to: "atc-vm, ATCLang-Runtime, VM-/Runtime-Schnittstellen (ABI/API, Contract Execution, Consensus-Anbindung)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-009 — VM & Runtime Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Pflege von ATC-VM, ABI/API, Runtime und Compatibility mit harter Pruefkette. P0-Governance-Standard.

## §1 Die harte Pruefkette (normativ)

Ein ATC-VM-Update ist NIEMALS ein normales Dependency-Update. Es prueft mindestens:

```
ATCLang compatibility → ATC-VM compatibility → Contract execution
   → State transition → Consensus → Node compatibility → Network compatibility
```

Jede Stufe erzeugt Evidence (MAINT-019); ein Abbruch einer Stufe blockiert das Update.

## §2 Regeln

1. VM-Inkompatibilitaet = M3 (Emergency-Governance; ABI/API-Bruch betrifft Chain und Vertraege).
2. ABI/API-Aenderungen folgen der Compatibility-Politik (MAINT-016, COMPAT-001) mit Versionierungs-Gate.
3. Determinismus der VM bleibt unberuehrt: jede Aenderung validiert gegen verifizierte Bytecode-Ausfuehrung.
4. SoD bei VM-Aenderungen zwingend (Implementer ≠ Validator ≠ Auditor).

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-045 | Die 7-Stufen-Pruefkette ist fuer jede VM-Aenderung verbindlich und evidenced | MUST |
| REQ-MAINT-046 | VM-Inkompatibilitaet wird als M3 klassifiziert | MUST |
| REQ-MAINT-047 | ABI/API-Aenderungen folgen MAINT-016/COMPAT-001 inkl. Versionierungs-Gate | MUST |
| REQ-MAINT-048 | VM-Aenderungen unterliegen Separation of Duties | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** atc-vm hat verifizierte Bytecode-Ausfuehrung und CI-Evidence; die formale 7-Stufen-Pruefkette ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
