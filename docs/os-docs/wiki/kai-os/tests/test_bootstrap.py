# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Tests: Bootstrap-Node DNS Seed & Peer Discovery (Fix #68)
26 Tests — DNS, AddrMan, Gossip, Fallback, Persistenz
"""
import sys, os, json, time, tempfile
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from blockchain.nodes.bootstrap import (
    BootstrapNode, AddrMan, DNSSeedResolver, PeerAddress, BOOTSTRAP_CONFIG
)

# ── 1. PeerAddress Tests ───────────────────────────
def test_peer_address_key():
    p = PeerAddress(ip="5.9.104.210", port=5005)
    assert p.key == "5.9.104.210:5005"
    print("  ✅ PeerAddress.key")

def test_peer_address_not_stale():
    p = PeerAddress(ip="1.2.3.4", port=5005, last_seen=int(time.time()))
    assert not p.is_stale
    print("  ✅ PeerAddress frisch (nicht stale)")

def test_peer_address_stale():
    p = PeerAddress(ip="1.2.3.4", port=5005,
                    last_seen=int(time.time()) - 15*86400)
    assert p.is_stale
    print("  ✅ PeerAddress stale (>14 Tage)")

def test_peer_address_to_dict():
    p = PeerAddress(ip="5.9.1.2", port=5005, source="dns")
    d = p.to_dict()
    assert d["ip"] == "5.9.1.2"
    assert d["source"] == "dns"
    print("  ✅ PeerAddress.to_dict")

def test_peer_address_from_dict():
    d = {"ip":"1.1.1.1","port":5005,"last_seen":12345,"last_tried":0,
         "attempt_count":0,"services":1,"source":"hardcoded","version":"3.2.1"}
    p = PeerAddress.from_dict(d)
    assert p.ip == "1.1.1.1"
    assert p.source == "hardcoded"
    print("  ✅ PeerAddress.from_dict")

# ── 2. AddrMan Tests ───────────────────────────────
def test_addrman_add_and_get():
    with tempfile.TemporaryDirectory() as td:
        am = AddrMan(f"{td}/peers.dat")
        p  = PeerAddress(ip="5.9.104.210", port=5005, source="dns")
        am.add(p)
        assert p.key in am.new_table
        print("  ✅ AddrMan: add + new_table")

def test_addrman_mark_tried():
    with tempfile.TemporaryDirectory() as td:
        am = AddrMan(f"{td}/peers.dat")
        p  = PeerAddress(ip="95.217.12.33", port=5005)
        am.add(p)
        am.mark_tried(p.key)
        assert p.key in am.tried_table
        assert p.key not in am.new_table
        print("  ✅ AddrMan: mark_tried")

def test_addrman_mark_failed_3x():
    with tempfile.TemporaryDirectory() as td:
        am = AddrMan(f"{td}/peers.dat")
        p  = PeerAddress(ip="10.0.0.1", port=5005)
        am.add(p)
        for _ in range(3):
            am.mark_failed(p.key)
        assert p.key not in am.new_table
        assert p.key not in am.tried_table
        print("  ✅ AddrMan: nach 3 Fehlern entfernt")

def test_addrman_no_duplicate_tried():
    with tempfile.TemporaryDirectory() as td:
        am = AddrMan(f"{td}/peers.dat")
        p  = PeerAddress(ip="144.76.21.88", port=5005, source="tried")
        am.tried_table[p.key] = p
        am.add(p)   # darf nicht in new_table landen
        assert p.key not in am.new_table
        print("  ✅ AddrMan: kein Duplikat tried→new")

def test_addrman_save_load():
    with tempfile.TemporaryDirectory() as td:
        path = f"{td}/peers.dat"
        am1  = AddrMan(path)
        p1   = PeerAddress(ip="78.46.222.55", port=5005, source="dns")
        p2   = PeerAddress(ip="148.251.190.4", port=5005, source="hardcoded")
        am1.add(p1); am1.mark_tried(p2.key) or am1.add(p2)
        am1.save()
        # Neu laden
        am2 = AddrMan(path)
        assert "78.46.222.55:5005" in am2.new_table
        print("  ✅ AddrMan: save + load (Persistenz)")

def test_addrman_get_candidates():
    with tempfile.TemporaryDirectory() as td:
        am = AddrMan(f"{td}/peers.dat")
        for i in range(5):
            am.add(PeerAddress(ip=f"10.0.0.{i+1}", port=5005))
        # Stale-Filter überschreiben
        for p in am.new_table.values():
            p.last_seen = int(time.time())
        cands = am.get_candidates(3)
        assert len(cands) <= 3
        print("  ✅ AddrMan: get_candidates")

def test_addrman_addr_sample():
    with tempfile.TemporaryDirectory() as td:
        am = AddrMan(f"{td}/peers.dat")
        for i in range(20):
            am.add(PeerAddress(ip=f"9.9.9.{i}", port=5005))
        sample = am.get_addr_sample(10)
        assert len(sample) <= 10
        assert all("ip" in s for s in sample)
        print("  ✅ AddrMan: get_addr_sample (max 1000)")

def test_addrman_stats():
    with tempfile.TemporaryDirectory() as td:
        am = AddrMan(f"{td}/peers.dat")
        am.add(PeerAddress(ip="1.2.3.4", port=5005))
        stats = am.stats
        assert stats["new_count"] == 1
        assert stats["total"] == 1
        print("  ✅ AddrMan: stats")

# ── 3. DNSSeedResolver Tests ───────────────────────
def test_dns_valid_ip():
    assert DNSSeedResolver._is_valid_ip("5.9.104.210")
    assert DNSSeedResolver._is_valid_ip("95.217.12.33")
    print("  ✅ DNS: valide öffentliche IPs erkannt")

def test_dns_invalid_ips():
    assert not DNSSeedResolver._is_valid_ip("127.0.0.1")    # loopback
    assert not DNSSeedResolver._is_valid_ip("10.0.0.1")     # privat
    assert not DNSSeedResolver._is_valid_ip("192.168.1.1")  # privat
    assert not DNSSeedResolver._is_valid_ip("172.16.0.1")   # privat
    assert not DNSSeedResolver._is_valid_ip("999.0.0.1")    # ungültig
    print("  ✅ DNS: private/loopback IPs gefiltert")

def test_dns_hardcoded_seeds():
    r     = DNSSeedResolver()
    seeds = r.get_hardcoded_seeds()
    assert len(seeds) == len(BOOTSTRAP_CONFIG["hardcoded_seeds"])
    for s in seeds:
        assert s.source == "hardcoded"
        assert s.port == 5005
    print(f"  ✅ DNS: {len(seeds)} hardcoded Seeds geladen")

def test_dns_resolve_fails_gracefully():
    r     = DNSSeedResolver(seeds=["invalid.nonexistent.xyz"], timeout=1)
    peers = r.resolve_seed("invalid.nonexistent.xyz")
    assert peers == []   # kein Exception, leere Liste
    print("  ✅ DNS: Fehler → leere Liste (kein Crash)")

def test_dns_resolve_all_fallback():
    r     = DNSSeedResolver(seeds=["invalid1.xyz","invalid2.xyz"], timeout=1)
    peers = r.resolve_all()
    assert isinstance(peers, list)
    print("  ✅ DNS: resolve_all gibt immer Liste zurück")

# ── 4. BootstrapNode Tests ─────────────────────────
def test_bootstrap_node_init():
    with tempfile.TemporaryDirectory() as td:
        node = BootstrapNode(node_id="TEST-001", data_dir=td)
        assert node.node_id == "TEST-001"
        assert node.addrman is not None
        print("  ✅ BootstrapNode: Init")

def test_bootstrap_node_gen_id():
    node1 = BootstrapNode(data_dir="/tmp")
    node2 = BootstrapNode(data_dir="/tmp")
    assert node1.node_id != node2.node_id   # unique IDs
    print("  ✅ BootstrapNode: eindeutige Node-IDs")

def test_bootstrap_node_handle_addr():
    with tempfile.TemporaryDirectory() as td:
        node  = BootstrapNode(data_dir=td)
        peers = [{"ip":"5.9.104.210","port":5005},
                 {"ip":"95.217.12.33","port":5005},
                 {"ip":"127.0.0.1","port":5005}]  # loopback → ignoriert
        added = node.handle_addr(peers, source_ip="1.2.3.4")
        assert added == 2   # loopback nicht gezählt
        print("  ✅ BootstrapNode: handle_addr (loopback gefiltert)")

def test_bootstrap_node_handle_getaddr():
    with tempfile.TemporaryDirectory() as td:
        node = BootstrapNode(data_dir=td)
        for i in range(5):
            node.addrman.add(PeerAddress(ip=f"5.9.{i}.1", port=5005))
        result = node.handle_getaddr("99.99.99.99")
        assert isinstance(result, list)
        assert len(result) <= 1000
        print("  ✅ BootstrapNode: handle_getaddr (max 1000 IPs)")

def test_bootstrap_node_get_peers_for_new():
    with tempfile.TemporaryDirectory() as td:
        node = BootstrapNode(data_dir=td)
        for i in range(10):
            p = PeerAddress(ip=f"5.9.{i}.1", port=5005,
                            last_seen=int(time.time()))
            node.addrman.add(p)
        peers = node.get_peers_for_new_node("9.9.9.9", count=5)
        assert len(peers) <= 5
        assert all("ip" in p and "port" in p for p in peers)
        print("  ✅ BootstrapNode: get_peers_for_new_node")

def test_bootstrap_node_mark_connected_failed():
    with tempfile.TemporaryDirectory() as td:
        node = BootstrapNode(data_dir=td)
        node.addrman.add(PeerAddress(ip="5.9.1.1", port=5005))
        node.mark_peer_connected("5.9.1.1", 5005)
        assert "5.9.1.1:5005" in node.connected
        node.mark_peer_failed("5.9.1.1", 5005)
        assert "5.9.1.1:5005" not in node.connected
        print("  ✅ BootstrapNode: mark_connected + mark_failed")

def test_bootstrap_node_status():
    with tempfile.TemporaryDirectory() as td:
        node   = BootstrapNode(data_dir=td)
        status = node.get_status()
        assert "node_id"   in status
        assert "config"    in status
        assert status["config"]["bootstrap_port"] == 5005
        assert status["config"]["kademlia_k"]     == 20
        assert status["config"]["kademlia_alpha"]  == 3
        print("  ✅ BootstrapNode: get_status (Port, K, Alpha)")

def test_bootstrap_hardcoded_fallback():
    """Bootstrap mit offline DNS → hardcoded Seeds als Fallback."""
    with tempfile.TemporaryDirectory() as td:
        node = BootstrapNode(data_dir=td)
        # DNS-Seeds auf ungültige Adressen setzen (Timeout)
        node.resolver.seeds  = ["invalid.xyz"]
        node.resolver.timeout = 0.1
        result = node.bootstrap()
        assert result["source"] == "hardcoded_fallback"
        assert result["hardcoded_peers"] == len(BOOTSTRAP_CONFIG["hardcoded_seeds"])
        print(f"  ✅ BootstrapNode: Fallback auf {result['hardcoded_peers']} hardcoded Seeds")

if __name__ == "__main__":
    print("\n=== Tests: Bootstrap-Node (Fix #68) ===")
    tests = [
        test_peer_address_key, test_peer_address_not_stale,
        test_peer_address_stale, test_peer_address_to_dict,
        test_peer_address_from_dict,
        test_addrman_add_and_get, test_addrman_mark_tried,
        test_addrman_mark_failed_3x, test_addrman_no_duplicate_tried,
        test_addrman_save_load, test_addrman_get_candidates,
        test_addrman_addr_sample, test_addrman_stats,
        test_dns_valid_ip, test_dns_invalid_ips,
        test_dns_hardcoded_seeds, test_dns_resolve_fails_gracefully,
        test_dns_resolve_all_fallback,
        test_bootstrap_node_init, test_bootstrap_node_gen_id,
        test_bootstrap_node_handle_addr, test_bootstrap_node_handle_getaddr,
        test_bootstrap_node_get_peers_for_new,
        test_bootstrap_node_mark_connected_failed,
        test_bootstrap_node_status, test_bootstrap_hardcoded_fallback,
    ]
    passed = failed = 0
    for t in tests:
        try:
            t()
            passed += 1
        except Exception as e:
            print(f"  ❌ {t.__name__}: {e}")
            failed += 1
    print(f"\n{'✅' if failed==0 else '❌'} {passed}/{len(tests)} Tests bestanden")
