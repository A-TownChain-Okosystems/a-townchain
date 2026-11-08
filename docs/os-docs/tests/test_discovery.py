# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für NodeDiscovery-Service."""

import pytest
import json
import time
from pathlib import Path
from blockchain.nodes.discovery import NodeDiscovery, PeerInfo


class TestNodeDiscovery:
    """Test-Suite für NodeDiscovery."""

    def test_peer_info_to_dict(self):
        """Test PeerInfo-Datenklasse."""
        peer = PeerInfo(
            node_id="peer-001",
            host="127.0.0.1",
            port=5105,
            last_seen=int(time.time()),
        )
        data = peer.to_dict()
        assert data["node_id"] == "peer-001"
        assert data["host"] == "127.0.0.1"
        assert data["port"] == 5105

    def test_discovery_init(self):
        """Test Initialisierung des Discovery-Service."""
        discovery = NodeDiscovery(
            node_id="test-node",
            my_port=5005,
            is_bootstrap=True,
        )
        assert discovery.node_id == "test-node"
        assert discovery.my_port == 5005
        assert discovery.is_bootstrap is True
        assert discovery.discovery_port == 6005  # 5005 + 1000
        assert len(discovery.peers) == 0

    def test_discovery_init_non_bootstrap(self):
        """Test Initialisierung als Non-Bootstrap-Node."""
        discovery = NodeDiscovery(
            node_id="peer-001",
            my_port=5105,
            is_bootstrap=False,
            bootstrap_nodes=["127.0.0.1:5005"],
        )
        assert discovery.is_bootstrap is False
        assert discovery.bootstrap_nodes == ["127.0.0.1:5005"]

    def test_get_peers_empty(self):
        """Test Abfrage von Peers wenn keine verbunden."""
        discovery = NodeDiscovery(
            node_id="test-node",
            my_port=5005,
            is_bootstrap=True,
        )
        peers = discovery.get_peers()
        assert isinstance(peers, dict)
        assert len(peers) == 0

    def test_add_peer_manually(self):
        """Test manuelles Hinzufügen eines Peers."""
        discovery = NodeDiscovery(
            node_id="bootstrap-001",
            my_port=5005,
            is_bootstrap=True,
        )
        
        peer = PeerInfo(
            node_id="peer-001",
            host="127.0.0.1",
            port=5105,
            last_seen=int(time.time()),
        )
        discovery.peers["peer-001"] = peer
        
        assert discovery.get_peer_count() == 1
        assert "peer-001" in discovery.get_peers()
        assert discovery.get_peers()["peer-001"].host == "127.0.0.1"

    def test_get_status(self):
        """Test Status-Abruf."""
        discovery = NodeDiscovery(
            node_id="test-node",
            my_port=5005,
            is_bootstrap=True,
        )
        
        status = discovery.get_status()
        assert status["node_id"] == "test-node"
        assert status["is_bootstrap"] is True
        assert "peers_count" in status
        assert "peers" in status

    def test_peer_persistence(self, tmp_path):
        """Test Speicherung und Laden von Peers."""
        discovery1 = NodeDiscovery(
            node_id="bootstrap-001",
            my_port=5005,
            is_bootstrap=True,
        )
        
        # Peers hinzufügen
        for i in range(3):
            peer = PeerInfo(
                node_id=f"peer-{i:03d}",
                host="127.0.0.1",
                port=5105 + i,
                last_seen=int(time.time()),
            )
            discovery1.peers[f"peer-{i:03d}"] = peer
        
        # Speichern
        peer_file = tmp_path / "peers.json"
        discovery1.save_peers(str(peer_file))
        assert peer_file.exists()
        
        # Laden
        discovery2 = NodeDiscovery(
            node_id="bootstrap-002",
            my_port=5005,
            is_bootstrap=True,
        )
        discovery2.load_peers(str(peer_file))
        
        assert discovery2.get_peer_count() == 3
        assert "peer-000" in discovery2.get_peers()
        assert discovery2.get_peers()["peer-001"].port == 5106

    def test_max_peers_config(self):
        """Test max_peers Konfiguration."""
        config = {"max_peers": 5}
        discovery = NodeDiscovery(
            node_id="test-node",
            my_port=5005,
            is_bootstrap=True,
            config=config,
        )
        assert discovery.max_peers == 5

    def test_peer_timeout_config(self):
        """Test peer_timeout Konfiguration."""
        config = {"peer_timeout_sec": 120}
        discovery = NodeDiscovery(
            node_id="test-node",
            my_port=5005,
            is_bootstrap=True,
            config=config,
        )
        assert discovery.peer_timeout_sec == 120


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
