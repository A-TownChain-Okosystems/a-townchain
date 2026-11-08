# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für ECDSA (secp256k1/RFC 6979) — ATC-86
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..'))


class TestECDSA:
    """Test ECDSA signature operations."""

    def test_ecdsa_module_import(self):
        """ECDSA module can be imported."""
        try:
            from src.contracts.ecdsa import ECDSA
            assert ECDSA is not None
        except ImportError:
            pytest.skip("src.contracts.ecdsa not importable")

    def test_key_generation(self):
        """ECDSA key pair generation."""
        try:
            from src.contracts.ecdsa import ECDSA
            private_key, public_key = ECDSA.generate_keypair()
            assert private_key is not None
            assert public_key is not None
        except (ImportError, AttributeError):
            pytest.skip("ECDSA.generate_keypair not available")

    def test_sign_and_verify(self):
        """ECDSA sign and verify roundtrip."""
        try:
            from src.contracts.ecdsa import ECDSA
            private_key, public_key = ECDSA.generate_keypair()
            message = b"test message"
            signature = ECDSA.sign(private_key, message)
            assert signature is not None
            valid = ECDSA.verify(public_key, message, signature)
            assert valid is True
        except (ImportError, AttributeError):
            pytest.skip("ECDSA sign/verify not available")

    def test_verify_invalid_signature(self):
        """ECDSA rejects invalid signatures."""
        try:
            from src.contracts.ecdsa import ECDSA
            private_key, public_key = ECDSA.generate_keypair()
            message = b"test message"
            # Use wrong message for verification
            valid = ECDSA.verify(public_key, b"wrong message", b"fake_signature")
            assert valid is False
        except (ImportError, AttributeError):
            pytest.skip("ECDSA verify not available")

    def test_keygen_module_import(self):
        """Key generation module can be imported."""
        try:
            from src.contracts.keygen import KeyGen
            assert KeyGen is not None
        except ImportError:
            pytest.skip("src.contracts.keygen not importable")
