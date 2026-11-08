# 📄 Issue #11 — Shivamon Breeding (Gen 2 NFTs)

> **Labels:** enhancement · game · nft · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #11](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/11)

---

## Ziel

Zwei Shivamon NFTs können gezüchtet werden und erzeugen ein einzigartiges Kind-NFT der **Generation 2** mit gemischten Stats, Hybrid-DNA und vererbter Rarity.

---

## Breeding-Algorithmus

```python
def breed(self, parent1_id: str, parent2_id: str, owner: str) -> dict:

    p1, p2 = self.tokens[parent1_id], self.tokens[parent2_id]

    # 1. Cooldown-Check (24h)
    now = int(time.time())
    if now - p1.last_breed < 86400:
        return {"success": False, "error": "Parent 1 in cooldown"}

    # 2. DNA mischen
    child_dna = hashlib.sha256(
        (p1.dna_hash + p2.dna_hash + str(now)).encode()
    ).hexdigest()

    # 3. Stats vererben (50/50 Mix + ±10% Mutation)
    child_stats = ShivamonStats(
        hp      = int((p1.stats.hp + p2.stats.hp) / 2 * random.uniform(0.9, 1.1)),
        attack  = int((p1.stats.attack + p2.stats.attack) / 2 * random.uniform(0.9, 1.1)),
        defense = int((p1.stats.defense + p2.stats.defense) / 2 * random.uniform(0.9, 1.1)),
        speed   = int((p1.stats.speed + p2.stats.speed) / 2 * random.uniform(0.9, 1.1)),
        special = int((p1.stats.special + p2.stats.special) / 2 * random.uniform(0.9, 1.1)),
    )

    # 4. Element: dominant (70%) / rezessiv (30%)
    child_el = random.choices([p1.element, p2.element], weights=[70, 30])[0]

    # 5. Rarity vererben
    child_rarity = self._inherit_rarity(p1.rarity, p2.rarity)

    # 6. Kind-NFT minten (Generation = max(p1.gen, p2.gen) + 1)
    return self.mint(owner=owner, element=child_el.value.split()[1],
                     rarity=child_rarity.value,
                     generation=max(p1.generation, p2.generation) + 1,
                     dna_override=child_dna)
```

### Rarity-Vererbungsmatrix

| Eltern | Common | Uncommon | Rare | Epic | Legendary | Genesis |
|--------|--------|----------|------|------|-----------|---------|
| C + C | 85% | 15% | — | — | — | — |
| U + U | 10% | 60% | 30% | — | — | — |
| R + R | — | 10% | 50% | 40% | — | — |
| E + E | — | — | 5% | 45% | 50% | — |
| L + L | — | — | — | 5% | 55% | 40% |
| G + G | — | — | — | — | 30% | 70% |

---

## Aufgaben

- [ ] `ShivamonContract.breed()` implementieren
- [ ] Cooldown-Tracking pro Shivamon (24h)
- [ ] DNA-Mixing Algorithmus
- [ ] Stat-Vererbung mit Mutations-Faktor
- [ ] Rarity-Vererbungsmatrix
- [ ] Breeding-Kosten: 25 ATC abziehen
- [ ] Max Generation: 10 (kein weiteres Breeding)
- [ ] `POST /api/game/shivamon/breed` API-Route
- [ ] Frontend Breeding-Interface im Shivamon-Tab
- [ ] Parent-Preview: geschätzte Kind-Stats anzeigen

---

## Akzeptanzkriterien

- [ ] Breeding erzeugt valides Gen-2 Shivamon
- [ ] Stats sind Mix beider Eltern (±10%)
- [ ] Cooldown von 24h wird korrekt durchgesetzt
- [ ] Breeding-Kosten von 25 ATC werden abgezogen
- [ ] Max Generation 10 wird erzwungen
