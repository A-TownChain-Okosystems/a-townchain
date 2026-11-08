# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für ATCNet (Network Module)
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..'))


class TestATCNet:
    """Test ATCNet network module."""

    def test_atcnet_protocol_exists(self):
        """ATCNet protocol documentation exists."""
        protocol_path = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'modules', 'atcnet', 'PROTOCOL.md')
        if os.path.exists(protocol_path):
            with open(protocol_path) as f:
                assert len(f.read()) > 100
        else:
            pytest.skip("ATCNet PROTOCOL.md not found")

    def test_atcnet_bootstrap_client(self):
        """ATCNet bootstrap client .atc file exists."""
        client_path = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'modules', 'atcnet', 'bootstrap_client.atc')
        assert os.path.exists(client_path), "bootstrap_client.atc not found"

    def test_atcnet_discovery(self):
        """ATCNet discovery .atc file exists."""
        discovery_path = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'modules', 'atcnet', 'discovery.atc')
        assert os.path.exists(discovery_path), "discovery.atc not found"

    def test_atcnet_p2p_propagation(self):
        """ATCNet P2P propagation .atc file exists."""
        prop_path = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'modules', 'atcnet', 'p2p_propagation.atc')
        assert os.path.exists(prop_path), "p2p_propagation.atc not found"

    def test_atcnet_gossip(self):
        """ATCNet gossip .atc file exists."""
        gossip_path = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'modules', 'atcnet', 'gossip.atc')
        assert os.path.exists(gossip_path), "gossip.atc not found"
