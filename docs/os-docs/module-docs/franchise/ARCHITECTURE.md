# 🏭 Franchise Factory — Architektur

## Überblick
```
FranchiseRegistry (NFT Lizenz)
        │
        ├── RevenueShare (Automatische Auszahlung)
        ├── FranchiseToken (FFT, ATC-8300)
        └── FranchiseDAO (Governance, ATC-9900)
```

## Smart Contract Stack
| Contract | Standard | Beschreibung |
|----------|---------|-------------|
| FranchiseRegistry | ATC-9000 | Lizenz als NFT |
| RevenueShare | ATC-8300 | Automatische Gewinnverteilung |
| FranchiseToken | ATC-8300 | FFT Utility Token |
| FranchiseDAO | ATC-9900 | Governance |

## Deployment
```
python deploy/deploy_franchise.py --name "Mein Franchise" --region "DACH"
```
