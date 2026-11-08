---
standard:
  id: ATC-STD-MAINT-019
  title: "ATC-STD-MAINT-019 — Maintenance Evidence Standard"
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
  applies_to: "Alle Maintenance-Aufgaben des Oekosystems (Nachweis- und Audit-Evidence)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-019 — Maintenance Evidence Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Verbindliches maschinenlesbares Evidence-Schema fuer Maintenance-Aufgaben (No Evidence, No Trust). P0-Governance-Standard.

## §1 Verbindliches Record-Schema (MAINT-Records, ID-Schema MAINT-YYYY-NNNN)

```yaml
maintenance:
  id: MAINT-2026-0001
  classification: M1
  component: atc-node
  detected: 2026-09-13
  reason: dependency_update
  change:
    before: "x.y.z"
    after: "x.y.z+1"
  validation:
    unit_tests: PASS
    integration_tests: PASS
    security_scan: PASS
    compatibility: PASS
    performance: PASS
  rollback:
    available: true
    tested: true
  evidence:
    commit: "<sha>"
    pull_request: "<url/id>"
    test_report: "<ref>"
    audit_report: "<ref>"
  status: CLOSED
```

## §2 Regeln (normativ)

1. Kein CLOSE ohne vollstaendigen Record (alle validation-Felder bearbeitet, Evidence-Referenzen gesetzt).
2. 'Update durchgefuehrt' ist KEIN gueltiger Abschluss.
3. Records werden maschinenlesbar im Repo (`docs/maintenance/`) oder Registry gefuehrt und sind auditierbar.
4. Maschinenlesbare Schemas (MAINT-000 §20.1 Machine-Readable Conformance): `schemas/maintenance/maintenance-record.schema.json`
   (Record), `schemas/maintenance/maintenance-evidence.schema.json` (Evidence-Envelope).
4. M2/M3 zusaetzlich: SoD-Vermerk (Implementer/Validator/Auditor) und Security- bzw. Emergency-Report.
5. Evidence Completeness ist KPI (Anteil vollstaendiger Records, generiert).

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-072 | Jede Maintenance-Aufgabe schliesst mit Record im §1-Schema | MUST |
| REQ-MAINT-073 | Kein CLOSE ohne vollstaendige validation- und evidence-Felder | MUST |
| REQ-MAINT-074 | M2/M3-Records enthalten SoD-Vermerke | MUST |
| REQ-MAINT-075 | Records sind maschinenlesbar gespeichert und auditierbar | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** Das Schema ist in diesem Standard definiert; Record-Erstellung/-Validierung in Workflows ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
