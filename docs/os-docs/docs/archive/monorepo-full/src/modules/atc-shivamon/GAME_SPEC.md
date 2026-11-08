# Shivamon — Game Specification v2.1.0

## NFT-Attribute (ATC-9000)
- `token_id`: u64 (unique, auto-increment)
- `name`: string (max 32 Zeichen)
- `element`: Fire|Water|Lightning|Plant|Dark|Light|Earth|Ice
- `rarity`: Common|Uncommon|Rare|Epic|Legendary
- `level`: 1–100
- `hp` / `atk` / `def` / `spd` / `sp_atk` / `sp_def`: u32
- `dna`: bytes32 = sha256(owner+token_id+timestamp)
- `wins` / `losses`: u32

## Rarity-Wahrscheinlichkeit beim Mint
| Rarity | % |
|--------|---|
| Common | 50% |
| Uncommon | 30% |
| Rare | 15% |
| Epic | 4% |
| Legendary | 1% |

## Element-Stärken (Typ-Effizienz)
| Angreifer | Stark gegen | Schwach gegen |
|-----------|------------|--------------|
| Fire | Plant, Ice | Water, Earth |
| Water | Fire, Earth | Lightning, Plant |
| Lightning | Water | Earth, Plant |
| Plant | Water, Earth | Fire, Ice |
| Dark | Light | Dark (neutral) |
| Light | Dark | — |

## Battle-Formel
```
Schaden = max(1, Angreifer.ATK - Verteidiger.DEF)
Mit Typ-Bonus: Schaden × 1.5
Mit Typ-Malus: Schaden × 0.5
```

## Breeding (v2.2.0)
- Voraussetzung: beide Eltern Level ≥ 5
- Cooldown: 24h
- Kosten: 500 ATC
- Kind-DNA: sha256(parent1.dna + parent2.dna + salt)
