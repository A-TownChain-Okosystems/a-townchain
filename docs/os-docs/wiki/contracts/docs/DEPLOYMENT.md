# Contract Deployment Guide

## Voraussetzungen
- A-TownChain OS v2.1.0+
- ATC-Wallet mit ≥ 1000 ATC
- ATCLang v0.2.0+

## Smart Contract deployen
```python
from blockchain.smart_contracts import atc_token, shivamon, governance

# Token-Balance prüfen
balance = atc_token.balance_of("ATC...")

# Shivamon minten
result = shivamon.mint(caller="ATC...", to="ATC...", name="FireDragon", element="fire")

# Governance-Proposal
proposal_id = governance.create_proposal(
    caller="ATC...",
    title="Neue Fee-Struktur",
    description="Reduce fees to 0.1%",
    options=["Zustimmen", "Ablehnen"]
)
```
