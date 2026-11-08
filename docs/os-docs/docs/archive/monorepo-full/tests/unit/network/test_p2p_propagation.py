# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für P2PBroadcaster
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..'))


class TestP2PPropagation:
    """Test P2P Broadcast."""

    def test_propagation_atc_file_exists(self):
        """P2P propagation ATCLang file exists."""
        path = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'modules', 'atcnet', 'p2p_propagation.atc')
        assert os.path.exists(path), "p2p_propagation.atc not found"

    def test_gossip_atc_file_exists(self):
        """Gossip protocol ATCLang file exists."""
        path = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'modules', 'atcnet', 'gossip.atc')
        assert os.path.exists(path), "gossip.atc not found"

    def test_protocol_doc_exists(self):
        """ATCNet protocol documentation exists."""
        path = os.path.join(os.path.dirname(__file__), '..', '..', '..', 'modules', 'atcnet', 'PROTOCOL.md')
        assert os.path.exists(path), "PROTOCOL.md not found"
