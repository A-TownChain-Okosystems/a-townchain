# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Bootstrap-Node: DNS Seed & Peer Discovery (Fix #68)

Implementiert das Bitcoin-artige Seed-Node-Muster (vgl. memory: hardcoded
DNS-Seeds als Fallback, AddrMan new/tried Tables, Gossip via ADDR/GETADDR).
"""
import ipaddress
import json
import os
import socket
import time
import uuid
from dataclasses import dataclass, field, asdict
from typing import Dict, List, Optional


BOOTSTRAP_CONFIG = {
    "bootstrap_port": 5005,
    "kademlia_k": 20,
    "kademlia_alpha": 3,
    "hardcoded_seeds": [
        {"ip": "5.9.104.210", "port": 5005},
        {"ip": "95.217.12.33", "port": 5005},
        {"ip": "78.46.222.55", "port": 5005},
        {"ip": "148.251.190.4", "port": 5005},
    ],
    "stale_after_days": 14,
    "max_failed_attempts": 3,
    "max_addr_sample": 1000,
}


@dataclass
class PeerAddress:
    ip: str
    port: int
    last_seen: int = field(default_factory=lambda: int(time.time()))
    last_tried: int = 0
    attempt_count: int = 0
    services: int = 1
    source: str = "unknown"
    version: str = "3.2.1"

    @property
    def key(self) -> str:
        return f"{self.ip}:{self.port}"

    @property
    def is_stale(self) -> bool:
        max_age = BOOTSTRAP_CONFIG["stale_after_days"] * 86400
        return (int(time.time()) - self.last_seen) > max_age

    def to_dict(self) -> Dict:
        return asdict(self)

    @staticmethod
    def from_dict(d: Dict) -> "PeerAddress":
        return PeerAddress(
            ip=d["ip"],
            port=d["port"],
            last_seen=d.get("last_seen", int(time.time())),
            last_tried=d.get("last_tried", 0),
            attempt_count=d.get("attempt_count", 0),
            services=d.get("services", 1),
            source=d.get("source", "unknown"),
            version=d.get("version", "3.2.1"),
        )


class AddrMan:
    """Adressverwaltung: 'new' (unverifiziert) + 'tried' (erfolgreich verbunden) Tables."""

    def __init__(self, path: str):
        self.path = path
        self.new_table: Dict[str, PeerAddress] = {}
        self.tried_table: Dict[str, PeerAddress] = {}
        self._load()

    def add(self, peer: PeerAddress):
        if peer.key in self.tried_table:
            return  # bereits verifiziert, kein Downgrade zu 'new'
        self.new_table[peer.key] = peer

    def mark_tried(self, key: str):
        peer = self.new_table.pop(key, None) or self.tried_table.get(key)
        if peer is None:
            return
        peer.last_tried = int(time.time())
        peer.attempt_count = 0
        self.tried_table[key] = peer

    def mark_failed(self, key: str):
        peer = self.new_table.get(key) or self.tried_table.get(key)
        if peer is None:
            return
        peer.attempt_count += 1
        peer.last_tried = int(time.time())
        if peer.attempt_count >= BOOTSTRAP_CONFIG["max_failed_attempts"]:
            self.new_table.pop(key, None)
            self.tried_table.pop(key, None)

    def get_candidates(self, count: int) -> List[PeerAddress]:
        fresh = [p for p in self.new_table.values() if not p.is_stale]
        fresh.sort(key=lambda p: p.last_seen, reverse=True)
        return fresh[:count]

    def get_addr_sample(self, count: int) -> List[Dict]:
        count = min(count, BOOTSTRAP_CONFIG["max_addr_sample"])
        all_peers = list(self.new_table.values()) + list(self.tried_table.values())
        sample = all_peers[:count]
        return [p.to_dict() for p in sample]

    @property
    def stats(self) -> Dict[str, int]:
        return {
            "new_count": len(self.new_table),
            "tried_count": len(self.tried_table),
            "total": len(self.new_table) + len(self.tried_table),
        }

    def save(self):
        data = {
            "new": [p.to_dict() for p in self.new_table.values()],
            "tried": [p.to_dict() for p in self.tried_table.values()],
        }
        os.makedirs(os.path.dirname(self.path) or ".", exist_ok=True)
        with open(self.path, "w") as f:
            json.dump(data, f)

    def _load(self):
        if not os.path.exists(self.path):
            return
        try:
            with open(self.path) as f:
                data = json.load(f)
            for d in data.get("new", []):
                p = PeerAddress.from_dict(d)
                self.new_table[p.key] = p
            for d in data.get("tried", []):
                p = PeerAddress.from_dict(d)
                self.tried_table[p.key] = p
        except (json.JSONDecodeError, OSError):
            pass


class DNSSeedResolver:
    """Loest DNS-Seed-Hostnamen zu Peer-IPs auf, mit hardcoded Fallback."""

    def __init__(self, seeds: Optional[List[str]] = None, timeout: float = 5.0):
        self.seeds = seeds if seeds is not None else [
            "seed1.a-townchain.dev",
            "seed2.a-townchain.dev",
        ]
        self.timeout = timeout

    @staticmethod
    def _is_valid_ip(ip: str) -> bool:
        try:
            addr = ipaddress.ip_address(ip)
        except ValueError:
            return False
        if addr.is_private or addr.is_loopback or addr.is_link_local or addr.is_reserved:
            return False
        return True

    def get_hardcoded_seeds(self) -> List[PeerAddress]:
        return [
            PeerAddress(ip=s["ip"], port=s["port"], source="hardcoded")
            for s in BOOTSTRAP_CONFIG["hardcoded_seeds"]
        ]

    def resolve_seed(self, hostname: str) -> List[PeerAddress]:
        try:
            socket.setdefaulttimeout(self.timeout)
            infos = socket.getaddrinfo(hostname, None)
            ips = {info[4][0] for info in infos}
            peers = []
            for ip in ips:
                if self._is_valid_ip(ip):
                    peers.append(PeerAddress(ip=ip, port=BOOTSTRAP_CONFIG["bootstrap_port"], source="dns"))
            return peers
        except (socket.gaierror, socket.timeout, OSError):
            return []

    def resolve_all(self) -> List[PeerAddress]:
        results: List[PeerAddress] = []
        for seed in self.seeds:
            results.extend(self.resolve_seed(seed))
        return results


class BootstrapNode:
    """Seed-Node: beantwortet ADDR/GETADDR, verwaltet Peers via AddrMan."""

    def __init__(self, node_id: Optional[str] = None, data_dir: str = "."):
        self.node_id = node_id or f"BOOTSTRAP-{uuid.uuid4().hex[:12]}"
        self.data_dir = data_dir
        self.addrman = AddrMan(os.path.join(data_dir, "peers.dat"))
        self.resolver = DNSSeedResolver()
        self.connected: Dict[str, PeerAddress] = {}

    def bootstrap(self) -> Dict:
        """Versucht DNS-Seeds aufzuloesen, faellt sonst auf hardcoded Seeds zurueck."""
        dns_peers = self.resolver.resolve_all()
        if dns_peers:
            for p in dns_peers:
                self.addrman.add(p)
            return {"source": "dns", "dns_peers": len(dns_peers)}

        hardcoded = self.resolver.get_hardcoded_seeds()
        for p in hardcoded:
            self.addrman.add(p)
        return {"source": "hardcoded_fallback", "hardcoded_peers": len(hardcoded)}

    def handle_addr(self, peers: List[Dict], source_ip: str) -> int:
        """Verarbeitet eingehende ADDR-Nachricht; filtert loopback/private IPs."""
        added = 0
        for p in peers:
            if not DNSSeedResolver._is_valid_ip(p["ip"]):
                continue
            self.addrman.add(PeerAddress(ip=p["ip"], port=p["port"], source="gossip"))
            added += 1
        return added

    def handle_getaddr(self, requester_ip: str) -> List[Dict]:
        """Beantwortet GETADDR mit einer Stichprobe bekannter Peers (max 1000)."""
        return self.addrman.get_addr_sample(BOOTSTRAP_CONFIG["max_addr_sample"])

    def get_peers_for_new_node(self, requester_ip: str, count: int = 8) -> List[Dict]:
        """Liefert frische Kandidaten-Peers fuer einen neu beitretenden Node."""
        candidates = self.addrman.get_candidates(count)
        return [p.to_dict() for p in candidates]

    def mark_peer_connected(self, ip: str, port: int):
        key = f"{ip}:{port}"
        peer = self.addrman.new_table.get(key) or self.addrman.tried_table.get(key)
        if peer:
            self.addrman.mark_tried(key)
            self.connected[key] = peer

    def mark_peer_failed(self, ip: str, port: int):
        key = f"{ip}:{port}"
        self.addrman.mark_failed(key)
        self.connected.pop(key, None)

    def get_status(self) -> Dict:
        return {
            "node_id": self.node_id,
            "connected_peers": len(self.connected),
            "addrman": self.addrman.stats,
            "config": {
                "bootstrap_port": BOOTSTRAP_CONFIG["bootstrap_port"],
                "kademlia_k": BOOTSTRAP_CONFIG["kademlia_k"],
                "kademlia_alpha": BOOTSTRAP_CONFIG["kademlia_alpha"],
            },
        }
