# A-TownChain OS — API Übersicht

## API Gateway (Port 4000)

### System
```
GET  /health              System-Status
GET  /api/version         Version Info
GET  /api/nodes           Verbundene Nodes
```

### Blockchain
```
GET  /api/blockchain/status     Chain-Info (Höhe, Hash, TPS)
GET  /api/blockchain/blocks     Letzte 20 Blöcke
GET  /api/blockchain/block/:h   Block by Höhe
GET  /api/blockchain/tx/:hash   TX by Hash
POST /api/blockchain/tx         TX einreichen
```

### Wallet
```
POST /api/wallet/generate        Neue Wallet generieren
GET  /api/wallet/balance/:addr   ATC-Balance
POST /api/wallet/send            ATC senden (ECDSA-signiert)
GET  /api/wallet/txs/:addr       TX-Historie
```

### Smart Contracts
```
GET  /api/contracts              Alle deployen Contracts
GET  /api/contracts/:addr        Contract-Info
POST /api/contracts/call         Contract-Methode aufrufen
```

### AI / Gemini
```
GET  /api/ai/status              Orchestrator-Status
POST /api/ai/chat                KI-Chat (BYOK)
POST /api/ai/generate-atclang    ATCLang-Code generieren
POST /api/ai/explain-contract    Contract erklären
POST /api/ai/analyze-tx          TX analysieren
```

### NFT / Shivamon
```
POST /api/game/mint              Shivamon minten
GET  /api/game/tokens/:addr      Tokens nach Adresse
POST /api/game/battle            Battle starten
GET  /api/marketplace/listings   Aktive Listings
POST /api/marketplace/buy        Kaufen
```

### Governance
```
GET  /api/governance/proposals   Alle Proposals
POST /api/governance/propose     Neues Proposal
POST /api/governance/vote        Abstimmen
```
