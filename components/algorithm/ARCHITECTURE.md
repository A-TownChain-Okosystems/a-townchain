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

`a-townchain/components/algorithm` enthält den aus `atc-algorithm` migrierten Rust-Prototyp (Chain-ID 658467). Er ist noch kein vollständiger Hybrid-Konsens. Die formelle Capability-Ownership und normative Protokollautorität bleiben bis zur SCR-0131-/§9-Freigabe offen.

## Komponenten

| Component | Purpose | Required |
|---|---|---|
| `poh` | PoH-Tick-Prototyp; Hashprimitive und Wire-Semantik noch nicht normativ freigegeben | Yes |
| `pos` | Proof of Stake Validator-Gewichtung & Staking-Logik | Yes |
| `pow` | Proof of Work Schwierigkeitsanteil gegen Stake-Zentralisierung | Yes |
| `hybrid_engine` | Hybrid-Auswahl, Fork-Choice-Rule & Finalitäts-Orchestrierung | Yes |

## Datenfluss & Abhängigkeiten

```text
Transaktionen -> PoH (Zeitstempel-Kette via SHA-256) -> Hybrid Selection (PoS/PoW) -> Finalisierung -> Block Ingestion
```

- **Inbound:** Transaktionen und Block-Header von der Node-/Orchestrierungsschicht.
- **Outbound:** Prototypische Tick-/Selection-Ergebnisse an die Integrationsschicht; keine Produktions- oder Finalitätsgarantie.
