# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""ATC Crypto — ECDSA signing and verification utilities."""

import hashlib
import hmac

from ecdsa import SECP256k1, VerifyingKey
from ecdsa.util import sigdecode_der


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
        """Verify a canonical secp256k1 ECDSA signature.

        The wallet signer emits ASN.1 DER signatures. Both compressed and
        uncompressed SEC1 public keys are accepted at this API boundary; the
        transaction layer remains responsible for enforcing its canonical
        compressed-public-key wire encoding.
        """
        try:
            if len(signature) == 0:
                return False
            key = VerifyingKey.from_string(public_key, curve=SECP256k1)
            r, s = sigdecode_der(signature, SECP256k1.order)
            if not (1 <= r < SECP256k1.order and 1 <= s <= SECP256k1.order // 2):
                return False
            return key.verify_digest(signature, CryptoUtils.sha256_bytes(message), sigdecode=sigdecode_der)
        except (ValueError, AssertionError):
            return False

    @staticmethod
    def derive_key(seed: bytes, index: int) -> bytes:
        """Derive a child key from seed and index."""
        return hashlib.sha256(seed + index.to_bytes(4, "big")).digest()

    @staticmethod
    def constant_time_compare(a: bytes, b: bytes) -> bool:
        """Constant-time byte comparison to prevent timing attacks."""
        if len(a) != len(b):
            return False
        result = 0
        for x, y in zip(a, b):
            result |= x ^ y
        return result == 0
