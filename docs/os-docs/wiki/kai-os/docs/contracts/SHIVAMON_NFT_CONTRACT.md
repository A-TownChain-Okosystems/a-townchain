# 🐉 Shivamon NFT Contract — Technische Dokumentation

> **Standard:** ATC-9000 · **Chain:** A-TownChain · **Version:** 2.0.0
> **Datei:** `blockchain/contracts/shivamon/shivamon_contract.py`

---

## Inhaltsverzeichnis

1. [Überblick](#1-überblick)
2. [Architektur](#2-architektur)
3. [Datenmodell](#3-datenmodell)
4. [Enumerationen](#4-enumerationen)
5. [Klassen](#5-klassen)
6. [Contract-Methoden](#6-contract-methoden)
7. [Algorithmen](#7-algorithmen)
8. [API-Referenz](#8-api-referenz)
9. [Fehlerbehandlung](#9-fehlerbehandlung)
10. [Sicherheit](#10-sicherheit)
11. [Beispiele](#11-beispiele)
12. [Deployment](#12-deployment)

---

## 1. Überblick

Der **Shivamon NFT Contract** implementiert den **ATC-9000 Standard** — das NFT-Protokoll des A-TownChain Ökosystems. Jedes Shivamon ist ein einzigartiges, nicht-fungibles Token (NFT) mit genetisch bestimmten Eigenschaften, Kampfwerten und einer unveränderlichen DNA.

### Kernprinzipien

| Eigenschaft | Wert |
|-------------|------|
| Standard | ATC-9000 |
| Max Supply | 9.900 NFTs |
| Elemente | 7 (Fire, Water, Earth, Air, Shadow, Neon, Quantum) |
| Rarities | 6 (Common → Genesis) |
| Generationen | Unbegrenzt (Gen 1 = native) |
| DNA | SHA-256 basiert, einzigartig pro Token |
| Minting-Kosten | 10 ATC |
| Battle-System | Rundenbasiert (max. 5 Runden) |

### Abhängigkeiten

```python
import hashlib    # SHA-256 DNA-Generierung
import time       # Zeitstempel für Minting
import os         # Systemzufälligkeit
import json       # Serialisierung
import random     # Gewichtete Rarity-Auswahl
from enum import Enum
from dataclasses import dataclass, asdict
```

---

## 2. Architektur

```
ShivamonContract
│
├── ShivamonNFT          ← Einzelnes NFT-Objekt
│   ├── ShivamonStats    ← HP/ATK/DEF/SPD/SPC Werte
│   ├── Element (Enum)   ← 7 Elementtypen
│   └── Rarity (Enum)    ← 6 Seltenheitsstufen
│
├── Token Registry       ← tokens: Dict[token_id → ShivamonNFT]
├── Owner Index          ← owner_tokens: Dict[address → List[token_id]]
└── Battle Log           ← battle_log: List[Dict]

                API Layer (game_routes.py)
                        │
                   Gateway :4000
                        │
                 Frontend api.js
```

### Integration im Gesamtsystem

```
Frontend (Shivamon UI)
  └─→ api.js → POST /api/game/shivamon/mint
                    │
              Gateway :4000
                    │
          backend/api/routes/game_routes.py
                    │
          ShivamonContract.mint()
                    │
          ShivamonNFT (Objekt erstellt)
                    │
          tokens[token_id] = nft  ← persistiert im RAM
```

---

## 3. Datenmodell

### ShivamonNFT — Vollständiges Schema

```python
@dataclass
class ShivamonNFT:
    # ── Identität ──────────────────────────────────────
    token_id:   str       # "SHV-" + 12 hex chars (z.B. "SHV-A3F9B2C1D4E5")
    name:       str       # z.B. "Voltrix-0042"
    element:    Element   # Enum: FIRE, WATER, EARTH, AIR, SHADOW, NEON, QUANTUM
    rarity:     Rarity    # Enum: COMMON, UNCOMMON, RARE, EPIC, LEGENDARY, GENESIS
    owner:      str       # ATC-Adresse (35 Zeichen, beginnt mit "ATC")
    generation: int       # Generationsnummer (Standard: 1)

    # ── Progression ────────────────────────────────────
    level:      int       # Start: 1 · Max: unbegrenzt
    xp:         int       # Erfahrungspunkte (XP für Level-Up: level × 100)
    wins:       int       # Gewonnene Kämpfe
    losses:     int       # Verlorene Kämpfe

    # ── Kryptographie ──────────────────────────────────
    dna_hash:   str       # SHA-256 aus token_id + name + element + timestamp
    minted_at:  int       # Unix-Timestamp

    # ── Kampfwerte ─────────────────────────────────────
    stats:      ShivamonStats   # Generiert aus DNA-Hash
    moves:      List[str]       # 4 Angriffe (Element-spezifisch)
```

### ShivamonStats — Kampfwerte

```python
@dataclass
class ShivamonStats:
    hp:      int   # Trefferpunkte   (Basis: 25–150 × Rarity-Multiplier)
    attack:  int   # Angriffsstärke  (Basis: 20–120 × Rarity-Multiplier)
    defense: int   # Verteidigung    (Basis: 20–120 × Rarity-Multiplier)
    speed:   int   # Geschwindigkeit (Basis: 17–105 × Rarity-Multiplier)
    special: int   # Spezialwert     (Basis: 22–135 × Rarity-Multiplier)

    def total(self) -> int:
        return hp + attack + defense + speed + special
```

### JSON-Ausgabe (`.to_dict()`)

```json
{
  "token_id":    "SHV-A3F9B2C1D4E5",
  "name":        "Voltrix-0042",
  "element":     "⚡ Neon",
  "rarity":      "Rare",
  "owner":       "ATC7F3A9B2C1D4E5F6A7B8C9D0E1F2A3B4C5",
  "generation":  1,
  "level":       1,
  "xp":          0,
  "wins":        0,
  "losses":      0,
  "dna_hash":    "a3f9b2c1d4e5f6a7...",
  "stats": {
    "hp":      112,
    "attack":  88,
    "defense": 74,
    "speed":   61,
    "special": 99
  },
  "total_stats": 434,
  "moves":       ["Volt Crash", "Neon Surge", "Thunder Arc", "Plasma Beam"],
  "minted_at":   1747691880,
  "standard":    "ATC-9000"
}
```

---

## 4. Enumerationen

### Element

Bestimmt das Element des Shivamon, seine Moves und die optische Darstellung.

| Enum-Wert | Anzeige | Emoji | Moves |
|-----------|---------|-------|-------|
| `FIRE` | "🔥 Fire" | 🔥 | Flame Burst, Inferno, Ember Strike, Solar Beam |
| `WATER` | "💧 Water" | 💧 | Aqua Jet, Hydro Pump, Tidal Wave, Ice Shard |
| `EARTH` | "🌍 Earth" | 🌍 | Rock Smash, Quake, Boulder Crush, Mudslide |
| `AIR` | "💨 Air" | 💨 | Wind Slash, Tornado, Gust, Sky Dive |
| `SHADOW` | "🌑 Shadow" | 🌑 | Dark Pulse, Void Strike, Soul Drain, Eclipse |
| `NEON` | "⚡ Neon" | ⚡ | Volt Crash, Neon Surge, Thunder Arc, Plasma Beam |
| `QUANTUM` | "🌀 Quantum" | 🌀 | Phase Shift, Quantum Leap, Reality Tear, Entangle |

### Rarity

Bestimmt die Stärke der generierten Stats via `RARITY_MULTIPLIER`.

| Enum-Wert | Anzeige | Multiplier | Drop-Rate |
|-----------|---------|-----------|-----------|
| `COMMON` | "Common" | `1.0×` | 50.0% |
| `UNCOMMON` | "Uncommon" | `1.2×` | 25.0% |
| `RARE` | "Rare" | `1.5×` | 15.0% |
| `EPIC` | "Epic" | `2.0×` | 7.0% |
| `LEGENDARY` | "Legendary" | `3.0×` | 2.5% |
| `GENESIS` | "Genesis" | `5.0×` | 0.5% |

```python
RARITY_MULTIPLIER = {
    "Common": 1.0, "Uncommon": 1.2, "Rare": 1.5,
    "Epic": 2.0, "Legendary": 3.0, "Genesis": 5.0
}
```

> **Hinweis:** Ein Genesis-Shivamon hat bis zu **5× stärkere Stats** als ein Common.
> Bei einer Drop-Rate von 0.5% ist alle ~200 Mints eines zu erwarten.

---

## 5. Klassen

### `ShivamonNFT`

Repräsentiert ein einzelnes NFT-Objekt. Wird vom Contract verwaltet, **nicht direkt instanziiert**.

#### Konstruktor

```python
ShivamonNFT(
    token_id:   str,
    name:       str,
    element:    Element,
    rarity:     Rarity,
    owner:      str,
    generation: int = 1
)
```

#### Private Methoden

```python
def _generate_dna(self) -> str:
    """
    Generiert einen einzigartigen DNA-Hash via SHA-256.
    Input: token_id + name + element.value + time.time()
    Output: 64-char Hex-String
    Eigenschaft: deterministisch bei gleichem Input — aber time.time()
                 macht jeden DNA-Hash einzigartig.
    """

def _generate_stats(self) -> ShivamonStats:
    """
    Generiert Stats deterministisch aus dem DNA-Hash.
    Liest verschiedene Byte-Positionen des DNA-Hashes aus:
      hp:      Bytes  0– 7 des DNA-Integers
      attack:  Bytes  8–15
      defense: Bytes 16–23
      speed:   Bytes 24–31
      special: Bytes 32–39
    Formel: int(base * multiplier * (byte_value / 128 + 0.5))
    """

def _assign_moves(self) -> list:
    """
    Weist 4 Element-spezifische Moves zu.
    Lookup via Element-Name → Move-Pool → erste 4 Moves.
    """
```

#### Public Methoden

```python
def gain_xp(self, amount: int) -> None:
    """
    Addiert XP und prüft Level-Up.
    Level-Up Schwelle: level × 100 XP
    Bei Level-Up:
      HP      += 5
      Attack  += 3
      Defense += 3
      Speed   += 2
      Special += 4
    """

def to_dict(self) -> dict:
    """Serialisiert das NFT als JSON-kompatibles Dict."""
```

---

### `ShivamonContract`

Der Haupt-Contract. Verwaltet alle NFTs, Eigentumsrechte und Kämpfe.

#### Klassenattribute

```python
NAMES: Dict[str, List[str]]   # Element → 5 mögliche Basisnamen
MAX_SUPPLY: int = 9900         # Absolutes Minting-Limit
```

#### Instanz-Attribute

```python
self.tokens:       Dict[str, ShivamonNFT]      # token_id → NFT-Objekt
self.owner_tokens: Dict[str, List[str]]        # ATC-Adresse → [token_ids]
self.total_minted: int                         # Zähler aller geminteten NFTs
self.battle_log:   List[Dict]                  # Protokoll aller Kämpfe
```

---

## 6. Contract-Methoden

### `mint()` — NFT erstellen

```python
def mint(
    self,
    owner:      str,            # ATC-Adresse des Empfängers
    element:    str  = None,    # Optional: "Fire","Water", etc. — sonst random
    rarity:     str  = None,    # Optional: "Common","Rare", etc. — sonst gewichtet random
    generation: int  = 1        # Generationsnummer
) -> dict
```

**Ablauf:**

```
1. Prüfe: total_minted < MAX_SUPPLY (9900)
2. Element wählen (Parameter oder random aus 7)
3. Rarity wählen (Parameter oder gewichtet random via weights=[50,25,15,7,2.5,0.5])
4. Name generieren: NAMES[element][random] + "-" + zero-padded(total_minted+1, 4)
5. Token-ID: "SHV-" + SHA-256(owner + name + time.time())[:12].upper()
6. ShivamonNFT instanziieren → DNA + Stats + Moves werden automatisch generiert
7. tokens[token_id] = nft
8. owner_tokens[owner].append(token_id)
9. total_minted += 1
10. Return: {"success": True, "shivamon": nft.to_dict()}
```

**Rückgabe (Erfolg):**
```json
{
  "success": true,
  "shivamon": { ...ShivamonNFT.to_dict()... }
}
```

**Rückgabe (Fehler — Max Supply):**
```json
{
  "success": false,
  "error": "Max supply reached (9900)"
}
```

---

### `transfer()` — Eigentumsübertragung

```python
def transfer(
    self,
    token_id:  str,   # Token-ID des NFTs
    from_addr: str,   # Aktuelle Owner-Adresse
    to_addr:   str    # Empfänger-Adresse
) -> dict
```

**Ablauf:**

```
1. Prüfe: token_id existiert in tokens
2. Prüfe: nft.owner == from_addr (Ownership-Check)
3. owner_tokens[from_addr].remove(token_id)
4. nft.owner = to_addr
5. owner_tokens[to_addr].append(token_id)
6. Return: {"success": True, "token_id": ..., "new_owner": ...}
```

**Fehlerfall:**
```json
{ "success": false, "error": "Not the owner" }
```

---

### `battle()` — Kampfsystem

```python
def battle(
    self,
    attacker_id: str,   # Token-ID des Angreifers
    defender_id: str    # Token-ID des Verteidigers
) -> dict
```

**Kampfalgorithmus:**

```
Initialisierung:
  a_hp = attacker.stats.hp
  d_hp = defender.stats.hp

Pro Runde (max. 5 Runden):
  1. Attacker schlägt Defender:
     damage = max(1, attacker.attack - defender.defense/2 + random(-5, +5))
     d_hp  -= damage

  2. Falls d_hp <= 0 → Attacker gewinnt (Runde endet)

  3. Defender schlägt zurück:
     damage = max(1, defender.attack - attacker.defense/2 + random(-5, +5))
     a_hp  -= damage

  4. Falls a_hp <= 0 → Defender gewinnt (Runde endet)

Sieger: Wer nach 5 Runden mehr HP hat (a_hp > d_hp → Attacker)

Nach dem Kampf:
  winner.wins   += 1
  loser.losses  += 1
  winner.gain_xp(50 + loser.level × 10)
  battle_log.append({attacker, defender, winner, timestamp})
```

**Schadensformel:**

```
damage = max(1, ATK - DEF/2 + rand(-5, +5))
```

Beispiel: ATK=88, DEF=74 → `max(1, 88 - 37 + rand) = 46–56 Schaden`

**Rückgabe:**
```json
{
  "success":   true,
  "winner":    { ...ShivamonNFT... },
  "loser":     { ...ShivamonNFT... },
  "rounds":    [
    { "round": 1, "attacker": "Voltrix-0042", "damage": 52, "defender_hp": 60 },
    { "round": 1, "attacker": "Aquarix-0007", "damage": 31, "defender_hp": 81 },
    ...
  ],
  "xp_gained": 60
}
```

---

### `get_token()` — NFT abfragen

```python
def get_token(self, token_id: str) -> dict
```
Gibt `nft.to_dict()` zurück oder `{"error": "Not found"}`.

---

### `get_owner_tokens()` — Collection abfragen

```python
def get_owner_tokens(self, owner: str) -> list
```
Gibt eine Liste aller `to_dict()` Objekte des Owners zurück.

---

### `get_stats()` — Contract-Statistiken

```python
def get_stats(self) -> dict
```

```json
{
  "total_minted": 42,
  "max_supply":   9900,
  "remaining":    9858,
  "owners":       15,
  "battles":      7,
  "standard":     "ATC-9000"
}
```

---

## 7. Algorithmen

### 7.1 DNA-Generierung

```python
seed     = f"{token_id}{name}{element.value}{time.time()}"
dna_hash = hashlib.sha256(seed.encode()).hexdigest()
# → 64-char Hex, z.B. "a3f9b2c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1"
```

**Eigenschaften:**
- Kollisionsresistent (SHA-256, 2²⁵⁶ mögliche Hashes)
- `time.time()` als Entropy-Quelle → jeder Mint einzigartig
- Deterministisch wenn Seed bekannt → reproduzierbar für Tests

### 7.2 Stat-Generierung aus DNA

```python
dna_int = int(dna_hash, 16)   # 256-bit Integer
mult    = RARITY_MULTIPLIER[rarity.value]
base    = 50 + (generation * 5)

# Jeder Stat liest andere Bits des DNA-Integers:
hp      = int(base * mult * ((dna_int >>  0 & 0xFF) / 128 + 0.5))
attack  = int(base * mult * ((dna_int >>  8 & 0xFF) / 128 + 0.5))
defense = int(base * mult * ((dna_int >> 16 & 0xFF) / 128 + 0.5))
speed   = int(base * mult * ((dna_int >> 24 & 0xFF) / 128 + 0.5))
special = int(base * mult * ((dna_int >> 32 & 0xFF) / 128 + 0.5))
```

**Wertebereich (Generation 1):**

| Stat | Common (1.0×) | Rare (1.5×) | Legendary (3.0×) | Genesis (5.0×) |
|------|--------------|------------|-----------------|----------------|
| Min | 25 | 38 | 75 | 125 |
| Max | 150 | 225 | 450 | 750 |
| Basis | 50 | 75 | 150 | 250 |

### 7.3 Gewichtetes Rarity-Minting

```python
rarity = random.choices(
    ["Common","Uncommon","Rare","Epic","Legendary","Genesis"],
    weights=[50, 25, 15, 7, 2.5, 0.5]
)[0]
```

Statistische Erwartungswerte bei 1000 Mints:

| Rarity | Erwartet | Wahrscheinlichkeit |
|--------|----------|-------------------|
| Common | ~500 | 50.0% |
| Uncommon | ~250 | 25.0% |
| Rare | ~150 | 15.0% |
| Epic | ~70 | 7.0% |
| Legendary | ~25 | 2.5% |
| Genesis | ~5 | 0.5% |

### 7.4 Level-Up Formel

```python
XP_threshold = level * 100

while xp >= xp_threshold:
    xp    -= xp_threshold
    level += 1
    # Stat-Erhöhungen pro Level:
    hp      += 5
    attack  += 3
    defense += 3
    speed   += 2
    special += 4
```

Level 1 → 2: 100 XP · Level 2 → 3: 200 XP · Level 10 → 11: 1000 XP

---

## 8. API-Referenz

Alle Endpoints laufen über das **API Gateway (Port 4000)**.

### `GET /api/game/health`
Service-Status.
```json
{ "service": "game", "status": "online" }
```

### `GET /api/game/shivamon/stats`
Contract-Gesamtstatistiken.

### `POST /api/game/shivamon/mint`
Neues Shivamon minting.

**Request Body:**
```json
{
  "owner":      "ATC7F3A9B2C1D4E5F6A7B8C9D0E1F2A3B4C5",
  "element":    "Neon",
  "rarity":     "Rare",
  "generation": 1
}
```
Alle Felder außer `owner` sind optional.

**Response:**
```json
{
  "success": true,
  "shivamon": {
    "token_id": "SHV-A3F9B2C1D4E5",
    "name": "Voltrix-0001",
    "element": "⚡ Neon",
    "rarity": "Rare",
    "level": 1,
    "stats": { "hp": 112, "attack": 88, "defense": 74, "speed": 61, "special": 99 },
    "moves": ["Volt Crash", "Neon Surge", "Thunder Arc", "Plasma Beam"]
  }
}
```

### `GET /api/game/shivamon/{token_id}`
Einzelnes NFT abfragen.

### `GET /api/game/shivamon/owner/{address}`
Alle NFTs eines Owners.

### `POST /api/game/shivamon/transfer`
**Request Body:**
```json
{
  "token_id": "SHV-A3F9B2C1D4E5",
  "from":     "ATC...",
  "to":       "ATC..."
}
```

### `POST /api/game/shivamon/battle`
**Request Body:**
```json
{
  "attacker": "SHV-A3F9B2C1D4E5",
  "defender": "SHV-B4E8C3D2A1F6"
}
```

### `GET /api/game/shivamon/battle/log`
Letzte 20 Kämpfe.

---

## 9. Fehlerbehandlung

| Fehler | HTTP-Code | Ursache |
|--------|-----------|---------|
| `"Max supply reached (9900)"` | 400 | Alle 9900 NFTs geminted |
| `"Token not found"` | 404 | Ungültige token_id |
| `"Not the owner"` | 403 | Transfer von falschem Sender |
| `"Service unavailable"` | 503 | Game-Service nicht erreichbar |

---

## 10. Sicherheit

### Aktuelle Implementierung (v2.0 — In-Memory)

> ⚠️ Der Contract läuft aktuell **in-memory**. Bei Server-Neustart gehen alle NFT-Daten verloren. Für Production ist eine persistente Speicherung (Datenbank oder On-Chain) erforderlich.

### Sicherheitsmaßnahmen

| Maßnahme | Status | Beschreibung |
|----------|--------|-------------|
| Ownership-Check bei Transfer | ✅ | `nft.owner == from_addr` |
| Max-Supply Limit | ✅ | Hartes Limit: 9900 |
| API-Key Auth | ✅ | Via Gateway Middleware |
| Rate Limiting | ✅ | 100 Requests / 60 Sekunden |
| ECDSA Signatur | ⏳ v2.1 | Geplant für Production |
| On-Chain Persistenz | ⏳ v2.1 | Aktuell in-memory |
| Reentrancy Guard | ⏳ v2.1 | Für Solidity-Version |
| Access Control (Admin) | ⏳ v2.1 | Owner-only Mint-Pause |

---

## 11. Beispiele

### Python — Wallet + Mint + Battle

```python
from blockchain.contracts.shivamon.shivamon_contract import ShivamonContract
from blockchain.wallet.keygen import ATCKeyGenerator

# Wallets erstellen
keygen  = ATCKeyGenerator()
wallet1 = keygen.generate_wallet()
wallet2 = keygen.generate_wallet()

# Contract initialisieren
contract = ShivamonContract()

# Shivamon minting
result1 = contract.mint(owner=wallet1["address"], element="Neon",   rarity="Rare")
result2 = contract.mint(owner=wallet2["address"], element="Shadow", rarity="Epic")

shv1 = result1["shivamon"]
shv2 = result2["shivamon"]

print(f"Geminted: {shv1['name']} ({shv1['rarity']}) — Total Stats: {shv1['total_stats']}")
print(f"Geminted: {shv2['name']} ({shv2['rarity']}) — Total Stats: {shv2['total_stats']}")

# Battle!
battle = contract.battle(shv1["token_id"], shv2["token_id"])
print(f"Gewinner: {battle['winner']['name']} ({battle['winner']['wins']} Siege)")
print(f"XP gewonnen: {battle['xp_gained']}")
```

### cURL — Via Gateway

```bash
# Shivamon minting
curl -X POST http://localhost:4000/api/game/shivamon/mint \
  -H "Content-Type: application/json" \
  -H "X-API-Key: atc-dev-key-2025" \
  -d '{"owner":"ATC7F3A9B2C1D4E5F6A7B8C9D0E1F2A3B4C5","element":"Quantum","rarity":"Legendary"}'

# Battle starten
curl -X POST http://localhost:4000/api/game/shivamon/battle \
  -H "Content-Type: application/json" \
  -H "X-API-Key: atc-dev-key-2025" \
  -d '{"attacker":"SHV-A3F9B2C1D4E5","defender":"SHV-B4E8C3D2A1F6"}'

# Collection abfragen
curl http://localhost:4000/api/game/shivamon/owner/ATC7F3A9B2C1D4E5F6A7B8C9D0E1F2A3B4C5 \
  -H "X-API-Key: atc-dev-key-2025"
```

### JavaScript — Frontend (api.js)

```javascript
// Shivamon minting
const result = await ATC_API.mintShivamon({
  owner:   walletAddress,
  element: "Neon",
  rarity:  "Rare"
});

console.log(result.shivamon.name);    // "Voltrix-0042"
console.log(result.shivamon.stats);   // { hp: 112, attack: 88, ... }

// Battle
const battle = await ATC_API.battleShivamon(token1, token2);
console.log("Sieger:", battle.winner.name);
```

---

## 12. Deployment

### Lokaler Start

```bash
# Backend (inkl. Game-Service)
cd backend
pip install -r requirements.txt
python main.py   # Port 5000 (Core) + 5004 (Game)

# Gateway
cd gateway
python main.py   # Port 4000
```

### Service registrieren

Der Game-Service wird automatisch im Orchestrator registriert:

```python
# backend/api/orchestrator/orchestrator.py
self.services = {
    ...
    "game": {"port": 5004, "status": "online", "version": "2.0"},
}
```

### Roadmap — v2.1.0

- [ ] **Persistenz:** SQLite / PostgreSQL statt in-memory
- [ ] **ECDSA Signatur:** Transfer erfordert Private-Key Signatur
- [ ] **Breeding:** Zwei Shivamon → Kind-NFT (Gen 2)
- [ ] **Solidity Contract:** On-Chain Version für echte Blockchain
- [ ] **Marketplace:** Buy / Sell für ATC Token
- [ ] **Battle UI:** Animierte Kämpfe im Dashboard

---

> **Dokument:** `docs/contracts/SHIVAMON_NFT_CONTRACT.md`
> **Version:** 2.0.0 · **Datum:** 2026-05-19
> **Autor:** A-TownChain-Okosystems × Aurora AI Agent
