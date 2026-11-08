# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Key-Generator — Keypair-Erzeugung. BIP39: PLANNED."""

from .ecdsa import ECDSAKeyPair

def generate_keypair():
    """Erzeugt ein ECDSA-Schlüsselpaar (delegiert an ECDSAKeyPair)."""
    return ECDSAKeyPair.generate_keypair()

class BIP39KeyGenerator:
    """BIP39-Mnemonic-Generator — PLANNED.

    Benötigt die offizielle BIP39-Wordlist (2048 Wörter) und ist
    laut COMPONENT_PLAN noch nicht implementiert.
    """
    def __init__(self, *args, **kwargs):
        raise NotImplementedError(
            "BIP39KeyGenerator ist PLANNED (siehe COMPONENT_PLAN.md) — "
            "offizielle BIP39-Wordlist erforderlich."
        )
