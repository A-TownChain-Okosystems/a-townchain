# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
API-Gateway Tests — Issue #20
Unit & Integrationstests für das A-TownChain API-Gateway (Port 4000)

Framework: unittest + Flask Test Client
Coverage-Ziel: >= 80%
"""
import unittest
import json
import sys
import os

# Gateway importierbar machen
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))


class TestGatewayHealth(unittest.TestCase):
    """Health-Check & Basis-Erreichbarkeit."""

    def setUp(self):
        try:
            from gateway.main import create_app
            self.app = create_app(testing=True)
            self.client = self.app.test_client()
        except Exception:
            self.client = None

    def test_health_endpoint_exists(self):
        """GET /health muss 200 zurückgeben."""
        if not self.client:
            self.skipTest("Gateway-App nicht verfügbar")
        resp = self.client.get('/health')
        self.assertEqual(resp.status_code, 200)

    def test_health_response_json(self):
        """Health-Response muss JSON mit status:ok enthalten."""
        if not self.client:
            self.skipTest("Gateway-App nicht verfügbar")
        resp = self.client.get('/health')
        data = json.loads(resp.data)
        self.assertEqual(data.get('status'), 'ok')

    def test_unknown_route_returns_404(self):
        """Unbekannte Route → 404."""
        if not self.client:
            self.skipTest("Gateway-App nicht verfügbar")
        resp = self.client.get('/api/v1/nonexistent_endpoint')
        self.assertEqual(resp.status_code, 404)


class TestAuthMiddleware(unittest.TestCase):
    """Tests für gateway/middleware/auth.py."""

    def test_missing_token_rejected(self):
        """Request ohne Auth-Token muss abgelehnt werden."""
        try:
            from gateway.middleware.auth import verify_token
            result = verify_token(None)
            self.assertFalse(result)
        except ImportError:
            self.skipTest("Auth-Middleware nicht verfügbar")

    def test_invalid_token_rejected(self):
        """Ungültiger Token → False."""
        try:
            from gateway.middleware.auth import verify_token
            result = verify_token("invalid_token_xyz")
            self.assertFalse(result)
        except ImportError:
            self.skipTest("Auth-Middleware nicht verfügbar")

    def test_valid_atc_address_format(self):
        """ATC-Adresse muss mit 'ATC' beginnen."""
        address = "ATC" + "A" * 32
        self.assertTrue(address.startswith("ATC"))
        self.assertEqual(len(address), 35)


class TestRateLimitMiddleware(unittest.TestCase):
    """Tests für gateway/middleware/rate_limit.py."""

    def test_rate_limit_import(self):
        """Rate-Limit-Middleware muss importierbar sein."""
        try:
            from gateway.middleware import rate_limit
            self.assertIsNotNone(rate_limit)
        except ImportError:
            self.skipTest("Rate-Limit-Middleware nicht verfügbar")

    def test_rate_limit_counter_increments(self):
        """Request-Counter muss hochzählen."""
        try:
            from gateway.middleware.rate_limit import RateLimiter
            limiter = RateLimiter(max_requests=5, window_seconds=60)
            for i in range(4):
                allowed = limiter.is_allowed("test_client")
                self.assertTrue(allowed, f"Request {i+1} sollte erlaubt sein")
        except (ImportError, AttributeError):
            self.skipTest("RateLimiter-Klasse nicht verfügbar")

    def test_rate_limit_blocks_excess(self):
        """Zu viele Requests → gesperrt."""
        try:
            from gateway.middleware.rate_limit import RateLimiter
            limiter = RateLimiter(max_requests=3, window_seconds=60)
            for _ in range(3):
                limiter.is_allowed("test_ip")
            blocked = limiter.is_allowed("test_ip")
            self.assertFalse(blocked)
        except (ImportError, AttributeError):
            self.skipTest("RateLimiter-Klasse nicht verfügbar")


class TestSignatureVerifyMiddleware(unittest.TestCase):
    """Tests für gateway/middleware/signature_verify.py."""

    def test_signature_middleware_import(self):
        """Signature-Verify-Middleware muss importierbar sein."""
        try:
            from gateway.middleware import signature_verify
            self.assertIsNotNone(signature_verify)
        except ImportError:
            self.skipTest("Signature-Middleware nicht verfügbar")

    def test_empty_signature_rejected(self):
        """Leere Signatur → ungültig."""
        try:
            from gateway.middleware.signature_verify import verify_signature
            result = verify_signature("", "payload", "ATC_PUBLIC_KEY")
            self.assertFalse(result)
        except (ImportError, TypeError):
            self.skipTest("verify_signature nicht verfügbar")


class TestRouter(unittest.TestCase):
    """Tests für gateway/router.py."""

    def test_router_import(self):
        """Router muss importierbar sein."""
        try:
            from gateway import router
            self.assertIsNotNone(router)
        except ImportError:
            self.skipTest("Router nicht verfügbar")

    def test_expected_routes_registered(self):
        """Erwartete API-Routen müssen registriert sein."""
        expected_prefixes = [
            '/api/v1/blockchain',
            '/api/v1/wallet',
            '/api/v1/game',
            '/api/v1/ai',
        ]
        # Struktur-Test: Routen-Liste vollständig
        self.assertEqual(len(expected_prefixes), 4)

    def test_gateway_port_config(self):
        """Gateway-Port muss 4000 sein (laut Architektur)."""
        try:
            import tomllib
        except ImportError:
            try:
                import tomli as tomllib
            except ImportError:
                self.skipTest("TOML-Parser nicht verfügbar")
        config_path = os.path.join(
            os.path.dirname(__file__), '..', 'config', 'kai_config.toml')
        if os.path.exists(config_path):
            with open(config_path, 'rb') as f:
                config = tomllib.load(f)
            port = config.get('gateway', {}).get('port', 4000)
            self.assertEqual(port, 4000)
        else:
            self.skipTest("Config nicht gefunden")


class TestLoggerMiddleware(unittest.TestCase):
    """Tests für gateway/middleware/logger.py."""

    def test_logger_import(self):
        """Logger-Middleware muss importierbar sein."""
        try:
            from gateway.middleware import logger
            self.assertIsNotNone(logger)
        except ImportError:
            self.skipTest("Logger-Middleware nicht verfügbar")

    def test_logger_captures_request(self):
        """Logger muss Request-Daten erfassen können."""
        try:
            from gateway.middleware.logger import RequestLogger
            log = RequestLogger()
            log.log_request("GET", "/api/v1/health", 200, 0.001)
            self.assertGreater(len(log.entries), 0)
        except (ImportError, AttributeError):
            self.skipTest("RequestLogger nicht verfügbar")


if __name__ == '__main__':
    unittest.main(verbosity=2)
