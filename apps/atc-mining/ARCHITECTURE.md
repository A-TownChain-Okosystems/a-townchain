---
document_id: ATC-DOC-MIN-ARC-001
title: Architecture — atc-mining
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-07
standard: ATC-STD-MD-001
---

# ARCHITECTURE — atc-mining

`atc-mining` entkoppelt die hardwarenahe Hashing- und Mining-Logik vom Blockchain-Core (`a-townchain`). Der Core bleibt regeldefinierend, während `atc-mining` die Ausführung auf Miner-Knoten und Netzwerken bereitstellt.

## Systemkomponenten

```text
atc-mining/
├── miner/              Hardware-Treiber & SHA3-ATC Hashing Engine
├── algorithms/         Difficulty-Berechnung & PoW-Verifikation
├── mining-manager/     Job-Orchestrierung & Nonce-Distributor
├── network/            Netzwerk-Protokoll- & Knoten-Schnittstellen
├── sc-018/             Reward-Emissions-Logic & Double-Claim-Schutz (ATC-STD-SC-018)
└── telemetry/          MinerWatcherGPT-Event-Handler (ATC-STD-SC-008)
```

## SC-018-Verzahnung & MinerWatcherGPT

1. **Emissionsplan**: Der Mining-Contract (ATC-STD-SC-018) regelt die exakte Block-Reward-Emission. `atc-mining` berechnet die zustehenden Rewards pro gültigem Proof-of-Work.
2. **Double-Claim-Schutz**: Jeder Claim wird mit kryptographischen Proof-Hashes und Nonce-Replays abgesichert.
3. **MinerWatcherGPT-Monitoring**: Alle Reward-Claims, Difficulty-Adjustments und Mining-Events senden strukturierte Audit-Events (ATC-STD-SC-008), die von MinerWatcherGPT in Echtzeit ausgewertet werden.

## Abhängigkeiten

- `a-townchain`: Blockchain-Core & Transaktionsverwaltung
- `atc-algorithm`: Konsensus-Spezifikation (PoH+PoS+PoW)
- `atc-contracts`: Referenzverträge für ATC-STD-SC-018
