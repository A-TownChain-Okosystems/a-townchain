# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Node Discovery Service — UDP-basiertes P2P-Discovery-Protokoll.

Ermöglicht neuen Nodes, den Bootstrap-Node zu finden und sich im Netzwerk zu registrieren.
"""

import socket
import json
import threading
import time
import logging
from typing import Dict, Optional, List
from dataclasses import dataclass, asdict
from pathlib import Path

logger = logging.getLogger(__name__)


@dataclass
class PeerInfo:
    """Information über einen Peer im Netzwerk."""
    node_id: str
    host: str
    port: int
    last_seen: int  # Unix timestamp
    version: str = "2.0"

    def to_dict(self):
        return asdict(self)


class NodeDiscovery:
    """UDP-basierter Node-Discovery Service.
    
    Neue Nodes senden ANNOUNCE → Bootstrap antwortet mit PEER_LIST.
    Health-Check entfernt offline Peers automatisch.
    """

    # Nachrichtentypen
    MSG_ANNOUNCE = "ANNOUNCE"
    MSG_PEER_LIST = "PEER_LIST"
    MSG_PING = "PING"
    MSG_PONG = "PONG"

    def __init__(
        self,
        node_id: str,
        my_port: int,
        is_bootstrap: bool = False,
        bootstrap_nodes: Optional[List[str]] = None,
        config: Optional[Dict] = None,
    ):
        """Initialisiert den Discovery-Service.
        
        Args:
            node_id: Eindeutige Node-ID
            my_port: P2P-Port dieser Node
            is_bootstrap: Ob diese Node ein Bootstrap-Node ist
            bootstrap_nodes: Liste von Bootstrap-Nodes im Format "host:port"
            config: Konfigurationsdictionary aus settings.json
        """
        self.node_id = node_id
        self.my_port = my_port
        self.is_bootstrap = is_bootstrap
        self.bootstrap_nodes = bootstrap_nodes or []
        
        # Standard-Konfiguration
        self.config = config or {}
        self.discovery_port_offset = self.config.get("discovery_port_offset", 1000)
        self.ping_interval_sec = self.config.get("ping_interval_sec", 30)
        self.peer_timeout_sec = self.config.get("peer_timeout_sec", 90)
        self.max_peers = self.config.get("max_peers", 50)
        
        # Peers: node_id → PeerInfo
        self.peers: Dict[str, PeerInfo] = {}
        
        # UDP-Socket
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        self.discovery_port = my_port + self.discovery_port_offset
        
        self._running = False
        self._threads: List[threading.Thread] = []

    def start(self) -> None:
        """Startet den Discovery-Service."""
        if self._running:
            logger.warning(f"Discovery-Service für {self.node_id} läuft bereits")
            return
        
        try:
            self.sock.bind(("0.0.0.0", self.discovery_port))
            self._running = True
            logger.info(f"Discovery-Service gestartet auf Port {self.discovery_port}")
            
            # Starte Listen-Thread
            listen_thread = threading.Thread(target=self.listen, daemon=True)
            listen_thread.start()
            self._threads.append(listen_thread)
            
            # Starte Health-Check-Thread
            health_thread = threading.Thread(target=self.health_check, daemon=True)
            health_thread.start()
            self._threads.append(health_thread)
            
            # Wenn nicht Bootstrap: ANNOUNCE senden
            if not self.is_bootstrap:
                announce_thread = threading.Thread(target=self._auto_announce, daemon=True)
                announce_thread.start()
                self._threads.append(announce_thread)
        except OSError as e:
            logger.error(f"Fehler beim Starten des Discovery-Service: {e}")
            self._running = False
            raise

    def stop(self) -> None:
        """Stoppt den Discovery-Service."""
        self._running = False
        try:
            self.sock.close()
        except:
            pass
        logger.info(f"Discovery-Service für {self.node_id} gestoppt")

    def announce(self) -> None:
        """Meldet sich bei allen Bootstrap-Nodes an."""
        msg = json.dumps({
            "type": self.MSG_ANNOUNCE,
            "node_id": self.node_id,
            "port": self.my_port,
            "version": "2.0",
        }).encode()
        
        for bootstrap in self.bootstrap_nodes:
            try:
                if isinstance(bootstrap, str):
                    host, port = bootstrap.split(":")
                    port = int(port)
                else:
                    host, port = bootstrap
                
                discovery_port = port + self.discovery_port_offset
                self.sock.sendto(msg, (host, discovery_port))
                logger.debug(f"ANNOUNCE an {host}:{discovery_port} gesendet")
            except Exception as e:
                logger.error(f"Fehler beim ANNOUNCE an {bootstrap}: {e}")

    def listen(self) -> None:
        """Empfängt Discovery-Nachrichten (UDP-Listener)."""
        while self._running:
            try:
                data, addr = self.sock.recvfrom(4096)
                msg = json.loads(data.decode())
                self._handle(msg, addr)
            except json.JSONDecodeError:
                logger.warning(f"Ungültige JSON von {addr}")
            except Exception as e:
                if self._running:
                    logger.error(f"Fehler in Discovery-Listener: {e}")

    def _handle(self, msg: Dict, addr: tuple) -> None:
        """Verarbeitet eingehende Discovery-Nachricht."""
        msg_type = msg.get("type")
        
        if msg_type == self.MSG_ANNOUNCE:
            self._handle_announce(msg, addr)
        elif msg_type == self.MSG_PEER_LIST:
            self._handle_peer_list(msg, addr)
        elif msg_type == self.MSG_PING:
            self._handle_ping(addr)
        elif msg_type == self.MSG_PONG:
            self._handle_pong(msg)

    def _handle_announce(self, msg: Dict, addr: tuple) -> None:
        """Bootstrap-Node registriert einen neuen Peer."""
        node_id = msg.get("node_id")
        port = msg.get("port")
        version = msg.get("version", "2.0")
        
        if not node_id or not port:
            logger.warning(f"Ungültiges ANNOUNCE von {addr}")
            return
        
        # Peer registrieren
        peer_info = PeerInfo(
            node_id=node_id,
            host=addr[0],
            port=port,
            last_seen=int(time.time()),
            version=version,
        )
        self.peers[node_id] = peer_info
        logger.info(f"Neuer Peer registriert: {node_id} ({addr[0]}:{port})")
        
        # Peer-Liste zurückschicken
        self._send_peer_list(addr)

    def _handle_peer_list(self, msg: Dict, addr: tuple) -> None:
        """Empfängt eine Peer-Liste vom Bootstrap-Node."""
        peers_data = msg.get("peers", {})
        
        for node_id, peer_dict in peers_data.items():
            if node_id != self.node_id:  # Sich selbst ignorieren
                peer_info = PeerInfo(
                    node_id=node_id,
                    host=peer_dict["host"],
                    port=peer_dict["port"],
                    last_seen=int(time.time()),
                    version=peer_dict.get("version", "2.0"),
                )
                self.peers[node_id] = peer_info
        
        logger.info(f"PEER_LIST empfangen: {len(peers_data)} Peers")

    def _handle_ping(self, addr: tuple) -> None:
        """Antwortet auf PING mit PONG."""
        response = json.dumps({"type": self.MSG_PONG}).encode()
        self.sock.sendto(response, addr)

    def _handle_pong(self, msg: Dict) -> None:
        """Aktualisiert last_seen bei PONG-Antwort."""
        # last_seen wird bereits in health_check aktualisiert
        pass

    def _send_peer_list(self, addr: tuple) -> None:
        """Sendet die aktuelle Peer-Liste an eine Adresse."""
        peers_dict = {nid: pi.to_dict() for nid, pi in self.peers.items()}
        msg = json.dumps({
            "type": self.MSG_PEER_LIST,
            "peers": peers_dict,
        }).encode()
        try:
            self.sock.sendto(msg, addr)
        except Exception as e:
            logger.error(f"Fehler beim Senden der PEER_LIST an {addr}: {e}")

    def health_check(self) -> None:
        """Pingt periodisch alle Peers — entfernt offline Peers."""
        while self._running:
            try:
                time.sleep(self.ping_interval_sec)
                
                now = int(time.time())
                dead_peers = [
                    nid for nid, info in self.peers.items()
                    if now - info.last_seen > self.peer_timeout_sec
                ]
                
                for nid in dead_peers:
                    del self.peers[nid]
                    logger.info(f"Offline-Peer entfernt: {nid}")
                
                # PING an alle Peers senden
                ping_msg = json.dumps({"type": self.MSG_PING}).encode()
                for nid, peer_info in list(self.peers.items()):
                    try:
                        self.sock.sendto(
                            ping_msg,
                            (peer_info.host, peer_info.port + self.discovery_port_offset)
                        )
                    except Exception as e:
                        logger.debug(f"Fehler beim Senden von PING an {nid}: {e}")
            except Exception as e:
                logger.error(f"Fehler in Health-Check: {e}")

    def _auto_announce(self) -> None:
        """Sendet alle 120s ANNOUNCE zur Reregistrierung."""
        while self._running:
            time.sleep(120)
            self.announce()

    def get_peers(self) -> Dict[str, PeerInfo]:
        """Gibt die aktuellen bekannten Peers zurück."""
        return dict(self.peers)

    def get_peer_count(self) -> int:
        """Gibt die Anzahl der Peers zurück."""
        return len(self.peers)

    def save_peers(self, path: str = "data/peers.json") -> None:
        """Speichert die Peer-Liste in eine Datei."""
        try:
            Path(path).parent.mkdir(parents=True, exist_ok=True)
            peers_dict = {nid: pi.to_dict() for nid, pi in self.peers.items()}
            with open(path, "w") as f:
                json.dump(peers_dict, f, indent=2)
            logger.info(f"Peers gespeichert: {path}")
        except Exception as e:
            logger.error(f"Fehler beim Speichern von Peers: {e}")

    def load_peers(self, path: str = "data/peers.json") -> None:
        """Lädt die Peer-Liste aus einer Datei."""
        try:
            with open(path) as f:
                peers_dict = json.load(f)
                for nid, pi_dict in peers_dict.items():
                    peer_info = PeerInfo(**pi_dict)
                    self.peers[nid] = peer_info
            logger.info(f"Peers geladen: {path} ({len(self.peers)} Peers)")
        except FileNotFoundError:
            logger.debug(f"Keine Peer-Datei gefunden: {path}")
        except Exception as e:
            logger.error(f"Fehler beim Laden von Peers: {e}")

    def get_status(self) -> Dict:
        """Gibt den aktuellen Status des Discovery-Service zurück."""
        return {
            "node_id": self.node_id,
            "is_bootstrap": self.is_bootstrap,
            "running": self._running,
            "discovery_port": self.discovery_port,
            "peers_count": len(self.peers),
            "peers": [pi.to_dict() for pi in self.peers.values()],
        }
