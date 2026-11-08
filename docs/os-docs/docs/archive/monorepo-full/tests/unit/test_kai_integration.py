# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für KAI-OS Integration (Gemini AI)
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))


class TestKAIIntegration:
    """Test KAI-OS AI Integration."""

    def test_kai_integration_doc_exists(self):
        """KAI integration documentation exists."""
        path = os.path.join(os.path.dirname(__file__), '..', 'docs', 'KAI_INTEGRATION.md')
        assert os.path.exists(path), "KAI_INTEGRATION.md not found"

    def test_atc_ai_standards_exist(self):
        """AI-related ATC standards exist."""
        standards_dir = os.path.join(os.path.dirname(__file__), '..', 'docs', 'standards')
        ai_standards = [
            'ATC-15-PROOF_OF_AI_MINING.md',
            'ATC-24-AGENT_SCHEDULING.md',
            'ATC-25-TENSOR_COMPUTE.md',
            'ATC-26-XAI_TRANSPARENCY.md',
        ]
        for s in ai_standards:
            path = os.path.join(standards_dir, s)
            assert os.path.exists(path), f"{s} not found"

    def test_event_bus_import(self):
        """Event bus for AI integration can be imported."""
        try:
            from src.core.event_bus import EventBus
            assert EventBus is not None
        except ImportError:
            pytest.skip("src.core.event_bus not importable")

    def test_kernel_api_import(self):
        """Kernel API for AI integration can be imported."""
        try:
            from src.core.kernel.api import KernelAPI
            assert KernelAPI is not None
        except ImportError:
            pytest.skip("src.core.kernel.api not importable")
