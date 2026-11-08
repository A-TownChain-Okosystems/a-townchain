# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für NodeDiscovery-Service
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..'))


class TestNodeDiscovery:
    """Test Node Discovery Service."""

    def test_discovery_atc_file_exists(self):
        """Discovery ATCLang file exists."""
        path = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'modules', 'atcnet', 'discovery.atc')
        assert os.path.exists(path), "discovery.atc not found"

    def test_p2p_node_atc_file_exists(self):
        """P2P Node ATCLang file exists."""
        path = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'modules', 'atcnet', 'p2p_node.atc')
        assert os.path.exists(path), "p2p_node.atc not found"

    def test_nat_traversal_atc_file_exists(self):
        """NAT Traversal ATCLang file exists."""
        path = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'modules', 'atcnet', 'nat_traversal.atc')
        assert os.path.exists(path), "nat_traversal.atc not found"
