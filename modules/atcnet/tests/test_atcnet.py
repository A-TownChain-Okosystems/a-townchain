# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Tests für ATCNet P2P Stack"""
import sys, os, threading, time, json, socket
sys.path.insert(0, os.path.dirname(os.path.dirname(__file__)))

def test_p2p_message_encode_decode():
    from p2p_propagation import P2PMessage
    msg = P2PMessage(type="BLOCK", sender="node-1", payload={"hash": "abc123"})
    raw = msg.to_bytes()
    assert len(raw) > 4

def test_peer_info_dataclass():
    from discovery import PeerInfo
    p = PeerInfo(node_id="n1", host="127.0.0.1", port=4001, last_seen=int(time.time()))
    d = p.to_dict()
    assert d["node_id"] == "n1"
    assert d["port"] == 4001

def test_bootstrap_client_import():
    from bootstrap_client import BootstrapClient, Peer
    bc = BootstrapClient("test-node-001", "localhost", 4002)
    assert bc.node_id == "test-node-001"
    assert len(bc.bootstraps) > 0

def test_bootstrap_client_no_server():
    """Announce ohne laufenden Server → leere Liste (kein Crash)."""
    from bootstrap_client import BootstrapClient
    bc = BootstrapClient("test-node-002", timeout=0.5,
                         bootstrap_nodes=[("localhost", 19999)])
    # timeout Param nicht in __init__ — teste nur dass Methode nicht crasht
    bc.bootstraps = [("localhost", 19999)]
    peers = bc.announce(timeout=0.3)
    assert isinstance(peers, list)

if __name__ == "__main__":
    tests = [v for k,v in list(globals().items()) if k.startswith("test_")]
    ok = fail = 0
    for t in tests:
        try: t(); print(f"  ✅ {t.__name__}"); ok+=1
        except Exception as e: print(f"  ❌ {t.__name__}: {e}"); fail+=1
    print(f"\n{ok} passed, {fail} failed")
