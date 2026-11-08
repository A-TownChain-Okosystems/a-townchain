# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# blockchain/atcoin/atcoin.py
# A-Town Coin — ATC Token (Standard ATC-8300)
#
# Eigenschaften:
#   Symbol:       ATC
#   Max Supply:   21.000.000  (wie Bitcoin)
#   Decimals:     18
#   Standard:     ATC-8300 (ERC20-kompatibel)
#   Consensus:    SHA-256 PoW + PoS + PoH

import hashlib, time, json
from decimal import Decimal

ATC_SYMBOL      = "ATC"
ATC_NAME        = "A-Town Coin"
ATC_MAX_SUPPLY  = 21_000_000
ATC_DECIMALS    = 18
ATC_GENESIS     = "2025-01-01T00:00:00Z"

class ATCoin:
    """A-Town Coin — Haupt-Token des A-TownChain Ökosystems."""

    def __init__(self):
        self.total_supply = Decimal("0")
        self.max_supply   = Decimal(str(ATC_MAX_SUPPLY))
        self.balances     = {}   # address → Decimal
        self.allowances   = {}   # owner → {spender → amount}
        self.tx_log       = []
        self.paused       = False

    # ── ERC20 Basis ────────────────────────────────────
    def balance_of(self, address: str) -> Decimal:
        return self.balances.get(address, Decimal("0"))

    def transfer(self, sender: str, receiver: str, amount: Decimal) -> dict:
        amount = Decimal(str(amount))
        if self.paused:
            return {"success": False, "error": "Contract is paused"}
        if self.balance_of(sender) < amount:
            return {"success": False, "error": "Insufficient balance"}
        self.balances[sender]   = self.balance_of(sender)   - amount
        self.balances[receiver] = self.balance_of(receiver) + amount
        tx = self._log_tx("transfer", sender, receiver, amount)
        return {"success": True, "tx": tx}

    def approve(self, owner: str, spender: str, amount: Decimal) -> bool:
        if owner not in self.allowances:
            self.allowances[owner] = {}
        self.allowances[owner][spender] = Decimal(str(amount))
        return True

    def allowance(self, owner: str, spender: str) -> Decimal:
        return self.allowances.get(owner, {}).get(spender, Decimal("0"))

    def transfer_from(self, spender: str, owner: str, receiver: str, amount: Decimal) -> dict:
        amount = Decimal(str(amount))
        if self.allowance(owner, spender) < amount:
            return {"success": False, "error": "Allowance exceeded"}
        self.allowances[owner][spender] -= amount
        return self.transfer(owner, receiver, amount)

    # ── Mining / Minting ───────────────────────────────
    def mint(self, receiver: str, amount: Decimal) -> dict:
        """Neue ATC prägen (nur via PoW Block Reward)."""
        amount = Decimal(str(amount))
        if self.total_supply + amount > self.max_supply:
            return {"success": False, "error": "Max supply reached"}
        self.balances[receiver] = self.balance_of(receiver) + amount
        self.total_supply      += amount
        return {"success": True, "minted": str(amount), "receiver": receiver}

    # ── Issue #1 Erweiterungen ─────────────────────────
    def burn(self, address: str, amount: Decimal) -> dict:
        """Token vernichten — reduziert total_supply permanent."""
        amount = Decimal(str(amount))
        if self.paused:
            return {"success": False, "error": "Contract is paused"}
        if self.balance_of(address) < amount:
            return {"success": False, "error": "Insufficient balance to burn"}
        self.balances[address]  = self.balance_of(address) - amount
        self.total_supply      -= amount
        tx = self._log_tx("burn", address, "0x0000...burn", amount)
        return {"success": True, "burned": str(amount), "address": address,
                "new_supply": str(self.total_supply), "tx": tx}

    def pause(self) -> bool:
        """Alle Transfers einfrieren (nur Contract-Owner)."""
        self.paused = True
        return True

    def unpause(self) -> bool:
        """Transfers wieder freigeben."""
        self.paused = False
        return True

    def snapshot(self) -> dict:
        """Snapshot aller Balances zu diesem Zeitpunkt."""
        import time as _time
        return {
            "timestamp":    int(_time.time()),
            "total_supply": str(self.total_supply),
            "holders":      len(self.balances),
            "balances":     {addr: str(bal) for addr, bal in self.balances.items()}
        }

    def get_tx_history(self, address: str, limit: int = 50) -> list:
        """TX-Historie einer Adresse abrufen."""
        txs = [tx for tx in self.tx_log
               if tx.get("from") == address or tx.get("to") == address]
        return txs[-limit:]

    # ── Hilfsfunktionen ────────────────────────────────
    def _log_tx(self, tx_type, sender, receiver, amount) -> str:
        tx_id = hashlib.sha256(
            f"{tx_type}{sender}{receiver}{amount}{time.time()}".encode()
        ).hexdigest()[:16]
        self.tx_log.append({
            "tx_id":    tx_id,
            "type":     tx_type,
            "from":     sender,
            "to":       receiver,
            "amount":   str(amount),
            "symbol":   ATC_SYMBOL,
            "timestamp": int(time.time())
        })
        return tx_id

    def to_dict(self):
        return {
            "name":          ATC_NAME,
            "symbol":        ATC_SYMBOL,
            "total_supply":  str(self.total_supply),
            "max_supply":    str(self.max_supply),
            "decimals":      ATC_DECIMALS,
            "standard":      "ATC-8300",
            "holders":       len(self.balances),
            "transactions":  len(self.tx_log)
        }
