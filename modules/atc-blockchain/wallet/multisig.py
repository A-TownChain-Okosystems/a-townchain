# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
MultiSig-Wallet (ATC-18-MULTISIG_AUTH) — M-of-N Multisignatur-Vaults
fuer Cross-Chain-Bridge und Franchise-Treuhandkonten (Issue #24/#26).
"""
import hashlib
import time
import uuid
from dataclasses import dataclass, field
from enum import Enum
from typing import Dict, List, Optional, Set


class TxStatus(Enum):
    PENDING = "pending"
    APPROVED = "approved"
    EXECUTED = "executed"
    REJECTED = "rejected"


@dataclass
class MultiSigTx:
    id: str
    proposer: str
    target: str
    amount: float
    asset: str
    metadata: dict = field(default_factory=dict)
    signatures: Set[str] = field(default_factory=set)
    rejections: Set[str] = field(default_factory=set)
    status: TxStatus = TxStatus.PENDING
    created_at: float = field(default_factory=time.time)
    tx_hash: Optional[str] = None


class MultiSigWallet:
    """M-of-N Multisig-Vault mit Propose/Sign/Execute/Reject-Workflow."""

    def __init__(self, name: str, owners: List[str], threshold: int):
        if threshold > len(owners):
            raise ValueError("threshold darf nicht groesser als Anzahl Owner sein")
        self.name = name
        self.owners = list(owners)
        self.threshold = threshold
        self.balances: Dict[str, float] = {}
        self.txs: Dict[str, MultiSigTx] = {}

    def deposit(self, asset: str, amount: float):
        self.balances[asset] = self.balances.get(asset, 0.0) + amount

    def balance(self, asset: str) -> float:
        return self.balances.get(asset, 0.0)

    def propose(self, proposer: str, target: str, amount: float,
                asset: str = "ATC", metadata: Optional[dict] = None) -> MultiSigTx:
        if proposer not in self.owners:
            raise PermissionError(f"'{proposer}' ist kein Owner dieses Vaults")
        tx = MultiSigTx(
            id=uuid.uuid4().hex, proposer=proposer, target=target,
            amount=amount, asset=asset, metadata=metadata or {},
        )
        tx.signatures.add(proposer)  # Proposer signiert automatisch mit
        if len(tx.signatures) >= self.threshold:
            tx.status = TxStatus.APPROVED
        self.txs[tx.id] = tx
        return tx

    def sign(self, tx_id: str, signer: str) -> MultiSigTx:
        tx = self.txs[tx_id]
        if signer not in self.owners:
            raise PermissionError(f"'{signer}' ist kein Owner dieses Vaults")
        if tx.status not in (TxStatus.PENDING,):
            return tx
        tx.signatures.add(signer)
        if len(tx.signatures) >= self.threshold:
            tx.status = TxStatus.APPROVED
        return tx

    def reject(self, tx_id: str, signer: str) -> MultiSigTx:
        tx = self.txs[tx_id]
        tx.rejections.add(signer)
        tx.status = TxStatus.REJECTED
        return tx

    def execute(self, tx_id: str, executor: str) -> dict:
        tx = self.txs[tx_id]
        if tx.status != TxStatus.APPROVED:
            raise RuntimeError(f"Tx {tx_id} ist nicht APPROVED (Status: {tx.status.value})")
        if self.balance(tx.asset) < tx.amount:
            raise ValueError("Unzureichendes Guthaben")
        self.balances[tx.asset] -= tx.amount
        tx_hash = hashlib.sha256(f"{tx.id}:{executor}:{time.time()}".encode()).hexdigest()
        tx.tx_hash = tx_hash
        tx.status = TxStatus.EXECUTED
        return {"tx_hash": tx_hash, "new_balance": self.balances[tx.asset]}


def create_bridge_vault(owners: List[str]) -> MultiSigWallet:
    """Cross-Chain-Bridge-Vault: 2-of-N (Standard fuer 3 Owner)."""
    threshold = 2 if len(owners) >= 2 else len(owners)
    return MultiSigWallet("BridgeVault", owners, threshold)


def create_franchise_vault(owners: List[str]) -> MultiSigWallet:
    """Franchise-Treuhandkonto: 3-of-N (Standard fuer 5 Owner)."""
    threshold = 3 if len(owners) >= 3 else len(owners)
    return MultiSigWallet("FranchiseVault", owners, threshold)
