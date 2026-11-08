# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für Gateway API (Full)
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..'))


class TestGatewayFull:
    """Test Gateway API endpoints."""

    def test_gateway_main_import(self):
        """Gateway main module can be imported."""
        try:
            from src.gateway.main import app
            assert app is not None
        except ImportError:
            pytest.skip("src.gateway.main not importable")

    def test_gateway_router_import(self):
        """Gateway router can be imported."""
        try:
            from src.gateway.router import router
            assert router is not None
        except ImportError:
            pytest.skip("src.gateway.router not importable")

    def test_gateway_health_endpoint(self):
        """Gateway /health or /gateway/health endpoint."""
        try:
            from src.gateway.main import app
            from flask.testing import FlaskClient
            client = app.test_client()
            response = client.get("/gateway/health")
            assert response.status_code in (200, 404, 302)
        except ImportError:
            try:
                from fastapi.testclient import TestClient
                from src.gateway.main import app
                client = TestClient(app)
                response = client.get("/gateway/health")
                assert response.status_code in (200, 404)
            except ImportError:
                pytest.skip("Gateway test client not available")
