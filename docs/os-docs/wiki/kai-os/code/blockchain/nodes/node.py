# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# blockchain/nodes/node.py
# A-TownChain Node — P2P Netzwerk-Teilnehmer
#
# Node-Typen:
#   FULL    — speichert gesamte Chain
#   LIGHT   — nur Block-Header
#   VALIDATOR — nimmt am PoS Konsens teil
#   MINER   — führt PoW aus

import hashlib, time, json, threading
from enum import Enum

class NodeType(Enum):
    FULL      = "full"
    LIGHT     = "light"
    VALIDATOR = "validator"
    MINER     = "miner"

class ATCNode:
    """Ein Knoten im A-TownChain Netzwerk."""

    def __init__(self, node_id: str, node_type: NodeType = NodeType.FULL):
        self.node_id    = node_id
        self.node_type  = node_type
        self.peers      = []      # verbundene Nodes
        self.mempool    = []      # ausstehende Transaktionen
        self.chain      = []      # lokale Blockchain-Kopie
        self.stake      = 0       # ATC Stake für PoS
        self.online     = True
        self.joined_at  = int(time.time())

    def connect_peer(self, peer_node: "ATCNode"):
        if peer_node.node_id not in [p.node_id for p in self.peers]:
            self.peers.append(peer_node)
            peer_node.peers.append(self)
            print(f"[NODE {self.node_id}] Connected to peer {peer_node.node_id}")

    def broadcast(self, message: dict):
        """Sendet eine Nachricht an alle Peers."""
        for peer in self.peers:
            peer.receive(message)

    def receive(self, message: dict):
        """Empfängt eine Nachricht von einem Peer."""
        msg_type = message.get("type")
        if msg_type == "new_tx":
            self.mempool.append(message.get("tx"))
        elif msg_type == "new_block":
            self._validate_and_add_block(message.get("block"))

    def _validate_and_add_block(self, block: dict) -> bool:
        if not block:
            return False
        # Basis-Validierung
        if block.get("prev_hash") == (self.chain[-1]["hash"] if self.chain else "0"*64):
            self.chain.append(block)
            # Bestätigte TXs aus Mempool entfernen
            confirmed = {tx["tx_id"] for tx in block.get("transactions", [])}
            self.mempool = [tx for tx in self.mempool if tx.get("tx_id") not in confirmed]
            return True
        return False

    def get_info(self) -> dict:
        return {
            "node_id":    self.node_id,
            "type":       self.node_type.value,
            "peers":      len(self.peers),
            "chain_len":  len(self.chain),
            "mempool":    len(self.mempool),
            "stake":      self.stake,
            "online":     self.online,
            "uptime":     int(time.time()) - self.joined_at
        }


class NodeNetwork:
    """Verwaltet das gesamte A-TownChain Node-Netzwerk."""

    def __init__(self):
        self.nodes = {}

    def add_node(self, node: ATCNode):
        self.nodes[node.node_id] = node

    def get_node(self, node_id: str) -> ATCNode:
        return self.nodes.get(node_id)

    def get_all_info(self) -> list:
        return [n.get_info() for n in self.nodes.values()]

    def get_stats(self) -> dict:
        online = sum(1 for n in self.nodes.values() if n.online)
        return {
            "total_nodes":  len(self.nodes),
            "online":       online,
            "offline":      len(self.nodes) - online,
            "total_stake":  sum(n.stake for n in self.nodes.values()),
            "avg_peers":    sum(len(n.peers) for n in self.nodes.values()) // max(len(self.nodes),1)
        }
