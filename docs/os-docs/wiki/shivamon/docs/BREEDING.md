# Shivamon Breeding System

## Übersicht
Zwei Shivamon können gepaart werden um ein neues Kind-Shivamon zu erzeugen.
Die DNA des Kindes erbt Eigenschaften beider Eltern.

## Breeding-Regeln
- Beide Eltern müssen Level ≥ 5 sein
- 24h Cooldown nach Breeding
- Breeding-Kosten: 500 ATC
- Seltenheit des Kindes: gewichteter Mittelwert + 10% Mutation

## DNA-Vererbung
```python
child_dna = sha256(parent1.dna + parent2.dna + random_salt)
child_stats = {
    "hp":  (p1.hp  + p2.hp)  // 2 + mutation(0, 10),
    "atk": (p1.atk + p2.atk) // 2 + mutation(0, 10),
    "def": (p1.def + p2.def) // 2 + mutation(0, 10),
    "spd": (p1.spd + p2.spd) // 2 + mutation(0, 10),
}
```

## Element-Vererbung
| Eltern | Kind (Wahrscheinlichkeit) |
|--------|--------------------------|
| Feuer + Feuer | Feuer (80%), Erde (20%) |
| Feuer + Wasser | Feuer (40%), Wasser (40%), Dampf-Neu (20%) |
| Licht + Dunkel | Light (30%), Dark (30%), Chaos-Neu (40%) |

## Seltenheits-Tabelle
| Eltern | Kind Common | Rare | Epic | Legendary |
|--------|-------------|------|------|-----------|
| 2x Common | 85% | 14% | 1% | 0% |
| 2x Rare | 20% | 60% | 19% | 1% |
| 2x Epic | 5% | 25% | 60% | 10% |
| 2x Legendary | 0% | 10% | 40% | 50% |
