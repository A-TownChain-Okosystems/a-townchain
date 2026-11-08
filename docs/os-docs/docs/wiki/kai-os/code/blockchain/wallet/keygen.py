# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# blockchain/wallet/keygen.py
# A-Town Wallet — Key Generation
#
# Generiert:
#   1. Entropy (256-bit Zufallszahl)
#   2. Seed Phrase (24 Wörter, BIP39-kompatibel)
#   3. Private Key (SHA-256 aus Seed, 64 hex chars)
#   4. Public Key  (SHA-256 des Private Key)
#   5. ATC Adresse (beginnt mit "ATC" + 32 hex chars, gesamt 35 chars)
#
# Sicherheitshinweis:
#   - Private Key NIEMALS teilen oder unverschlüsselt speichern
#   - Seed Phrase ist der Master-Key — verloren = kein Zugriff

import os, hashlib, hmac, struct
from blockchain.wallet.wordlist import BIP39_WORDLIST

class ATCKeyGenerator:
    """Generiert kryptographisch sichere A-Town Wallet Keys."""

    # ── Entropy → Seed Phrase ──────────────────────────
    def generate_entropy(self, bits: int = 256) -> bytes:
        """Erzeugt kryptographisch sichere Zufalls-Entropy."""
        if bits not in (128, 160, 192, 224, 256):
            raise ValueError("bits must be 128/160/192/224/256")
        return os.urandom(bits // 8)

    def entropy_to_mnemonic(self, entropy: bytes) -> list[str]:
        """
        Konvertiert Entropy zu BIP39 Mnemonic (Seed Phrase).
        256-bit Entropy → 24 Wörter
        128-bit Entropy → 12 Wörter
        """
        # Checksum: erste (len*8/32) Bits des SHA-256 Hash
        h         = hashlib.sha256(entropy).digest()
        checksum_bits = len(entropy) * 8 // 32
        # Entropy + Checksum als Bitstring
        ent_int   = int.from_bytes(entropy, "big")
        cs_int    = int.from_bytes(h, "big") >> (256 - checksum_bits)
        combined  = (ent_int << checksum_bits) | cs_int
        total_bits = len(entropy) * 8 + checksum_bits
        words     = []
        for i in range(total_bits // 11):
            idx = (combined >> (total_bits - 11 * (i + 1))) & 0x7FF
            words.append(BIP39_WORDLIST[idx])
        return words

    def mnemonic_to_seed(self, mnemonic: list[str], passphrase: str = "A-TownChain") -> bytes:
        """Konvertiert Mnemonic zu 512-bit Seed via PBKDF2-HMAC-SHA512."""
        mnemonic_str = " ".join(mnemonic)
        salt         = ("mnemonic" + passphrase).encode("utf-8")
        seed = hashlib.pbkdf2_hmac(
            "sha512",
            mnemonic_str.encode("utf-8"),
            salt,
            iterations=2048,
            dklen=64
        )
        return seed

    # ── Seed → Private Key ─────────────────────────────
    def seed_to_private_key(self, seed: bytes) -> str:
        """Deriviert den Private Key aus dem Seed (HMAC-SHA256)."""
        key = hmac.new(b"A-TownChain seed", seed, hashlib.sha256).hexdigest()
        return key  # 64 hex chars = 256-bit

    # ── Private Key → Public Key ───────────────────────
    def private_to_public_key(self, private_key: str) -> str:
        """Deriviert den Public Key aus dem Private Key (SHA-256)."""
        pub = hashlib.sha256(bytes.fromhex(private_key)).hexdigest()
        return pub  # 64 hex chars

    # ── Public Key → ATC Adresse ───────────────────────
    def public_key_to_address(self, public_key: str) -> str:
        """
        Erstellt eine ATC-Adresse aus dem Public Key.
        Format: ATC + [32 hex chars uppercase] = 35 Zeichen
        Beispiel: ATC7F3A9B2C1D4E5F6A7B8C9D0E1F2A3B4C
        """
        # RIPEMD-160 Equivalent: doppelter SHA-256 → nimm erste 32 chars
        step1   = hashlib.sha256(bytes.fromhex(public_key)).hexdigest()
        step2   = hashlib.sha256(step1.encode()).hexdigest()
        # Checksum: letzte 4 chars von nochmaligem SHA-256
        checksum = hashlib.sha256(step2.encode()).hexdigest()[:4].upper()
        address  = "ATC" + step2[:28].upper() + checksum
        return address  # ATC + 32 chars = 35 Zeichen total

    # ── Alles auf einmal ───────────────────────────────
    def generate_wallet(self, passphrase: str = "A-TownChain") -> dict:
        """
        Generiert ein komplettes A-Town Wallet.
        Gibt zurück:
          - seed_phrase  (24 Wörter)
          - private_key  (256-bit hex)
          - public_key   (256-bit hex)
          - address      (ATC + 32 chars)
        """
        entropy    = self.generate_entropy(256)
        mnemonic   = self.entropy_to_mnemonic(entropy)
        seed       = self.mnemonic_to_seed(mnemonic, passphrase)
        priv_key   = self.seed_to_private_key(seed)
        pub_key    = self.private_to_public_key(priv_key)
        address    = self.public_key_to_address(pub_key)
        return {
            "address":     address,
            "public_key":  pub_key,
            "private_key": priv_key,
            "seed_phrase": mnemonic,
            "entropy_bits": 256,
            "word_count":  len(mnemonic),
            "standard":    "ATC-8300 / BIP39-kompatibel",
            "chain":       "A-TownChain"
        }

    def restore_from_mnemonic(self, mnemonic: list[str], passphrase: str = "A-TownChain") -> dict:
        """Stellt ein Wallet aus der Seed Phrase wieder her."""
        seed     = self.mnemonic_to_seed(mnemonic, passphrase)
        priv_key = self.seed_to_private_key(seed)
        pub_key  = self.private_to_public_key(priv_key)
        address  = self.public_key_to_address(pub_key)
        return {
            "address":    address,
            "public_key": pub_key,
            "private_key": priv_key,
            "restored":   True
        }

    def validate_address(self, address: str) -> bool:
        """Prüft ob eine ATC-Adresse gültig ist."""
        if not address.startswith("ATC"):
            return False
        if len(address) != 35:
            return False
        # Nur Hex-Zeichen nach ATC erlaubt
        try:
            int(address[3:], 16)
            return True
        except ValueError:
            return False
