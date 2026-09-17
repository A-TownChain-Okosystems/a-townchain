---
document_id: ATC-DOC-ARC-NODE-001
title: Repository Architecture Specification
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-13
updated: 2026-09-13
standard: ATC-STD-MD-001
---

# Architecture Specification — atc-node

## Übersicht

`atc-node` ist das Full-Node-Binary & die Runtime der A-TownChain (SCR-0005 Option A, AD-046): das Distribution-Ziel — `git clone && cargo build` → lauffähiger Node mit Bootstrap, Discovery und Validator-Betrieb. Implementiert selbst KEINE Chain-Semantik.

## Subsysteme

1. **Node-Runtime (`src/`):** Lifecycle eines Netzwerk-Teilnehmers (Start, Bootstrap, Shutdown).
2. **Bootstrap & Discovery:** Peers finden und verbinden (S4-Netzwerk-Angriffsfläche).
3. **Validator-Betrieb:** Key-Handling, Block-Produktion-Treiber (Konsens via `atc-algorithm`).
4. **CLI/Konfiguration (`config/`, `tools/`):** Operator-Schnittstelle.

## Verantwortungsgrenzen

- `a-townchain`: definiert das Chain-Protokoll — der Node betreibt eine reale Instanz.
- `atc-algorithm`: Hybrid-Konsens (kanonisch).
- `atc-vm`: Vertrags-Ausführung (kanonisch).

## Registry-Einordnung

| Property | Value |
|---|---|
| Layer | L3 |
| Criticality | C1 |
| Security-Klasse | S4 |
| Maturity | R-Level laut `.atc/repository.yaml` · Statusleiter in `.atc/evidence/evidence.yaml` (SCR-0080) |
| Canonical | atc-node (Node-Distribution/Runtime) |
| Domäne | domaene laut registry/repositories.yaml |

> Ehrlichkeitsregel: CLAIMED ≠ PASS · IMPLEMENTED ≠ VERIFIED — der verbindliche Implementierungsstand
> liegt ausschließlich in `.atc/evidence/evidence.yaml`, nicht in dieser Spezifikation.
