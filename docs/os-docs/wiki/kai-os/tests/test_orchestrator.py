# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Tests für APIOrchestrator (ATS-1000)"""
import sys, os, time
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))
from backend.api.orchestrator.orchestrator import APIOrchestrator, TaskType, TaskStatus

def test_register_and_dispatch():
    orch = APIOrchestrator()
    orch.register_fn("mock-chain", lambda p: {"hash": "abc"}, [TaskType.BLOCKCHAIN])
    orch.start(workers=2)
    result = orch.dispatch_sync(TaskType.BLOCKCHAIN, {"action": "status"})
    assert result["hash"] == "abc"
    orch.stop()

def test_multiple_workers():
    orch = APIOrchestrator()
    results = []
    orch.register_fn("mock-wallet", lambda p: {"balance": 100}, [TaskType.WALLET])
    orch.start(workers=4)
    for _ in range(10):
        r = orch.dispatch_sync(TaskType.WALLET, {})
        results.append(r)
    assert len(results) == 10
    orch.stop()

def test_no_service_fails():
    orch = APIOrchestrator()
    orch.start(workers=1)
    task = orch.dispatch(TaskType.AI, {})
    deadline = time.time() + 2
    while task.status == TaskStatus.PENDING and time.time() < deadline:
        time.sleep(0.01)
    assert task.status == TaskStatus.FAILED
    orch.stop()

def test_health():
    orch = APIOrchestrator()
    orch.register_fn("mock", lambda p: {}, [TaskType.SYSTEM])
    orch.start()
    h = orch.health()
    assert h["status"] == "ok"
    orch.stop()

if __name__ == "__main__":
    tests = [v for k,v in globals().items() if k.startswith("test_")]
    ok = fail = 0
    for t in tests:
        try:
            t(); print(f"  ✅ {t.__name__}"); ok += 1
        except Exception as e:
            print(f"  ❌ {t.__name__}: {e}"); fail += 1
    print(f"\n{ok} passed, {fail} failed")
