# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# blockchain/wallet/ecdsa.py
# Issue #6 — ECDSA Signatur (secp256k1)

from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.backends import default_backend
import hashlib, json, time

class ECDSASigner:
    """
    ECDSA secp256k1 Signatur-System für A-TownChain.
    Alle TX und NFT-Transfers müssen mit dem Private Key signiert werden.
    """

    CURVE = ec.SECP256K1()

    @staticmethod
    def generate_keypair() -> tuple:
        private_key = ec.generate_private_key(ec.SECP256K1(), default_backend())
        public_key  = private_key.public_key()
        priv_hex = private_key.private_bytes(
            serialization.Encoding.Raw,
            serialization.PrivateFormat.Raw,
            serialization.NoEncryption()
        ).hex()
        pub_hex = public_key.public_bytes(
            serialization.Encoding.X962,
            serialization.PublicFormat.UncompressedPoint
        ).hex()
        return priv_hex, pub_hex

    @staticmethod
    def sign(tx_data: dict, private_key_hex: str) -> str:
        message   = json.dumps(tx_data, sort_keys=True).encode()
        msg_hash  = hashlib.sha256(message).digest()
        priv_int  = int(private_key_hex, 16)
        priv_key  = ec.derive_private_key(priv_int, ec.SECP256K1(), default_backend())
        signature = priv_key.sign(msg_hash, ec.ECDSA(hashes.Prehashed()))
        return signature.hex()

    @staticmethod
    def verify(tx_data: dict, signature_hex: str, public_key_hex: str) -> bool:
        try:
            message    = json.dumps(tx_data, sort_keys=True).encode()
            msg_hash   = hashlib.sha256(message).digest()
            pub_bytes  = bytes.fromhex(public_key_hex)
            public_key = ec.EllipticCurvePublicKey.from_encoded_point(ec.SECP256K1(), pub_bytes)
            public_key.verify(bytes.fromhex(signature_hex), msg_hash, ec.ECDSA(hashes.Prehashed()))
            return True
        except Exception:
            return False

    @staticmethod
    def build_tx(from_addr, to_addr, amount, fee=0.001, nonce=None) -> dict:
        return {
            "from":      from_addr,
            "to":        to_addr,
            "amount":    amount,
            "fee":       fee,
            "nonce":     nonce or int(time.time() * 1000),
            "timestamp": int(time.time())
        }

# ── SELF-TEST ──────────────────────────────────────────
if __name__ == "__main__":
    print("🔐 ECDSA secp256k1 — Self-Test")
    priv, pub = ECDSASigner.generate_keypair()
    print(f"  Private Key : {priv[:16]}...")
    print(f"  Public Key  : {pub[:16]}...")

    tx = ECDSASigner.build_tx("ATC_SENDER", "ATC_RECEIVER", 100.0)
    sig = ECDSASigner.sign(tx, priv)
    print(f"  Signatur    : {sig[:24]}...")

    ok = ECDSASigner.verify(tx, sig, pub)
    print(f"  Verifikation: {'✅ VALID' if ok else '❌ INVALID'}")

    # Tamper-Test
    tx_fake = {**tx, "amount": 999.0}
    bad = ECDSASigner.verify(tx_fake, sig, pub)
    print(f"  Tamper-Test : {'❌ FAIL' if bad else '✅ Manipulierter TX abgelehnt'}")
