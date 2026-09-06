# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""P2P-Netzwerk für Block- und TX-Propagation.

TCP-basiertes Messaging-System mit Duplikat-Filter und Flood-Fill-Broadcasting.
"""

import socket
import json
import threading
import logging
from typing import Dict, Optional, Callable, Any
from collections import deque
from dataclasses import dataclass
import time

logger = logging.getLogger(__name__)


@dataclass
class P2PMessage:
    """Struktur einer P2P-Nachricht."""
    type: str
    sender: str
    payload: Dict[str, Any]

    def to_bytes(self) -> bytes:
        """Konvertiert die Nachricht zu Bytes mit Length-Prefix."""
        data = json.dumps({
            "type": self.type,
            "sender": self.sender,
            "payload": self.payload,
        }).encode()
        # Length-Prefix (4 bytes, big-endian)
        return len(data).to_bytes(4, "big") + data

    @staticmethod
    def from_bytes(data: bytes) -> "P2PMessage":
        """Rekonstruiert eine Nachricht aus Bytes."""
        msg_dict = json.loads(data.decode())
        return P2PMessage(
            type=msg_dict["type"],
            sender=msg_dict["sender"],
            payload=msg_dict["payload"],
        )


class P2PBroadcaster:
    """TCP-basiertes P2P-Netzwerk für Block- und TX-Propagation."""

    # Nachrichtentypen
    MSG_NEW_BLOCK = "NEW_BLOCK"
    MSG_NEW_TX = "NEW_TX"
    MSG_GET_BLOCKS = "GET_BLOCKS"
    MSG_BLOCKS = "BLOCKS"
    MSG_GET_HEIGHT = "GET_HEIGHT"
    MSG_HEIGHT = "HEIGHT"
    MSG_HANDSHAKE = "HANDSHAKE"

    def __init__(
        self,
        node_id: str,
        port: int,
        consensus=None,
        config: Optional[Dict] = None,
    ):
        """Initialisiert das P2P-Netzwerk.
        
        Args:
            node_id: Eindeutige Node-ID
            port: TCP-Port für P2P-Verbindungen
            consensus: Referenz zum Consensus-Modul (optional)
            config: Konfigurationsdictionary
        """
        self.node_id = node_id
        self.port = port
        self.consensus = consensus
        self.config = config or {}
        
        # Peer-Verwaltung: node_id → socket
        self.peers: Dict[str, socket.socket] = {}
        self.peer_lock = threading.Lock()
        
        # Duplikat-Filter: Message-Hash im Circular Buffer (max 10.000)
        self.seen_msgs = deque(maxlen=10000)
        self.seen_lock = threading.Lock()
        
        # Message-Handler: msg_type → callable
        self.handlers: Dict[str, Callable] = {}
        self.register_default_handlers()
        
        # Server-Socket
        self.server_sock: Optional[socket.socket] = None
        self._running = False
        self._threads: list = []

    def register_handler(self, msg_type: str, handler: Callable) -> None:
        """Registriert einen Handler für einen Nachrichtentyp."""
        self.handlers[msg_type] = handler
        logger.debug(f"Handler für {msg_type} registriert")

    def register_default_handlers(self) -> None:
        """Registriert Standard-Handler."""
        self.register_handler(self.MSG_NEW_BLOCK, self._handle_new_block)
        self.register_handler(self.MSG_NEW_TX, self._handle_new_tx)
        self.register_handler(self.MSG_HANDSHAKE, self._handle_handshake)

    def start(self) -> None:
        """Startet den P2P-Listener."""
        if self._running:
            logger.warning(f"P2P-Service für {self.node_id} läuft bereits")
            return
        
        try:
            self.server_sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.server_sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            self.server_sock.bind(("0.0.0.0", self.port))
            self.server_sock.listen(50)
            self._running = True
            logger.info(f"P2P-Service gestartet auf Port {self.port}")
            
            # Starte Listener-Thread
            listener_thread = threading.Thread(target=self._listener_loop, daemon=True)
            listener_thread.start()
            self._threads.append(listener_thread)
        except OSError as e:
            logger.error(f"Fehler beim Starten des P2P-Service: {e}")
            self._running = False
            raise

    def stop(self) -> None:
        """Stoppt den P2P-Service."""
        self._running = False
        
        # Schließe alle Peer-Verbindungen
        with self.peer_lock:
            for node_id, sock in self.peers.items():
                try:
                    sock.close()
                except:
                    pass
            self.peers.clear()
        
        # Schließe Server-Socket
        if self.server_sock:
            try:
                self.server_sock.close()
            except:
                pass
        
        logger.info(f"P2P-Service für {self.node_id} gestoppt")

    def _listener_loop(self) -> None:
        """Haupt-Listener-Schleife (akzeptiert eingehende Verbindungen)."""
        while self._running:
            try:
                conn, addr = self.server_sock.accept()
                thread = threading.Thread(
                    target=self._handle_connection,
                    args=(conn, addr),
                    daemon=True,
                )
                thread.start()
            except Exception as e:
                if self._running:
                    logger.error(f"Fehler in P2P-Listener: {e}")

    def _handle_connection(self, conn: socket.socket, addr: tuple) -> None:
        """Behandelt eine eingehende P2P-Verbindung."""
        try:
            while self._running:
                # Lese Length-Prefix (4 bytes)
                raw_len = conn.recv(4)
                if not raw_len:
                    break
                
                length = int.from_bytes(raw_len, "big")
                if length > 1_000_000:  # Max 1MB pro Nachricht
                    logger.warning(f"Nachricht zu groß: {length} bytes von {addr}")
                    break
                
                # Lese Nachricht-Daten
                data = b""
                while len(data) < length:
                    chunk = conn.recv(min(4096, length - len(data)))
                    if not chunk:
                        break
                    data += chunk
                
                if len(data) < length:
                    break
                
                # Parse und verarbeite Nachricht
                try:
                    msg = P2PMessage.from_bytes(data)
                    self._handle_message(msg)
                except json.JSONDecodeError:
                    logger.warning(f"Ungültige JSON von {addr}")
        except Exception as e:
            logger.error(f"Fehler in Peer-Verbindung {addr}: {e}")
        finally:
            try:
                conn.close()
            except:
                pass

    def _handle_message(self, msg: P2PMessage) -> None:
        """Verarbeitet eine empfangene Nachricht."""
        # Duplikat-Check
        msg_hash = hash(json.dumps({
            "type": msg.type,
            "sender": msg.sender,
            "payload": msg.payload,
        }, sort_keys=True))
        
        with self.seen_lock:
            if msg_hash in self.seen_msgs:
                logger.debug(f"Duplikat ignoriert: {msg.type} von {msg.sender}")
                return
            self.seen_msgs.append(msg_hash)
        
        # Handler aufrufen
        handler = self.handlers.get(msg.type)
        if handler:
            try:
                handler(msg)
            except Exception as e:
                logger.error(f"Fehler in Handler {msg.type}: {e}")
        else:
            logger.debug(f"Kein Handler für {msg.type}")

    def broadcast_block(self, block: Dict[str, Any]) -> None:
        """Sendet einen neuen Block an alle Peers (Flood-Fill)."""
        msg = P2PMessage(
            type=self.MSG_NEW_BLOCK,
            sender=self.node_id,
            payload={"block": block},
        )
        self._broadcast(msg)

    def broadcast_tx(self, tx: Dict[str, Any]) -> None:
        """Sendet eine neue Transaktion an alle Peers."""
        msg = P2PMessage(
            type=self.MSG_NEW_TX,
            sender=self.node_id,
            payload={"tx": tx},
        )
        self._broadcast(msg)

    def _broadcast(self, msg: P2PMessage) -> None:
        """Sendet eine Nachricht an alle verbundenen Peers (Flood-Fill)."""
        data = msg.to_bytes()
        dead_peers = []
        
        with self.peer_lock:
            for node_id, sock in list(self.peers.items()):
                try:
                    sock.sendall(data)
                except (OSError, BrokenPipeError):
                    dead_peers.append(node_id)
        
        # Tote Verbindungen entfernen
        for node_id in dead_peers:
            self._remove_peer(node_id)
            logger.debug(f"Tote Peer-Verbindung entfernt: {node_id}")

    def connect_to_peer(
        self,
        node_id: str,
        host: str,
        port: int,
        timeout: int = 10,
    ) -> bool:
        """Verbindet sich mit einem anderen Peer."""
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(timeout)
            sock.connect((host, port))
            sock.settimeout(None)  # Blocking-Mode nach erfolgreicher Verbindung
            
            with self.peer_lock:
                self.peers[node_id] = sock
            
            logger.info(f"Mit Peer verbunden: {node_id} ({host}:{port})")
            
            # Starte Handshake
            self._send_handshake(sock, node_id)
            
            return True
        except Exception as e:
            logger.error(f"Fehler beim Verbinden mit {node_id}: {e}")
            return False

    def _send_handshake(self, sock: socket.socket, node_id: str) -> None:
        """Sendet einen Handshake an einen neuen Peer."""
        chain_height = 0
        if self.consensus:
            chain_height = getattr(self.consensus, "height", 0)
        
        msg = P2PMessage(
            type=self.MSG_HANDSHAKE,
            sender=self.node_id,
            payload={
                "version": "2.0",
                "chain_height": chain_height,
                "node_type": "full",
            },
        )
        try:
            sock.sendall(msg.to_bytes())
        except Exception as e:
            logger.error(f"Fehler beim Senden von Handshake: {e}")

    def _remove_peer(self, node_id: str) -> None:
        """Entfernt einen Peer."""
        with self.peer_lock:
            if node_id in self.peers:
                try:
                    self.peers[node_id].close()
                except:
                    pass
                del self.peers[node_id]

    # Standard-Handler
    def _handle_new_block(self, msg: P2PMessage) -> None:
        """Handler für NEW_BLOCK-Nachrichten."""
        block = msg.payload.get("block")
        if not block:
            return
        
        # Consensus validieren und akzeptieren
        if self.consensus and hasattr(self.consensus, "_validate_block"):
            if self.consensus._validate_block(block):
                # Block zur Chain hinzufügen
                if hasattr(self.consensus, "blocks"):
                    self.consensus.blocks.append(block)
                    if hasattr(self.consensus, "height"):
                        self.consensus.height += 1
                    # Weiterleiten an andere Peers
                    self.broadcast_block(block)
                    logger.info(f"Block #{block.get('height', '?')} akzeptiert und weitergeleitet")

    def _handle_new_tx(self, msg: P2PMessage) -> None:
        """Handler für NEW_TX-Nachrichten."""
        tx = msg.payload.get("tx")
        if not tx:
            return
        
        # TX in Mempool hinzufügen
        if self.consensus:
            if not hasattr(self.consensus, "mempool"):
                self.consensus.mempool = []
            self.consensus.mempool.append(tx)
            logger.debug(f"TX in Mempool hinzugefügt: {tx.get('hash', '?')[:8]}")

    def _handle_handshake(self, msg: P2PMessage) -> None:
        """Handler für HANDSHAKE-Nachrichten."""
        version = msg.payload.get("version")
        chain_height = msg.payload.get("chain_height")
        logger.info(
            f"Handshake von {msg.sender}: Version {version}, "
            f"Chain Height {chain_height}"
        )

    def get_status(self) -> Dict[str, Any]:
        """Gibt den aktuellen Status des P2P-Netzwerks zurück."""
        with self.peer_lock:
            peer_list = list(self.peers.keys())
        
        return {
            "node_id": self.node_id,
            "running": self._running,
            "port": self.port,
            "peers": peer_list,
            "peers_count": len(peer_list),
            "seen_messages_count": len(self.seen_msgs),
        }

    def get_peers(self) -> list:
        """Gibt die Liste der verbundenen Peers zurück."""
        with self.peer_lock:
            return list(self.peers.keys())
