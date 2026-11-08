# 📄 Issue #8 — Multi-Node Testnet

> **Labels:** enhancement · blockchain · networking · priority:high
> **Priorität:** 🔴 High · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #8](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/8)

---

## Ziel

Mehrere A-TownChain Nodes zu einem echten P2P-Testnetzwerk verbinden — Blöcke werden propagiert, Konsens dezentral erreicht, neue Nodes können sich synchronisieren.

---

## Netzwerk-Architektur

```
Bootstrap Node (immer online)
  node-001:5005  ←── neue Nodes melden sich hier an

      ↕  P2P (TCP/UDP)

node-002:5105        node-003:5205
(Validator)          (MINER)
    ↕                    ↕
node-004:5305        node-005:5405
(FULL)               (LIGHT)
```

---

## Protokoll-Spezifikation

### Nachrichten-Typen

```python
MSG_TYPES = {
    "HANDSHAKE":   {"version": str, "chain_height": int, "node_type": str},
    "NEW_BLOCK":   {"block": dict},
    "NEW_TX":      {"tx": dict},
    "GET_BLOCKS":  {"from_height": int, "to_height": int},
    "BLOCKS":      {"blocks": list},
    "GET_PEERS":   {},
    "PEERS":       {"peers": list},
    "PING":        {"timestamp": int},
    "PONG":        {"timestamp": int},
}
```

### Node Discovery

```python
# blockchain/nodes/discovery.py
class NodeDiscovery:
    BOOTSTRAP_NODES = [
        "127.0.0.1:5005",   # lokaler Bootstrap
        # Mainnet: "node1.atownchain.io:5005"
    ]

    def announce(self, my_address: str):
        """Meldet sich bei Bootstrap-Nodes an."""

    def get_peers(self) -> list:
        """Gibt bekannte Peers zurück."""

    def health_check(self):
        """Regelmäßiger Ping an alle Peers."""
```

### Docker Compose (5 lokale Nodes)

```yaml
# docker-compose.testnet.yml
version: '3.8'
services:
  node-001:
    build: .
    environment:
      NODE_ID: node-001
      NODE_TYPE: full
      NODE_PORT: 5005
      IS_BOOTSTRAP: "true"
    ports: ["5005:5005"]

  node-002:
    build: .
    environment:
      NODE_ID: node-002
      NODE_TYPE: validator
      NODE_PORT: 5105
      BOOTSTRAP: "node-001:5005"
      STAKE: "50000"
    depends_on: [node-001]

  node-003:
    build: .
    environment:
      NODE_ID: node-003
      NODE_TYPE: miner
      NODE_PORT: 5205
      BOOTSTRAP: "node-001:5005"
    depends_on: [node-001]
```

---

## Aufgaben

- [ ] `blockchain/nodes/discovery.py` — Node Discovery via UDP
- [ ] `blockchain/nodes/p2p.py` — TCP-basiertes P2P Protokoll
- [ ] `broadcast_block()` an alle Peers implementieren
- [ ] Initial Sync: neuer Node lädt Chain von Peers
- [ ] Longest-Chain-Rule (Fork-Auflösung)
- [ ] `docker-compose.testnet.yml` für 5 lokale Nodes
- [ ] Testnet Faucet: `GET /api/faucet/:address` (100 Test-ATC)
- [ ] Node-Monitoring im Dashboard (Live-Status aller Peers)
- [ ] Checkpoint-System (vertrauenswürdige Block-Hashes in `config/`)

---

## Akzeptanzkriterien

- [ ] 5 Nodes starten via `docker-compose up`
- [ ] Block wird von Miner an alle Peers propagiert
- [ ] Neuer Node synchronisiert sich vollständig
- [ ] Fork wird korrekt aufgelöst (Longest Chain gewinnt)
- [ ] Dashboard zeigt Live-Node-Status
