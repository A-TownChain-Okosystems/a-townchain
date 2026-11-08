# 🏛 A-TownChain Ökosystem-Standards

> Version: 1.0.0 | Stand: 2026-06-09  
> Autoren: Michael Wroblewski, KAI-OS Agent (Superagent)  
> Lizenz: Apache 2.0

---

## Übersicht

Die A-TownChain Ökosystem-Standards (ATC-Standards und ATS-Standards) definieren verbindliche technische Protokolle, Datenstrukturen und Verhaltensregeln für alle Komponenten im KAI-OS / A-TownChain Ökosystem.

---

## ATC-Standards (A-TownChain Core Standards)

### ATC-001 — Genesis Block Standard

**Zweck:** Definiert die Struktur des unveränderlichen Genesis-Blocks.

```python
GENESIS_BLOCK = {
    "index": 0,
    "timestamp": "2026-01-01T00:00:00Z",
    "data": "KAI-OS Genesis — A-TownChain v1.0",
    "prev_hash": "0" * 64,
    "hash": "<BLAKE2b-256 des obigen>",
    "nonce": 0,
    "difficulty": 1,
    "validator": "genesis",
    "signature": None
}
```

**Regeln:**
- `prev_hash` ist immer 64 Nullen
- `index` ist immer 0
- Unveränderlich — kann nie modifiziert werden
- Alle Nodes müssen denselben Genesis-Hash haben

---

### ATC-8300 — Fungible Token Standard

**Zweck:** Standard für fungible ATC Token (vergleichbar mit ERC-20).

```python
class ATC8300Token:
    token_id: str       # Format: "ATC-8300-{uuid4}"
    name: str           # z.B. "A-TownCoin"
    symbol: str         # z.B. "ATC"
    decimals: int       # Standard: 18
    total_supply: int   # In kleinster Einheit (Zatoshi)
    owner: str          # Wallet-Adresse (ECDSA public key hash)
    balances: dict      # {address: balance}
    allowances: dict    # {owner: {spender: amount}}
```

**Pflicht-Methoden:**
- `transfer(to, amount)` → bool
- `approve(spender, amount)` → bool
- `transfer_from(from, to, amount)` → bool
- `balance_of(address)` → int
- `allowance(owner, spender)` → int
- `total_supply()` → int

**Events:**
- `Transfer(from, to, amount)`
- `Approval(owner, spender, amount)`
- `Mint(to, amount)`
- `Burn(from, amount)`

**Sicherheitsregeln:**
- Kein Overflow: max. `2^256 - 1` Einheiten
- Keine negativen Transfers
- `transfer_from` prüft Allowance vor Ausführung
- Reentrancy-Schutz: State-Update vor externen Calls

---

### ATC-9000 — NFT Standard (Shivamon)

**Zweck:** Standard für nicht-fungible Token (vergleichbar mit ERC-721).

```python
class ATC9000NFT:
    token_id: int          # Eindeutige ID (auto-increment)
    contract_address: str  # Contract-Identifier
    owner: str             # Aktuelle Wallet-Adresse
    creator: str           # Ursprünglicher Ersteller (unveränderlich)
    metadata: dict         # {name, description, image_uri, attributes}
    transfer_history: list # [{from, to, timestamp, tx_hash}]
    is_soulbound: bool     # True = nicht übertragbar (Agent-NFTs)
    generation: int        # Gen 1 = geminted, Gen 2+ = Breeding
    royalty_bps: int       # Basis-Punkte (250 = 2.5%)
```

**Pflicht-Methoden:**
- `transfer(to, token_id)` → bool
- `approve(spender, token_id)` → bool
- `owner_of(token_id)` → str
- `token_uri(token_id)` → str
- `total_supply()` → int
- `tokens_of_owner(address)` → list[int]

**Shivamon-spezifische Attribute:**
```python
SHIVAMON_ATTRIBUTES = {
    "name": str,          # "Flamara", "Aquarix", etc.
    "element": str,       # "Fire", "Water", "Earth", "Air", "Lightning"
    "level": int,         # 1–100
    "hp": int,            # 50–500
    "attack": int,        # 10–200
    "defense": int,       # 10–200
    "speed": int,         # 10–200
    "rarity": str,        # "Common", "Rare", "Epic", "Legendary"
    "generation": int,    # 1 = Genesis, 2+ = Bred
    "xp": int,            # Erfahrungspunkte
    "wins": int,          # Battle-Siege
    "losses": int,        # Battle-Niederlagen
    "image_uri": str,     # IPFS-CID
    "dna": str            # 64-Hex-String (genetischer Code)
}
```

---

### ATC-9900 — Governance Standard (DAO)

**Zweck:** Dezentrale Entscheidungsfindung für Protokoll-Änderungen.

```python
class ATC9900Proposal:
    proposal_id: int
    title: str
    description: str
    proposer: str           # Wallet-Adresse
    options: list[str]      # ["Ja", "Nein", "Enthaltung"]
    voting_start: int       # Block-Nummer
    voting_end: int         # Block-Nummer
    quorum_bps: int         # 1000 = 10% der ATC-Supply
    votes: dict             # {option: total_atc_weight}
    voters: dict            # {address: option} (verhindert Doppelabstimmung)
    status: str             # "active", "passed", "rejected", "executed"
    execution_data: bytes   # Calldata für automatische Ausführung
    timelock_blocks: int    # Wartezeit nach Annahme (Standard: 5760 = ~24h)
```

**Regeln:**
- Mindest-ATC-Balance zum Erstellen: 1.000 ATC
- Mindest-Abstimmungsdauer: 17.280 Blöcke (~72h)
- Quorum: min. 10% der zirkulierenden Supply
- Einfache Mehrheit: >50% für Annahme
- Time-Lock: 24h zwischen Annahme und Ausführung
- Veto: >33% Nein = automatisch abgelehnt
- Snapshot: Voting-Power wird bei Proposal-Erstellung eingefroren

---

## ATS-Standards (A-TownChain Technical Standards)

### ATC-98 — Block-Struktur

```python
class Block:
    index: int              # Blockhöhe
    timestamp: float        # Unix-Timestamp (UTC)
    transactions: list      # Liste von TX-Hashes
    prev_hash: str          # BLAKE2b-256 des Vorgänger-Blocks
    hash: str               # BLAKE2b-256 dieses Blocks
    nonce: int              # Proof-of-Work Nonce
    difficulty: int         # Aktuelle Mining-Schwierigkeit
    validator: str          # Wallet-Adresse des Miners/Validators
    signature: str          # ECDSA-Signatur des Validators
    merkle_root: str        # Merkle-Root aller Transaktionen
    gas_used: int           # Verbrauchtes Gas
    gas_limit: int          # Standard: 10.000.000
    poh_hash: str           # Proof-of-History Hash (BLAKE2b-256)
    poh_sequence: int       # PoH Tick-Sequenz-Nummer
    size_bytes: int         # Block-Größe in Bytes
    version: int            # Block-Version (aktuell: 2)
```

**Hash-Berechnung:**
```python
import hashlib
def calculate_hash(block):
    data = f"{block.index}{block.timestamp}{block.prev_hash}{block.merkle_root}{block.nonce}"
    return hashlib.blake2b(data.encode(), digest_size=32).hexdigest()
```

---

### ATS-002 — Transaktions-Standard

```python
class Transaction:
    tx_hash: str            # BLAKE2b-256(sender+receiver+amount+nonce+timestamp)
    sender: str             # Wallet-Adresse (44-Zeichen Base58-kodiert)
    receiver: str           # Wallet-Adresse
    amount: int             # In Zatoshi (1 ATC = 10^18 Zatoshi)
    fee: int                # Miner-Fee in Zatoshi
    nonce: int              # Sender-Nonce (verhindert Replay-Angriffe)
    timestamp: float        # Unix-Timestamp
    signature: str          # ECDSA secp256k1 Signatur
    public_key: str         # Sender-Public-Key (für Verifikation)
    data: bytes             # Optional: Smart-Contract-Calldata
    tx_type: str            # "transfer", "contract_call", "nft_mint", "stake", "unstake"
    status: str             # "pending", "confirmed", "failed"
    block_index: int        # Block in dem die TX enthalten ist (-1 = pending)
    gas_price: int          # Zatoshi pro Gas-Einheit
    gas_limit: int          # Max. Gas für diese TX
```

**Gültigkeits-Regeln:**
- Signatur muss gültig sein (secp256k1 ECDSA)
- `sender` muss ausreichend Balance haben (amount + fee)
- `nonce` muss exakt `account_nonce + 1` sein
- `timestamp` darf max. 300 Sekunden in der Zukunft liegen
- `tx_hash` muss eindeutig sein (kein Replay)
- `gas_limit` muss ≤ Block-Gas-Limit sein

---

### ATS-003 — P2P-Netzwerk-Protokoll

**Port-Belegung:**

| Port | Service | Protokoll |
|------|---------|-----------|
| 4000 | API Gateway | HTTP/REST |
| 5000 | Core Service | HTTP/REST |
| 5001 | Chain Service | HTTP/REST |
| 5002 | Wallet Service | HTTP/REST |
| 5003 | AI Service | HTTP/REST + WebSocket |
| 5004 | Game Service | HTTP/REST + WebSocket |
| 6000 | P2P Bootstrap Node | UDP |
| 6001–6099 | P2P Full Nodes | TCP/UDP |
| 8080 | Frontend Dashboard | HTTP |

**Nachrichten-Format:**
```python
P2P_MESSAGE = {
    "version": "1.0",
    "type": str,          # "block", "tx", "ping", "pong", "peers", "sync_request", "sync_response"
    "sender": str,        # Node-ID (public key hash)
    "timestamp": float,
    "payload": dict,      # Typ-spezifische Daten
    "signature": str      # ECDSA-Signatur des Senders
}
```

**Nachrichtentypen:**
- `ping` / `pong` — Heartbeat (alle 30s)
- `block` — Neuer Block broadcast
- `tx` — Neue Transaktion broadcast
- `peers` — Peer-Liste anfordern/senden
- `sync_request` — Chain-Sync anfordern (mit Start-Block-Höhe)
- `sync_response` — Chain-Daten senden (50 Blöcke pro Batch)

---

### ATS-004 — Konsens-Standard (Hybrid PoI + PoS)

**Proof-of-Importance (PoI):**
```python
IMPORTANCE_SCORE = (
    0.35 * normalized_balance +
    0.25 * normalized_tx_count +
    0.20 * normalized_tx_volume +
    0.20 * normalized_age
)
```

**Proof-of-Stake (PoS):**
- Minimum-Stake: 1.000 ATC
- Unbonding-Period: 7 Tage
- Slashing: 10% Stake bei Doppel-Signierung
- Validator-Rotation: alle 100 Blöcke

**Proof-of-History (PoH):**
```python
def tick(prev_hash: str) -> str:
    return hashlib.blake2b(prev_hash.encode(), digest_size=32).hexdigest()

def tick_n(prev_hash: str, n: int) -> str:
    h = prev_hash
    for _ in range(n):
        h = tick(h)
    return h
```

**Block-Zeit:** Ziel 3 Sekunden  
**Difficulty-Anpassung:** alle 100 Blöcke (Ziel: 3s/Block)

---

### ATS-005 — Wallet & Kryptographie-Standard

**Key-Generation:**
```python
# secp256k1 ECDSA
from cryptography.hazmat.primitives.asymmetric import ec

private_key = ec.generate_private_key(ec.SECP256K1())
public_key = private_key.public_key()

# Adress-Ableitung
import hashlib, base58
pubkey_bytes = public_key.public_bytes(...)
sha256_hash = hashlib.sha256(pubkey_bytes).digest()
ripemd160_hash = hashlib.new('ripemd160', sha256_hash).digest()
address = base58.b58encode_check(b'\x41' + ripemd160_hash).decode()
# Ergibt: 44-Zeichen Base58-Adresse
```

**BIP-39 Mnemonic:**
- 12 oder 24 Wörter (englische Wordlist)
- PBKDF2-HMAC-SHA512 (2048 Iterationen)
- Derivation Path: `m/44'/658467'/0'/0/0`

**Signatur-Format:**
- Algorithmus: secp256k1 ECDSA
- Hash: BLAKE2b-256 der Transaktion
- Format: DER-kodiert, Base64-URL-safe

---

### ATS-006 — Smart-Contract-Standard

**Contract-Registrierung:**
```python
class SmartContract:
    contract_id: str        # Format: "ATC-{type}-{hash[:8]}"
    contract_type: str      # "ATC8300", "ATC9000", "GOVERNANCE", "CUSTOM"
    owner: str              # Deployer-Adresse
    code_hash: str          # BLAKE2b-256 des Contract-Codes
    abi: dict               # Function-Signaturen + Parameter
    state: dict             # Mutable Contract State
    deployed_at: int        # Block-Nummer
    version: str            # Semantic Versioning
    is_upgradeable: bool    # Proxy-Pattern erlaubt
    admin: str              # Nur bei upgradeable Contracts
```

**Gas-Kosten:**

| Operation | Gas |
|-----------|-----|
| Transfer | 21.000 |
| Contract Deploy | 500.000 |
| Contract Call | 50.000 |
| NFT Mint | 100.000 |
| NFT Transfer | 30.000 |
| Storage (32 Bytes) | 20.000 |

---

### ATS-007 — API-Gateway-Standard

**Authentifizierung:**
```
Authorization: Bearer <api_key>
X-ATC-Signature: <ECDSA-Signatur der Request-Body>
X-ATC-Timestamp: <Unix-Timestamp> (max. 300s alt)
X-ATC-Nonce: <uuid4> (verhindert Replay)
```

**Standard-Response-Format:**
```json
{
    "success": true,
    "data": {},
    "error": null,
    "timestamp": 1749000000,
    "request_id": "uuid4",
    "version": "2.0"
}
```

**Rate-Limiting:**
- Anonym: 10 req/min
- API-Key (Basic): 100 req/min
- API-Key (Pro): 1.000 req/min
- API-Key (Node): unbegrenzt (mit Signatur-Verifikation)

**HTTP-Status-Codes:**

| Code | Bedeutung |
|------|-----------|
| 200 | Erfolg |
| 201 | Erstellt |
| 400 | Ungültige Anfrage |
| 401 | Nicht authentifiziert |
| 403 | Keine Berechtigung |
| 404 | Nicht gefunden |
| 429 | Rate-Limit überschritten |
| 500 | Server-Fehler |
| 503 | Service nicht verfügbar |

---

### ATS-008 — Fehlercode-Standard

| Code | Name | Beschreibung |
|------|------|-------------|
| E1000 | INVALID_SIGNATURE | Ungültige ECDSA-Signatur |
| E1001 | INVALID_NONCE | Nonce bereits verwendet |
| E1002 | INSUFFICIENT_BALANCE | Nicht genug ATC |
| E1003 | INVALID_ADDRESS | Ungültige Wallet-Adresse |
| E2000 | BLOCK_INVALID_HASH | Block-Hash stimmt nicht |
| E2001 | BLOCK_INVALID_PREV | Vorgänger-Hash falsch |
| E2002 | BLOCK_ALREADY_EXISTS | Block-Index bereits vorhanden |
| E2003 | CHAIN_FORK_DETECTED | Fork erkannt — Auflösung läuft |
| E3000 | CONTRACT_NOT_FOUND | Contract-ID unbekannt |
| E3001 | CONTRACT_EXECUTION_FAILED | Contract-Ausführung fehlgeschlagen |
| E3002 | CONTRACT_OUT_OF_GAS | Gas-Limit überschritten |
| E4000 | P2P_PEER_NOT_FOUND | Peer nicht erreichbar |
| E4001 | P2P_SYNC_FAILED | Synchronisation fehlgeschlagen |
| E4002 | P2P_INVALID_MESSAGE | Nachricht-Format ungültig |
| E5000 | NFT_NOT_FOUND | Token-ID unbekannt |
| E5001 | NFT_NOT_OWNER | Kein Besitzer dieses NFTs |
| E5002 | NFT_SOULBOUND | NFT ist nicht übertragbar |
| E6000 | GOVERNANCE_QUORUM_NOT_MET | Quorum nicht erreicht |
| E6001 | GOVERNANCE_VOTING_CLOSED | Abstimmung beendet |
| E6002 | GOVERNANCE_ALREADY_VOTED | Bereits abgestimmt |

---

## Versions-Historie

| Version | Datum | Änderungen |
|---------|-------|------------|
| 1.0.0 | 2026-06-09 | Initiale Version — ATC-001, ATC-8300, ATC-9000, ATC-9900, ATC-98 bis ATS-008 |

---

## Beitragende

- **Michael Wroblewski** — Architektur & Implementierung
- **Superagent (KAI-OS Agent)** — Standards-Dokumentation

---

*Dieses Dokument ist verbindlich für alle A-TownChain Ökosystem-Implementierungen.*  
*Änderungen müssen durch ATC-9900 Governance-Voting ratifiziert werden.*  
*Apache 2.0 Lizenz — 2026 A-TownChain Ökosystems*
