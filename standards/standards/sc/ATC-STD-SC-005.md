---
standard:
  id: ATC-STD-SC-005
  title: "ATC-STD-SC-005 — Smart Contract Audit Standard"
  version: "1.0.0"
  status: approved
  category: sc
  authority: A-TownChain-Okosystems
  owner: Michael (Owner-Entwurf) / ATC-AI-ARCH-001 (Formalfassung)
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  applies_to: "Alle Smart Contracts des A-TownChain-Oekosystems"
  supersedes: []
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  superseded_by: null
---

# ATC-STD-SC-005 — Smart Contract Audit Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Verbindliche Audit-Pipeline vor Mainnet: Static Analysis → Automated Security Scan → Unit/Fuzz/Invariant Testing → Internal Review → Independent Audit → Remediation → Re-Audit → Approval; mit einheitlichem Auditstatus-Enum.

## Scope

Gilt fuer alle Mainnet-Contracts (SC-G7); Audit-Records in der Contract Registry (SC-019).

## 1. Audit-Pipeline (REQ-SC-014)

```text
Static Analysis → Automated Security Scan → Unit/Fuzz/Invariant Testing
→ Internal Review → Independent Audit → Remediation → Re-Audit → Approval
```

## 2. Auditstatus-Enum (REQ-SC-015)

Der Auditstatus MUSS aus dieser Wertemenge stammen: NOT_AUDITED, INTERNAL_REVIEW, AUDIT_REQUESTED, AUDIT_IN_PROGRESS, FINDINGS_OPEN, REMEDIATION, RE_AUDIT, APPROVED, REJECTED.

## 3. Remediation (REQ-SC-016)

Findings MUESSEN remediiert und ein Re-Audit durchlaufen sein, bevor SC-G7 als bestanden gilt.



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-014 | Audit-Pipeline (§1) | MUSS |
| REQ-SC-015 | Status-Enum (§2) | MUSS |
| REQ-SC-016 | Re-Audit nach Remediation (§3) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

Audit-Berichte sind S2-classifiziert; keine oeffentliche Offenlegung von Details vor Behebung (ATC-STD-203).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Entwurf Michael, Formalfassung ATC-AI-ARCH-001) |

## References

- **NORMATIVE:** ATC-STD-000 (Verfassung), ATC-STD-201/202, ATC-STD-203
  (Security), ATC-STD-204 (Dependency & Interface), ATC-STD-SC-001
  (Familien-Hauptstandard)
- **INFORMATIVE:** ATC-AAS-001..025 (Agenten-Standards), AI-DEV-001..012,
  contracts/registry/ (Contract Registry)
