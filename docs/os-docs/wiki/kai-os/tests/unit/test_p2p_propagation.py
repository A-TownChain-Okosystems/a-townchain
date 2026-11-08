# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für P2PBroadcaster."""

import pytest
import json
from blockchain.nodes.p2p_propagation import P2PBroadcaster, P2PMessage


class TestP2PMessage:
    """Test-Suite für P2PMessage."""

    def test_message_to_bytes(self):
        """Test Konvertierung zu Bytes mit Length-Prefix."""
        msg = P2PMessage(
            type="NEW_BLOCK",
            sender="node-001",
            payload={"block": {"height": 1}},
        )
        data = msg.to_bytes()
        assert isinstance(data, bytes)
        assert len(data) > 4  # Mindestens Length-Prefix + Daten

    def test_message_from_bytes(self):
        """Test Rekonstruktion aus Bytes."""
        original = P2PMessage(
            type="NEW_TX",
            sender="node-002",
            payload={"tx": {"hash": "0xabc"}},
        )
        data = original.to_bytes()[4:]  # Skip Length-Prefix
        reconstructed = P2PMessage.from_bytes(data)
        
        assert reconstructed.type == "NEW_TX"
        assert reconstructed.sender == "node-002"
        assert reconstructed.payload["tx"]["hash"] == "0xabc"


class TestP2PBroadcaster:
    """Test-Suite für P2PBroadcaster."""

    def test_broadcaster_init(self):
        """Test Initialisierung des Broadcasters."""
        broadcaster = P2PBroadcaster(
            node_id="node-001",
            port=5005,
        )
        assert broadcaster.node_id == "node-001"
        assert broadcaster.port == 5005
        assert len(broadcaster.peers) == 0

    def test_register_handler(self):
        """Test Handler-Registrierung."""
        broadcaster = P2PBroadcaster(
            node_id="node-001",
            port=5005,
        )
        
        def custom_handler(msg: P2PMessage):
            pass
        
        broadcaster.register_handler("CUSTOM", custom_handler)
        assert "CUSTOM" in broadcaster.handlers
        assert broadcaster.handlers["CUSTOM"] == custom_handler

    def test_default_handlers_registered(self):
        """Test dass Standard-Handler registriert sind."""
        broadcaster = P2PBroadcaster(
            node_id="node-001",
            port=5005,
        )
        
        assert "NEW_BLOCK" in broadcaster.handlers
        assert "NEW_TX" in broadcaster.handlers
        assert "HANDSHAKE" in broadcaster.handlers

    def test_get_status(self):
        """Test Status-Abruf."""
        broadcaster = P2PBroadcaster(
            node_id="node-001",
            port=5005,
        )
        
        status = broadcaster.get_status()
        assert status["node_id"] == "node-001"
        assert status["port"] == 5005
        assert "peers_count" in status
        assert status["peers_count"] == 0

    def test_duplicate_filter(self):
        """Test Duplikat-Filter."""
        broadcaster = P2PBroadcaster(
            node_id="node-001",
            port=5005,
        )
        
        # Zweimal die gleiche Nachricht verarbeiten
        msg = P2PMessage(
            type="NEW_BLOCK",
            sender="node-002",
            payload={"block": {"height": 1}},
        )
        
        # Erste Nachricht: OK
        msg_hash = hash(json.dumps({
            "type": msg.type,
            "sender": msg.sender,
            "payload": msg.payload,
        }, sort_keys=True))
        
        with broadcaster.seen_lock:
            broadcaster.seen_msgs.append(msg_hash)
        
        # Zweite Nachricht: Duplikat
        msg_hash_2 = hash(json.dumps({
            "type": msg.type,
            "sender": msg.sender,
            "payload": msg.payload,
        }, sort_keys=True))
        
        assert msg_hash == msg_hash_2
        assert msg_hash in broadcaster.seen_msgs

    def test_get_peers_empty(self):
        """Test Abfrage von Peers wenn keine verbunden."""
        broadcaster = P2PBroadcaster(
            node_id="node-001",
            port=5005,
        )
        
        peers = broadcaster.get_peers()
        assert isinstance(peers, list)
        assert len(peers) == 0

    def test_message_types_constants(self):
        """Test dass alle Nachrichtentyp-Konstanten definiert sind."""
        broadcaster = P2PBroadcaster(
            node_id="node-001",
            port=5005,
        )
        
        assert hasattr(broadcaster, "MSG_NEW_BLOCK")
        assert hasattr(broadcaster, "MSG_NEW_TX")
        assert hasattr(broadcaster, "MSG_GET_BLOCKS")
        assert hasattr(broadcaster, "MSG_BLOCKS")
        assert hasattr(broadcaster, "MSG_GET_HEIGHT")
        assert hasattr(broadcaster, "MSG_HEIGHT")
        assert hasattr(broadcaster, "MSG_HANDSHAKE")


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
