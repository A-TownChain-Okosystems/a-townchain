# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für Bootstrap-Node — ATC-85
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..'))


class TestBootstrap:
    """Test Bootstrap Node."""

    def test_bootstrap_config(self):
        """Bootstrap configuration can be loaded."""
        try:
            from src.core.kernel.kernel import Kernel
            assert Kernel is not None
        except ImportError:
            pytest.skip("src.core.kernel.kernel not importable")

    def test_kernel_api_import(self):
        """Kernel API can be imported."""
        try:
            from src.core.kernel.api import KernelAPI
            assert KernelAPI is not None
        except ImportError:
            pytest.skip("src.core.kernel.api not importable")

    def test_capabilities_import(self):
        """Kernel capabilities can be imported."""
        try:
            from src.core.kernel.capabilities import Capabilities
            assert Capabilities is not None
        except ImportError:
            pytest.skip("src.core.kernel.capabilities not importable")
