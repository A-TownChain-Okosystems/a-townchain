"""ATC-9900 DAO-Modell (factory.atc / factory.py-Prototyp, portiert & bereinigt).

Dezentrale Franchise-DAOs: jede Franchise ist eine Mini-DAO mit Token,
Governance, Vault und Royalty-Tiers (ATC-9900).
Quelle: docs/archive/monorepo-full/src/modules/atc-franchise (Org-Adoption 13.09.2026).
"""

from __future__ import annotations

import hashlib
import time
from dataclasses import dataclass, field
from enum import Enum


class FranchiseStatus(Enum):
    """ATC-9900 FranchiseStatus (DAO-Sicht — getrennt von AD-20 Product-Sicht)."""

    PROPOSAL = "proposal"
    ACTIVE = "active"
    SUSPENDED = "suspended"
    DISSOLVED = "dissolved"


class RoyaltyTier(Enum):
    """Royalty-Tiers (ATC-9900): anteilige Plattform-Beteiligung."""

    BRONZE = 0.05
    SILVER = 0.04
    GOLD = 0.03
    PLATINUM = 0.02


@dataclass
class FranchiseVault:
    """Dezentraler Tresor fuer Franchise-Einnahmen (Overflow-sicher geprueft)."""

    balance: float = 0.0
    total_in: float = 0.0
    total_out: float = 0.0
    transactions: list[dict] = field(default_factory=list)

    def deposit(self, amount: float, from_addr: str, note: str = "") -> None:
        if amount < 0:
            raise ValueError("deposit: negativer Betrag")
        self.balance += amount
        self.total_in += amount
        self.transactions.append(
            {
                "type": "deposit",
                "amount": amount,
                "from": from_addr,
                "note": note,
                "ts": time.time(),
            }
        )

    def withdraw(self, amount: float, to_addr: str, note: str = "") -> bool:
        if amount > self.balance:
            return False
        self.balance -= amount
        self.total_out += amount
        self.transactions.append(
            {"type": "withdraw", "amount": amount, "to": to_addr, "note": note, "ts": time.time()}
        )
        return True


@dataclass
class Franchise:
    """Eine Franchise-DAO-Instanz (ATC-9900)."""

    id: str
    name: str
    owner: str
    description: str
    token_symbol: str
    token_supply: float
    royalty_tier: RoyaltyTier = RoyaltyTier.BRONZE
    status: FranchiseStatus = FranchiseStatus.PROPOSAL
    created: float = field(default_factory=time.time)
    members: dict[str, float] = field(default_factory=dict)
    vault: FranchiseVault = field(default_factory=FranchiseVault)
    proposals: list[dict] = field(default_factory=list)

    def add_member(self, addr: str, stake: float) -> None:
        if stake < 0:
            raise ValueError("add_member: negativer Stake")
        self.members[addr] = self.members.get(addr, 0) + stake

    def total_stake(self) -> float:
        return sum(self.members.values())

    def voting_power(self, addr: str) -> float:
        total = self.total_stake()
        return 0.0 if total == 0 else self.members.get(addr, 0) / total

    def distribute_revenue(self, amount: float) -> dict:
        """Einnahmen: Royalty an Plattform, Rest proportional nach Stake."""
        if amount < 0:
            raise ValueError("distribute_revenue: negativer Betrag")
        total = self.total_stake()
        self.vault.deposit(amount, "revenue", "Einnahmen")
        royalty = amount * self.royalty_tier.value
        net = amount - royalty
        if total == 0:
            return {"royalty": royalty, "net": net, "distribution": {}}
        distribution = {addr: (stake / total) * net for addr, stake in self.members.items()}
        return {"royalty": royalty, "net": net, "distribution": distribution}


class FranchiseFactory:
    """Haupt-Contract der Franchise Factory (ATC-9900): erstellt und verwaltet Franchise-DAOs."""

    def __init__(self) -> None:
        self._franchises: dict[str, Franchise] = {}
        self._owner_index: dict[str, list[str]] = {}

    def create(
        self,
        name: str,
        owner: str,
        description: str,
        token_symbol: str,
        token_supply: float = 1_000_000,
        royalty_tier: RoyaltyTier = RoyaltyTier.BRONZE,
    ) -> Franchise:
        fid = hashlib.sha256(f"{name}{owner}{time.time()}".encode()).hexdigest()[:16]
        f = Franchise(
            id=fid,
            name=name,
            owner=owner,
            description=description,
            token_symbol=token_symbol,
            token_supply=token_supply,
            royalty_tier=royalty_tier,
            status=FranchiseStatus.ACTIVE,
        )
        f.add_member(owner, token_supply * 0.2)  # 20 % fuer Owner (ATC-9900)
        self._franchises[fid] = f
        self._owner_index.setdefault(owner, []).append(fid)
        return f

    def get(self, fid: str) -> Franchise | None:
        return self._franchises.get(fid)

    def list_all(self, status: FranchiseStatus | None = None) -> list[Franchise]:
        if status:
            return [f for f in self._franchises.values() if f.status == status]
        return list(self._franchises.values())

    def by_owner(self, owner: str) -> list[Franchise]:
        return [
            self._franchises[i] for i in self._owner_index.get(owner, []) if i in self._franchises
        ]

    def join(self, fid: str, member: str, stake: float) -> bool:
        f = self.get(fid)
        if not f or f.status != FranchiseStatus.ACTIVE or stake < 0:
            return False
        f.add_member(member, stake)
        f.vault.deposit(stake, member, "Beitritt")
        return True

    def suspend(self, fid: str) -> bool:
        f = self.get(fid)
        if not f:
            return False
        f.status = FranchiseStatus.SUSPENDED
        return True

    def stats(self) -> dict:
        return {
            "total": len(self._franchises),
            "active": sum(
                1 for f in self._franchises.values() if f.status == FranchiseStatus.ACTIVE
            ),
            "total_vault": sum(f.vault.balance for f in self._franchises.values()),
        }
