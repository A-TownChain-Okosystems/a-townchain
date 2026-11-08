# A-TownChain Solidity Smart Contracts

Solidity-Implementierung der Python-basierten A-TownChain Blockchain-Standards.

## Standards

- **ATC-8300** — ERC20 Token (ATC Token.sol)
- **ATC-9000** — ERC721 NFT (Shivamon.sol)
- **ATC-9900** — DAO Governance (ATCGovernance.sol)

## Struktur

```
blockchain/contracts/solidity/
├── contracts/
│   ├── ATC Token.sol          # ERC20 Token mit Halving-Mechanismus
│   ├── Shivamon.sol          # ERC721 NFT mit Breeding-System
│   └── ATCGovernance.sol     # DAO Voting
├── test/
│   ├── ATC Token.test.js
│   ├── Shivamon.test.js
│   └── ATCGovernance.test.js
├── scripts/
│   └── deploy.js             # Deployment Script
├── hardhat.config.js
├── package.json
└── README.md
```

## Installation

```bash
cd blockchain/contracts/solidity
npm install
```

## Kompilierung

```bash
npm run compile
```

## Tests

```bash
npm run test
npm run coverage  # Mit Coverage-Report
```

## Deployment

```bash
# Localhost
npm run deploy

# Testnet
npm run deploy:testnet

# Mainnet
npm run deploy -- --network mainnet
```

## Features

### ATC Token.sol (ATC-8300)

- ERC20 Standard
- Max Supply: 21.000.000 ATC
- Halving alle 210.000 Blocks (wie Bitcoin)
- Pausable Transfers
- Burn Mechanism
- Miner Registry

### Shivamon.sol (ATC-9000)

- ERC721 Standard (NFT)
- Max Supply: 9.900 Shivamon
- Minting Fee: 10 ATC
- DNA-basierte Stat-Generierung
- Battle System (PvP)
- Breeding System (Gen 2 Züchtung)
- 24h Breeding-Cooldown
- Breeding Cost: 25 ATC

### ATCGovernance.sol (ATC-9900)

- Dezentrale Abstimmungen
- 1 ATC = 1 Vote
- Proposal-System
- 7-Day Voting Duration
- Multi-Option Support
- Quorum: 10%

## Integration

### ABI-Export

Contracts werden automatisch zu `/config/abis/` exportiert für Backend-Integration via Web3.py:

```python
import json
with open('config/abis/ATC Token.json') as f:
    abi = json.load(f)
contract = web3.eth.contract(address=token_address, abi=abi)
```

## Environment Variables

Erstelle `.env`:

```env
ATC_TESTNET_RPC=http://localhost:8545
PRIVATE_KEYS=0x...

# Optional
ETHERSCAN_API_KEY=xxx
COINMARKETCAP_API_KEY=xxx
```

## Security

- OpenZeppelin Contracts v5.0
- Audits zu Vulnerability-Erkennung empfohlen
- ERC20Pausable für Emergency-Stops
- Access Control für Admin-Funktionen

## Lizenz

MIT
