# 🌐 ATCNet P2P Stack — Technische Dokumentation
**Stand:** 09.06.2026 | **Version:** v2.1.0 | **Datei:** `shivaos/net/atcnet.py` (486 Zeilen)

---

## Überblick

ATCNet ist der proprietäre P2P-Netzwerk-Stack von A-TownChain. Er basiert auf einem **Kademlia DHT** für Peer-Discovery und nutzt einen eigenen **Handshake-Protokoll** für sichere Verbindungen.

```
┌─────────────────────────────────────────────────┐
│                    ATCNet                        │
├───────────────┬─────────────────────────────────┤
│  Discovery    │  Message Propagation             │
│  (Kademlia)   │  (Gossip Protocol)               │
├───────────────┴─────────────────────────────────┤
│             Transport Layer (TCP/UDP)            │
├─────────────────────────────────────────────────┤
│         Bootstrap Nodes (Port 4001)              │
└─────────────────────────────────────────────────┘
```

---

## Netzwerk-Konfiguration

**Datei:** `config/settings.json`

```json
{
  "network": {
    "bootstrap_nodes":       ["127.0.0.1:5005"],
    "max_peers":             50,
    "ping_interval_sec":     30,
    "peer_timeout_sec":      90,
    "handshake_timeout_sec": 10,
    "discovery_port_offset": 1000
  },
  "rpc": {
    "host": "127.0.0.1",
    "port": 9933
  },
  "websocket": {
    "host": "127.0.0.1",
    "port": 9944,
    "max_connections": 100
  }
}
```

---

## Peer-Discovery (Kademlia DHT)

```python
class ATCNet:
    """
    Proprietärer P2P-Stack — Kademlia DHT + Gossip Propagation.
    Port: 4001 (P2P) | 9933 (RPC) | 9944 (WebSocket)
    """

    def __init__(self, host="0.0.0.0", port=4001):
        self.host        = host
        self.port        = port
        self.peers       = {}          # peer_id → PeerInfo
        self.routing_table = KademliaTable(node_id=self._generate_node_id())
        self.message_queue = asyncio.Queue()
        self._running    = False

    async def start(self):
        """Startet P2P-Node und verbindet zu Bootstrap-Nodes."""
        await self._bind_server()
        await self._connect_bootstrap_nodes()
        await self._start_discovery_loop()

    async def connect(self, host: str, port: int) -> bool:
        """Verbindet zu einem Peer."""
        peer = await self._handshake(host, port)
        if peer:
            self.peers[peer.id] = peer
            return True
        return False

    async def broadcast(self, message: dict) -> int:
        """Sendet Nachricht an alle Peers (Gossip)."""
        sent = 0
        for peer_id, peer in self.peers.items():
            try:
                await self._send_to_peer(peer, message)
                sent += 1
            except Exception:
                self._remove_peer(peer_id)
        return sent
```

---

## Handshake-Protokoll

```
Client                          Server
  │                               │
  │── HELLO (version, node_id) ──►│
  │                               │
  │◄─ HELLO_ACK (version, node_id)│
  │                               │
  │── CHALLENGE (random nonce) ──►│
  │                               │
  │◄─ CHALLENGE_RESP (sig(nonce)) │
  │                               │
  │── CONNECTED ─────────────────►│
  │                               │
  └─── Bidirektionale Kommunikation ──
```

**Sicherheit:**
- Version-Check: inkompatible Versionen werden abgelehnt
- ECDSA-Signatur des Nonce → Authentizität des Peers
- Peer-ID = SHA-256(public_key) → unveränderlich

---

## Message-Typen

| Message-Typ | Beschreibung |
|------------|-------------|
| `NEW_BLOCK` | Neuer Block gefunden (Propagation) |
| `NEW_TX` | Neue Transaktion (Mempool) |
| `PING` | Peer am Leben prüfen |
| `PONG` | Antwort auf PING |
| `GET_PEERS` | Peer-Liste anfragen |
| `PEERS` | Peer-Liste senden |
| `GET_BLOCK` | Block by Hash anfragen |
| `BLOCK` | Block senden |
| `GET_HEADERS` | Block-Header-Kette anfragen |
| `HEADERS` | Header-Kette senden |
| `CONSENSUS` | Consensus-Nachrichten (PoS Votes) |

---

## Block-Propagation

A-TownChain verwendet ein **Gossip-Protokoll** für Block-Propagation:

```
1. Node findet/validiert neuen Block
2. Sendet NEW_BLOCK an alle direkten Peers
3. Peers validieren den Block
4. Wenn valide: weiterleiten an ihre Peers
5. TTL = 7 Hops (verhindert infinites Flooding)
```

**Datei:** `blockchain/nodes/p2p_propagation.py`

```python
class P2PPropagation:
    def propagate_block(self, block, exclude_peer=None):
        """Verbreitet Block im Netzwerk."""
        msg = {
            "type":  "NEW_BLOCK",
            "block": block.to_dict(),
            "ttl":   7,
            "origin": self.node_id
        }
        for peer in self.peers:
            if peer.id != exclude_peer:
                peer.send(msg)
```

---

## Port-Übersicht

| Port | Protokoll | Zweck |
|------|----------|-------|
| 4000 | HTTP | API Gateway (Frontend → Backend) |
| 4001 | TCP/UDP | P2P Node Discovery & Block Sync |
| 5000 | HTTP | Backend REST API (intern) |
| 5005 | TCP | Bootstrap Node |
| 9933 | HTTP | RPC-Endpunkt (externe Clients) |
| 9944 | WebSocket | Real-Time Events (Block-Updates, TX) |

---

## Node-Typen

| Typ | Beschreibung | Anforderungen |
|-----|-------------|--------------|
| Full Node | Komplette Blockchain, Mining | 2+ GB RAM, 100+ GB Speicher |
| Light Node | Nur Headers + SPV | 512 MB RAM, 1 GB Speicher |
| Bootstrap Node | Discovery-Dienst | 1 GB RAM, statische IP |
| Validator Node | PoS-Validator | 1000+ ATC Stake + Full Node |
| Archive Node | Komplette History | 8+ GB RAM, 1+ TB Speicher |
