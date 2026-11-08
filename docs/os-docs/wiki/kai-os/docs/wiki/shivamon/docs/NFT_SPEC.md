# Shivamon NFT Spezifikation

## Attribute
- `token_id`: u64 (unique)
- `name`: string
- `element`: Fire, Water, Lightning, Plant, Dark, Light, Earth, Ice
- `rarity`: Common, Uncommon, Rare, Epic, Legendary
- `level`: 1–100
- `stats`: HP, ATK, DEF, SPD, SP_ATK, SP_DEF
- `dna`: bytes32 (SHA-256, einzigartig)

## Basis-Stats nach Seltenheit
| Rarity | Base-Stat |
|--------|---------|
| Common | 50 |
| Uncommon | 60 |
| Rare | 75 |
| Epic | 95 |
| Legendary | 130 |

## ATC-9000 Standard
- Max Supply: 10.000.000
- Transfer: ATC-Wallet-signiert
- DNA: SHA-256(owner + token_id + timestamp)

## Contract Auszug
```python
# blockchain/contracts/shivamon/shivamon_contract.py
# Shivamon NFT Contract — ATC-9000 Standard
#
# Shivamon sind NFT-Battle-Kreaturen im A-TownChain Ökosystem.
# Jedes Shivamon ist einzigartig (NFT) mit:
#   - Element (Feuer/Wasser/Erde/Luft/Shadow/Neon/Quantum)
#   - Stats (HP, Attack, Defense, Speed, Special)
#   - Rarity (Common → Legendary → Genesis)
#   - DNA Hash (einzigartiger genetischer Fingerabdruck)
#   - Level & Erfahrungspunkte

import hashlib, time, os, json
from enum import Enum
from dataclasses import dataclass, asdict

class Element(Enum):
    FIRE     = "🔥 Fire"
    WATER    = "💧 Water"
    EARTH    = "🌍 Earth"
    AIR      = "💨 Air"
    SHADOW   = "🌑 Shadow"
    NEON     = "⚡ Neon"
    QUANTUM  = "🌀 Quantum"

class Rarity(Enum):
    COMMON    = "Common"
    UNCOMMON  =
```
