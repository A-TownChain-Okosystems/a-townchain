# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ATCNet — Proprietärer P2P Netzwerk-Stack
Version: 1.0.0-alpha | ATS-1006 konform
Kein libp2p-Klon — eigene DHT + Routing-Implementierung
"""

import socket, threading, time, json, hashlib, struct, random
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Callable, Tuple, Set
from enum import IntEnum, auto


# ══════════════════════════════════════════════════════════
#  PROTOKOLL-KONSTANTEN
# ══════════════════════════════════════════════════════════

ATCNET_VERSION  = 1
ATCNET_PORT     = 4001
MAGIC_BYTES     = b"\xAT\xC0\x01"   # ATC Magic
MAX_MSG_SIZE    = 4 * 1024 * 1024   # 4MB
K_BUCKET_SIZE   = 20                # Kademlia k
ALPHA           = 3                 # Parallele Lookups
TTL_DEFAULT     = 10                # Max Hops


# ══════════════════════════════════════════════════════════
#  NACHRICHTENTYPEN (ATC-0007)
# ══════════════════════════════════════════════════════════

class MsgType(IntEnum):
    HELLO          = 1
    PING           = 2
    PONG           = 3
    GET_PEERS      = 4
    PEERS          = 5
    GET_BLOCK      = 6
    BLOCK          = 7
    BROADCAST_TX   = 8
    TX             = 9
    CONSENSUS_VOTE = 10
    CONSENSUS_BLOCK= 11
    KI_QUERY       = 12
    KI_RESPONSE    = 13
    FIND_NODE      = 14
    FOUND_NODE     = 15
    DISCONNECT     = 16
    ERROR          = 99


@dataclass
class ATCMessage:
    version:   int
    msg_type:  MsgType
    from_node: str          # NodeID (hex)
    to_node:   str          # NodeID oder "ff" * 20 für Broadcast
    payload:   bytes
    ttl:       int = TTL_DEFAULT
    timestamp: int = 0
    msg_id:    str = ""

    def __post_init__(self):
        if not self.timestamp:
            self.timestamp = int(time.time() * 1000)
        if not self.msg_id:
            raw = f"{self.from_node}{self.timestamp}{self.msg_type}".encode()
            self.msg_id = hashlib.sha3_256(raw).hexdigest()[:16]

    def encode(self) -> bytes:
        """ATCMessage → Bytes serialisieren."""
        header = json.dumps({
            "v":    self.version,
            "t":    int(self.msg_type),
            "from": self.from_node,
            "to":   self.to_node,
            "ttl":  self.ttl,
            "ts":   self.timestamp,
            "id":   self.msg_id,
        }).encode()
        length = struct.pack(">I", len(header)) + struct.pack(">I", len(self.payload))
        return MAGIC_BYTES + length + header + self.payload

    @classmethod
    def decode(cls, data: bytes) -> Optional['ATCMessage']:
        """Bytes → ATCMessage deserialisieren."""
        try:
            if data[:3] != MAGIC_BYTES:
                return None
            h_len = struct.unpack(">I", data[3:7])[0]
            p_len = struct.unpack(">I", data[7:11])[0]
            header  = json.loads(data[11:11 + h_len])
            payload = data[11 + h_len:11 + h_len + p_len]
            return cls(
                version   = header["v"],
                msg_type  = MsgType(header["t"]),
                from_node = header["from"],
                to_node   = header["to"],
                payload   = payload,
                ttl       = header["ttl"],
                timestamp = header["ts"],
                msg_id    = header["id"],
            )
        except Exception:
            return None


# ══════════════════════════════════════════════════════════
#  NODE-IDENTITÄT
# ══════════════════════════════════════════════════════════

@dataclass
class NodeInfo:
    node_id:   str          # 20-Byte Hex
    address:   str          # IP:Port
    atc_addr:  str = ""     # ATC-Wallet-Adresse
    version:   int = ATCNET_VERSION
    last_seen: int = 0
    latency_ms: int = 0
    reputation: int = 5000  # 0–10000

    def distance(self, other: 'NodeInfo') -> int:
        """XOR-Distanz für Kademlia-Routing."""
        a = int(self.node_id,  16)
        b = int(other.node_id, 16)
        return a ^ b

    @property
    def is_alive(self) -> bool:
        return (int(time.time() * 1000) - self.last_seen) < 30_000  # 30s Timeout


def generate_node_id(seed: str = "") -> str:
    """Eindeutige Node-ID generieren."""
    raw = (seed or str(time.time()) + str(random.random())).encode()
    return hashlib.sha3_256(raw).hexdigest()[:40]  # 20 Bytes hex


# ══════════════════════════════════════════════════════════
#  KADEMLIA DHT (eigene Variante)
# ══════════════════════════════════════════════════════════

class ATCRoutingTable:
    """
    Eigene Kademlia-Variante für ATCNet.
    k-Bucket basiertes Routing.
    """

    def __init__(self, own_id: str, k: int = K_BUCKET_SIZE):
        self.own_id   = own_id
        self.k        = k
        self.buckets: List[List[NodeInfo]] = [[] for _ in range(160)]

    def _bucket_index(self, node_id: str) -> int:
        dist = int(self.own_id, 16) ^ int(node_id, 16)
        if dist == 0: return 0
        return dist.bit_length() - 1

    def add_node(self, node: NodeInfo) -> bool:
        if node.node_id == self.own_id:
            return False
        idx    = self._bucket_index(node.node_id)
        bucket = self.buckets[idx]

        # Bereits bekannt → aktualisieren
        for i, n in enumerate(bucket):
            if n.node_id == node.node_id:
                bucket.pop(i)
                bucket.append(node)
                return True

        if len(bucket) < self.k:
            bucket.append(node)
            return True
        else:
            # Bucket voll → ältesten toten Node ersetzen
            for i, n in enumerate(bucket):
                if not n.is_alive:
                    bucket.pop(i)
                    bucket.append(node)
                    return True
        return False

    def remove_node(self, node_id: str):
        idx = self._bucket_index(node_id)
        self.buckets[idx] = [n for n in self.buckets[idx] if n.node_id != node_id]

    def find_closest(self, target_id: str, count: int = K_BUCKET_SIZE) -> List[NodeInfo]:
        """k nächste Nodes zum Ziel finden."""
        all_nodes = [n for bucket in self.buckets for n in bucket if n.is_alive]
        target_int = int(target_id, 16)
        all_nodes.sort(key=lambda n: int(n.node_id, 16) ^ target_int)
        return all_nodes[:count]

    def get_all_nodes(self) -> List[NodeInfo]:
        return [n for bucket in self.buckets for n in bucket]

    def size(self) -> int:
        return sum(len(b) for b in self.buckets)


# ══════════════════════════════════════════════════════════
#  ATCNET NODE
# ══════════════════════════════════════════════════════════

class ATCNetNode:
    """
    ATCNet P2P-Knoten.
    Implementiert Discovery, Routing und Message-Passing.
    """

    def __init__(self, host: str = "127.0.0.1", port: int = ATCNET_PORT,
                 atc_address: str = ""):
        self.node_id    = generate_node_id(f"{host}:{port}")
        self.host       = host
        self.port       = port
        self.atc_addr   = atc_address
        self.routing    = ATCRoutingTable(self.node_id)
        self.handlers:  Dict[MsgType, List[Callable]] = {}
        self._seen_msgs: Set[str] = set()
        self._running   = False
        self._sock:     Optional[socket.socket] = None
        self._lock      = threading.Lock()
        self.stats      = {"sent": 0, "recv": 0, "dropped": 0, "peers": 0}
        self._setup_handlers()

    def _setup_handlers(self):
        """Standard-Handler registrieren."""
        self.on(MsgType.PING,      self._handle_ping)
        self.on(MsgType.PONG,      self._handle_pong)
        self.on(MsgType.HELLO,     self._handle_hello)
        self.on(MsgType.GET_PEERS, self._handle_get_peers)
        self.on(MsgType.PEERS,     self._handle_peers)
        self.on(MsgType.FIND_NODE, self._handle_find_node)

    def on(self, msg_type: MsgType, handler: Callable):
        """Handler für Nachrichtentyp registrieren."""
        if msg_type not in self.handlers:
            self.handlers[msg_type] = []
        self.handlers[msg_type].append(handler)

    # ── Netzwerk-Start/Stop ───────────────────────────────

    def start(self) -> bool:
        """Node starten — Socket öffnen, Threads starten."""
        try:
            self._sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
            self._sock.bind((self.host, self.port))
            self._sock.settimeout(1.0)
            self._running = True

            threading.Thread(target=self._recv_loop, daemon=True,
                             name=f"atcnet-recv-{self.port}").start()
            threading.Thread(target=self._heartbeat_loop, daemon=True,
                             name=f"atcnet-hb-{self.port}").start()

            print(f"  🌐 ATCNet Node gestartet | {self.host}:{self.port}")
            print(f"     NodeID: {self.node_id[:16]}...")
            return True
        except OSError as e:
            print(f"  ⚠️  ATCNet Start fehlgeschlagen: {e}")
            return False

    def stop(self):
        self._running = False
        if self._sock:
            self._sock.close()

    # ── Senden ───────────────────────────────────────────

    def send(self, msg: ATCMessage, addr: Tuple[str, int]) -> bool:
        """Nachricht an Adresse senden."""
        try:
            data = msg.encode()
            if len(data) > MAX_MSG_SIZE:
                return False
            self._sock.sendto(data, addr)
            self.stats["sent"] += 1
            return True
        except Exception:
            return False

    def broadcast(self, msg_type: MsgType, payload: bytes,
                  ttl: int = TTL_DEFAULT):
        """An alle bekannten Peers broadcasten."""
        msg = ATCMessage(
            version   = ATCNET_VERSION,
            msg_type  = msg_type,
            from_node = self.node_id,
            to_node   = "ff" * 20,
            payload   = payload,
            ttl       = ttl,
        )
        for node in self.routing.get_all_nodes():
            host, port = node.address.split(":")
            self.send(msg, (host, int(port)))

    def send_to(self, target_id: str, msg_type: MsgType, payload: bytes) -> bool:
        """Nachricht an spezifischen Node senden (routing)."""
        targets = self.routing.find_closest(target_id, count=3)
        if not targets:
            return False
        msg = ATCMessage(
            version   = ATCNET_VERSION,
            msg_type  = msg_type,
            from_node = self.node_id,
            to_node   = target_id,
            payload   = payload,
        )
        for node in targets:
            host, port = node.address.split(":")
            self.send(msg, (host, int(port)))
        return True

    # ── Empfangen ─────────────────────────────────────────

    def _recv_loop(self):
        """Empfangs-Thread."""
        while self._running:
            try:
                data, addr = self._sock.recvfrom(MAX_MSG_SIZE)
                msg = ATCMessage.decode(data)
                if msg:
                    self._dispatch(msg, addr)
                    self.stats["recv"] += 1
                else:
                    self.stats["dropped"] += 1
            except socket.timeout:
                continue
            except Exception:
                continue

    def _dispatch(self, msg: ATCMessage, from_addr: Tuple):
        """Nachricht an Handler weiterleiten."""
        # Duplikat-Erkennung
        if msg.msg_id in self._seen_msgs:
            return
        self._seen_msgs.add(msg.msg_id)
        if len(self._seen_msgs) > 10000:
            self._seen_msgs.clear()

        # TTL-Prüfung
        if msg.ttl <= 0:
            return

        # Sender-Node aktualisieren
        sender = NodeInfo(
            node_id   = msg.from_node,
            address   = f"{from_addr[0]}:{from_addr[1]}",
            last_seen = int(time.time() * 1000),
        )
        self.routing.add_node(sender)

        # Handler aufrufen
        handlers = self.handlers.get(msg.msg_type, [])
        for handler in handlers:
            try:
                handler(msg, from_addr)
            except Exception as e:
                pass

        # Weiterleiten wenn Broadcast und TTL > 0
        if msg.to_node == "ff" * 20 and msg.ttl > 1:
            msg.ttl -= 1
            self._forward(msg)

    def _forward(self, msg: ATCMessage):
        """Nachricht weiterleiten (Flooding mit TTL)."""
        for node in self.routing.get_all_nodes():
            if node.node_id != msg.from_node:
                host, port = node.address.split(":")
                self.send(msg, (host, int(port)))

    # ── Standard-Handler ──────────────────────────────────

    def _handle_ping(self, msg: ATCMessage, addr: Tuple):
        pong = ATCMessage(
            version=ATCNET_VERSION, msg_type=MsgType.PONG,
            from_node=self.node_id, to_node=msg.from_node,
            payload=b"pong", ttl=1,
        )
        self.send(pong, addr)

    def _handle_pong(self, msg: ATCMessage, addr: Tuple):
        self.stats["peers"] = self.routing.size()

    def _handle_hello(self, msg: ATCMessage, addr: Tuple):
        try:
            info = json.loads(msg.payload)
            node = NodeInfo(
                node_id   = msg.from_node,
                address   = f"{addr[0]}:{info.get('port', addr[1])}",
                atc_addr  = info.get("atc_addr", ""),
                version   = info.get("version", 1),
                last_seen = int(time.time() * 1000),
            )
            self.routing.add_node(node)
            # Mit eigenen Peers antworten
            self._send_peers(addr, msg.from_node)
        except Exception:
            pass

    def _handle_get_peers(self, msg: ATCMessage, addr: Tuple):
        self._send_peers(addr, msg.from_node)

    def _send_peers(self, addr: Tuple, to_node: str):
        peers = [
            {"id": n.node_id, "addr": n.address, "atc": n.atc_addr}
            for n in self.routing.get_all_nodes()[:10]
        ]
        resp = ATCMessage(
            version=ATCNET_VERSION, msg_type=MsgType.PEERS,
            from_node=self.node_id, to_node=to_node,
            payload=json.dumps(peers).encode(), ttl=1,
        )
        self.send(resp, addr)

    def _handle_peers(self, msg: ATCMessage, addr: Tuple):
        try:
            peers = json.loads(msg.payload)
            for p in peers:
                node = NodeInfo(
                    node_id=p["id"], address=p["addr"],
                    atc_addr=p.get("atc", ""),
                    last_seen=int(time.time() * 1000),
                )
                self.routing.add_node(node)
        except Exception:
            pass

    def _handle_find_node(self, msg: ATCMessage, addr: Tuple):
        target_id = msg.payload.decode()
        closest = self.routing.find_closest(target_id)
        payload = json.dumps([
            {"id": n.node_id, "addr": n.address}
            for n in closest
        ]).encode()
        resp = ATCMessage(
            version=ATCNET_VERSION, msg_type=MsgType.FOUND_NODE,
            from_node=self.node_id, to_node=msg.from_node,
            payload=payload, ttl=1,
        )
        self.send(resp, addr)

    # ── Discovery ─────────────────────────────────────────

    def connect_to(self, host: str, port: int) -> bool:
        """Mit einem Peer verbinden (HELLO senden)."""
        payload = json.dumps({
            "port":     self.port,
            "atc_addr": self.atc_addr,
            "version":  ATCNET_VERSION,
        }).encode()
        msg = ATCMessage(
            version=ATCNET_VERSION, msg_type=MsgType.HELLO,
            from_node=self.node_id, to_node="?" * 40,
            payload=payload, ttl=1,
        )
        return self.send(msg, (host, port))

    def ping(self, host: str, port: int) -> bool:
        """Einen Peer anpingen."""
        msg = ATCMessage(
            version=ATCNET_VERSION, msg_type=MsgType.PING,
            from_node=self.node_id, to_node="?" * 40,
            payload=b"ping", ttl=1,
        )
        return self.send(msg, (host, port))

    def _heartbeat_loop(self):
        """Regelmäßig Peers anpingen."""
        while self._running:
            time.sleep(10)
            for node in self.routing.get_all_nodes():
                host, port = node.address.split(":")
                self.ping(host, int(port))
            self.stats["peers"] = self.routing.size()

    # ── Info ─────────────────────────────────────────────

    def info(self) -> dict:
        return {
            "node_id":  self.node_id,
            "address":  f"{self.host}:{self.port}",
            "atc_addr": self.atc_addr,
            "peers":    self.routing.size(),
            "stats":    self.stats,
        }
