# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Unit Tests — Smart Contracts Issue #1
ATC-8300 Token + ATC-001 Genesis Token + Registry
"""
import unittest
from blockchain.contracts.atc8300.atc8300_token import ATC8300Token
from blockchain.contracts.atc001.genesis_token import GenesisToken
from blockchain.smart_contract_registry import SmartContractRegistry


class TestATC8300(unittest.TestCase):

    def setUp(self):
        self.owner = "ATC_OWNER_001"
        self.token = ATC8300Token(
            self.owner, "ATCoin", "ATC", 8, initial_supply=1_000_000.0
        )

    def test_initial_supply(self):
        self.assertEqual(self.token.total_supply(), 1_000_000.0)
        self.assertEqual(self.token.balance_of(self.owner), 1_000_000.0)

    def test_mint(self):
        self.token.mint(self.owner, "ATC_USER_001", 500.0)
        self.assertEqual(self.token.balance_of("ATC_USER_001"), 500.0)
        self.assertEqual(self.token.total_supply(), 1_000_500.0)

    def test_burn(self):
        self.token.burn(self.owner, 100.0)
        self.assertEqual(self.token.total_supply(), 1_000_000.0 - 100.0)

    def test_transfer(self):
        self.token.transfer(self.owner, "ATC_USER_002", 1000.0)
        self.assertEqual(self.token.balance_of("ATC_USER_002"), 1000.0)

    def test_transfer_insufficient_funds(self):
        with self.assertRaises(ValueError):
            self.token.transfer("ATC_BROKE", "ATC_USER", 999.0)

    def test_pause(self):
        self.token.pause(self.owner)
        with self.assertRaises(RuntimeError):
            self.token.transfer(self.owner, "ATC_USER", 1.0)
        self.token.unpause(self.owner)

    def test_approve_and_transfer_from(self):
        self.token.approve(self.owner, "ATC_SPENDER", 200.0)
        self.assertEqual(self.token.allowance(self.owner, "ATC_SPENDER"), 200.0)
        self.token.transfer_from("ATC_SPENDER", self.owner, "ATC_RECV", 100.0)
        self.assertEqual(self.token.balance_of("ATC_RECV"), 100.0)

    def test_snapshot(self):
        snap = self.token.snapshot(self.owner)
        self.assertEqual(snap["id"], 1)
        self.assertIn(self.owner, snap["balances"])

    def test_only_owner_mint(self):
        with self.assertRaises(PermissionError):
            self.token.mint("HACKER", "HACKER", 9999.0)


class TestGenesisToken(unittest.TestCase):

    def setUp(self):
        self.creator = "ATC_CREATOR"
        self.genesis = GenesisToken(self.creator)

    def test_initial_state(self):
        self.assertEqual(self.genesis.holder(), self.creator)
        self.assertEqual(self.genesis.supply(), 21_000_000.0)

    def test_verify(self):
        v = self.genesis.verify()
        self.assertTrue(v["valid"])
        self.assertEqual(v["symbol"], "ATC-001")

    def test_transfer(self):
        result = self.genesis.transfer(self.creator, "ATC_NEW_OWNER")
        self.assertEqual(result["to"], "ATC_NEW_OWNER")
        self.assertEqual(self.genesis.holder(), "ATC_NEW_OWNER")

    def test_lock(self):
        self.genesis.lock(self.creator)
        with self.assertRaises(RuntimeError):
            self.genesis.transfer(self.creator, "ANYONE")

    def test_provenance(self):
        self.genesis.transfer(self.creator, "ATC_HOLDER_2")
        self.assertEqual(len(self.genesis.provenance()), 1)


class TestRegistry(unittest.TestCase):

    def setUp(self):
        self.reg   = SmartContractRegistry()
        self.token = ATC8300Token("ATC_OWNER", initial_supply=500.0)
        self.reg.deploy(self.token, "ATC_OWNER")

    def test_deploy_and_get(self):
        contracts = self.reg.list_all()
        self.assertEqual(len(contracts), 1)

    def test_call(self):
        bal = self.reg.call(self.token.address, "balance_of", "ATC_OWNER")
        self.assertEqual(bal, 500.0)

    def test_get_nonexistent(self):
        with self.assertRaises(KeyError):
            self.reg.get("NONEXISTENT")


if __name__ == "__main__":
    unittest.main(verbosity=2)
