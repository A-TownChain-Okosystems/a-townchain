# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/wallet/wallet.py
# A-Town Wallet Service — mit echter Kryptographie
#
# Nutzt blockchain/wallet/keygen.py für:
#   - Private Key (256-bit)
#   - Seed Phrase (24 Wörter, BIP39)
#   - ATC Adresse (beginnt mit "ATC...")

import time
from blockchain.wallet.keygen import ATCKeyGenerator
from blockchain.atcoin.atcoin import ATCoin

class ATCWallet:
    """A-Town Multi-Standard Wallet mit echter Kryptographie."""

    def __init__(self, atcoin: ATCoin):
        self.atcoin   = atcoin
        self.keygen   = ATCKeyGenerator()
        self.accounts = {}   # address → wallet_data (ohne private key im RAM)
        self.nfts     = {}   # address → [nft_data]

    # ── Wallet erstellen ───────────────────────────────
    def create_wallet(self, label: str = "Main Wallet", passphrase: str = "A-TownChain") -> dict:
        """
        Erstellt ein neues Wallet mit:
          - 24 Wörter Seed Phrase
          - Private Key (256-bit)
          - ATC Adresse (ATC...)
        """
        wallet = self.keygen.generate_wallet(passphrase)
        address = wallet["address"]

        # Im Service nur Adresse + Public Key speichern
        # Private Key + Seed Phrase gehören zum User — NICHT im Server speichern!
        self.accounts[address] = {
            "label":      label,
            "public_key": wallet["public_key"],
            "created_at": int(time.time())
        }

        # Faucet: 100 ATC zum Start
        self.atcoin.mint(address, 100)

        return {
            "address":     wallet["address"],
            "public_key":  wallet["public_key"],
            "private_key": wallet["private_key"],   # ⚠️ Nur einmalig anzeigen!
            "seed_phrase": wallet["seed_phrase"],    # ⚠️ Sicher aufbewahren!
            "word_count":  wallet["word_count"],
            "label":       label,
            "balance":     "100 ATC",
            "standard":    "ATC-8300",
            "warning":     "Seed Phrase und Private Key sicher aufbewahren — werden nie wieder angezeigt!"
        }

    # ── Wallet wiederherstellen ────────────────────────
    def restore_wallet(self, seed_phrase: list[str], passphrase: str = "A-TownChain") -> dict:
        """Stellt Wallet aus Seed Phrase wieder her."""
        data    = self.keygen.restore_from_mnemonic(seed_phrase, passphrase)
        address = data["address"]
        self.accounts[address] = {
            "label":      "Restored Wallet",
            "public_key": data["public_key"],
            "created_at": int(time.time())
        }
        balance = str(self.atcoin.balance_of(address))
        return {
            "address":    address,
            "public_key": data["public_key"],
            "balance":    balance + " ATC",
            "restored":   True
        }

    # ── Balance ────────────────────────────────────────
    def get_balance(self, address: str) -> dict:
        if not self.keygen.validate_address(address):
            return {"error": "Invalid ATC address format"}
        return {
            "address": address,
            "balance": str(self.atcoin.balance_of(address)),
            "symbol":  "ATC",
            "nfts":    self.nfts.get(address, [])
        }

    # ── Transfer ───────────────────────────────────────
    def send(self, sender: str, receiver: str, amount: float) -> dict:
        if not self.keygen.validate_address(sender):
            return {"success": False, "error": "Invalid sender address"}
        if not self.keygen.validate_address(receiver):
            return {"success": False, "error": "Invalid receiver address"}
        result = self.atcoin.transfer(sender, receiver, amount)
        if result["success"]:
            result["fee"] = "0.001 ATC"
        return result

    # ── Adresse validieren ─────────────────────────────
    def validate_address(self, address: str) -> dict:
        valid = self.keygen.validate_address(address)
        return {
            "address": address,
            "valid":   valid,
            "format":  "ATC + 32 hex chars (35 Zeichen)" if valid else "Ungültig"
        }

    # ── NFT (ATC-9000) ─────────────────────────────────
    def mint_nft(self, owner: str, nft_id: str, metadata: dict) -> dict:
        if owner not in self.nfts:
            self.nfts[owner] = []
        self.nfts[owner].append({"id": nft_id, "metadata": metadata, "minted": int(time.time())})
        return {"success": True, "nft_id": nft_id, "owner": owner}

    def get_accounts(self) -> list:
        return [
            {"address": addr, "label": d["label"],
             "balance": str(self.atcoin.balance_of(addr))}
            for addr, d in self.accounts.items()
        ]
