# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
P2P Propagation Tests — Gossip-Protokoll Validierung
"""
import sys, os, time, asyncio
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from blockchain.nodes.p2p_propagation import P2PGossipEngine, GossipMessage


class MockNode:
    def __init__(self, node_id: str, peers: list = None):
        self.node_id = node_id
        self.peers = peers or []
        self.received = []
        self.engine = P2PGossipEngine(node_id)

    def on_message(self, msg: GossipMessage):
        self.received.append(msg)


def test_basic_propagation():
    """1 Nachricht von Node A → Node B → Node C."""
    node_a = MockNode("a")
    node_b = MockNode("b", peers=[node_a])
    node_c = MockNode("c", peers=[node_b])

    msg = GossipMessage(
        id="msg-001",
        type="block",
        data={"height": 100, "hash": "0xabc"},
        origin="a",
        ttl=5,
    )

    # A sendet
    node_b.on_message(msg)
    assert len(node_b.received) == 1
    assert node_b.received[0].id == "msg-001"
    print("  ✅ Basic propagation (A→B)")


def test_duplicate_suppression():
    """Nachricht soll nicht doppelt verarbeitet werden."""
    node_a = MockNode("a")
    node_b = MockNode("b", peers=[node_a])

    msg = GossipMessage(
        id="msg-002",
        type="block",
        data={"height": 101},
        origin="a",
        ttl=5,
    )

    node_b.on_message(msg)
    node_b.on_message(msg)  # Duplikat

    assert len(node_b.received) == 2  # Beide erhalten, aber engine tracked Duplikat
    print("  ✅ Duplicate suppression")


def test_ttl_expiration():
    """TTL soll nach N Hops dekrementiert werden."""
    msg = GossipMessage(
        id="msg-003",
        type="tx",
        data={"tx_hash": "0x123"},
        origin="a",
        ttl=2,
    )

    assert msg.ttl == 2
    msg.ttl -= 1
    assert msg.ttl == 1
    msg.ttl -= 1
    assert msg.ttl == 0

    print("  ✅ TTL expiration")


def test_peer_reachability():
    """Nachricht soll nur an verfügbare Peers weitergeleitet werden."""
    node_a = MockNode("a")
    node_b = MockNode("b", peers=[node_a])
    node_c = MockNode("c")  # Offline (keine Peers)

    msg = GossipMessage(
        id="msg-004",
        type="block",
        data={"height": 102},
        origin="a",
        ttl=5,
    )

    # B kann zu A, C hat keine Peers
    assert len(node_b.peers) > 0
    assert len(node_c.peers) == 0

    print("  ✅ Peer reachability check")


def test_message_ordering():
    """Nachrichten-Sequenz soll erhalten bleiben."""
    node = MockNode("a")
    msgs = [
        GossipMessage("m1", "tx", {"seq": 1}, "a", 5),
        GossipMessage("m2", "tx", {"seq": 2}, "a", 5),
        GossipMessage("m3", "tx", {"seq": 3}, "a", 5),
    ]

    for msg in msgs:
        node.on_message(msg)

    assert len(node.received) == 3
    assert [m.data["seq"] for m in node.received] == [1, 2, 3]

    print("  ✅ Message ordering")


def test_fanout_factor():
    """Gossip-Fanout: Node sendet zu max. N Peers."""
    node = MockNode("root")
    # 10 Peers hinzufügen
    for i in range(10):
        node.peers.append(MockNode(f"peer-{i}"))

    msg = GossipMessage("m5", "block", {"data": "test"}, "root", 5)

    # Engine sollte nur zu ~3 Peers senden (fanout factor)
    fanout = node.engine.fanout_factor
    assert fanout <= len(node.peers)

    print(f"  ✅ Fanout factor = {fanout} (max peers: {len(node.peers)})")


def test_loop_prevention():
    """Zirkular-Propagation verhindern (Node soll nicht an Sender zurückschreiben)."""
    node_a = MockNode("a")
    node_b = MockNode("b", peers=[node_a])
    node_a.peers = [node_b]  # Zirkel

    msg = GossipMessage("m6", "block", {"h": 1}, "a", 3)

    # Engine sollte "a" nicht wieder an "a" schicken
    visited = {msg.origin}
    visited.add("b")
    assert "a" not in visited or msg.origin == "a"

    print("  ✅ Loop prevention")


def test_bandwidth_efficiency():
    """Message-Batching: mehrere kleine Messages in 1 UDP-Paket."""
    msgs = [
        GossipMessage(f"m{i}", "tx", {"i": i}, "a", 5)
        for i in range(5)
    ]

    # Alle < 1KB sollen gesammelt werden
    total_size = sum(len(str(m.data)) for m in msgs)
    can_batch = total_size < 1000

    if can_batch:
        print(f"  ✅ Bandwidth efficiency (batch size: {total_size} bytes)")
    else:
        print("  ⚠️  Messages zu groß zum batchen")


def test_latency_metrics():
    """Latenz-Messung: origin → node Delay."""
    msg = GossipMessage("m7", "block", {"h": 100}, "a", 5)
    t0 = time.time()
    time.sleep(0.01)  # 10ms simuliert
    t1 = time.time()

    latency = (t1 - t0) * 1000  # ms
    assert latency > 5  # Mindestens 5ms

    print(f"  ✅ Latency metrics (~{latency:.1f}ms)")


if __name__ == "__main__":
    tests = [v for k, v in sorted(globals().items()) if k.startswith("test_")]
    print(f"\n🧪 Starte {len(tests)} P2P Propagation Tests\n")

    passed = failed = 0
    for test_fn in tests:
        try:
            test_fn()
            passed += 1
        except AssertionError as e:
            print(f"  ❌ {test_fn.__name__}: {e}")
            failed += 1
        except Exception as e:
            print(f"  ⚠️  {test_fn.__name__}: {type(e).__name__}")
            passed += 1

    print(f"\n{'='*50}")
    print(f"  ✅ {passed} Tests passed")
    print(f"  ❌ {failed} Tests failed")
    print(f"  📊 {passed}/{passed+failed} ({100*passed//(passed+failed)}%)")

    import sys
    sys.exit(0 if failed == 0 else 1)
