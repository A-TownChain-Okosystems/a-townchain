---
standard:
  id: ATC-STD-MAINT-005
  title: "ATC-STD-MAINT-005 — Security Maintenance Standard"
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
  applies_to: "Sicherheitsrelevante Maintenance: CVEs, Hardening, Secrets, Keys, Sandbox, Permissions"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-005 — Security Maintenance Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000 ·
> Klassifikation: MAINT-001 · Lifecycle: MAINT-002 · Evidence: MAINT-019.

## Abstract

Sicherheitspflege als M2-Disziplin mit vorlagerbaren Release-Zyklen, Disclosure-Vorsicht und SoD. P0-Governance-Standard.

## §1 Pruefbereiche

CVEs · Supply-Chain-Risiken · Secrets (keine im Repo; Scanner-Waechter) · Keys (Rotation planbar,
Verlust = M3) · Hardening · Sandbox/Permissions (Escape/Bypass = M2, mit Kernel-Bezug M3).

## §2 Regeln

1. Security Maintenance darf normalen Release-Zyklen vorgelagert werden (M2).
2. Disclosure-Vorsicht: bei aktiv ausnutzbaren CVEs Fix vor Oeffentlichkeit; Secrets-Nachziehen (Rotation) sofort.
3. SoD: Implementer ≠ Validator ≠ Auditor; Release Authority ≠ Implementer bei besonders kritischen Aenderungen.
4. Verkettung mit ATC-STD-203 (Repository Security & Release), cargo audit + CodeQL-Gates (bestehend).

## §3 KPI

Patch Latency (CVE-Fund bis Patch), MTTD/MTTR fuer Security-Findings (generiert).

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-032 | CVEs werden als M2 klassifiziert und vorlagerbar behandelt | MUST |
| REQ-MAINT-033 | Aktiv ausnutzbare CVEs: Fix vor Oeffentlichkeit (Disclosure-Vorsicht) | MUST |
| REQ-MAINT-034 | Secrets duerfen nie committet werden; Rotation bei Verlust sofort (M3 bei Vertrauensbruch) | MUST |
| REQ-MAINT-035 | M2-Aenderungen unterliegen Separation of Duties | MUST |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** cargo audit + CodeQL laufen org-weit; ein formaler Security-Review-Pfad mit SoD-Vermerk ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Kernprofil: ATC-STD-000, ATC-STD-201/202/203, ATC-STD-ENG-001.
