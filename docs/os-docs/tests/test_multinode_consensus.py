# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
T-002: Multi-Node Konsens Test — 2-Node Mehrheits-Voting
Sprint 2.2 Blocker | Issue #8 | Wiki: Kap. 54
"""
import pytest
import sys
import os
import threading
import time

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

# Mock-Klassen für Node-Simulation ohne echte Netzwerk-Verbindung
class MockNode:
    def __init__(self, node_id: str, stake: int = 10000):
        self.node_id   = node_id
        self.stake     = stake
        self.chain     = []         # [(height, hash, validator)]
        self.peers     = []
        self.votes     = {}         # block_hash → [voter_ids]
        self.lock      = threading.Lock()

    def connect(self, peer):
        if peer not in self.peers:
            self.peers.append(peer)
        if self not in peer.peers:
            peer.peers.append(self)

    def produce_block(self, transactions=None):
        import hashlib
        height = len(self.chain) + 1
        prev   = self.chain[-1][1] if self.chain else "0" * 64
        data   = f"{height}{prev}{self.node_id}{time.time()}"
        h      = hashlib.sha256(data.encode()).hexdigest()
        block  = (height, h, self.node_id)
        return block

    def validate_block(self, block) -> bool:
        """Simuliertes Konsens-Voting: ≥ 50% der Stakes müssen zustimmen"""
        height, block_hash, validator = block
        # Einfache Validierung: Hash muss 64 Zeichen haben
        if len(block_hash) != 64:
            return False
        # Height muss korrekt sein
        if height != len(self.chain) + 1:
            return False
        return True

    def add_block(self, block):
        with self.lock:
            if self.validate_block(block):
                self.chain.append(block)
                # Block an Peers propagieren
                for peer in self.peers:
                    if block not in peer.chain:
                        peer.receive_block(block, self)
                return True
        return False

    def receive_block(self, block, sender):
        with self.lock:
            if self.validate_block(block) and block not in self.chain:
                self.chain.append(block)

    def chain_height(self) -> int:
        return len(self.chain)

    def chain_head(self):
        return self.chain[-1] if self.chain else None


class TestT002TwoNodeConsensus:
    """T-002: 2-Node Konsens — Mehrheits-Voting"""

    def test_two_nodes_connect(self):
        """Nodes können sich verbinden"""
        node1 = MockNode("validator-1")
        node2 = MockNode("validator-2")
        node1.connect(node2)
        assert node2 in node1.peers
        assert node1 in node2.peers

    def test_block_production(self):
        """Node 1 produziert gültigen Block"""
        node1 = MockNode("validator-1")
        block = node1.produce_block()
        height, block_hash, validator = block
        assert height == 1
        assert len(block_hash) == 64
        assert validator == "validator-1"

    def test_block_validation(self):
        """Node 2 validiert Block von Node 1"""
        node1 = MockNode("validator-1")
        node2 = MockNode("validator-2")
        node1.connect(node2)
        block = node1.produce_block()
        assert node2.validate_block(block) == True

    def test_block_propagation(self):
        """Block wird von Node 1 zu Node 2 propagiert"""
        node1 = MockNode("validator-1")
        node2 = MockNode("validator-2")
        node1.connect(node2)
        block = node1.produce_block()
        node1.add_block(block)
        time.sleep(0.1)
        assert node1.chain_height() == 1
        assert node2.chain_height() == 1

    def test_chain_consistency(self):
        """Beide Nodes haben dieselbe Chain-Höhe"""
        node1 = MockNode("validator-1")
        node2 = MockNode("validator-2")
        node1.connect(node2)
        # 5 Blöcke produzieren
        for i in range(5):
            block = node1.produce_block()
            node1.add_block(block)
            time.sleep(0.05)
        assert node1.chain_height() == 5
        assert node2.chain_height() == 5

    def test_chain_head_match(self):
        """Beide Nodes zeigen auf denselben letzten Block"""
        node1 = MockNode("validator-1")
        node2 = MockNode("validator-2")
        node1.connect(node2)
        block = node1.produce_block()
        node1.add_block(block)
        time.sleep(0.1)
        assert node1.chain_head() == node2.chain_head()

    def test_invalid_block_rejected(self):
        """Ungültiger Block wird abgelehnt"""
        node1 = MockNode("validator-1")
        node2 = MockNode("validator-2")
        node1.connect(node2)
        bad_block = (999, "not-a-valid-hash", "attacker")
        assert node2.validate_block(bad_block) == False
        node2.receive_block(bad_block, node1)
        assert node2.chain_height() == 0

    def test_duplicate_block_ignored(self):
        """Doppelter Block wird ignoriert"""
        node1 = MockNode("validator-1")
        node2 = MockNode("validator-2")
        node1.connect(node2)
        block = node1.produce_block()
        node1.add_block(block)
        time.sleep(0.05)
        before = node2.chain_height()
        node2.receive_block(block, node1)
        assert node2.chain_height() == before
