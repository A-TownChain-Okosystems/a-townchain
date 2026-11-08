---
document_id: ATC-DOC-ARC-CMPT-001
title: Repository Architecture Specification
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-13
updated: 2026-09-13
standard: ATC-STD-MD-001
---

# Architecture Specification — atc-compute

## Übersicht

`atc-compute` ist die dezentrale Compute-Schicht (Layer L5) für AI-Workloads und Engine-Berechnungen: Job-Scheduling, Ausführungs-Verifikation und Ergebnis-Attestation. Vertikales Produkt-Repo gemäß AD-024, Implementierung folgt qualitätsgetrieben (AD-023).

## Subsysteme

1. **Jobs/Scheduler (`src/`):** Annahme, Verteilung und Orchestrierung von Compute-Jobs.
2. **Verification:** Ergebnis-Attestation von Berechnungen (Determinismus-Anforderungen aus ATC-STD).
3. **Integration aurora-ai:** Model-Ausführung für AI-Workloads.
4. **Integration genesis-engine:** Engine-Berechnungen (Rendering/Physics-Offloading).

## Verantwortungsgrenzen

- `aurora-ai` stellt die AI-/Agentenschicht — hier nur Compute-Orchestrierung.
- `genesis-engine` konsumiert Compute — implementiert keine eigene Verteilung.
- On-Chain-Abwicklung (Bezahlung von Compute) via Chain-Protokoll (`a-townchain`).

## Registry-Einordnung

| Property | Value |
|---|---|
| Layer | L5 |
| Criticality | C2 |
| Security-Klasse | S3 |
| Maturity | R-Level laut `.atc/repository.yaml` · Statusleiter in `.atc/evidence/evidence.yaml` (SCR-0080) |
| Canonical | atc-compute (Compute-Orchestrierung) |
| Domäne | domaene laut registry/repositories.yaml |

> Ehrlichkeitsregel: CLAIMED ≠ PASS · IMPLEMENTED ≠ VERIFIED — der verbindliche Implementierungsstand
> liegt ausschließlich in `.atc/evidence/evidence.yaml`, nicht in dieser Spezifikation.
