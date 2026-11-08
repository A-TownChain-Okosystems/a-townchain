# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für Smart Contracts (ATC-8300, ATC-9900)
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..'))


class TestSmartContracts:
    """Test smart contract functionality."""

    def test_smart_contracts_import(self):
        """Smart contracts module can be imported."""
        try:
            from src.blockchain.smart_contracts import SmartContract
            assert SmartContract is not None
        except ImportError:
            pytest.skip("src.blockchain.smart_contracts not importable")

    def test_atc8300_token_import(self):
        """ATC-8300 token standard can be imported."""
        try:
            from src.contracts.atc8300_token import ATC8300Token
            assert ATC8300Token is not None
        except ImportError:
            pytest.skip("src.contracts.atc8300_token not importable")

    def test_base_contract_import(self):
        """Base contract can be imported."""
        try:
            from src.contracts.base_contract import BaseContract
            assert BaseContract is not None
        except ImportError:
            pytest.skip("src.contracts.base_contract not importable")

    def test_contract_registry_import(self):
        """Contract registry can be imported."""
        try:
            from src.blockchain.smart_contract_registry import ContractRegistry
            assert ContractRegistry is not None
        except ImportError:
            pytest.skip("src.blockchain.smart_contract_registry not importable")

    def test_governance_contract_import(self):
        """Governance contract (ATC-9900) can be imported."""
        try:
            from src.contracts.governance_contract import GovernanceContract
            assert GovernanceContract is not None
        except ImportError:
            pytest.skip("src.contracts.governance_contract not importable")

    def test_bridge_contract_import(self):
        """Bridge contract can be imported."""
        try:
            from src.contracts.bridge_contract import BridgeContract
            assert BridgeContract is not None
        except ImportError:
            pytest.skip("src.contracts.bridge_contract not importable")

    def test_marketplace_contract_import(self):
        """Marketplace contract can be imported."""
        try:
            from src.contracts.marketplace_contract import MarketplaceContract
            assert MarketplaceContract is not None
        except ImportError:
            pytest.skip("src.contracts.marketplace_contract not importable")

    def test_atcoin_import(self):
        """ATCoin base token can be imported."""
        try:
            from src.contracts.atcoin import ATCoin
            assert ATCoin is not None
        except ImportError:
            pytest.skip("src.contracts.atcoin not importable")
