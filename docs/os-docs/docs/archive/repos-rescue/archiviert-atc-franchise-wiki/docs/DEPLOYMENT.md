# 🚀 Deployment Guide

## Voraussetzungen
- A-TownChain OS v2.1.0+
- ATCLang v0.2.0+
- ATC-Wallet mit ≥ 1000 ATC
- Python 3.10+

## Schnellstart
```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os
cd a-townchain-os
python start.py

# In einem neuen Terminal:
git clone https://github.com/A-TownChain-Okosystems/atc-franchise
cd atc-franchise
python deploy/deploy.py
```

## Konfiguration (config/franchise.json)
```json
{
  "network": "testnet",
  "gateway": "http://localhost:4000",
  "wallet":  "ATC...",
  "registry_fee": "1000000000000000000000"
}
```

## Lizenz registrieren
```python
from contracts import FranchiseRegistry
reg = FranchiseRegistry(owner_address)
license_id = reg.register(
    brand_name="MeinFranchise",
    category="food",
    region="DACH",
    revenue_share=15,
    duration_days=365
)
print(f"Lizenz-ID: {license_id}")
```
