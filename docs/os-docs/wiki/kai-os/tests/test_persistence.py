# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Unit Tests — SQLite Persistenz (Issue #4)
"""
import unittest, os, time, tempfile

# Test-DB in temporaerem Verzeichnis
os.environ["ATC_DB_PATH"] = os.path.join(tempfile.gettempdir(), "atc_test.db")

from backend.db.repository import (
    WalletRepository, ShivamonRepository,
    TransactionRepository, BlockRepository
)


class TestWalletRepo(unittest.TestCase):
    def setUp(self):
        self.repo = WalletRepository()
        self.addr = f"ATC_TEST_{int(time.time())}"

    def test_save_and_find(self):
        self.repo.save(self.addr, "pub_key_hex", "Test Wallet", 500.0)
        w = self.repo.find(self.addr)
        self.assertIsNotNone(w)
        self.assertEqual(w["balance"], 500.0)

    def test_update_balance(self):
        self.repo.save(self.addr, "pub_key_hex", balance=100.0)
        self.repo.update_balance(self.addr, 250.0)
        self.assertEqual(self.repo.find(self.addr)["balance"], 250.0)

    def test_find_nonexistent(self):
        self.assertIsNone(self.repo.find("ATC_NONEXISTENT"))


class TestShivamonRepo(unittest.TestCase):
    def setUp(self):
        self.repo = ShivamonRepository()
        self.nft  = {
            "token_id": f"SHV-{int(time.time())}",
            "name": "Ignarex", "element": "Fire", "rarity": "Rare",
            "owner": "ATC_OWNER_001", "generation": 1, "level": 3,
            "xp": 120, "wins": 5, "losses": 2,
            "hp": 120, "attack": 95, "defense": 80, "speed": 70, "special": 85,
            "dna_hash": "abc123def456", "moves": ["Flame", "Ember"],
            "minted_at": int(time.time())
        }

    def test_save_and_find(self):
        self.repo.save(self.nft)
        found = self.repo.find(self.nft["token_id"])
        self.assertIsNotNone(found)
        self.assertEqual(found["name"], "Ignarex")
        self.assertIsInstance(found["moves"], list)

    def test_find_by_owner(self):
        self.repo.save(self.nft)
        collection = self.repo.find_by_owner("ATC_OWNER_001")
        self.assertGreater(len(collection), 0)

    def test_update_level(self):
        self.repo.save(self.nft)
        self.repo.update(self.nft["token_id"], {"level": 5, "xp": 300})
        updated = self.repo.find(self.nft["token_id"])
        self.assertEqual(updated["level"], 5)


class TestBlockRepo(unittest.TestCase):
    def setUp(self):
        self.repo = BlockRepository()

    def test_save_and_height(self):
        block = {
            "height": int(time.time()) % 100000,
            "hash": f"0000{os.urandom(14).hex()}",
            "prev_hash": "0" * 32,
            "miner": "ATC_MINER", "validator": "ATC_VAL",
            "reward": 50.0, "difficulty": 3, "nonce": 42,
            "poh_hash": "poh_abc", "timestamp": int(time.time()),
            "transactions": []
        }
        self.repo.save(block)
        self.assertGreater(self.repo.count(), 0)


if __name__ == "__main__":
    unittest.main(verbosity=2)
