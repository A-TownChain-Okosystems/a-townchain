# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# blockchain/consensus/poh.py
# Proof of History — Kryptographische Zeitkette
# (Inspiriert von Solana)
#
# PoH erzeugt einen verifizierbaren Zeitstempel-Beweis
# durch rekursives SHA-256 Hashing.
# Jeder Hash beweist, dass Zeit vergangen ist.

import hashlib, time

class ProofOfHistory:
    """
    Erzeugt eine kryptographische Zeitkette.
    Jeder Eintrag beweist, dass er NACH dem vorherigen erstellt wurde.
    """

    def __init__(self, genesis_hash: str = None):
        self.current_hash = genesis_hash or hashlib.sha256(b"A-TownChain Genesis").hexdigest()
        self.sequence     = 0
        self.history      = []
        self.start_time   = time.time()

    def tick(self, data: bytes = None) -> dict:
        """
        Einen PoH-Tick ausführen.
        Optional: externe Daten (z.B. Transaktion) einmischen.
        """
        if data:
            # Daten in den Hash einmischen (Transaktion beweisbar verankern)
            combined = (self.current_hash + data.hex()).encode()
        else:
            combined = self.current_hash.encode()

        new_hash      = hashlib.sha256(combined).hexdigest()
        self.current_hash = new_hash
        self.sequence    += 1

        entry = {
            "sequence": self.sequence,
            "hash":     new_hash,
            "has_data": data is not None,
            "elapsed":  round(time.time() - self.start_time, 6)
        }
        self.history.append(entry)
        return entry

    def tick_n(self, n: int) -> str:
        """Führt n Ticks aus und gibt den finalen Hash zurück."""
        for _ in range(n):
            self.tick()
        return self.current_hash

    def verify(self, start_hash: str, ticks: int, expected_hash: str) -> bool:
        """Verifiziert eine PoH-Sequenz."""
        h = start_hash
        for _ in range(ticks):
            h = hashlib.sha256(h.encode()).hexdigest()
        return h == expected_hash

    def get_state(self) -> dict:
        return {
            "sequence":     self.sequence,
            "current_hash": self.current_hash,
            "elapsed":      round(time.time() - self.start_time, 3),
            "ticks_per_sec": round(self.sequence / max(time.time() - self.start_time, 0.001))
        }
