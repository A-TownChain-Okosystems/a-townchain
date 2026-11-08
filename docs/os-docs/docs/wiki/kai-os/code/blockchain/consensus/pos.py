# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# blockchain/consensus/pos.py
# Proof of Stake — Validator Selection
#
# A-TownChain PoS:
#   - Validatoren sperren ATC als Stake
#   - Auswahl proportional zum Stake (weighted random)
#   - Minimum Stake: 10.000 ATC
#   - Slashing bei Fehlverhalten

import random, time, hashlib

MIN_STAKE = 10_000  # ATC

class ProofOfStake:

    def __init__(self):
        self.validators = {}  # address → stake_amount
        self.slashed    = set()

    def stake(self, address: str, amount: float) -> dict:
        if amount < MIN_STAKE:
            return {"success": False, "error": f"Minimum stake is {MIN_STAKE} ATC"}
        self.validators[address] = self.validators.get(address, 0) + amount
        return {"success": True, "address": address, "total_stake": self.validators[address]}

    def unstake(self, address: str, amount: float) -> dict:
        current = self.validators.get(address, 0)
        if amount > current:
            return {"success": False, "error": "Insufficient stake"}
        self.validators[address] = current - amount
        if self.validators[address] == 0:
            del self.validators[address]
        return {"success": True, "remaining_stake": self.validators.get(address, 0)}

    def select_validator(self, seed: str = None) -> str:
        """Wählt einen Validator proportional zum Stake."""
        if not self.validators:
            return None
        addresses = list(self.validators.keys())
        weights   = [self.validators[a] for a in addresses]
        total     = sum(weights)
        # Deterministisch via Seed (Block-Hash)
        if seed:
            rng = random.Random(int(hashlib.sha256(seed.encode()).hexdigest(), 16))
        else:
            rng = random.Random()
        r, cumulative = rng.uniform(0, total), 0
        for addr, weight in zip(addresses, weights):
            cumulative += weight
            if r <= cumulative:
                return addr
        return addresses[-1]

    def slash(self, address: str, reason: str = "double_sign") -> dict:
        """Bestraft einen Validator (50% Stake verlieren)."""
        if address not in self.validators:
            return {"success": False, "error": "Validator not found"}
        penalty = self.validators[address] * 0.5
        self.validators[address] -= penalty
        self.slashed.add(address)
        return {"success": True, "slashed": penalty, "reason": reason}

    def get_stats(self) -> dict:
        return {
            "total_validators": len(self.validators),
            "total_staked":     sum(self.validators.values()),
            "min_stake":        MIN_STAKE,
            "slashed_count":    len(self.slashed)
        }
