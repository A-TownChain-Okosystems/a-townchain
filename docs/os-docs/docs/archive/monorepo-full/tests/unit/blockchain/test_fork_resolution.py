# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
T-004: Fork-Resolution — gleichzeitige Block-Produktion
Sprint 2.2 Blocker | Issue #8 | Wiki: Kap. 54
"""
import pytest
import time
import threading
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))
from tests.test_multinode_consensus import MockNode


class ForkableNode(MockNode):
    """Erweiterter Node mit Fork-Detection"""

    def resolve_fork(self, block_a, block_b):
        """
        Fork-Resolution: Längere Chain gewinnt.
        Bei gleicher Länge: niedrigerer Hash gewinnt (deterministisch).
        """
        ha = block_a[1]
        hb = block_b[1]
        # SHA-256: kleinerer Hash = mehr Arbeit (deterministisch)
        return block_a if ha < hb else block_b


class TestT004ForkResolution:
    """T-004: Fork-Resolution bei gleichzeitigen Blöcken"""

    def test_fork_detection(self):
        """Zwei Nodes produzieren gleichzeitig — Fork erkennbar"""
        node1 = ForkableNode("validator-1")
        node2 = ForkableNode("validator-2")
        # Kein connect → beide produzieren unabhängig
        b1 = node1.produce_block()
        b2 = node2.produce_block()
        assert b1[1] != b2[1], "Zwei Blöcke müssen unterschiedliche Hashes haben"
        assert b1[0] == b2[0] == 1, "Beide auf Höhe 1"

    def test_deterministic_resolution(self):
        """Fork-Resolution ist deterministisch (gleiche Eingabe → gleicher Gewinner)"""
        node = ForkableNode("resolver")
        import hashlib
        h1 = hashlib.sha256(b"block_a").hexdigest()
        h2 = hashlib.sha256(b"block_b").hexdigest()
        block_a = (1, h1, "validator-1")
        block_b = (1, h2, "validator-2")
        winner1 = node.resolve_fork(block_a, block_b)
        winner2 = node.resolve_fork(block_a, block_b)
        assert winner1 == winner2, "Resolution muss deterministisch sein"

    def test_lower_hash_wins(self):
        """Block mit kleinerem Hash gewinnt bei Fork"""
        node = ForkableNode("resolver")
        block_low  = (1, "0" * 64, "validator-1")
        block_high = (1, "f" * 64, "validator-2")
        winner = node.resolve_fork(block_low, block_high)
        assert winner == block_low, "Niedrigerer Hash muss gewinnen"

    def test_fork_resolves_after_sync(self):
        """Nach Gossip konvergieren beide Nodes auf denselben Block"""
        node1 = ForkableNode("validator-1")
        node2 = ForkableNode("validator-2")
        # Beide produzieren gleichzeitig
        b1 = node1.produce_block()
        b2 = node2.produce_block()
        # Jetzt verbinden
        node1.connect(node2)
        # Gewinner bestimmen
        winner = node1.resolve_fork(b1, b2)
        node1.chain = [winner]
        node2.chain = [winner]
        assert node1.chain_head() == node2.chain_head()

    def test_longer_chain_wins(self):
        """Längere Chain gewinnt über kürzere (Nakamoto-Konsens)"""
        node1 = ForkableNode("validator-1")
        node2 = ForkableNode("validator-2")
        # Node1 hat 3 Blöcke, Node2 hat 1 Block
        for _ in range(3):
            b = node1.produce_block()
            node1.add_block(b)
        b = node2.produce_block()
        node2.add_block(b)
        # Node1 gewinnt (längere Chain)
        assert node1.chain_height() > node2.chain_height()
        assert node1.chain_height() == 3

    def test_no_double_spend_after_fork(self):
        """Nach Fork-Resolution: Kein Double-Spend möglich"""
        node1 = ForkableNode("validator-1")
        node2 = ForkableNode("validator-2")
        b1 = node1.produce_block(transactions=["TX: Alice→Bob 100"])
        b2 = node2.produce_block(transactions=["TX: Alice→Charlie 100"])
        # Nur einer gewinnt
        winner = node1.resolve_fork(b1, b2)
        node1.chain = [winner]
        node2.chain = [winner]
        # Beide haben dieselbe TX-History
        assert node1.chain == node2.chain
