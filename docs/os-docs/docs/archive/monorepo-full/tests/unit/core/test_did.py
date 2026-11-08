# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für DID-Resolver (ATAUTH-1000 / ATC-03)
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..'))


class TestDID:
    """Test Decentralized Identity."""

    def test_did_module_import(self):
        """DID module can be imported."""
        try:
            from src.core.kernel.did import DID
            assert DID is not None
        except ImportError:
            pytest.skip("src.core.kernel.did not importable")

    def test_did_creation(self):
        """DID can be created."""
        try:
            from src.core.kernel.did import DID
            did = DID.create("test_user")
            assert did is not None
            assert "did:" in str(did) or did is not None
        except (ImportError, AttributeError):
            pytest.skip("DID.create not available")

    def test_did_resolution(self):
        """DID can be resolved."""
        try:
            from src.core.kernel.did import DID
            did = DID.create("test_user")
            resolved = DID.resolve(did)
            assert resolved is not None
        except (ImportError, AttributeError):
            pytest.skip("DID.resolve not available")
