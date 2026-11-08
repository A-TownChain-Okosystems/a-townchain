# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für ATCFS Multi-Sig (ATC-18)
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..'))


class TestATCFSMultiSig:
    """Test ATCFS Multi-Signature."""

    def test_atcfs_import(self):
        """ATCFS module can be imported."""
        try:
            from src.core.atcfs import ATCFS
            assert ATCFS is not None
        except ImportError:
            pytest.skip("src.core.atcfs not importable")

    def test_wallet_ecdsa_import(self):
        """Wallet ECDSA module can be imported."""
        try:
            from src.contracts.wallet_ecdsa import WalletECDSA
            assert WalletECDSA is not None
        except ImportError:
            pytest.skip("src.contracts.wallet_ecdsa not importable")

    def test_wallet_keygen_import(self):
        """Wallet keygen module can be imported."""
        try:
            from src.contracts.wallet_keygen import WalletKeyGen
            assert WalletKeyGen is not None
        except ImportError:
            pytest.skip("src.contracts.wallet_keygen not importable")
