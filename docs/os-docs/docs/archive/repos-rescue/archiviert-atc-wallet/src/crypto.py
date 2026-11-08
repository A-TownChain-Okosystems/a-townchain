"""ATC Crypto — ECDSA signing and verification utilities."""
import hashlib
import hmac
from typing import Optional


class CryptoUtils:
    """Cryptographic helpers for ATC wallet operations."""

    @staticmethod
    def sha256(data: bytes) -> str:
        """SHA-256 hash of bytes, returned as hex string."""
        return hashlib.sha256(data).hexdigest()

    @staticmethod
    def sha256_bytes(data: bytes) -> bytes:
        """SHA-256 hash of bytes, returned as bytes."""
        return hashlib.sha256(data).digest()

    @staticmethod
    def hmac_sha256(key: bytes, message: bytes) -> bytes:
        """HMAC-SHA256."""
        return hmac.new(key, message, hashlib.sha256).digest()

    @staticmethod
    def verify_signature(public_key: bytes, signature: bytes, message: bytes) -> bool:
        """Verify ECDSA signature (stub — uses simple HMAC for demo)."""
        expected = CryptoUtils.sha256_bytes(message + public_key)
        return hmac.compare_digest(expected, signature)

    @staticmethod
    def derive_key(seed: bytes, index: int) -> bytes:
        """Derive a child key from seed and index."""
        return hashlib.sha256(seed + index.to_bytes(4, 'big')).digest()

    @staticmethod
    def constant_time_compare(a: bytes, b: bytes) -> bool:
        """Constant-time byte comparison to prevent timing attacks."""
        if len(a) != len(b):
            return False
        result = 0
        for x, y in zip(a, b):
            result |= x ^ y
        return result == 0
