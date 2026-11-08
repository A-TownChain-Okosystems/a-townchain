# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Tests für ProofOfHistory (ATC-1000)"""
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))
from blockchain.consensus.poh import ProofOfHistory

def test_genesis():
    poh = ProofOfHistory()
    assert len(poh.entries) == 1
    assert poh.sequence == 0

def test_single_tick():
    poh = ProofOfHistory()
    e = poh.tick()
    assert e.seq == 1
    assert len(e.hash) == 64
    assert e.prev_hash == poh.entries[0].hash

def test_tick_n():
    poh = ProofOfHistory()
    entries = poh.tick_n(10)
    assert len(entries) == 10
    assert poh.sequence == 10

def test_tick_n_with_data():
    poh = ProofOfHistory()
    entries = poh.tick_n(5, data=b"test-transaction-data")
    assert entries[0].data_hash is not None
    assert entries[1].data_hash is None  # nur erster Tick hat Daten

def test_record():
    poh = ProofOfHistory()
    e = poh.record(b"block-header-data")
    assert e.data_hash is not None

def test_verify_valid():
    poh = ProofOfHistory()
    poh.tick_n(50)
    assert poh.verify() is True

def test_hash_chain_integrity():
    poh = ProofOfHistory()
    poh.tick_n(20)
    for i in range(1, len(poh.entries)):
        assert poh.entries[i].prev_hash == poh.entries[i-1].hash

def test_snapshot_restore():
    poh = ProofOfHistory()
    poh.tick_n(30)
    snap = poh.snapshot()
    poh2 = ProofOfHistory.from_snapshot(snap)
    assert poh2.current_hash == poh.current_hash
    assert poh2.sequence == poh.sequence

if __name__ == "__main__":
    tests = [v for k,v in globals().items() if k.startswith("test_")]
    ok = fail = 0
    for t in tests:
        try:
            t(); print(f"  ✅ {t.__name__}"); ok += 1
        except Exception as e:
            print(f"  ❌ {t.__name__}: {e}"); fail += 1
    print(f"\n{ok} passed, {fail} failed")
