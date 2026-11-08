# 🌐 atcnet

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> **P2P Netzwerk-Stack: Kademlia DHT, Bootstrap Node, Peer Discovery, Gossip Protocol**

[![Layer](https://img.shields.io/badge/Layer-L5-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/docs/kai-os-wiki.md)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atcnet-wiki)

---

## 📋 Beschreibung

`atcnet` ist das Peer-to-Peer (P2P) Netzwerksystem (Layer L5) des A-TownChain OS Ökosystems. Es stellt dezentrale Kommunikation, Kademlia DHT Peer Discovery, Gossip-basierte Block- und Transaktions-Propagierung sowie Bootstrap-Server für das Initial Networking zur Verfügung.

---

## 🏛️ Architektur

```
 [ Node A ] <--- Gossip Sub-Protocol ---> [ Node B ]
     |                                        |
     +------------ Distributed Hash Table ----+
                     (Kademlia DHT)
                           |
                           v
                [ Bootstrap Client / Server ]
```

---

## 🧩 Komponenten

- **`atcnet.py` / `atcnet.atc`**: Haupt-Netzwerk-Daemon, Socket Event Loop & Protocol Parser
- **`discovery.py`**: Kademlia DHT Peer Discovery & Routing Tables (160-bit Distance Metric)
- **`p2p_propagation.py`**: High-Speed Gossip Flooding Protocol mit De-Duplizierung
- **`bootstrap_client.py`**: Client für Verbindungsaufbau zu Seeds & Bootstrap-Knoten
- **`node.py`**: Node Identity (Peer-ID, Public Key), Ping/Pong & Health Monitor
- **`tests/test_atcnet.py`**: Unit Tests für P2P Netzwerkschicht

---

## 🚀 Usage

### P2P Node starten
```python
from node import P2PNode

node = P2PNode(host="0.0.0.0", port=8000)
node.start()
node.connect_to_peer("bootstrap.a-townchain.org", 8000)
```

### Nachricht via Gossip senden
```python
from p2p_propagation import GossipBroadcaster

broadcaster = GossipBroadcaster(node)
broadcaster.broadcast(topic="blocks", payload=b"block_data_bytes")
```

---

## 🛠️ Build & Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atcnet.git
cd atcnet
python3 -m unittest discover tests
```

---

## 🔗 Verwandte Repos & Wiki

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | `L2–L4` | Haupt-Repo — KAI-OS Core |
| [atcnet-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atcnet-wiki) | `Docs` | Offizielles P2P Netzwerk Wiki |
| [atc-blockchain](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-blockchain) | `L3` | Blockchain Core |
| [atc-kernel](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-kernel) | `L2` | ShivaOS Microkernel |

**📖 Offizielle Dokumentation:** [atcnet-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atcnet-wiki)

---

## Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **All Rights Reserved.**

Dieses Projekt nutzt das **ATC-LIC Lizenzmodell** — ein monetarisiertes, autonomes Open-Source-Ökosystem. Unlizenzierter Code wird von der ATVM physisch nicht ausgeführt.

- [ATC-LIC — Smart Contract Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md)
- [ATC-LIC — System & Hardware Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md)
- [Compliance-Handbuch (BaFin)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/compliance/COMPLIANCE_HANDBUCH.md)
- [Lizenz-Übersicht](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/LICENSING_OVERVIEW.md)
