# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Hash-Utilities — SHA-256 & BLAKE3."""
import hashlib

def sha256_hash(data) -> str:
    """SHA-256 als Hex-String. Akzeptiert bytes oder str."""
    if isinstance(data, str):
        data = data.encode()
    return hashlib.sha256(data).hexdigest()

try:
    import blake3 as _blake3
    def blake3_hash(data) -> str:
        if isinstance(data, str):
            data = data.encode()
        return _blake3.blake3(data).hexdigest()
except ImportError:
    def blake3_hash(data) -> str:
        """BLAKE3 (Fallback: BLAKE2b-32, da blake3-Paket nicht installiert)."""
        if isinstance(data, str):
            data = data.encode()
        return hashlib.blake2b(data, digest_size=32).hexdigest()
