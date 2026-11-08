# shivamon — L12: Shivamon NFT Gaming — Battle Engine, Breeding, Marketplace

Part of [A-TownChain OS Monorepo](../../README.md)

## Inhalt

```
modules/shivamon/
├── CHANGELOG.md
├── GAME_SPEC.md
├── api/game_routes.py
├── api/marketplace_routes.py
├── contracts/marketplace_contract.py
├── contracts/shivamon.atc
├── contracts/shivamon_contract.py
├── engine/battle_engine.py
├── requirements.txt
```

## Starten

```bash
cd modules/shivamon
pip install -r requirements.txt  # falls vorhanden
python -m shivamon  # oder spezifischer Einstiegspunkt
```

## Layer-Kontext

| Layer | Modul | Ports | Abhängigkeiten |
|-------|-------|-------|----------------|
| L12 | shivamon | — | core/, blockchain/ |

---
*Teil des A-TownChain OS Monorepo — Apache 2.0*
