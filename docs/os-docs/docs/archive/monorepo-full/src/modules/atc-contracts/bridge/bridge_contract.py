# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Bridge Contract — ATC Cross-Chain Bridge
ATC-5000 Standard: Lock-Mint / Burn-Release Schema
Unterstützt: ATC ↔ ETH, ATC ↔ BSC, ATC ↔ Polygon
"""
import hashlib, time, json
from dataclasses import dataclass, field, asdict
from typing import Dict, List, Optional
from enum import Enum

class ChainId(Enum):
    ATC     = "atc-mainnet-1"
    ETH     = "eth-mainnet"
    BSC     = "bsc-mainnet"
    POLYGON = "polygon-mainnet"

class BridgeTxStatus(Enum):
    PENDING   = "pending"
    LOCKED    = "locked"
    MINTED    = "minted"
    RELEASED  = "released"
    FAILED    = "failed"
    REFUNDED  = "refunded"

@dataclass
class BridgeTx:
    id:          str
    from_chain:  ChainId
    to_chain:    ChainId
    from_addr:   str
    to_addr:     str
    amount:      float
    token:       str
    status:      BridgeTxStatus = BridgeTxStatus.PENDING
    created:     float = field(default_factory=time.time)
    completed:   Optional[float] = None
    fee:         float = 0.0
    tx_hash:     Optional[str] = None

    def to_dict(self): return asdict(self)

class BridgeContract:
    """
    ATC Cross-Chain Bridge.
    Protokoll: Lock (ATC) → Event → Mint (Ziel-Chain)
               Burn (Ziel-Chain) → Event → Release (ATC)
    """
    BRIDGE_FEE   = 0.001   # 0.1%
    MIN_AMOUNT   = 10.0
    MAX_AMOUNT   = 1_000_000.0
    SUPPORTED    = {ChainId.ETH, ChainId.BSC, ChainId.POLYGON}

    def __init__(self, owner: str):
        self.owner      = owner
        self._txs: Dict[str, BridgeTx] = {}
        self._locked: Dict[str, float] = {}   # addr → locked amount
        self._paused    = False

    def _tx_id(self, from_addr, to_addr, amount) -> str:
        raw = f"{from_addr}{to_addr}{amount}{time.time()}"
        return hashlib.sha256(raw.encode()).hexdigest()[:24]

    def lock_and_bridge(self, from_addr: str, to_addr: str,
                        amount: float, to_chain: ChainId,
                        token: str = "ATC") -> BridgeTx:
        if self._paused:
            raise RuntimeError("Bridge ist pausiert")
        if to_chain not in self.SUPPORTED:
            raise ValueError(f"Chain {to_chain} nicht unterstützt")
        if not (self.MIN_AMOUNT <= amount <= self.MAX_AMOUNT):
            raise ValueError(f"Betrag muss zwischen {self.MIN_AMOUNT} und {self.MAX_AMOUNT} liegen")

        fee = amount * self.BRIDGE_FEE
        net = amount - fee
        tx_id = self._tx_id(from_addr, to_addr, amount)
        tx = BridgeTx(
            id=tx_id, from_chain=ChainId.ATC, to_chain=to_chain,
            from_addr=from_addr, to_addr=to_addr,
            amount=net, token=token, fee=fee,
            status=BridgeTxStatus.LOCKED,
        )
        self._txs[tx_id]        = tx
        self._locked[from_addr] = self._locked.get(from_addr, 0) + amount
        return tx

    def confirm_mint(self, tx_id: str, tx_hash: str) -> bool:
        tx = self._txs.get(tx_id)
        if not tx or tx.status != BridgeTxStatus.LOCKED:
            return False
        tx.status    = BridgeTxStatus.MINTED
        tx.tx_hash   = tx_hash
        tx.completed = time.time()
        return True

    def burn_and_release(self, tx_id: str, from_addr: str) -> bool:
        tx = self._txs.get(tx_id)
        if not tx or tx.status != BridgeTxStatus.MINTED:
            return False
        tx.status    = BridgeTxStatus.RELEASED
        tx.completed = time.time()
        locked = self._locked.get(from_addr, 0)
        self._locked[from_addr] = max(0, locked - (tx.amount + tx.fee))
        return True

    def get_tx(self, tx_id: str) -> Optional[BridgeTx]:
        return self._txs.get(tx_id)

    def list_txs(self, addr: Optional[str] = None) -> List[BridgeTx]:
        txs = list(self._txs.values())
        if addr:
            txs = [t for t in txs if t.from_addr == addr or t.to_addr == addr]
        return sorted(txs, key=lambda t: t.created, reverse=True)

    def pause(self, by: str):
        if by != self.owner: raise PermissionError("Nur Owner kann pausieren")
        self._paused = True

    def resume(self, by: str):
        if by != self.owner: raise PermissionError("Nur Owner kann fortsetzen")
        self._paused = False

    def stats(self) -> dict:
        txs = list(self._txs.values())
        return {
            "total":    len(txs),
            "locked":   sum(1 for t in txs if t.status == BridgeTxStatus.LOCKED),
            "minted":   sum(1 for t in txs if t.status == BridgeTxStatus.MINTED),
            "released": sum(1 for t in txs if t.status == BridgeTxStatus.RELEASED),
            "volume":   sum(t.amount for t in txs),
            "fees":     sum(t.fee for t in txs),
            "paused":   self._paused,
        }
