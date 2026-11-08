# 🌐 Issue #14 — Bootstrap Node — P2P Discovery Service

## Status: ✅ ABGESCHLOSSEN

**Milestone:** v2.2.0  
**Abhängigkeit:** Keine  
**Abhängig von:** Issue #14 wird von #15, #16, #17 benötigt  

---

## 📋 Aufgabenbeschreibung

Bootstrap Node ist der **Einstiegspunkt für das P2P-Netzwerk**. Alle neuen Nodes verbinden sich zuerst mit dem Bootstrap-Node, um andere Peers zu entdecken und sich ins Netzwerk zu integrieren.

### Was wurde implementiert:

1. **Node Discovery Service** (`blockchain/nodes/discovery.py`)
   - UDP-basiertes Protokoll für Peer-Discovery
   - ANNOUNCE/PEER_LIST Handshake
   - Peer-Registry Verwaltung
   - Health-Check (entfernt offline Peers)

2. **P2P Network Base** (`blockchain/nodes/p2p_propagation.py`)
   - TCP-basiertes Messaging
   - Block- und TX-Propagation
   - Duplikat-Filter
   - Peer-Verbindungsverwaltung

---

## ✅ Implementierte Komponenten

### 1. NodeDiscovery Klasse

**Datei:** `blockchain/nodes/discovery.py`

```python
class NodeDiscovery:
    """UDP-basierter Node-Discovery Service"""
    
    def __init__(self, node_id: str, my_port: int, is_bootstrap: bool = False)
    def start()                          # Startet Discovery-Service
    def stop()                           # Stoppt Service
    def announce()                       # Meldet Node beim Bootstrap an
    def listen()                         # UDP-Listener
    def health_check()                   # Entfernt offline Peers (alle 30s)
    def get_peers() -> Dict             # Gibt Peer-Liste zurück
    def get_peer_count() -> int         # Anzahl Peers
    def save_peers(path)                # Persistierung in JSON
    def load_peers(path)                # Laden aus JSON
    def get_status() -> dict            # Status-Information
```

**Protokoll (UDP):**

```
ANNOUNCE (Node → Bootstrap):
  {
    "type": "ANNOUNCE",
    "node_id": "peer-001",
    "port": 5105,
    "version": "2.0"
  }

PEER_LIST (Bootstrap → Node):
  {
    "type": "PEER_LIST",
    "peers": {
      "bootstrap-001": {"host": "127.0.0.1", "port": 5005},
      "validator-001": {"host": "127.0.0.1", "port": 5105},
      ...
    }
  }

PING/PONG (Health-Check):
  {"type": "PING"}
  {"type": "PONG"}
```

### 2. P2PNetwork Klasse

**Datei:** `blockchain/nodes/p2p_propagation.py`

```python
class P2PBroadcaster:
    """TCP-basiertes P2P-Netzwerk"""
    
    def __init__(self, node_id: str, port: int, consensus=None)
    def start()                         # Startet TCP-Listener
    def stop()                          # Stoppt Network
    def connect_to_peer(node_id, host, port) -> bool
    def broadcast_block(block: dict)    # Sendet Block an alle Peers
    def broadcast_tx(tx: dict)          # Sendet TX an alle Peers
    def register_handler(msg_type, handler)  # Registriert Custom-Handler
    def get_status() -> dict
```

**Nachrichtentypen (TCP):**

```
NEW_BLOCK:
  {"type": "NEW_BLOCK", "sender": "miner-001", "block": {...}}

NEW_TX:
  {"type": "NEW_TX", "sender": "node-002", "tx": {...}}

GET_BLOCKS:
  {"type": "GET_BLOCKS", "from_height": 1, "to_height": 50}

BLOCKS:
  {"type": "BLOCKS", "blocks": [{...}, {...}]}

GET_HEIGHT:
  {"type": "GET_HEIGHT"}

HEIGHT:
  {"type": "HEIGHT", "height": 1247}

HANDSHAKE:
  {"type": "HANDSHAKE", "node_id": "node-002", "version": "2.0", "chain_height": 1247}
```

---

## 🏗 Architektur

```
Bootstrap Node (is_bootstrap=True)
├── NodeDiscovery (UDP :6005)
│   ├── listen() → empfängt ANNOUNCE
│   ├── _send_peer_list() → sendet PEER_LIST
│   ├── health_check() → entfernt offline Peers
│   └── peers = {node_id → PeerInfo}
│
└── P2PBroadcaster (TCP :5005)
    ├── start_listener() → wartet auf eingehende Verbindungen
    ├── _handle_incoming_connection() → akzeptiert Peers
    ├── broadcast_block() → sendet an alle
    └── peers = {node_id → socket}


Regular Node
├── NodeDiscovery (UDP :6105)
│   ├── announce() → sendet ANNOUNCE an Bootstrap
│   ├── listen() → empfängt PEER_LIST
│   └── peers = [bootstrap-001, validator-001, ...]
│
└── P2PBroadcaster (TCP :5105)
    ├── connect_to_peer() → verbindet sich mit Peers
    ├── handshake() → HTTP-Handshake
    └── broadcast_block() → sendet an verbundene Peers
```

---

## 📝 Konfiguration

**Datei:** `config/settings.json`

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

## 🚀 Verwendungsbeispiel

### Bootstrap Node starten:

```python
from blockchain.nodes.discovery import NodeDiscovery
from blockchain.nodes.p2p_propagation import P2PBroadcaster

# Discovery-Service
discovery = NodeDiscovery(
    node_id="bootstrap-001",
    my_port=5005,
    is_bootstrap=True
)
discovery.start()

# P2P Network
p2p = P2PBroadcaster(
    node_id="bootstrap-001",
    port=5005
)
p2p.start()

# Warte auf Peers...
time.sleep(10)
print(f"Connected peers: {discovery.get_peer_count()}")
```

### Regular Node verbindet sich:

```python
# Discovery-Service
discovery = NodeDiscovery(
    node_id="node-002",
    my_port=5105,
    is_bootstrap=False,
    bootstrap_nodes=["127.0.0.1:5005"]
)
discovery.start()

# P2P Network
p2p = P2PBroadcaster(
    node_id="node-002",
    port=5105
)
p2p.start()

# Verbinde mit Peers aus Discovery
time.sleep(2)  # Warte auf ANNOUNCE/PEER_LIST
peers = discovery.get_peers()
for node_id, peer_info in peers.items():
    p2p.connect_to_peer(
        node_id,
        peer_info.host,
        peer_info.port
    )
```

---

## 🧪 Tests

**Datei:** `tests/test_discovery.py` und `tests/test_p2p_propagation.py`

```bash
pytest tests/test_discovery.py -v
pytest tests/test_p2p_propagation.py -v

# Output:
# test_peer_info_to_dict PASSED
# test_discovery_init PASSED
# test_discovery_init_non_bootstrap PASSED
# test_get_peers_empty PASSED
# test_add_peer_manually PASSED
# test_get_status PASSED
# test_peer_persistence PASSED
# test_message_to_bytes PASSED
# test_message_from_bytes PASSED
# test_broadcaster_init PASSED
# test_register_handler PASSED
# ...
```

---

## 📦 Abhängigkeiten

- `socket` (stdlib)
- `json` (stdlib)
- `threading` (stdlib)
- `logging` (stdlib)
- `dataclasses` (stdlib)
- `time` (stdlib)

**Keine externen Abhängigkeiten!**

---

## ✨ Features

✅ UDP-basierte Node-Discovery  
✅ Bootstrap-Peer-Registry  
✅ TCP-P2P-Messaging  
✅ Duplikat-Filter (Message-Hash)  
✅ Health-Check (Peer-Timeout)  
✅ Peer-Persistierung (JSON)  
✅ Logging und Debugging  
✅ Thread-Safe Operationen  
✅ Graceful Error Handling  
✅ Auto-Reconnect nach Neustart  

---

## 🔗 Nächste Schritte

Nach Issue #14 folgt:

- **Issue #15:** Block Propagation — P2P Block Broadcasting ✅ DONE
- **Issue #16:** Initial Sync — Neue Nodes synchronisieren
- **Issue #17:** Longest-Chain-Rule — Fork-Auflösung
- **Issue #18:** Docker Compose — 5-Node Testnet

---

## 📖 Dokumentation

- [Testnet.md](../../../../architecture/TESTNET.md#3-node-discovery-issue-14) — Vollständige Testnet-Doku
- [TESTNET_INDEX.md](../../../../issues/TESTNET_INDEX.md) — Abhängigkeitsbaum

---

**Implementiert:** 2026-05-22  
**Autor:** A-TownChain-Okosystems × Aurora AI  
**Status:** ✅ Code Complete — Bereit für Testing & Issue #15/16
