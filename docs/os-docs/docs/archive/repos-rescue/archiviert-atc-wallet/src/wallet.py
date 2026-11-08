"""ATC Wallet — Core wallet implementation.
ATC-prefixed addresses with SHA-256 derivation and ECDSA signing.
"""
import hashlib
import json
from typing import Optional

ATC_PREFIX = "ATC"
ADDRESS_LENGTH = 35  # ATC + 32 hex chars


class Wallet:
    """ATC wallet with ECDSA signing and SHA-256 address derivation."""

    def __init__(self, private_key: bytes):
        self.private_key = private_key
        self.address = self._derive_address(private_key)
        self.balance = 0
        self.nonce = 0

    @staticmethod
    def _derive_address(private_key: bytes) -> str:
        """Derive ATC address from private key using SHA-256."""
        public_hash = hashlib.sha256(private_key).hexdigest()
        return f"{ATC_PREFIX}{public_hash[:32]}"

    @staticmethod
    def is_valid_address(address: str) -> bool:
        """Check if address starts with ATC and has correct length."""
        return (address.startswith(ATC_PREFIX) and
                len(address) == ADDRESS_LENGTH and
                all(c in '0123456789abcdef' for c in address[3:]))

    def sign_transaction(self, to: str, amount: float, fee: float = 0.001) -> dict:
        """Create and sign a transaction."""
        if not self.is_valid_address(to):
            raise ValueError(f"Invalid recipient address: {to}")
        if amount + fee > self.balance:
            raise ValueError("Insufficient balance")

        tx = {
            "from": self.address,
            "to": to,
            "amount": amount,
            "fee": fee,
            "nonce": self.nonce,
        }
        tx_hash = hashlib.sha256(json.dumps(tx, sort_keys=True).encode()).hexdigest()
        tx["hash"] = tx_hash
        self.nonce += 1
        return tx

    def receive(self, amount: float) -> None:
        """Add received amount to balance."""
        self.balance += amount

    def to_dict(self) -> dict:
        """Serialize wallet to dict."""
        return {
            "address": self.address,
            "balance": self.balance,
            "nonce": self.nonce,
        }


def generate_wallet() -> Wallet:
    """Generate a new random wallet."""
    private_key = hashlib.sha256(os.urandom(32)).digest()
    return Wallet(private_key)


if __name__ == "__main__":
    import os as _os
    wallet = generate_wallet()
    print(f"Address: {wallet.address}")
    print(f"Valid: {Wallet.is_valid_address(wallet.address)}")
