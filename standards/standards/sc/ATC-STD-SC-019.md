---
standard:
  id: ATC-STD-SC-019
  title: "ATC-STD-SC-019 — Contract Registry Standard"
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

# ATC-STD-SC-019 — Contract Registry Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Zentrale Contract Registry im atc-standards Repository: contracts/registry/{contracts.yaml, deployments.yaml, versions.yaml} mit Kategorie-Verzeichnissen; Registry-First fuer jeden Contract und Gate SC-G12.

## Scope

Gilt fuer die Registry-Struktur in atc-standards und alle registrierungspflichtigen Contracts.

## 1. Registry-Struktur (REQ-SC-053)

```text
contracts/
├── registry/{contracts.yaml, deployments.yaml, versions.yaml}
├── token/ nft/ defi/ governance/ bridge/ oracle/ gamefi/ mining/ system/
```
contracts.yaml erfasst je Contract: contract_id, name, repository, version, network, status, verified, audited (plus SC-002-Identitaet).

## 2. Registry-First (REQ-SC-054)

Jeder Contract MUSS vor Implementierung (SC-G0) in der Registry angelegt und vor Mainnet (SC-G12) vollstaendig gepflegt sein — Analogon zum Registry-First der Standards (ATC-STD-000 §22).

## 3. Deployment-Referenzen (REQ-SC-055)

deployments.yaml MUSS jede Deployment-Record-Kette (SC-006) referenzieren; versions.yaml dokumentiert die Contract-Historie.



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-053 | Registry-Struktur (§1) | MUSS |
| REQ-SC-054 | Registry-First (§2) | MUSS |
| REQ-SC-055 | Deployment-Referenzen (§3) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

Die Registry enthaelt KEINE Secrets und KEINE privaten Keys; Adressen sind oeffentliche On-Chain-Daten.

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

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), ATC-STD-202 (Naming)
- **INFORMATIVE:** ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/
