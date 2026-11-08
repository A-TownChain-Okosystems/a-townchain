# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""ECDSA-Adapter — basiert auf src/wallet/ecdsa.py (kanonische Implementierung)."""

# Kanonische Implementierung:
from ...wallet.ecdsa import ECDSASigner  # noqa: F401

class ECDSAKeyPair(ECDSASigner):
    """ECDSAKeyPair — Kompatibilitäts-Alias für ECDSASigner."""
    pass

def sign_transaction(tx_data: dict, private_key_hex: str) -> str:
    return ECDSASigner.sign(tx_data, private_key_hex)

def verify_signature(tx_data: dict, signature_hex: str, public_key_hex: str) -> bool:
    return ECDSASigner.verify(tx_data, signature_hex, public_key_hex)
