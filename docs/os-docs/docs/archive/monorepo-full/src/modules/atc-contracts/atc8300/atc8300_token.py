# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ATC-8300 Fungible Token Standard
Issue #1: Smart Contract Implementation

Features: mint, burn, transfer, pause, snapshot, allowances
"""
import time, hashlib
from blockchain.contracts.base.base_contract import BaseContract


class ATC8300Token(BaseContract):
    """
    ATC-8300 Fungible Token Standard.
    Inspiriert von ERC-20, angepasst fuer A-TownChain.
    """

    def __init__(self, owner: str, token_name: str = "ATCoin",
                 symbol: str = "ATC", decimals: int = 8,
                 initial_supply: float = 0.0):
        super().__init__(owner)
        self._name       = token_name
        self._symbol     = symbol
        self._decimals   = decimals
        self._balances: dict[str, float]             = {}
        self._allowances: dict[str, dict[str, float]] = {}
        self._snapshots: list[dict]                  = []
        self._total_supply = 0.0

        if initial_supply > 0:
            self._mint(owner, initial_supply)

    def name(self) -> str:
        return self._name

    # ── Supply ────────────────────────────────────────────
    def total_supply(self) -> float:
        return self._total_supply

    def balance_of(self, address: str) -> float:
        return self._balances.get(address, 0.0)

    # ── Mint / Burn ───────────────────────────────────────
    def mint(self, caller: str, to: str, amount: float) -> dict:
        """Nur Owner darf minten."""
        self.only_owner(caller)
        self.when_not_paused()
        return self._mint(to, amount)

    def _mint(self, to: str, amount: float) -> dict:
        if amount <= 0:
            raise ValueError("Mint amount must be > 0")
        self._balances[to]  = self._balances.get(to, 0.0) + amount
        self._total_supply += amount
        self._emit("Transfer", {"from": "0x0", "to": to, "amount": amount})
        return {"success": True, "minted": amount, "to": to,
                "total_supply": self._total_supply}

    def burn(self, caller: str, amount: float) -> dict:
        """Token vernichten — reduziert Supply."""
        self.when_not_paused()
        bal = self._balances.get(caller, 0.0)
        if bal < amount:
            raise ValueError(f"Insufficient balance: {bal} < {amount}")
        self._balances[caller] -= amount
        self._total_supply     -= amount
        self._emit("Burn", {"from": caller, "amount": amount})
        return {"success": True, "burned": amount, "caller": caller,
                "new_balance": self._balances[caller]}

    # ── Transfer ──────────────────────────────────────────
    def transfer(self, sender: str, recipient: str, amount: float,
                 fee: float = 0.001) -> dict:
        self.when_not_paused()
        total = amount + fee
        bal   = self._balances.get(sender, 0.0)
        if bal < total:
            raise ValueError(f"Insufficient funds: {bal} < {total}")
        self._balances[sender]    = bal - total
        self._balances[recipient] = self._balances.get(recipient, 0.0) + amount
        # Fee → Owner
        self._balances[self.owner] = self._balances.get(self.owner, 0.0) + fee
        self._emit("Transfer", {"from": sender, "to": recipient,
                                "amount": amount, "fee": fee})
        return {"success": True, "from": sender, "to": recipient,
                "amount": amount, "fee": fee}

    # ── Allowance (ERC-20 style) ──────────────────────────
    def approve(self, owner: str, spender: str, amount: float) -> dict:
        if owner not in self._allowances:
            self._allowances[owner] = {}
        self._allowances[owner][spender] = amount
        self._emit("Approval", {"owner": owner, "spender": spender, "amount": amount})
        return {"success": True, "owner": owner, "spender": spender, "amount": amount}

    def allowance(self, owner: str, spender: str) -> float:
        return self._allowances.get(owner, {}).get(spender, 0.0)

    def transfer_from(self, spender: str, owner: str,
                      recipient: str, amount: float) -> dict:
        self.when_not_paused()
        allowed = self.allowance(owner, spender)
        if allowed < amount:
            raise PermissionError(f"Allowance {allowed} < {amount}")
        self._allowances[owner][spender] -= amount
        return self.transfer(owner, recipient, amount)

    # ── Snapshot ──────────────────────────────────────────
    def snapshot(self, caller: str) -> dict:
        """Snapshot aller Balances fuer Governance/Dividends."""
        self.only_owner(caller)
        snap = {
            "id":        len(self._snapshots) + 1,
            "timestamp": int(time.time()),
            "balances":  dict(self._balances),
            "supply":    self._total_supply,
        }
        self._snapshots.append(snap)
        self._emit("Snapshot", {"id": snap["id"], "ts": snap["timestamp"]})
        return snap

    def get_snapshot(self, snapshot_id: int) -> dict:
        for s in self._snapshots:
            if s["id"] == snapshot_id:
                return s
        return {}
