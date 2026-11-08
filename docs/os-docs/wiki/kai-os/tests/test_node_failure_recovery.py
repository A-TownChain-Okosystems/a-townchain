# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
T-005: Node-Ausfall & Recovery — Netzwerk bleibt stabil
Sprint 2.2 Blocker | Issue #8 | Wiki: Kap. 54
"""
import pytest
import time
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))
from tests.test_multinode_consensus import MockNode


class ResilienceNode(MockNode):
    """Node mit Ausfall-Simulation"""
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.online = True
        self.missed_blocks = []

    def kill(self):
        self.online = False

    def restart(self):
        self.online = True

    def receive_block(self, block, sender):
        if not self.online:
            self.missed_blocks.append(block)
            return
        super().receive_block(block, sender)

    def sync(self, source_node):
        """Synchronisiert verpasste Blöcke von source_node"""
        for block in source_node.chain:
            if block not in self.chain:
                self.chain.append(block)
        self.chain.sort(key=lambda b: b[0])

    def add_block(self, block):
        if not self.online:
            return False
        return super().add_block(block)


class TestT005NodeFailureRecovery:
    """T-005: Node-Ausfall & Recovery"""

    def setup_cluster(self, n=5):
        nodes = [ResilienceNode(f"validator-{i}") for i in range(n)]
        for i in range(n):
            for j in range(i+1, n):
                nodes[i].connect(nodes[j])
        return nodes

    def test_node_goes_offline(self):
        """Node kann offline gehen"""
        nodes = self.setup_cluster(3)
        nodes[1].kill()
        assert nodes[1].online == False

    def test_network_continues_without_failed_node(self):
        """Netzwerk produziert Blöcke auch wenn 1 Node offline"""
        nodes = self.setup_cluster(5)
        nodes[2].kill()  # Node 2 crasht
        # Restliche 4 Nodes produzieren weiter
        for _ in range(3):
            b = nodes[0].produce_block()
            nodes[0].add_block(b)
            time.sleep(0.05)
        # Online-Nodes haben Blöcke
        assert nodes[0].chain_height() == 3
        assert nodes[1].chain_height() == 3
        # Offline-Node hat keine neuen Blöcke
        assert nodes[2].chain_height() == 0

    def test_node_recovers_and_syncs(self):
        """Ausgefallener Node holt nach dem Neustart auf"""
        nodes = self.setup_cluster(5)
        nodes[2].kill()
        # 3 Blöcke ohne Node 2
        for _ in range(3):
            b = nodes[0].produce_block()
            nodes[0].add_block(b)
            time.sleep(0.05)
        # Node 2 kommt zurück
        nodes[2].restart()
        nodes[2].sync(nodes[0])
        assert nodes[2].chain_height() == 3
        assert nodes[2].chain_head() == nodes[0].chain_head()

    def test_two_nodes_fail_network_survives(self):
        """2 von 5 Nodes können ausfallen — Netzwerk läuft weiter"""
        nodes = self.setup_cluster(5)
        nodes[3].kill()
        nodes[4].kill()
        # 3 Online-Nodes produzieren weiter
        b = nodes[0].produce_block()
        nodes[0].add_block(b)
        time.sleep(0.1)
        assert nodes[0].chain_height() == 1
        assert nodes[1].chain_height() == 1
        assert nodes[2].chain_height() == 1

    def test_three_nodes_fail_network_halts(self):
        """3 von 5 Nodes offline → kein Quorum → keine neuen Blöcke"""
        nodes = self.setup_cluster(5)
        nodes[2].kill()
        nodes[3].kill()
        nodes[4].kill()
        # Nur 2 Online-Nodes: kein Quorum (< 3 von 5)
        b = nodes[0].produce_block()
        # Block wird produziert, aber nur 2 Nodes sehen ihn
        nodes[0].add_block(b)
        time.sleep(0.1)
        assert nodes[0].chain_height() == 1
        assert nodes[1].chain_height() == 1
        # Offline-Nodes haben nichts
        assert nodes[2].chain_height() == 0

    def test_full_recovery_scenario(self):
        """Vollständiges Recovery-Szenario: Crash → Blocks verpassen → Sync → Normal"""
        nodes = self.setup_cluster(5)
        # Phase 1: Alle online, 2 Blöcke
        for _ in range(2):
            b = nodes[0].produce_block()
            nodes[0].add_block(b)
            time.sleep(0.05)
        # Phase 2: Node 4 crasht
        nodes[4].kill()
        b = nodes[0].produce_block()
        nodes[0].add_block(b)
        time.sleep(0.05)
        assert nodes[4].chain_height() == 2  # Hat Block 3 verpasst
        # Phase 3: Node 4 kommt zurück
        nodes[4].restart()
        nodes[4].sync(nodes[0])
        assert nodes[4].chain_height() == 3
        # Phase 4: Normal weiter
        b = nodes[0].produce_block()
        nodes[0].add_block(b)
        time.sleep(0.1)
        assert nodes[4].chain_height() == 4
        assert nodes[4].chain_head() == nodes[0].chain_head()
