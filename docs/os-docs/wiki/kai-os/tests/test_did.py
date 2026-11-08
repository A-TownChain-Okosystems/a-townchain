# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Tests für DID-Resolver (ATAUTH-1000)"""
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))
from blockchain.wallet.did import DIDResolver

def test_create_did():
    did = DIDResolver.create_did("my-pub-key-abc123")
    assert did.startswith("did:kai:")
    assert len(did) > 20

def test_deterministic():
    d1 = DIDResolver.create_did("same-key")
    d2 = DIDResolver.create_did("same-key")
    assert d1 == d2

def test_register_and_resolve():
    r = DIDResolver()
    doc = r.register("pubkey-xyz")
    resolved = r.resolve(doc.did)
    assert resolved is not None
    assert resolved.did == doc.did

def test_verify():
    r = DIDResolver()
    doc = r.register("pubkey-test-001")
    assert r.verify(doc.did, "pubkey-test-001") is True
    assert r.verify(doc.did, "wrong-key") is False

def test_revoke():
    r = DIDResolver()
    doc = r.register("pubkey-revoke")
    assert r.revoke(doc.did) is True
    assert r.verify(doc.did, "pubkey-revoke") is False

def test_update():
    r = DIDResolver()
    doc = r.register("pubkey-update")
    r.update(doc.did, capabilities=["auth", "admin"])
    resolved = r.resolve(doc.did)
    assert "admin" in resolved.capabilities

def test_stats():
    r = DIDResolver()
    r.register("key1"); r.register("key2")
    doc = r.register("key3")
    r.revoke(doc.did)
    s = r.stats()
    assert s["total"] == 3
    assert s["active"] == 2
    assert s["revoked"] == 1

if __name__ == "__main__":
    tests = [v for k,v in globals().items() if k.startswith("test_")]
    ok = fail = 0
    for t in tests:
        try:
            t(); print(f"  ✅ {t.__name__}"); ok += 1
        except Exception as e:
            print(f"  ❌ {t.__name__}: {e}"); fail += 1
    print(f"\n{ok} passed, {fail} failed")
