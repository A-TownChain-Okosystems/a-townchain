---
standard:
  id: ATC-STD-032
  title: "Automated Update Management Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle ATC-Repositories und die Organisation gesamt"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
----

# ATC-STD-032 — Automated Update Management Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Automated Assurance 11.09.2026
> (SCR-0095; Owner-Referenz s. Kopf-Mapping). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-032 · Release-Klassifizierung und Update-Automatisierung · **Governance:** ATC-STD-000
> **ID-Mapping:** Owner-Referenz "ATC-STD-030" — §37 ergab 032.

## Abstract

ATC-STD-032 regelt, dass das System Updates nicht nur erkennt, sondern
automatisch klassifiziert, prüft und — bei gefahrlosen Patches — über
automatisierte Update-PRs einspielt; Major Releases erfordern
Kompatibilitätsprüfung.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für Dependency-/Toolchain-Updates aller Repositories. Security-Patches
unterliegen zusätzlich ATC-STD-022 (SLA).

## §1 Klassifizierung (REQ-STD-001, MUST)

Jedes neue Release MUSS klassifiziert werden: Security Patch, Bugfix, Minor,
Major, Breaking Change. Die Klassifizierung MUSS vor der Übernahme feststehen
und dokumentiert sein.

## §2 Update-Pipeline (REQ-STD-002, MUST)

CLASSIFY → COMPATIBILITY CHECK → AUTOMATED TEST → SECURITY TEST →
UPDATE / REJECT. Kein Update DARF die Pipeline überspringen; REJECT MUSS
begründet und als Finding (P2-DEP) geführt werden, wenn ein Update
erforderlich wäre (ATC-STD-018 §7 UPDATE_REQUIRED).

## §3 Automatisierte Update-PRs (REQ-STD-003, MUST)

Für ungefährliche Patch-Releases (Security Patch/Bugfix/Patch-Minor ohne
Breaking Risk) MÜSSEN automatisierte Update-PRs erstellt werden können;
sie DURCHLAUFEN die normale CI (inkl. Security-Gates). Major Releases
MUSS NICHT automatisch eingespielt werden — Kompatibilitätsprüfung und
Adoption-Gate (ATC-STD-023) sind Pflicht.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Klassifizierung: 5 Kategorien MÜSSEN je Release dokumentiert sein.
- id: REQ-STD-002 — Pipeline: 5 Stufen MÜSSEN vollständig durchlaufen werden.
- id: REQ-STD-003 — Update-PRs: Patch-Automatisierung MUSS über CI laufen; Major MUSS NICHT automatisch erfolgen.

## Compliance

Prüfung: Update-PR-Historie je Repo, Klassifizierungs-Records; SLA-Kopplung
ATC-STD-022 für Security Patches.

## Security Considerations

- Automatisierte Update-PRs sind Supply-Chain-Angriffsvektor: Herkunfts- und
  Integritätsprüfung (ATC-STD-019 §3/§4) MUSS vor Merge stehen.
- Dependabot-Rollout (Org-P2-Backlog) als bevorzugte Umsetzungsform.

## Implementierungsstatus

**Status: SPECIFIED** — mit Dependabot/Automator-Rollout zu IMPLEMENTED zu
heben. SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Automated Assurance (SCR-0095) —
  Klassifizierung, Pipeline, Update-PR-Regeln. CANDIDATE.

## References

- ATC-STD-018 — Technology Reviews · ATC-STD-019 — Supply Chain
- ATC-STD-022 — Patch-SLAs · ATC-STD-023 — Adoption Gate
