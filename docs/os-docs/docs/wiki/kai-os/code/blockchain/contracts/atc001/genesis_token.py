# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ATC-001 Genesis Token Contract
Issue #1: Smart Contract Implementation

Der Genesis-Token ist der erste und einzige ATC-001 Token.
Unveraenderlich, nicht mintbar — symbolisiert den Ursprung der Chain.
"""
import hashlib, time
from blockchain.contracts.base.base_contract import BaseContract


class GenesisToken(BaseContract):
    """
    ATC-001 Genesis Token — einmaliger Ursprungs-Token.
    Nicht mintbar, nicht brennbar. Nur transferierbar vom Owner.
    """

    GENESIS_SUPPLY = 21_000_000.0  # 21 Mio ATC — wie Bitcoin Referenz
    SYMBOL         = "ATC-001"
    GENESIS_HASH   = hashlib.sha256(b"A-TownChain Genesis Block 2026").hexdigest()

    def __init__(self, creator: str):
        super().__init__(creator, contract_id="ATC_GENESIS_001")
        self._holder     = creator
        self._locked     = False
        self._transfer_log: list[dict] = []
        self._emit("Genesis", {
            "creator": creator,
            "supply":  self.GENESIS_SUPPLY,
            "hash":    self.GENESIS_HASH,
            "ts":      int(time.time()),
        })

    def name(self) -> str:
        return "A-TownChain Genesis Token"

    def holder(self) -> str:
        return self._holder

    def supply(self) -> float:
        return self.GENESIS_SUPPLY

    def lock(self, caller: str):
        """Sperrt den Transfer dauerhaft (nur Owner)."""
        self.only_owner(caller)
        self._locked = True
        self._emit("Locked", {"by": caller})

    def transfer(self, caller: str, new_holder: str) -> dict:
        """Einmaliger Transfer an neuen Hüter."""
        self.only_owner(caller)
        if self._locked:
            raise RuntimeError("Genesis Token is permanently locked")
        old = self._holder
        self._holder = new_holder
        self.owner   = new_holder  # Neuer Owner = neuer Hüter
        log = {"from": old, "to": new_holder, "ts": int(time.time())}
        self._transfer_log.append(log)
        self._emit("GenesisTransfer", log)
        return {"success": True, **log}

    def provenance(self) -> list:
        return self._transfer_log

    def verify(self) -> dict:
        return {
            "valid":        True,
            "symbol":       self.SYMBOL,
            "genesis_hash": self.GENESIS_HASH,
            "supply":       self.GENESIS_SUPPLY,
            "holder":       self._holder,
            "locked":       self._locked,
        }
