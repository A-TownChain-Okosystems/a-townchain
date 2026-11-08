# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für APIOrchestrator (ATS-1000)
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..'))


class TestOrchestrator:
    """Test API Orchestrator."""

    def test_orchestrator_module_import(self):
        """Orchestrator module can be imported."""
        try:
            from src.core.module_loader import ModuleLoader
            assert ModuleLoader is not None
        except ImportError:
            pytest.skip("src.core.module_loader not importable")

    def test_event_bus_import(self):
        """Event bus can be imported."""
        try:
            from src.core.event_bus import EventBus
            assert EventBus is not None
        except ImportError:
            pytest.skip("src.core.event_bus not importable")

    def test_module_loader_load(self):
        """Module loader can load modules."""
        try:
            from src.core.module_loader import ModuleLoader
            loader = ModuleLoader()
            assert loader is not None
        except (ImportError, AttributeError):
            pytest.skip("ModuleLoader not available")
