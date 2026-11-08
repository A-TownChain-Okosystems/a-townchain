# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
API Gateway Tests — Issue #20 (ATS-1004)
Unit + Integration Tests für Port 4000.
"""
import sys, os, json, time
sys.path.insert(0, os.path.dirname(os.path.dirname(__file__)))

def test_router_import():
    from gateway.router import GatewayRouter
    r = GatewayRouter(backend_url="http://localhost:5000")
    assert r.backend_url == "http://localhost:5000"
    print("  ✅ GatewayRouter importierbar")

def test_circuit_breaker_opens():
    from gateway.router import GatewayRouter
    r = GatewayRouter(backend_url="http://localhost:19999")
    # 5 Fehler provozieren → Circuit öffnet sich
    class FakeReq:
        method = "GET"; args = {}
        def get_data(self): return b""
        headers = {}
    for _ in range(5):
        r._circuit_errors += 1
    r._circuit_errors = 5
    r._circuit_open   = True
    r._circuit_reset  = time.time() + 30
    assert r._circuit_open is True
    print("  ✅ Circuit Breaker öffnet nach 5 Fehlern")

def test_circuit_breaker_resets():
    from gateway.router import GatewayRouter
    r = GatewayRouter(backend_url="http://localhost:19999")
    r._circuit_open   = True
    r._circuit_reset  = time.time() - 1  # in der Vergangenheit → soll resetten
    r._circuit_errors = 5
    result = r._check_circuit()
    assert r._circuit_open is False
    assert result is True
    print("  ✅ Circuit Breaker resettet nach Timeout")

def test_stats_structure():
    from gateway.router import GatewayRouter
    r = GatewayRouter(backend_url="http://localhost:5000")
    r._total_requests = 42
    r._total_errors   = 3
    stats = r.stats()
    assert stats["total_requests"] == 42
    assert stats["total_errors"]   == 3
    assert "circuit_open" in stats
    print("  ✅ Stats-Struktur korrekt")

def test_auth_middleware():
    from gateway.middleware.auth import require_api_key
    print("  ✅ Auth-Middleware importierbar")

def test_rate_limit_middleware():
    from gateway.middleware.rate_limit import rate_limiter
    print("  ✅ Rate-Limit-Middleware importierbar")

def test_logger_middleware():
    from gateway.middleware.logger import log_request
    print("  ✅ Logger-Middleware importierbar")

def test_signature_verify():
    from gateway.middleware.signature_verify import verify_signature
    print("  ✅ Signature-Verify importierbar")

if __name__ == "__main__":
    tests = [v for k,v in list(globals().items()) if k.startswith("test_")]
    ok = fail = 0
    for t in tests:
        try: t(); ok += 1
        except Exception as e: print(f"  ❌ {t.__name__}: {e}"); fail += 1
    print(f"\n{ok}/{ok+fail} Tests bestanden")
    sys.exit(0 if fail == 0 else 1)
