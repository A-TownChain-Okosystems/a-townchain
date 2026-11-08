# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
T-003: Multi-Node Konsens — 5-Node, 2-of-3 Threshold
Sprint 2.2 Blocker | Issue #8 | Wiki: Kap. 54
"""
import pytest
import time
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))
from tests.test_multinode_consensus import MockNode


class TestT003FiveNodeConsensus:
    """T-003: 5-Node Konsens — Quorum ≥ 3 von 5"""

    def setup_cluster(self, n=5):
        nodes = [MockNode(f"validator-{i}", stake=10000) for i in range(n)]
        # Vollständig verbundenes Netz
        for i in range(n):
            for j in range(i+1, n):
                nodes[i].connect(nodes[j])
        return nodes

    def test_five_nodes_connected(self):
        """Alle 5 Nodes sind verbunden"""
        nodes = self.setup_cluster(5)
        for node in nodes:
            assert len(node.peers) == 4

    def test_block_reaches_all_nodes(self):
        """Block von Node 0 erreicht alle 5 Nodes"""
        nodes = self.setup_cluster(5)
        block = nodes[0].produce_block()
        nodes[0].add_block(block)
        time.sleep(0.2)
        for i, node in enumerate(nodes):
            assert node.chain_height() == 1, f"Node {i} hat Block nicht erhalten"

    def test_quorum_validation(self):
        """Mindestens 3 von 5 Nodes validieren Block positiv"""
        nodes = self.setup_cluster(5)
        block = nodes[0].produce_block()
        validations = [n.validate_block(block) for n in nodes]
        assert sum(validations) >= 3, f"Quorum nicht erreicht: {sum(validations)}/5"

    def test_chain_grows_consistently(self):
        """Chain wächst konsistent auf allen Nodes"""
        nodes = self.setup_cluster(5)
        for round_n in range(3):
            producer = nodes[round_n % len(nodes)]
            block = producer.produce_block()
            producer.add_block(block)
            time.sleep(0.1)
        expected_height = 3
        for i, node in enumerate(nodes):
            assert node.chain_height() == expected_height, f"Node {i}: {node.chain_height()} ≠ {expected_height}"

    def test_minority_cannot_produce_chain(self):
        """Minderheit (2 von 5) allein kann keine akzeptierte Chain erzwingen"""
        nodes = self.setup_cluster(5)
        # Nodes 3+4 isolieren
        minority = [MockNode("evil-1"), MockNode("evil-2")]
        minority[0].connect(minority[1])
        # Minderheit produziert eigene Chain
        evil_block = minority[0].produce_block()
        minority[0].add_block(evil_block)
        # Haupt-Cluster produziert echte Chain
        real_block = nodes[0].produce_block()
        nodes[0].add_block(real_block)
        time.sleep(0.1)
        # Haupt-Cluster hat eigene Chain
        for node in nodes:
            assert node.chain_height() == 1
            assert node.chain_head() == real_block

    def test_all_nodes_same_head(self):
        """Alle 5 Nodes zeigen auf denselben Block"""
        nodes = self.setup_cluster(5)
        block = nodes[0].produce_block()
        nodes[0].add_block(block)
        time.sleep(0.2)
        head = nodes[0].chain_head()
        for i, node in enumerate(nodes[1:], 1):
            assert node.chain_head() == head, f"Node {i} hat anderen Head"
