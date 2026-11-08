# Smart Contract Deployment

## Voraussetzungen
- A-TownChain OS v2.1.0+ läuft (`python start.py`)
- ATC-Wallet mit ≥ 10 ATC für Gas
- ATCLang Security Analyzer: keine CRITICAL/HIGH Issues

## Deploy via API
```bash
curl -X POST http://localhost:4000/api/contracts/call \
  -H 'X-API-Key: <key>' \
  -H 'X-Signature: <ecdsa_sig>' \
  -d '{"address":"deploy","method":"deploy","args":{"source":"..."}}'
```

## System-Contracts (bereits deployed)
| Contract | Adresse |
|----------|---------|
| ATCoin | ATC_CONTRACT_ATCOIN |
| ShivamonNFT | ATC_CONTRACT_SHIVAMON |
| GovernanceDAO | ATC_CONTRACT_GOVERNANCE |
| Marketplace | ATC_CONTRACT_MARKETPLACE |
| Bridge | ATC_CONTRACT_BRIDGE |

## Sicherheits-Checkliste
- [ ] Security Analyzer: kein CRITICAL/HIGH
- [ ] Reentrancy-Guard via BaseContract
- [ ] require(!self.paused) in transfer/mint/burn
- [ ] fn init() mit self.owner = owner_addr
