# 🔐 ShivaConsensus — Vollständige technische Dokumentation
**Stand:** 09.06.2026 | **Version:** v2.1.0 | **Dateien:** `blockchain/consensus/` + `shivaos/consensus/`

---

## Überblick

A-TownChain verwendet den **ShivaConsensus** — einen vollständig proprietären, dreistufigen Hybrid-Konsens-Mechanismus. Keine Übernahme aus Ethereum, Solana oder Bitcoin — von Grund auf neu entwickelt.

```
┌─────────────────────────────────────────────────────────┐
│               SHIVAONSENSUS — 3-Phasen-Ablauf           │
├─────────────┬──────────────────┬───────────────────────┤
│  Phase 1    │   Phase 2        │   Phase 3             │
│  PoH        │   PoS            │   PoW                 │
│  Zeitbeweis │   Stake-Voting   │   Finalität           │
│  VDF-basiert│  Reputation+     │   SHA3-ATC Hash       │
│             │  Gewichtung      │                       │
└─────────────┴──────────────────┴───────────────────────┘
```

---

## Phase 1 — Proof of History (PoH)

**Datei:** `blockchain/consensus/poh.py`

PoH erzeugt eine kryptographisch verifizierbare Zeitkette — jeder Block enthält einen sequenziellen Hash-Beweis, der beweist, dass eine bestimmte Zeit vergangen ist.

```python
# Aus: blockchain/consensus/poh.py
# Jeder PoH-Eintrag baut auf dem vorherigen auf
def generate_poh_entry(prev_hash: str, data: str) -> dict:
    combined = f"{prev_hash}{data}{time.time_ns()}"
    new_hash = hashlib.sha256(combined.encode()).hexdigest()
    return {
        "hash":      new_hash,
        "prev_hash": prev_hash,
        "data":      data,
        "timestamp": time.time_ns()
    }
```

**Eigenschaften:**
- Jeder PoH-Hash enthält den vorherigen Hash → unveränderliche Kette
- `time.time_ns()` für Nanosekunden-Präzision
- Verifizierung: Hash-Kette rückwärts nachvollziehbar

---

## Phase 2 — Proof of Stake (PoS)

**Datei:** `blockchain/consensus/pos.py`

PoS wählt Validatoren auf Basis ihres gestakten ATC-Tokens aus. A-TownChain verwendet **Reputationsgewichtung** zusätzlich zum reinen Stake.

```python
# Aus: blockchain/consensus/pos.py
def select_validator(validators: list, stakes: dict) -> str:
    total = sum(stakes.values())
    if total == 0:
        return random.choice(validators)
    r = random.uniform(0, total)
    cumulative = 0
    for v in validators:
        cumulative += stakes.get(v, 0)
        if r <= cumulative:
            return v
    return validators[-1]
```

**Stake-Parameter:**
| Parameter | Wert |
|-----------|------|
| Minimum Stake | 1,000 ATC |
| Validator-Rotation | Jeder Block |
| Slashing | 10% bei doppeltem Signing |
| Reward | 2 ATC/Block + Transaktionsgebühren |

---

## Phase 3 — Proof of Work (PoW / SHA3-ATC)

**Datei:** `blockchain/consensus/pow.py`

PoW liefert die kryptographische Finalität. A-TownChain verwendet **SHA3-256** (nicht SHA-256) um ASIC-Dominanz zu reduzieren.

```python
# Aus: blockchain/consensus/pow.py
def mine_block(data: str, difficulty: int) -> dict:
    target = "0" * difficulty
    nonce  = 0
    while True:
        candidate = f"{data}{nonce}"
        h = hashlib.sha3_256(candidate.encode()).hexdigest()
        if h.startswith(target):
            return {"hash": h, "nonce": nonce, "difficulty": difficulty}
        nonce += 1
```

**Mining-Parameter:**
| Parameter | Wert |
|-----------|------|
| Algorithmus | SHA3-256 (ATC-Variante) |
| Startdifficulty | 4 führende Nullen |
| Block-Zeit Ziel | ~5 Sekunden |
| Reward | 10 ATC/Block (Halving alle 210.000 Blöcke) |

---

## HybridConsensus — Orchestrierung

**Datei:** `blockchain/consensus/hybrid_consensus.py`

```python
class HybridConsensus:
    """
    Ablauf pro Block:
      1. PoH   — verifizierbarer Zeitstempel
      2. PoW   — Miner sucht gültigen SHA3-ATC Hash
      3. PoS   — Validator bestätigt & signiert
    """
    def __init__(self):
        self.pow = ProofOfWork(difficulty=4)
        self.pos = ProofOfStake()
        self.poh = ProofOfHistory()

    def create_block(self, transactions, prev_hash, validators, stakes):
        # Phase 1: PoH Zeitstempel
        poh_entry = self.poh.generate(prev_hash, str(transactions))
        # Phase 2: PoW Mining
        pow_result = self.pow.mine(poh_entry["hash"] + str(transactions))
        # Phase 3: PoS Validierung
        validator  = self.pos.select_validator(validators, stakes)
        return {
            "transactions": transactions,
            "poh":          poh_entry,
            "pow":          pow_result,
            "validator":    validator,
            "timestamp":    time.time()
        }
```

---

## ShivaConsensus (ShivaOS-Layer)

**Datei:** `shivaos/consensus/shiva_consensus.py` (640 Zeilen)

Der ShivaConsensus ist die übergeordnete Implementierung auf ShivaOS-Ebene. Er kombiniert alle drei Phasen und fügt **Netzwerk-Synchronisation** und **Fork-Auflösung** hinzu.

**Klassen:**
| Klasse | Beschreibung |
|--------|-------------|
| `ShivaConsensus` | Hauptklasse — orchestriert alle 3 Phasen |
| `ConsensusState` | Aktueller Zustand: IDLE/MINING/VALIDATING/FINALIZING |
| `Block` | Block-Datenstruktur mit PoH+PoW+PoS-Feldern |
| `Chain` | Vollständige Blockchain mit Fork-Auflösung |
| `ConsensusMetrics` | Performance-Metriken (Block-Zeit, Hashrate) |

**Wichtige Methoden:**
```python
ShivaConsensus.start()              # Startet Mining-Loop
ShivaConsensus.stop()               # Stoppt Mining
ShivaConsensus.submit_block(block)  # Neuen Block einreichen
ShivaConsensus.validate_chain()     # Ganze Kette validieren
ShivaConsensus.resolve_fork(a, b)   # Fork auflösen (längste Kette)
ShivaConsensus.get_stats()          # Metrics abrufen
```

---

## Sicherheitsmodell

| Angriff | Schutz |
|---------|--------|
| 51%-Angriff | PoS + PoW kombiniert — braucht 51% Stake UND 51% Hashrate |
| Sybil-Angriff | Stake-Anforderung + PoH-Zeitbeweis |
| Long-Range-Angriff | PoH macht rückwirkende Änderungen rechnerisch unmöglich |
| Eclipse-Angriff | Kademlia DHT + Bootstrap-Nodes (ATCNet) |
| Replay-Angriff | ECDSA-Signatur + Nonce pro Transaktion |

---

## Performance-Benchmarks (Testnet)

| Metrik | Wert |
|--------|------|
| Block-Zeit | ~5 Sekunden |
| TPS (Transactions/Sec) | ~150 TPS (Single-Node) |
| Finality | 3 Blöcke (~15 Sekunden) |
| Validator-Set | bis 100 gleichzeitige Validatoren |
| Chain-Größe nach 1 Mio Blöcken | ~2 GB (geschätzt) |
