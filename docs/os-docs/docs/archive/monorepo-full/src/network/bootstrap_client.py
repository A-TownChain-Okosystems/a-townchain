# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ATCNet Bootstrap Client — ATN-1000 Standard
Verbindet sich mit Bootstrap Node und holt Peer-Liste.
"""
import socket, json, time, logging
from typing import List, Optional
from dataclasses import dataclass

logger = logging.getLogger("atcnet.bootstrap_client")

BOOTSTRAP_NODES = [
    ("bootstrap.atcnet.io", 4001),
    ("bootstrap2.atcnet.io", 4001),
    ("localhost", 4001),
]

@dataclass
class Peer:
    node_id: str
    host: str
    port: int
    last_seen: float = 0.0

class BootstrapClient:
    """Verbindet neuen Node mit dem ATCNet via Bootstrap."""

    def __init__(self, node_id: str, host: str = "0.0.0.0", port: int = 4002,
                 bootstrap_nodes=None):
        self.node_id   = node_id
        self.host      = host
        self.port      = port
        self.bootstraps = bootstrap_nodes or BOOTSTRAP_NODES
        self.peers: List[Peer] = []

    def announce(self, timeout: float = 5.0) -> List[Peer]:
        """Meldet diesen Node beim Bootstrap an und holt Peer-Liste."""
        msg = json.dumps({
            "type":    "ANNOUNCE",
            "node_id": self.node_id,
            "host":    self.host,
            "port":    self.port,
            "capabilities": ["relay", "validator"],
        }).encode()

        for bs_host, bs_port in self.bootstraps:
            try:
                sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
                sock.settimeout(timeout)
                sock.sendto(msg, (bs_host, bs_port))
                data, _ = sock.recvfrom(65535)
                resp = json.loads(data.decode())
                if resp.get("type") == "PEER_LIST":
                    self.peers = [
                        Peer(p["node_id"], p["host"], p["port"], p.get("last_seen", 0))
                        for p in resp.get("peers", [])
                    ]
                    logger.info(f"[BootstrapClient] {len(self.peers)} Peers von {bs_host}:{bs_port}")
                    return self.peers
            except Exception as e:
                logger.warning(f"[BootstrapClient] {bs_host}:{bs_port} nicht erreichbar: {e}")
            finally:
                sock.close()
        return []

    def ping(self, bs_host: str, bs_port: int = 4001) -> bool:
        """Heartbeat an Bootstrap Node."""
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
            sock.settimeout(3.0)
            sock.sendto(json.dumps({"type":"PING","node_id":self.node_id}).encode(),
                        (bs_host, bs_port))
            data, _ = sock.recvfrom(1024)
            resp = json.loads(data.decode())
            return resp.get("type") == "PONG"
        except:
            return False
        finally:
            sock.close()

    def get_peers(self, limit: int = 50) -> List[Peer]:
        """Aktuelle Peer-Liste vom Bootstrap holen."""
        for bs_host, bs_port in self.bootstraps:
            try:
                sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
                sock.settimeout(5.0)
                sock.sendto(json.dumps({"type":"GET_PEERS","limit":limit}).encode(),
                            (bs_host, bs_port))
                data, _ = sock.recvfrom(65535)
                resp = json.loads(data.decode())
                if resp.get("type") == "PEER_LIST":
                    return [Peer(p["node_id"],p["host"],p["port"]) for p in resp.get("peers",[])]
            except:
                pass
            finally:
                sock.close()
        return []
