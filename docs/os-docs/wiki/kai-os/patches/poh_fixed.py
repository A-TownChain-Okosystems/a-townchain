# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# blockchain/consensus/poh.py -- FIX: tick_n() + verify_sequence()
import hashlib, time

class ProofOfHistory:
    def __init__(self, genesis_hash: str = None):
        self.current_hash = genesis_hash or hashlib.sha256(b"A-TownChain Genesis").hexdigest()
        self.sequence = 0
        self.history  = []
        self.start_time = time.time()

    def tick(self, data: bytes = None) -> dict:
        combined = (self.current_hash + data.hex()).encode() if data else self.current_hash.encode()
        new_hash  = hashlib.sha256(combined).hexdigest()
        self.current_hash = new_hash
        self.sequence += 1
        entry = {"sequence": self.sequence, "hash": new_hash,
                 "has_data": data is not None, "elapsed": round(time.time()-self.start_time,6)}
        self.history.append(entry)
        return entry

    def tick_n(self, n: int) -> str:
        """FIX: n Ticks ausfuehren, finalen Hash zurueckgeben."""
        for _ in range(n):
            self.tick()
        return self.current_hash

    def verify_sequence(self, entries: list) -> bool:
        for i in range(1, len(entries)):
            if entries[i]["has_data"]: continue
            if entries[i]["hash"] != hashlib.sha256(entries[i-1]["hash"].encode()).hexdigest():
                return False
        return True

    def export(self) -> dict:
        return {"current_hash": self.current_hash, "sequence": self.sequence,
                "history_len": len(self.history), "elapsed": round(time.time()-self.start_time,3)}
