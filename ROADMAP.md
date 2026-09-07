---
document_id: ATC-DOC-MIN-RDM-001
title: Roadmap — atc-mining
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-07
standard: ATC-STD-MD-001
---

# ROADMAP — atc-mining

Die Entwicklung von `atc-mining` orientiert sich an der kanonischen Lauffähigkeits-Roadmap M1–M8 (AD-027) und der Bauhierarchie L0–L7 (AD-026).

## Meilenstein-Planung

### M1–M3: Vorbereitung & Core-Anbindung (L0–L3)
- [x] R1-Repository-Skeleton & Governance-Compliance (ATC-STD-201/202/203)
- [x] Dokumentations-Konformität (ATC-STD-README-001, ATC-STD-MD-001)
- [ ] Interface-Spezifikation für PoW/Difficulty-Schnittstelle zu `a-townchain` und `atc-algorithm`

### M4: Smart Contract & Reward-Integration (L4)
- [ ] Integration der Smart-Contract-Interfaces nach ATC-STD-SC-018 (Mining Rewards & Emissionen)
- [ ] Implementierung des Double-Claim-Schutzes in Reward-Claim-Handlern
- [ ] Validierung der Event-Emittierung nach ATC-STD-SC-008 für MinerWatcherGPT

### M5–M6: Mining-Services & Telemetrie (L5)
- [ ] Implementierung der SHA3-ATC Hashing-Engine (Rust Core)
- [ ] Netzwerk-Protokoll-Handler und Job-Distributor
- [ ] Anbindung an MinerWatcherGPT zur Echtzeit-Telemetrie und Anomalieerkennung
- [ ] End-to-End-Tests der Reward-Verteilung im Testnet (Chain-ID 658467)

### M7–M8: Ökosystem-Rollout (L6–L7)
- [ ] Release ATC-REL-1.0.0
- [ ] Integration in `a-townchain-os`
