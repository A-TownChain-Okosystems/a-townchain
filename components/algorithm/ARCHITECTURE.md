---
document_id: ATC-DOC-ALG-004
title: "Architecture — ATC Algorithm"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-08
updated: 2026-09-08
standard: ATC-STD-MD-001
---

# Architecture — ATC Algorithm

## Übersicht

`atc-algorithm` enthält die kanonische Implementierung des proprietären Hybrid-Konsens-Mechanismus der A-TownChain (Chain-ID 658467). Der Algorithmus kombiniert Proof of History (PoH), Proof of Stake (PoS) und Proof of Work (PoW).

## Komponenten

| Component | Purpose | Required |
|---|---|---|
| `poh` | Proof of History Zeitstempel-Kette (SHA-256 Verifizierbare Ticks) | Yes |
| `pos` | Proof of Stake Validator-Gewichtung & Staking-Logik | Yes |
| `pow` | Proof of Work Schwierigkeitsanteil gegen Stake-Zentralisierung | Yes |
| `hybrid_engine` | Hybrid-Auswahl, Fork-Choice-Rule & Finalitäts-Orchestrierung | Yes |

## Datenfluss & Abhängigkeiten

```text
Transaktionen -> PoH (Zeitstempel-Kette via SHA-256) -> Hybrid Selection (PoS/PoW) -> Finalisierung -> Block Ingestion
```

- **Inbound:** Transaktionen und Block-Header von `a-townchain` Node.
- **Outbound:** Verifizierte Ticks, Validator-Auswahl und Konsens-Entscheidungen an `a-townchain` und Execution Engine `atc-vm`.
