# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ShivaConsensus — Proprietärer Hybrid-Konsensus-Algorithmus
A-TownChain Ökosystem | Version: 1.0.0-alpha
Autor: ShivaCore

Kein Klon von PoW/PoS/PoH — eigene Implementierung:
  Phase 1: PoH  — Proof of History   (VDF-basierte Zeitkette)
  Phase 2: PoS  — Proof of Stake     (Validator-Voting, Stake-gewichtet)
  Phase 3: PoW  — Proof of Work      (Finaler SHA3-ATC Seal)

Reihenfolge: PoH → PoS → PoW = ShivaConsensus™
"""

import hashlib
import time
import math
import json
import struct
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Tuple
from enum import IntEnum, auto


# ══════════════════════════════════════════════════════════
#  KRYPTO-PRIMITIVE (ATC-eigene Hash-Funktion)
# ══════════════════════════════════════════════════════════

def sha3_atc(data: bytes, domain: bytes = b"atcchain_v1") -> bytes:
    """
    ATC-eigene Hash-Funktion.
    SHA3-256 mit Domain Separation — kein direkter SHA256-Klon.
    """
    h = hashlib.sha3_256()
    h.update(domain)
    h.update(b"||")
    h.update(data)
    return h.digest()


def sha3_atc_hex(data: bytes) -> str:
    return sha3_atc(data).hex()


def count_leading_zeros(hash_bytes: bytes) -> int:
    """Zählt führende Nullbits im Hash."""
    count = 0
    for byte in hash_bytes:
        if byte == 0:
            count += 8
        else:
            count += (7 - int(math.log2(byte)))
            break
    return count


# ══════════════════════════════════════════════════════════
#  DATENSTRUKTUREN
# ══════════════════════════════════════════════════════════

@dataclass
class ATCTransaction:
    tx_id:     str
    sender:    str
    receiver:  str
    amount:    int          # in nano-ATC
    nonce:     int
    data:      bytes = b""
    timestamp: int = 0
    signature: str = ""

    def __post_init__(self):
        if not self.timestamp:
            self.timestamp = int(time.time() * 1000)

    def to_bytes(self) -> bytes:
        return json.dumps({
            "id": self.tx_id, "from": self.sender, "to": self.receiver,
            "amount": self.amount, "nonce": self.nonce, "ts": self.timestamp
        }, sort_keys=True).encode()

    def hash(self) -> str:
        return sha3_atc_hex(self.to_bytes())


@dataclass
class PoHEntry:
    """Ein Eintrag in der Proof-of-History Zeitkette."""
    sequence:   int
    prev_hash:  str
    tick_hash:  str         # VDF-Output dieses Ticks
    timestamp:  int         # Mikrosekunden
    tx_hash:    str = ""    # Optional: eingebettete TX

    def to_bytes(self) -> bytes:
        return f"{self.sequence}|{self.prev_hash}|{self.timestamp}|{self.tx_hash}".encode()


@dataclass
class ValidatorVote:
    """PoS-Stimme eines Validators."""
    validator:   str        # ATC-Adresse
    block_hash:  str
    slot:        int
    stake:       int        # ATC-Stake
    timestamp:   int
    signature:   str = ""

    @property
    def weight(self) -> float:
        """Stake-gewichtete Stimmkraft (log-skaliert gegen Whale-Dominanz)."""
        return math.log1p(self.stake / 1_000_000)


@dataclass
class BlockHeader:
    """ATC-0004 konformer Block-Header."""
    version:      int
    height:       int
    timestamp:    int
    prev_hash:    str
    tx_root:      str       # Merkle-Root
    state_root:   str
    poh_hash:     str       # PoH-Checkpoint
    validator:    str       # Block-Produzent (ATC-Adresse)
    nonce:        int       # PoW-Nonce
    difficulty:   int       # Target-Schwierigkeit (Leading Zeros)
    gas_limit:    int
    gas_used:     int
    stake_total:  int       # Gesamt-Stake der Validator-Epoche

    def to_bytes(self) -> bytes:
        return json.dumps({
            "v": self.version, "h": self.height, "ts": self.timestamp,
            "prev": self.prev_hash, "txr": self.tx_root, "sr": self.state_root,
            "poh": self.poh_hash, "val": self.validator, "n": self.nonce,
            "diff": self.difficulty
        }, sort_keys=True).encode()

    def hash(self) -> str:
        return sha3_atc_hex(self.to_bytes())


@dataclass
class ATCBlock:
    header:       BlockHeader
    transactions: List[ATCTransaction] = field(default_factory=list)
    poh_entries:  List[PoHEntry]       = field(default_factory=list)
    votes:        List[ValidatorVote]  = field(default_factory=list)
    block_hash:   str = ""

    def finalize(self):
        self.block_hash = self.header.hash()
        return self


@dataclass
class ValidatorInfo:
    address:    str
    stake:      int         # ATC in nano-ATC
    active:     bool = True
    reputation: int  = 5000 # 0–10000
    blocks_produced: int = 0
    last_vote:  int  = 0

    @property
    def voting_power(self) -> float:
        """Kombinierte Stimmkraft: Stake + Reputation."""
        stake_score = math.log1p(self.stake / 1_000_000)
        rep_score   = self.reputation / 10000
        return stake_score * (0.7 + 0.3 * rep_score)


# ══════════════════════════════════════════════════════════
#  PHASE 1 — PROOF OF HISTORY (VDF-Zeitkette)
# ══════════════════════════════════════════════════════════

class ProofOfHistory:
    """
    Eigene VDF-basierte Zeitkette.
    Jeder Tick ist kryptografisch mit dem vorherigen verkettet.
    Kein Leader nötig — jeder Node kann die Kette verifizieren.
    """

    TICK_ITERATIONS = 1000  # VDF-Iterationen pro Tick (Delay-Parameter)

    def __init__(self, genesis_hash: str = ""):
        self.sequence  = 0
        self.last_hash = genesis_hash or sha3_atc_hex(b"ShivaGenesis_PoH_v1")
        self.entries:  List[PoHEntry] = []

    def _vdf_tick(self, input_hash: str, iterations: int) -> str:
        """
        Verifiable Delay Function (VDF).
        Sequenzielle Hash-Kette — kann nicht parallelisiert werden.
        """
        h = bytes.fromhex(input_hash)
        for i in range(iterations):
            h = sha3_atc(h, domain=f"poh_tick_{i}".encode())
        return h.hex()

    def tick(self, tx_hash: str = "") -> PoHEntry:
        """Einen neuen PoH-Tick erzeugen."""
        ts        = int(time.time() * 1_000_000)  # Mikrosekunden
        combined  = self.last_hash + tx_hash
        tick_hash = self._vdf_tick(combined, self.TICK_ITERATIONS)

        entry = PoHEntry(
            sequence  = self.sequence,
            prev_hash = self.last_hash,
            tick_hash = tick_hash,
            timestamp = ts,
            tx_hash   = tx_hash
        )
        self.entries.append(entry)
        self.last_hash = tick_hash
        self.sequence += 1
        return entry

    def batch_tick(self, count: int, txs: List[ATCTransaction] = None) -> List[PoHEntry]:
        """Mehrere Ticks auf einmal erzeugen."""
        result = []
        for i in range(count):
            tx_hash = txs[i].hash() if txs and i < len(txs) else ""
            result.append(self.tick(tx_hash))
        return result

    def verify_entry(self, entry: PoHEntry) -> bool:
        """Einen PoH-Eintrag verifizieren."""
        combined   = entry.prev_hash + entry.tx_hash
        recomputed = self._vdf_tick(combined, self.TICK_ITERATIONS)
        return recomputed == entry.tick_hash

    def verify_chain(self, entries: List[PoHEntry]) -> bool:
        """Die gesamte PoH-Kette verifizieren."""
        for i, entry in enumerate(entries):
            if not self.verify_entry(entry):
                return False
            if i > 0 and entry.prev_hash != entries[i-1].tick_hash:
                return False
        return True

    def get_checkpoint(self) -> str:
        """Aktueller PoH-Checkpoint-Hash."""
        return self.last_hash


# ══════════════════════════════════════════════════════════
#  PHASE 2 — PROOF OF STAKE (Validator-Voting)
# ══════════════════════════════════════════════════════════

class ProofOfStake:
    """
    Eigener PoS-Mechanismus.
    Stake-gewichtetes Voting mit Reputation-Modifikator.
    Kein Delegated-PoS-Klon — eigene Validator-Auswahl.
    """

    MIN_STAKE       = 1_000_000_000    # 1000 ATC in nano-ATC
    FINALITY_RATIO  = 2 / 3            # 2/3 + 1 für Finalität
    MAX_VALIDATORS  = 100
    EPOCH_BLOCKS    = 100              # Validator-Rotation alle 100 Blöcke

    def __init__(self):
        self.validators:   Dict[str, ValidatorInfo] = {}
        self.epoch:        int = 0
        self.votes:        Dict[str, List[ValidatorVote]] = {}  # block_hash → votes

    def register_validator(self, address: str, stake: int) -> bool:
        """Validator registrieren."""
        if stake < self.MIN_STAKE:
            return False
        self.validators[address] = ValidatorInfo(address=address, stake=stake)
        return True

    def slash(self, address: str, reason: str, penalty_pct: int = 10):
        """Validator bestrafen (Slashing)."""
        if address in self.validators:
            v = self.validators[address]
            penalty = v.stake * penalty_pct // 100
            v.stake -= penalty
            v.reputation = max(0, v.reputation - 1000)
            if v.stake < self.MIN_STAKE:
                v.active = False
            print(f"  ⚡ SLASH: {address[:12]}... | -{penalty_pct}% Stake | Grund: {reason}")

    def get_active_validators(self) -> List[ValidatorInfo]:
        """Aktive Validators nach Voting-Power sortiert."""
        active = [v for v in self.validators.values() if v.active and v.stake >= self.MIN_STAKE]
        return sorted(active, key=lambda v: v.voting_power, reverse=True)[:self.MAX_VALIDATORS]

    def select_block_producer(self, slot: int) -> Optional[ValidatorInfo]:
        """
        Block-Produzenten für diesen Slot wählen.
        Gewichtete Zufallsauswahl basierend auf Voting-Power.
        """
        validators = self.get_active_validators()
        if not validators:
            return None

        # Deterministischer Seed aus Slot-Nummer
        seed_data = f"producer_slot_{slot}_epoch_{self.epoch}".encode()
        seed_hash = sha3_atc(seed_data)
        seed_int  = int.from_bytes(seed_hash[:8], 'big')

        total_power = sum(v.voting_power for v in validators)
        target      = (seed_int / (2**64)) * total_power

        cumulative = 0
        for v in validators:
            cumulative += v.voting_power
            if cumulative >= target:
                return v
        return validators[0]

    def cast_vote(self, validator_addr: str, block_hash: str, slot: int) -> Optional[ValidatorVote]:
        """Validator-Stimme abgeben."""
        if validator_addr not in self.validators:
            return None
        v = self.validators[validator_addr]
        if not v.active:
            return None

        vote = ValidatorVote(
            validator  = validator_addr,
            block_hash = block_hash,
            slot       = slot,
            stake      = v.stake,
            timestamp  = int(time.time() * 1000)
        )

        if block_hash not in self.votes:
            self.votes[block_hash] = []
        self.votes[block_hash].append(vote)
        v.last_vote = slot
        return vote

    def check_finality(self, block_hash: str) -> Tuple[bool, float]:
        """
        Prüfen ob 2/3 + 1 Voting-Power erreicht ist.
        Gibt (is_final, ratio) zurück.
        """
        if block_hash not in self.votes:
            return False, 0.0

        validators = self.get_active_validators()
        total_power  = sum(v.voting_power for v in validators)
        voted_power  = sum(vote.weight for vote in self.votes[block_hash])
        ratio        = voted_power / total_power if total_power > 0 else 0

        return ratio >= self.FINALITY_RATIO, ratio

    def update_epoch(self, block_height: int):
        """Epoch-Update — Validator-Rotation."""
        new_epoch = block_height // self.EPOCH_BLOCKS
        if new_epoch > self.epoch:
            self.epoch = new_epoch
            self.votes.clear()
            print(f"  🔄 Epoch {self.epoch} — Validator-Rotation")


# ══════════════════════════════════════════════════════════
#  PHASE 3 — PROOF OF WORK (Finaler Seal)
# ══════════════════════════════════════════════════════════

class ProofOfWork:
    """
    Eigener PoW-Algorithmus als finaler Block-Seal.
    Leichter als Bitcoin-PoW — Hybrid-Rolle: Spam-Schutz + Finalität.
    Basis: SHA3-ATC mit Domain Separation (kein SHA256d-Klon).
    """

    BASE_DIFFICULTY  = 4       # Führende Null-Bits
    MAX_DIFFICULTY   = 24
    TARGET_TIME_MS   = 3000    # 3 Sekunden Ziel-Blockzeit
    ADJUSTMENT_WINDOW = 10     # Alle 10 Blöcke anpassen

    def __init__(self):
        self.difficulty      = self.BASE_DIFFICULTY
        self.block_times:    List[int] = []

    def _meets_target(self, hash_bytes: bytes, difficulty: int) -> bool:
        """Prüft ob Hash die Schwierigkeit erfüllt."""
        return count_leading_zeros(hash_bytes) >= difficulty

    def mine(self, header: BlockHeader, max_iterations: int = 10_000_000) -> Tuple[bool, int, str]:
        """
        Block minen — Nonce finden.
        Gibt (success, nonce, hash) zurück.
        """
        start_ts = int(time.time() * 1000)
        nonce    = 0

        while nonce < max_iterations:
            header.nonce = nonce
            raw  = header.to_bytes()
            h    = sha3_atc(raw, domain=b"atc_pow_v1")

            if self._meets_target(h, self.difficulty):
                elapsed = int(time.time() * 1000) - start_ts
                self.block_times.append(elapsed)
                return True, nonce, h.hex()

            nonce += 1

        return False, nonce, ""

    def verify_pow(self, header: BlockHeader, difficulty: int) -> bool:
        """PoW-Lösung verifizieren."""
        raw = header.to_bytes()
        h   = sha3_atc(raw, domain=b"atc_pow_v1")
        return self._meets_target(h, difficulty)

    def adjust_difficulty(self, block_height: int) -> int:
        """
        Dynamische Schwierigkeitsanpassung.
        Ziel: TARGET_TIME_MS pro Block.
        """
        if block_height % self.ADJUSTMENT_WINDOW != 0 or len(self.block_times) < self.ADJUSTMENT_WINDOW:
            return self.difficulty

        recent    = self.block_times[-self.ADJUSTMENT_WINDOW:]
        avg_time  = sum(recent) / len(recent)
        ratio     = avg_time / self.TARGET_TIME_MS

        if ratio < 0.5:
            self.difficulty = min(self.difficulty + 1, self.MAX_DIFFICULTY)
        elif ratio > 2.0:
            self.difficulty = max(self.difficulty - 1, 1)

        print(f"  ⚙️  Schwierigkeit angepasst: {self.difficulty} | ∅ Blockzeit: {avg_time:.0f}ms")
        return self.difficulty


# ══════════════════════════════════════════════════════════
#  MERKLE-TREE (eigene Implementierung)
# ══════════════════════════════════════════════════════════

class ATCMerkleTree:
    """Eigener Merkle-Tree für TX-Root Berechnung."""

    def __init__(self, items: List[bytes]):
        self.leaves = [sha3_atc(item) for item in items]

    def _hash_pair(self, a: bytes, b: bytes) -> bytes:
        return sha3_atc(a + b, domain=b"merkle_v1")

    def root(self) -> str:
        if not self.leaves:
            return sha3_atc_hex(b"empty_tree")
        nodes = self.leaves.copy()
        while len(nodes) > 1:
            if len(nodes) % 2 == 1:
                nodes.append(nodes[-1])  # Duplizieren wenn ungerade
            nodes = [self._hash_pair(nodes[i], nodes[i+1]) for i in range(0, len(nodes), 2)]
        return nodes[0].hex()


# ══════════════════════════════════════════════════════════
#  SHIVA CONSENSUS ENGINE (Haupt-Orchestrator)
# ══════════════════════════════════════════════════════════

class ShivaConsensus:
    """
    ShivaConsensus™ — Der proprietäre Hybrid-Konsensus.

    Ablauf pro Block:
    1. PoH-Ticks erzeugen (Zeitkette, beweisbare Reihenfolge)
    2. Block-Produzent per PoS wählen (Stake-gewichtet)
    3. Transaktionen einbetten + Merkle-Root berechnen
    4. Block-Header mit PoH-Checkpoint erstellen
    5. PoW-Seal berechnen (Spam-Schutz)
    6. Validator-Votes sammeln (PoS-Finalität)
    7. Bei 2/3+ Votes → Block finalisiert
    """

    BLOCK_TIME_MS   = 3000      # Ziel-Blockzeit
    POH_TICKS_PER_BLOCK = 10   # PoH-Ticks zwischen Blöcken

    def __init__(self):
        self.poh        = ProofOfHistory()
        self.pos        = ProofOfStake()
        self.pow        = ProofOfWork()
        self.chain:     List[ATCBlock]  = []
        self.mempool:   List[ATCTransaction] = []
        self.slot       = 0
        self._init_genesis()

    def _init_genesis(self):
        """Genesis-Block erzeugen."""
        genesis_header = BlockHeader(
            version     = 1,
            height      = 0,
            timestamp   = int(time.time() * 1000),
            prev_hash   = "0" * 64,
            tx_root     = sha3_atc_hex(b"genesis_txroot"),
            state_root  = sha3_atc_hex(b"genesis_stateroot"),
            poh_hash    = self.poh.get_checkpoint(),
            validator   = "ATC" + "0" * 32,
            nonce       = 0,
            difficulty  = self.pow.difficulty,
            gas_limit   = 10_000_000,
            gas_used    = 0,
            stake_total = 0
        )
        genesis = ATCBlock(header=genesis_header)
        genesis.finalize()
        self.chain.append(genesis)
        print(f"  🌱 Genesis Block | Hash: {genesis.block_hash[:16]}...")

    def add_transaction(self, tx: ATCTransaction) -> bool:
        """Transaktion in Mempool aufnehmen."""
        self.mempool.append(tx)
        return True

    def produce_block(self, validator_addr: str, max_txs: int = 100) -> Optional[ATCBlock]:
        """
        Neuen Block produzieren (ShivaConsensus Ablauf).
        """
        prev_block = self.chain[-1]
        height     = len(self.chain)
        self.slot += 1

        print(f"\n{'═'*55}")
        print(f"  ⛓  ShivaConsensus | Block #{height} | Slot {self.slot}")
        print(f"{'═'*55}")

        # ── Phase 1: PoH-Ticks ─────────────────────────────
        print(f"  [1/3] PoH — Zeitkette ({self.POH_TICKS_PER_BLOCK} Ticks)...")
        pending_txs = self.mempool[:max_txs]
        poh_entries = self.poh.batch_tick(self.POH_TICKS_PER_BLOCK, pending_txs)
        poh_checkpoint = self.poh.get_checkpoint()
        print(f"        PoH-Checkpoint: {poh_checkpoint[:16]}... | Seq: {self.poh.sequence}")

        # ── Phase 2: PoS — Block-Produzent wählen ─────────
        print(f"  [2/3] PoS — Validator-Auswahl...")
        producer = self.pos.select_block_producer(self.slot)
        active   = self.pos.get_active_validators()
        print(f"        Produzent: {validator_addr[:16]}... | Aktive Validators: {len(active)}")

        # Transaktionen auswählen + Merkle-Root
        selected_txs = self.mempool[:max_txs]
        tx_bytes     = [tx.to_bytes() for tx in selected_txs]
        merkle       = ATCMerkleTree(tx_bytes)
        tx_root      = merkle.root()

        # ── Block-Header aufbauen ──────────────────────────
        validators    = self.pos.get_active_validators()
        stake_total   = sum(v.stake for v in validators)
        header = BlockHeader(
            version     = 1,
            height      = height,
            timestamp   = int(time.time() * 1000),
            prev_hash   = prev_block.block_hash,
            tx_root     = tx_root,
            state_root  = sha3_atc_hex(f"state_{height}".encode()),
            poh_hash    = poh_checkpoint,
            validator   = validator_addr,
            nonce       = 0,
            difficulty  = self.pow.difficulty,
            gas_limit   = 10_000_000,
            gas_used    = len(selected_txs) * 21_000,
            stake_total = stake_total
        )

        # ── Phase 3: PoW-Seal ──────────────────────────────
        print(f"  [3/3] PoW — Seal (Schwierigkeit: {self.pow.difficulty} Bits)...")
        start = time.time()
        success, nonce, pow_hash = self.pow.mine(header, max_iterations=500_000)
        elapsed = (time.time() - start) * 1000

        if not success:
            print(f"        ⚠️  PoW-Timeout nach 500k Iterationen")
            nonce    = 0
            pow_hash = header.hash()

        header.nonce = nonce
        print(f"        Nonce: {nonce} | Zeit: {elapsed:.0f}ms | Hash: {pow_hash[:16]}...")

        # ── Block finalisieren ─────────────────────────────
        block = ATCBlock(
            header       = header,
            transactions = selected_txs,
            poh_entries  = poh_entries,
        )
        block.finalize()

        # ── PoS-Votes simulieren ───────────────────────────
        for v in active[:5]:  # Ersten 5 Validators stimmen ab
            vote = self.pos.cast_vote(v.address, block.block_hash, self.slot)
            if vote:
                block.votes.append(vote)

        is_final, ratio = self.pos.check_finality(block.block_hash)
        print(f"        Votes: {len(block.votes)} | Finalität: {ratio:.1%} | {'✅ FINAL' if is_final else '⏳ AUSSTEHEND'}")

        # ── Block in Chain aufnehmen ───────────────────────
        self.chain.append(block)
        self.mempool = self.mempool[max_txs:]  # Verarbeitete TXs entfernen

        # ── Schwierigkeit anpassen ─────────────────────────
        self.pow.adjust_difficulty(height)
        self.pos.update_epoch(height)

        print(f"\n  ✅ Block #{height} produziert | Hash: {block.block_hash[:20]}...")
        return block

    def verify_block(self, block: ATCBlock) -> Tuple[bool, str]:
        """Einen Block vollständig verifizieren."""
        # 1. PoH verifizieren
        if block.poh_entries and not self.poh.verify_chain(block.poh_entries):
            return False, "PoH-Kette ungültig"

        # 2. PoW verifizieren
        if not self.pow.verify_pow(block.header, block.header.difficulty):
            return False, "PoW-Seal ungültig"

        # 3. Merkle-Root verifizieren
        tx_bytes = [tx.to_bytes() for tx in block.transactions]
        merkle   = ATCMerkleTree(tx_bytes)
        if merkle.root() != block.header.tx_root:
            return False, "Merkle-Root ungültig"

        # 4. Hash-Verkettung prüfen
        if len(self.chain) > 0:
            expected_prev = self.chain[block.header.height - 1].block_hash if block.header.height > 0 else "0" * 64
            if block.header.prev_hash != expected_prev:
                return False, "Prev-Hash stimmt nicht überein"

        return True, "OK"

    def get_chain_stats(self) -> dict:
        """Chain-Statistiken."""
        return {
            "height":      len(self.chain),
            "difficulty":  self.pow.difficulty,
            "validators":  len(self.pos.get_active_validators()),
            "poh_seq":     self.poh.sequence,
            "mempool":     len(self.mempool),
            "slot":        self.slot,
        }
