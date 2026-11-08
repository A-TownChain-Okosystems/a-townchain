# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Franchise Factory — ATC-9900 DAO Standard
Dezentrales Business-Ökosystem auf A-TownChain.
Jede Franchise ist eine Mini-DAO mit eigenem Token, Governance, Vault.
"""
import hashlib, time, json, uuid
from dataclasses import dataclass, field, asdict
from typing import Dict, List, Optional
from enum import Enum, auto

class FranchiseStatus(Enum):
    PROPOSAL  = "proposal"
    ACTIVE    = "active"
    SUSPENDED = "suspended"
    DISSOLVED = "dissolved"

class RoyaltyTier(Enum):
    BRONZE   = 0.05   # 5%
    SILVER   = 0.04   # 4%
    GOLD     = 0.03   # 3%
    PLATINUM = 0.02   # 2%

@dataclass
class FranchiseVault:
    """Dezentraler Tresor für Franchise-Einnahmen."""
    balance: float = 0.0
    total_in: float = 0.0
    total_out: float = 0.0
    transactions: List[dict] = field(default_factory=list)

    def deposit(self, amount: float, from_addr: str, note: str = ""):
        self.balance  += amount
        self.total_in += amount
        self.transactions.append({
            "type": "deposit", "amount": amount,
            "from": from_addr, "note": note, "ts": time.time()
        })

    def withdraw(self, amount: float, to_addr: str, note: str = "") -> bool:
        if amount > self.balance: return False
        self.balance   -= amount
        self.total_out += amount
        self.transactions.append({
            "type": "withdraw", "amount": amount,
            "to": to_addr, "note": note, "ts": time.time()
        })
        return True

@dataclass
class Franchise:
    """Eine einzelne Franchise-Instanz."""
    id:          str
    name:        str
    owner:       str
    description: str
    token_symbol: str
    token_supply: float
    royalty_tier: RoyaltyTier = RoyaltyTier.BRONZE
    status:      FranchiseStatus = FranchiseStatus.PROPOSAL
    created:     float = field(default_factory=time.time)
    members:     Dict[str, float] = field(default_factory=dict)  # addr → stake
    vault:       FranchiseVault = field(default_factory=FranchiseVault)
    proposals:   List[dict] = field(default_factory=list)

    def add_member(self, addr: str, stake: float):
        self.members[addr] = self.members.get(addr, 0) + stake

    def total_stake(self) -> float:
        return sum(self.members.values())

    def voting_power(self, addr: str) -> float:
        total = self.total_stake()
        if total == 0: return 0
        return self.members.get(addr, 0) / total

    def distribute_revenue(self, amount: float):
        """Einnahmen proportional an Members verteilen."""
        total = self.total_stake()
        if total == 0: return {}
        self.vault.deposit(amount, "revenue", "Einnahmen")
        royalty = amount * self.royalty_tier.value
        net = amount - royalty
        dist = {}
        for addr, stake in self.members.items():
            share = (stake / total) * net
            dist[addr] = share
        return {"royalty": royalty, "net": net, "distribution": dist}

class FranchiseFactory:
    """
    Haupt-Contract der Franchise Factory.
    Erstellt und verwaltet dezentrale Franchise-DAOs.
    """

    def __init__(self):
        self._franchises: Dict[str, Franchise] = {}
        self._owner_index: Dict[str, List[str]] = {}

    def create(self, name: str, owner: str, description: str,
               token_symbol: str, token_supply: float = 1_000_000,
               royalty_tier: RoyaltyTier = RoyaltyTier.BRONZE) -> Franchise:
        fid = hashlib.sha256(f"{name}{owner}{time.time()}".encode()).hexdigest()[:16]
        f = Franchise(
            id=fid, name=name, owner=owner, description=description,
            token_symbol=token_symbol, token_supply=token_supply,
            royalty_tier=royalty_tier, status=FranchiseStatus.ACTIVE,
        )
        f.add_member(owner, token_supply * 0.2)  # 20% für Owner
        self._franchises[fid] = f
        self._owner_index.setdefault(owner, []).append(fid)
        return f

    def get(self, fid: str) -> Optional[Franchise]:
        return self._franchises.get(fid)

    def list_all(self, status: Optional[FranchiseStatus] = None) -> List[Franchise]:
        if status:
            return [f for f in self._franchises.values() if f.status == status]
        return list(self._franchises.values())

    def by_owner(self, owner: str) -> List[Franchise]:
        ids = self._owner_index.get(owner, [])
        return [self._franchises[i] for i in ids if i in self._franchises]

    def join(self, fid: str, member: str, stake: float) -> bool:
        f = self.get(fid)
        if not f or f.status != FranchiseStatus.ACTIVE: return False
        f.add_member(member, stake)
        f.vault.deposit(stake, member, "Beitritt")
        return True

    def stats(self) -> dict:
        return {
            "total":     len(self._franchises),
            "active":    sum(1 for f in self._franchises.values() if f.status == FranchiseStatus.ACTIVE),
            "total_vault": sum(f.vault.balance for f in self._franchises.values()),
        }
