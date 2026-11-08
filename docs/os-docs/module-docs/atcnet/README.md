# atcnet — L5: ATCNet P2P — Kademlia DHT, Bootstrap, Block-Propagation

Part of [A-TownChain OS Monorepo](../../README.md)

## Inhalt

```
modules/atcnet/
├── CHANGELOG.md
├── PROTOCOL.md
├── SECURITY.md
├── atcnet.atc
├── atcnet.py
├── bootstrap_client.py
├── discovery.py
├── node.py
├── p2p_propagation.py
├── requirements.txt
├── tests/test_atcnet.py
```

## Starten

```bash
cd modules/atcnet
pip install -r requirements.txt  # falls vorhanden
python -m atcnet  # oder spezifischer Einstiegspunkt
```

## Layer-Kontext

| Layer | Modul | Ports | Abhängigkeiten |
|-------|-------|-------|----------------|
| L5 | atcnet | — | core/, blockchain/ |

---
*Teil des A-TownChain OS Monorepo — Apache 2.0*
