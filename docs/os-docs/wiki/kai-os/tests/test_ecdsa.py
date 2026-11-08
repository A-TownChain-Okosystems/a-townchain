# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Unit Tests — ECDSA Signatur (Issue #6)
Coverage: generate_keypair, sign, verify, build_tx, Angriffs-Szenarien
"""
import unittest, time
from blockchain.wallet.ecdsa import ECDSASigner


class TestECDSA(unittest.TestCase):

    def setUp(self):
        self.priv, self.pub = ECDSASigner.generate_keypair()
        self.tx = ECDSASigner.build_tx(
            "ATC7F3A9B2C1D4E5F", "ATC9B2C1D4E5F6A7B", 150.0)

    # ── Keypair ─────────────────────────────────────
    def test_keypair_length(self):
        self.assertEqual(len(self.priv), 64)    # 32 Bytes hex
        self.assertEqual(len(self.pub),  130)   # 65 Bytes uncompressed hex

    def test_keypair_unique(self):
        priv2, pub2 = ECDSASigner.generate_keypair()
        self.assertNotEqual(self.priv, priv2)
        self.assertNotEqual(self.pub, pub2)

    # ── Sign & Verify ────────────────────────────────
    def test_sign_and_verify_valid(self):
        sig = ECDSASigner.sign(self.tx, self.priv)
        self.assertTrue(ECDSASigner.verify(self.tx, sig, self.pub))

    def test_verify_tampered_amount(self):
        sig = ECDSASigner.sign(self.tx, self.priv)
        bad_tx = {**self.tx, "amount": 999999.0}
        self.assertFalse(ECDSASigner.verify(bad_tx, sig, self.pub))

    def test_verify_wrong_public_key(self):
        sig = ECDSASigner.sign(self.tx, self.priv)
        _, other_pub = ECDSASigner.generate_keypair()
        self.assertFalse(ECDSASigner.verify(self.tx, sig, other_pub))

    def test_verify_replay_attack(self):
        sig = ECDSASigner.sign(self.tx, self.priv)
        replay_tx = {**self.tx, "nonce": self.tx["nonce"] + 1}
        self.assertFalse(ECDSASigner.verify(replay_tx, sig, self.pub))

    def test_verify_invalid_signature_hex(self):
        self.assertFalse(ECDSASigner.verify(self.tx, "deadbeef" * 9, self.pub))

    # ── build_tx ─────────────────────────────────────
    def test_build_tx_fields(self):
        self.assertIn("nonce",     self.tx)
        self.assertIn("timestamp", self.tx)
        self.assertEqual(self.tx["amount"], 150.0)
        self.assertEqual(self.tx["fee"],    0.001)

    def test_build_tx_nonce_unique(self):
        tx1 = ECDSASigner.build_tx("ATC_A", "ATC_B", 1.0)
        time.sleep(0.01)
        tx2 = ECDSASigner.build_tx("ATC_A", "ATC_B", 1.0)
        self.assertNotEqual(tx1["nonce"], tx2["nonce"])


if __name__ == "__main__":
    unittest.main(verbosity=2)
