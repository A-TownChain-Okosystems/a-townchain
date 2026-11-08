# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# blockchain/consensus/hybrid_consensus.py
# A-TownChain Hybrid Consensus: SHA-256 PoW + PoS + PoH
#
# Reihenfolge pro Block:
#   1. PoH   — erzeugt verifizierbaren Zeitstempel
#   2. PoW   — Miner sucht gültigen Hash (SHA-256)
#   3. PoS   — Validator bestätigt & signiert den Block

import hashlib, time, json
from blockchain.consensus.pow import ProofOfWork
from blockchain.consensus.pos import ProofOfStake
from blockchain.consensus.poh import ProofOfHistory

class HybridConsensus:
    """
    Kombinierter Konsens-Mechanismus:
      PoH → erzeugt Zeitbeweis
      PoW → Miner findet Hash (SHA-256, difficulty-adjusting)
      PoS → Validator bestätigt Block
    """

    def __init__(self, difficulty: int = 3):
        self.pow      = ProofOfWork(difficulty)
        self.pos      = ProofOfStake()
        self.poh      = ProofOfHistory()
        self.blocks   = []
        self.height   = 0

    def create_block(self, transactions: list, miner: str) -> dict:
        """Erstellt einen neuen Block durch den hybriden Konsens."""

        # ── Schritt 1: PoH Tick ────────────────────────
        poh_entry = self.poh.tick(json.dumps(transactions).encode())

        # ── Schritt 2: Block-Daten zusammenstellen ─────
        prev_hash = self.blocks[-1]["hash"] if self.blocks else "0" * 64
        block_data = {
            "height":       self.height + 1,
            "prev_hash":    prev_hash,
            "poh_hash":     poh_entry["hash"],
            "poh_sequence": poh_entry["sequence"],
            "transactions": transactions,
            "miner":        miner,
            "timestamp":    int(time.time())
        }

        # ── Schritt 3: PoW Mining ──────────────────────
        pow_result = self.pow.mine_block(block_data)
        block_data["hash"]       = pow_result["hash"]
        block_data["nonce"]      = pow_result["nonce"]
        block_data["difficulty"] = pow_result["difficulty"]

        # ── Schritt 4: PoS Validator-Bestätigung ──────
        validator = self.pos.select_validator(seed=pow_result["hash"])
        block_data["validator"]  = validator or "genesis"
        block_data["reward"]     = self.pow.get_block_reward(self.height + 1)

        # ── Finalisierung ──────────────────────────────
        self.blocks.append(block_data)
        self.height += 1

        return block_data

    def get_chain_info(self) -> dict:
        return {
            "height":     self.height,
            "algorithm":  "SHA-256 PoW + PoS + PoH",
            "difficulty": self.pow.difficulty,
            "pow":        {"difficulty": self.pow.difficulty, "target": self.pow.target},
            "pos":        self.pos.get_stats(),
            "poh":        self.poh.get_state(),
            "last_block": self.blocks[-1]["hash"][:16] + "..." if self.blocks else None
        }

    def validate_chain(self) -> bool:
        """Verifiziert die gesamte Chain."""
        for i in range(1, len(self.blocks)):
            b, prev = self.blocks[i], self.blocks[i-1]
            if b["prev_hash"] != prev["hash"]:
                return False
            if not self.pow.validate_block(
                {k: v for k, v in b.items() if k not in ("hash","nonce")},
                b["nonce"], b["hash"]
            ):
                return False
        return True
