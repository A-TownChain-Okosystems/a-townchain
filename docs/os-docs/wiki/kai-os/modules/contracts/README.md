# contracts — L6: Smart Contracts — ATC-8300 Token, Governance, Bridge, Shivamon

Part of [A-TownChain OS Monorepo](../../README.md)

## Inhalt

```
modules/contracts/
├── CHANGELOG.md
├── DEPLOYMENT.md
├── SECURITY.md
├── atc8300/atc8300.atc
├── atc8300/atc8300_token.py
├── atcoin/atcoin.py
├── base/base_contract.py
├── bridge/bridge_contract.py
├── governance/governance.atc
├── governance/governance_contract.py
├── marketplace/marketplace_contract.py
├── requirements.txt
├── shivamon/shivamon.atc
├── shivamon/shivamon_contract.py
├── wallet/ecdsa.py
├── wallet/keygen.py
├── wallet/wallet.atc
```

## Starten

```bash
cd modules/contracts
pip install -r requirements.txt  # falls vorhanden
python -m contracts  # oder spezifischer Einstiegspunkt
```

## Layer-Kontext

| Layer | Modul | Ports | Abhängigkeiten |
|-------|-------|-------|----------------|
| L6 | contracts | — | core/, blockchain/ |

---
*Teil des A-TownChain OS Monorepo — Apache 2.0*
