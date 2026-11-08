# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# blockchain/consensus/pow.py
# Proof of Work — SHA-256 (wie Bitcoin)
#
# A-TownChain PoW:
#   - Ziel-Hash muss mit N Nullen beginnen (difficulty)
#   - Miner erhält Block Reward in ATC
#   - Reward halbiert sich alle 210.000 Blöcke (Halving)

import hashlib, time

INITIAL_REWARD = 50.0   # ATC pro Block
HALVING_INTERVAL = 210_000

class ProofOfWork:

    def __init__(self, difficulty: int = 4):
        self.difficulty = difficulty  # Anzahl führender Nullen im Hash
        self.target     = "0" * difficulty

    def mine_block(self, block_data: dict) -> dict:
        """Findet eine Nonce, die den Block-Hash gültig macht."""
        nonce     = 0
        start     = time.time()
        block_str = self._serialize(block_data)

        while True:
            candidate = block_str + str(nonce)
            hash_val  = hashlib.sha256(candidate.encode()).hexdigest()
            if hash_val.startswith(self.target):
                elapsed = round(time.time() - start, 3)
                return {
                    "hash":       hash_val,
                    "nonce":      nonce,
                    "difficulty": self.difficulty,
                    "time_sec":   elapsed,
                    "valid":      True
                }
            nonce += 1

    def validate_block(self, block_data: dict, nonce: int, expected_hash: str) -> bool:
        candidate = self._serialize(block_data) + str(nonce)
        computed  = hashlib.sha256(candidate.encode()).hexdigest()
        return computed == expected_hash and computed.startswith(self.target)

    def get_block_reward(self, block_height: int) -> float:
        halvings = block_height // HALVING_INTERVAL
        return INITIAL_REWARD / (2 ** halvings)

    def adjust_difficulty(self, avg_block_time: float, target_time: float = 10.0) -> int:
        """Passt die Difficulty an (Ziel: 10 Sekunden pro Block)."""
        if avg_block_time < target_time * 0.9:
            self.difficulty += 1
        elif avg_block_time > target_time * 1.1 and self.difficulty > 1:
            self.difficulty -= 1
        self.target = "0" * self.difficulty
        return self.difficulty

    def _serialize(self, data: dict) -> str:
        import json
        return json.dumps(data, sort_keys=True)
