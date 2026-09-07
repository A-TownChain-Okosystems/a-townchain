# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.backends import default_backend
import json, time


class ECDSASigner:
    """
    ECDSA secp256k1 Signatur-System fuer A-TownChain OS.
    Issue #6 — Sichere TX-Autorisierung.

    Alle Transfers und NFT-Transaktionen muessen mit dem
    Private Key des Senders signiert und verifiziert werden.
    """

    @staticmethod
    def generate_keypair() -> tuple:
        """Generiert ein neues ECDSA secp256k1 Schluesselpaar.
        Returns: (private_key_hex: str, public_key_hex: str)
        WICHTIG: Private Key nur einmal anzeigen, niemals speichern!
        """
        priv_key = ec.generate_private_key(ec.SECP256K1(), default_backend())
        priv_hex = format(priv_key.private_numbers().private_value, "064x")
        pub_hex  = priv_key.public_key().public_bytes(
            serialization.Encoding.X962,
            serialization.PublicFormat.UncompressedPoint
        ).hex()
        return priv_hex, pub_hex

    @staticmethod
    def sign(tx_data: dict, private_key_hex: str) -> str:
        """Signiert eine Transaktion mit dem Private Key.
        Args:
            tx_data:         TX-Dict (from, to, amount, fee, nonce, timestamp)
            private_key_hex: 64-char hex Private Key
        Returns: DER-kodierte Signatur als hex string
        """
        message  = json.dumps(tx_data, sort_keys=True).encode()
        priv_key = ec.derive_private_key(
            int(private_key_hex, 16), ec.SECP256K1(), default_backend())
        return priv_key.sign(message, ec.ECDSA(hashes.SHA256())).hex()

    @staticmethod
    def verify(tx_data: dict, signature_hex: str, public_key_hex: str) -> bool:
        """Verifiziert eine TX-Signatur.
        Returns: True wenn gueltig, False wenn manipuliert/ungueltig
        """
        try:
            message    = json.dumps(tx_data, sort_keys=True).encode()
            public_key = ec.EllipticCurvePublicKey.from_encoded_point(
                ec.SECP256K1(), bytes.fromhex(public_key_hex))
            public_key.verify(
                bytes.fromhex(signature_hex), message, ec.ECDSA(hashes.SHA256()))
            return True
        except Exception:
            return False

    @staticmethod
    def build_tx(from_addr: str, to_addr: str, amount: float,
                 fee: float = 0.001, nonce: int = None) -> dict:
        """Erstellt ein standardisiertes TX-Dict (vor dem Signieren).
        Der Nonce verhindert Replay-Attacks.
        """
        return {
            "from":      from_addr,
            "to":        to_addr,
            "amount":    float(amount),
            "fee":       fee,
            "nonce":     nonce or int(time.time() * 1000),
            "timestamp": int(time.time())
        }
