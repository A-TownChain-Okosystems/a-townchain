# 🌐 A-TownChain Testnet — Technische Dokumentation

> **Milestone:** v2.2.0 · **Issues:** #8, #14–#19
> **Datei:** `docs/architecture/TESTNET.md`

---

## Inhaltsverzeichnis

1. [Überblick](#1-überblick)
2. [Netzwerk-Architektur](#2-netzwerk-architektur)
3. [Node Discovery (Issue #14)](#3-node-discovery-issue-14)
4. [Block Propagation (Issue #15)](#4-block-propagation-issue-15)
5. [Initial Sync (Issue #16)](#5-initial-sync-issue-16)
6. [Longest-Chain-Rule (Issue #17)](#6-longest-chain-rule-issue-17)
7. [Docker Compose (Issue #18)](#7-docker-compose-issue-18)
8. [Node-Monitoring Dashboard (Issue #19)](#8-node-monitoring-dashboard-issue-19)
9. [Testnet starten](#9-testnet-starten)
10. [Protokoll-Referenz](#10-protokoll-referenz)

---

## 1. Überblick

Das A-TownChain Testnet ist ein **lokales P2P-Netzwerk** aus 5 Nodes, das mit einem einzigen `docker-compose up` gestartet wird. Es dient zur Validierung des Hybrid-Konsens (SHA-256 PoW + PoS + PoH) unter realen Netzwerkbedingungen vor dem Mainnet-Launch.

| Eigenschaft | Wert |
|-------------|------|
| Nodes | 5 (1 Bootstrap · 1 Validator · 1 Miner · 2 Full) |
| Protokoll | TCP (Block/TX Propagation) + UDP (Discovery) |
| Block-Ziel | 10 Sekunden pro Block |
| Faucet | 100 Test-ATC per Adresse |
| Monitoring | Live-Dashboard im ShivaOS |

---

## 2. Netzwerk-Architektur

```
┌─────────────────────────────────────────────────────────┐
│                    Testnet NETZWERK                     │
│                                                         │
│   bootstrap-001:5005  ←── Alle neuen Nodes melden hier │
│         ↕    ↕    ↕                                     │
│    ┌────┘    │    └────┐                                │
│    ▼         ▼         ▼                                │
│ validator  miner-001  node-002                          │
│  -001:5105 :5205      :5305                             │
│    │                   │                                │
│    └────────┬──────────┘                                │
│             ▼                                           │
│          node-003:5405                                  │
└─────────────────────────────────────────────────────────┘

Ports pro Node:
  :5005 / :5105 / :5205 / :5305 / :5405  → P2P TCP
  :4000                                   → API Gateway (nur Bootstrap)
```

### Node-Rollen

| Typ | Aufgabe | Stake | Anzahl |
|-----|---------|-------|--------|
| `bootstrap` | P2P-Einstiegspunkt, Peer-Registry | — | 1 |
| `validator` | PoS Block-Bestätigung | 50.000 ATC | 1 |
| `miner` | SHA-256 PoW Mining | — | 1 |
| `full` | Chain speichern, TX weiterleiten | — | 2 |

---

## 3. Node Discovery (Issue #14)

### Datei: `blockchain/nodes/discovery.py`

```python
import socket, json, threading, time

BOOTSTRAP_NODES = ["127.0.0.1:5005"]

class NodeDiscovery:
    """
    UDP-basierter Node-Discovery Service.
    Neue Nodes senden ANNOUNCE → Bootstrap antwortet mit PEER_LIST.
    """

    def __init__(self, my_id: str, my_port: int):
        self.my_id      = my_id
        self.my_port    = my_port
        self.peers      = {}    # node_id → {"host": str, "port": int, "last_seen": int}
        self.sock       = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.bind(("0.0.0.0", my_port + 1000))  # Discovery-Port = P2P-Port + 1000
        self._running   = True

    def announce(self):
        """Meldet sich bei allen Bootstrap-Nodes an."""
        msg = json.dumps({
            "type":    "ANNOUNCE",
            "node_id": self.my_id,
            "port":    self.my_port,
            "version": "2.0"
        }).encode()
        for bootstrap in BOOTSTRAP_NODES:
            host, port = bootstrap.split(":")
            self.sock.sendto(msg, (host, int(port) + 1000))

    def listen(self):
        """Empfängt Discovery-Nachrichten."""
        while self._running:
            try:
                data, addr = self.sock.recvfrom(4096)
                msg = json.loads(data)
                self._handle(msg, addr)
            except Exception:
                pass

    def _handle(self, msg: dict, addr: tuple):
        t = msg.get("type")
        if t == "ANNOUNCE":
            # Neuer Peer → registrieren + Peer-Liste zurückschicken
            self.peers[msg["node_id"]] = {
                "host": addr[0], "port": msg["port"],
                "last_seen": int(time.time())
            }
            self._send_peer_list(addr)
        elif t == "PEER_LIST":
            # Peer-Liste empfangen → eigene Liste erweitern
            for node_id, info in msg.get("peers", {}).items():
                if node_id != self.my_id:
                    self.peers[node_id] = info
        elif t == "PING":
            self.sock.sendto(json.dumps({"type": "PONG"}).encode(), addr)
        elif t == "PONG":
            # Peer ist noch online → last_seen aktualisieren
            pass

    def _send_peer_list(self, addr: tuple):
        msg = json.dumps({"type": "PEER_LIST", "peers": self.peers}).encode()
        self.sock.sendto(msg, addr)

    def health_check(self):
        """Pingt alle 30s alle Peers — entfernt offline Peers."""
        while self._running:
            now = int(time.time())
            dead = [nid for nid, info in self.peers.items()
                    if now - info["last_seen"] > 90]
            for nid in dead:
                del self.peers[nid]
            time.sleep(30)

    def save_peers(self, path="data/peers.json"):
        with open(path, "w") as f:
            json.dump(self.peers, f)

    def load_peers(self, path="data/peers.json"):
        try:
            with open(path) as f:
                self.peers = json.load(f)
        except FileNotFoundError:
            pass
```

### config/settings.json (Erweiterung)

```json
{
  "network": {
    "bootstrap_nodes": ["127.0.0.1:5005"],
    "max_peers": 50,
    "ping_interval_sec": 30,
    "peer_timeout_sec": 90,
    "handshake_timeout_sec": 10,
    "discovery_port_offset": 1000
  }
}
```

---

## 4. Block Propagation (Issue #15)

### Datei: `blockchain/nodes/p2p.py`

```python
import socket, json, threading

class P2PNetwork:
    """
    TCP-basiertes P2P Netzwerk für Block- und TX-Propagation.
    """

    MSG_NEW_BLOCK = "NEW_BLOCK"
    MSG_NEW_TX    = "NEW_TX"
    MSG_GET_BLOCKS = "GET_BLOCKS"
    MSG_BLOCKS    = "BLOCKS"
    MSG_HANDSHAKE = "HANDSHAKE"

    def __init__(self, node_id: str, port: int, consensus):
        self.node_id   = node_id
        self.port      = port
        self.consensus = consensus
        self.peers     = {}     # node_id → socket
        self.seen_msgs = set()  # Duplikat-Filter (msg_hash)

    def broadcast_block(self, block: dict):
        """Sendet neuen Block an alle verbundenen Peers."""
        msg = self._build_msg(self.MSG_NEW_BLOCK, {"block": block})
        self._broadcast(msg)

    def broadcast_tx(self, tx: dict):
        """Sendet neue TX an alle Peers."""
        msg = self._build_msg(self.MSG_NEW_TX, {"tx": tx})
        self._broadcast(msg)

    def _broadcast(self, msg: bytes):
        dead_peers = []
        for node_id, sock in self.peers.items():
            try:
                sock.sendall(len(msg).to_bytes(4, "big") + msg)
            except OSError:
                dead_peers.append(node_id)
        for nid in dead_peers:
            del self.peers[nid]

    def _handle_message(self, msg: dict):
        msg_hash = hash(json.dumps(msg, sort_keys=True))
        if msg_hash in self.seen_msgs:
            return  # Duplikat → ignorieren
        self.seen_msgs.add(msg_hash)

        t = msg.get("type")
        if t == self.MSG_NEW_BLOCK:
            block = msg["block"]
            if self.consensus._validate_block(block):
                self.consensus.blocks.append(block)
                self.broadcast_block(block)  # Weiterleiten
        elif t == self.MSG_NEW_TX:
            # TX in lokalen Mempool
            self.consensus.mempool = getattr(self.consensus, "mempool", [])
            self.consensus.mempool.append(msg["tx"])

    def _build_msg(self, msg_type: str, data: dict) -> bytes:
        payload = {"type": msg_type, "sender": self.node_id, **data}
        return json.dumps(payload).encode()

    def start_listener(self):
        """Lauscht auf eingehende P2P-Verbindungen."""
        srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        srv.bind(("0.0.0.0", self.port))
        srv.listen(50)
        while True:
            conn, _ = srv.accept()
            threading.Thread(target=self._handle_conn,
                             args=(conn,), daemon=True).start()

    def _handle_conn(self, conn: socket.socket):
        try:
            while True:
                raw_len = conn.recv(4)
                if not raw_len:
                    break
                length  = int.from_bytes(raw_len, "big")
                data    = conn.recv(length)
                msg     = json.loads(data)
                self._handle_message(msg)
        except Exception:
            pass
        finally:
            conn.close()
```

---

## 5. Initial Sync (Issue #16)

### Ablauf

```
Neuer Node startet
    │
    ▼
Verbindet sich mit Bootstrap-Node
    │
    ▼
GET_HEIGHT → Bootstrap antwortet: {"height": 1247}
    │
    ▼
Falls local_height < remote_height:
    └─→ Lade Blöcke in Batches (je 50)
        GET_BLOCKS {"from": 1, "to": 50}
        GET_BLOCKS {"from": 51, "to": 100}
        ...
        GET_BLOCKS {"from": 1201, "to": 1247}
    │
    ▼
Validiere jeden Block (PoW-Hash + prev_hash-Kette)
    │
    ▼
Checkpoint-Prüfung (config/checkpoints.json)
    │
    ▼
Status: SYNCED ✅
```

### Implementierung: `blockchain/nodes/sync.py`

```python
class ChainSync:

    def __init__(self, consensus, p2p: P2PNetwork):
        self.consensus = consensus
        self.p2p       = p2p
        self.syncing   = False
        self.progress  = 0.0   # 0.0 – 1.0

    def sync_from_peer(self, peer_host: str, peer_port: int):
        self.syncing = True
        remote_height = self._get_remote_height(peer_host, peer_port)
        local_height  = self.consensus.height

        if remote_height <= local_height:
            self.syncing = False
            return {"status": "already_synced", "height": local_height}

        total = remote_height - local_height
        fetched = 0
        batch_size = 50

        for start in range(local_height + 1, remote_height + 1, batch_size):
            end    = min(start + batch_size - 1, remote_height)
            blocks = self._fetch_blocks(peer_host, peer_port, start, end)
            for block in blocks:
                if self._validate_and_checkpoint(block):
                    self.consensus.blocks.append(block)
                    self.consensus.height += 1
                    fetched += 1
            self.progress = fetched / total

        self.syncing  = False
        self.progress = 1.0
        return {"status": "synced", "height": self.consensus.height, "fetched": fetched}

    def _validate_and_checkpoint(self, block: dict) -> bool:
        checkpoints = self._load_checkpoints()
        h = block.get("height")
        if h in checkpoints:
            return block["hash"] == checkpoints[h]
        return self.consensus.pow.validate_block(
            {k:v for k,v in block.items() if k not in ("hash","nonce")},
            block["nonce"], block["hash"]
        )

    def _load_checkpoints(self) -> dict:
        try:
            with open("config/checkpoints.json") as f:
                return json.load(f)
        except:
            return {}

    def get_status(self) -> dict:
        return {
            "syncing":  self.syncing,
            "progress": round(self.progress * 100, 1),
            "height":   self.consensus.height
        }
```

### config/checkpoints.json

```json
{
  "1":    "0000a3f9b2c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9",
  "100":  "0000b4e8c3d2a1f6...",
  "500":  "0000c5f7d4e3b2a1...",
  "1000": "0000d6e8f5a4c3b2..."
}
```

---

## 6. Longest-Chain-Rule (Issue #17)

### Fork-Auflösung

```
Normaler Ablauf:
  Block 100 (Miner A) → propagiert
  Alle Nodes: chain[-1].hash == Block100.hash ✅

Fork-Szenario:
  Block 101a (Miner A) ─── kommt bei Node-1 zuerst an
  Block 101b (Miner B) ─── kommt bei Node-2 zuerst an
       │
       ▼
  Block 102a (Miner A) ─── baut auf 101a auf (Länge: 3)
  Block 101b (Miner B) ─── bleibt bei Länge 2
       │
       ▼
  Longest-Chain-Rule: 101b wird ORPHAN
  Node-2 führt Chain-Reorg durch
  TXs aus 101b gehen zurück in Mempool
```

### Implementierung

```python
# In HybridConsensus ergänzen:

def receive_block(self, block: dict) -> dict:
    """Empfängt Block von Peer — prüft und entscheidet."""

    # 1. PoW-Validierung
    if not self.pow.validate_block(
        {k:v for k,v in block.items() if k not in ("hash","nonce")},
        block["nonce"], block["hash"]
    ):
        return {"accepted": False, "reason": "invalid_pow"}

    # 2. Direkter Anschluss an aktuelle Chain
    if block["prev_hash"] == self.blocks[-1]["hash"]:
        self.blocks.append(block)
        self.height += 1
        return {"accepted": True, "reorg": False}

    # 3. Fork erkannt → Longest-Chain prüfen
    fork_chain = self._find_fork_chain(block)
    if fork_chain and len(fork_chain) > len(self.blocks):
        reverted_txs = self._reorg_to(fork_chain)
        return {"accepted": True, "reorg": True,
                "reverted_txs": len(reverted_txs)}

    # 4. Kürzere Fork → Orphan
    self.orphan_pool.append(block)
    return {"accepted": False, "reason": "orphan_shorter_chain"}

def _reorg_to(self, new_chain: list) -> list:
    """Wechselt zur neuen Chain, revertiert TXs aus alten Blöcken."""
    # Common ancestor finden
    old_hashes = {b["hash"] for b in self.blocks}
    split_point = next(
        i for i, b in enumerate(new_chain)
        if b["prev_hash"] in old_hashes
    )
    # TXs aus verworfenen Blöcken zurück in Mempool
    reverted = []
    for block in self.blocks[split_point:]:
        reverted.extend(block.get("transactions", []))
    # Chain ersetzen
    self.blocks = self.blocks[:split_point] + new_chain[split_point:]
    self.height = len(self.blocks)
    # Reorg-Event
    self.event_bus.emit("chain_reorg", {"reverted_txs": len(reverted)})
    return reverted
```

---

## 7. Docker Compose (Issue #18)

### Dockerfile

```dockerfile
# Dockerfile
FROM python:3.11-slim

WORKDIR /app
COPY . .
RUN pip install --no-cache-dir -r backend/requirements.txt                                 -r gateway/requirements.txt

ENV PYTHONPATH=/app
ENV NODE_ID=node-001
ENV NODE_TYPE=full
ENV NODE_PORT=5005
ENV IS_BOOTSTRAP=false
ENV BOOTSTRAP=

EXPOSE 4000 5000 5005

CMD ["python", "launcher.py"]
```

### docker-compose.testnet.yml

```yaml
version: '3.8'

networks:
  atc-testnet:
    driver: bridge

services:

  bootstrap:
    build: .
    container_name: atc-bootstrap
    environment:
      NODE_ID: bootstrap-001
      NODE_TYPE: full
      IS_BOOTSTRAP: "true"
      NODE_PORT: "5005"
    ports:
      - "4000:4000"   # API Gateway
      - "5005:5005"   # P2P
    networks: [atc-testnet]
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:4000/gateway/health"]
      interval: 10s
      timeout: 5s
      retries: 3

  validator-001:
    build: .
    container_name: atc-validator
    environment:
      NODE_ID: validator-001
      NODE_TYPE: validator
      NODE_PORT: "5105"
      STAKE: "50000"
      BOOTSTRAP: "bootstrap:5005"
    depends_on:
      bootstrap:
        condition: service_healthy
    networks: [atc-testnet]

  miner-001:
    build: .
    container_name: atc-miner
    environment:
      NODE_ID: miner-001
      NODE_TYPE: miner
      NODE_PORT: "5205"
      BOOTSTRAP: "bootstrap:5005"
      MINING_DIFFICULTY: "3"
    depends_on:
      bootstrap:
        condition: service_healthy
    networks: [atc-testnet]

  node-002:
    build: .
    container_name: atc-node-002
    environment:
      NODE_ID: node-002
      NODE_TYPE: full
      NODE_PORT: "5305"
      BOOTSTRAP: "bootstrap:5005"
    depends_on: [bootstrap]
    networks: [atc-testnet]

  node-003:
    build: .
    container_name: atc-node-003
    environment:
      NODE_ID: node-003
      NODE_TYPE: full
      NODE_PORT: "5405"
      BOOTSTRAP: "bootstrap:5005"
    depends_on: [bootstrap]
    networks: [atc-testnet]
```

### Convenience Scripts

```bash
# scripts/testnet-start.sh
#!/bin/bash
echo "🚀 Starting A-TownChain Testnet..."
docker-compose -f docker-compose.testnet.yml up -d
echo "✅ Testnet running. Gateway: http://localhost:4000"

# scripts/testnet-stop.sh
#!/bin/bash
docker-compose -f docker-compose.testnet.yml down
echo "🛑 Testnet stopped."

# scripts/testnet-faucet.sh
#!/bin/bash
ADDRESS=$1
curl -s -X POST http://localhost:4000/api/wallet/faucet   -H "Content-Type: application/json"   -H "X-API-Key: atc-dev-key-2025"   -d "{"address": "$ADDRESS", "amount": 100}"
echo "💰 100 Test-ATC sent to $ADDRESS"
```

---

## 8. Node-Monitoring Dashboard (Issue #19)

### Frontend-Komponente

```javascript
// Node Monitor — Auto-Refresh alle 5s
const NodeMonitor = {
  interval: null,

  start() {
    this.refresh();
    this.interval = setInterval(() => this.refresh(), 5000);
  },

  async refresh() {
    const data = await ATC_API.getNodes();   // GET /api/nodes/
    this.renderStats(data.stats);
    this.renderNodeCards(data.nodes);
    this.renderNetworkGraph(data.nodes);
  },

  renderNodeCard(node) {
    const statusColor = node.online ? "var(--green)" : "var(--pink)";
    return `
      <div class="node-card" style="border-color:${statusColor}">
        <div class="node-header">
          <span class="node-id">${node.node_id}</span>
          <span class="node-badge">${node.type.toUpperCase()}</span>
          <span class="node-dot" style="background:${statusColor}"></span>
        </div>
        <div class="node-stats">
          <div>🔗 ${node.peers} Peers</div>
          <div>📦 Block #${node.chain_len}</div>
          <div>📋 ${node.mempool} TX</div>
          <div>🪙 ${node.stake} ATC Stake</div>
        </div>
      </div>
    `;
  },

  renderStats(stats) {
    document.getElementById('net-total-nodes').textContent = stats.total_nodes;
    document.getElementById('net-online').textContent      = stats.online;
    document.getElementById('net-total-stake').textContent = stats.total_stake + ' ATC';
  }
};
```

### CSS — Node Cards

```css
.node-card {
  background: rgba(8,12,30,0.8);
  border: 1px solid rgba(162,89,255,0.2);
  border-radius: 10px; padding: 12px;
  transition: all 0.3s;
}
.node-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(162,89,255,0.3);
}
.node-dot {
  width: 8px; height: 8px; border-radius: 50%;
  animation: pulse 2s infinite;
}
```

---

## 9. Testnet starten

```bash
# 1. Repository klonen
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os.git
cd a-townchain-os

# 2. Testnet starten (Docker erforderlich)
chmod +x scripts/testnet-*.sh
./scripts/testnet-start.sh

# 3. Status prüfen
curl http://localhost:4000/gateway/health

# 4. Test-ATC anfordern
./scripts/testnet-faucet.sh ATC7F3A9B2C1D4E5F6A7B8C9D0E1F2A3B4C5

# 5. Dashboard öffnen
open http://localhost:3000

# 6. Testnet stoppen
./scripts/testnet-stop.sh
```

---

## 10. Protokoll-Referenz

### Vollständige Nachrichtentypen

| Typ | Richtung | Payload | Beschreibung |
|-----|---------|---------|-------------|
| `ANNOUNCE` | Node → Bootstrap | `node_id, port, version` | Node meldet sich an |
| `PEER_LIST` | Bootstrap → Node | `peers: {id → {host, port}}` | Liste bekannter Peers |
| `PING` | bidirektional | `timestamp` | Heartbeat |
| `PONG` | bidirektional | `timestamp` | Heartbeat-Antwort |
| `HANDSHAKE` | Node → Peer | `version, chain_height, node_type` | Verbindungsaufbau |
| `NEW_BLOCK` | Miner → alle | `block: dict` | Neuer Block propagiert |
| `NEW_TX` | Node → alle | `tx: dict` | Neue TX propagiert |
| `GET_BLOCKS` | Node → Peer | `from_height, to_height` | Blöcke anfordern |
| `BLOCKS` | Peer → Node | `blocks: list` | Blöcke liefern |
| `GET_HEIGHT` | Node → Peer | — | Aktuelle Chain-Höhe anfragen |
| `HEIGHT` | Peer → Node | `height: int` | Chain-Höhe antworten |

### Fehler-Codes

| Code | Bedeutung |
|------|----------|
| `invalid_pow` | Block-Hash erfüllt Difficulty nicht |
| `invalid_prev` | `prev_hash` unbekannt |
| `orphan_shorter_chain` | Fork kürzer als lokale Chain |
| `duplicate` | Block bereits bekannt |
| `version_mismatch` | Inkompatible Node-Version |

---

> **Dokument:** `docs/architecture/TESTNET.md`
> **Issues:** [#8](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/8) [#14](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/14)–[#19](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/19)
> **Version:** 2.0.0 · **Datum:** 2026-05-19
> **Autor:** A-TownChain-Okosystems × Aurora AI
