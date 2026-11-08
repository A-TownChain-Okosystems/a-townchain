# A-TownChain — Vollständige Systemarchitektur

## Schichten-Modell (Layer-Architektur)

```
┌──────────────────────────────────────────────────────┐
│  Layer 7: Applications                               │
│  Shivamon · Franchise · DEX · Mobile Wallet          │
├──────────────────────────────────────────────────────┤
│  Layer 6: API Gateway (Port 4000)                    │
│  REST · WebSocket · Rate-Limit · ECDSA-Auth          │
├──────────────────────────────────────────────────────┤
│  Layer 5: AI & Orchestration                         │
│  Gemini AI (BYOK) · Federated Learning · LLM-Router  │
├──────────────────────────────────────────────────────┤
│  Layer 4: Smart Contracts                            │
│  ATC-8300 · ATC-9000 · ATC-9900 · Bridge · MultiSig  │
├──────────────────────────────────────────────────────┤
│  Layer 3: Blockchain Core                            │
│  ATCoin · ShivaConsensus · ECDSA Wallet · Gas-Fee    │
├──────────────────────────────────────────────────────┤
│  Layer 2: ShivaOS Kernel                             │
│  ATCFS · ATCNet P2P · EventBus · ProcessManager      │
├──────────────────────────────────────────────────────┤
│  Layer 1: ATCLang Runtime                            │
│  Lexer · Parser · Compiler · VM · Stdlib             │
└──────────────────────────────────────────────────────┘
```

## Komponenten-Abhängigkeiten

```
ATCLang VM
  └── atclang/vm/atcvm.py
  └── atclang/compiler/compiler.py
  └── atclang/lexer/lexer.py
  └── atclang/parser/parser.py

ShivaOS Kernel
  ├── core/kernel.py          → EventBus, ProcessManager
  ├── core/event_bus.py       → Pub/Sub, Ringbuffer
  ├── blockchain/consensus/   → ShivaConsensus
  ├── ATCFS (kernel-modul)    → Dateisystem, Syscalls
  └── ATCNet (p2p)            → Kademlia DHT

Blockchain Core
  ├── blockchain/wallet/      → ECDSA, KeyGen, BIP39
  ├── blockchain/atcoin/      → ATCoin Token
  ├── blockchain/consensus/   → PoH + PoS + PoW
  └── blockchain/nodes/       → P2P Node

Smart Contracts
  ├── blockchain/contracts/base/       → BaseContract
  ├── blockchain/contracts/atc001/     → GenesisToken
  ├── blockchain/contracts/atc8300/    → ATC-8300 Token
  ├── blockchain/contracts/governance/ → DAO
  ├── blockchain/contracts/marketplace/→ Marketplace
  ├── blockchain/contracts/shivamon/   → NFT
  └── blockchain/contracts/bridge/    → Cross-Chain Bridge

API Gateway
  ├── gateway/main.py         → Flask App, alle Middlewares
  ├── gateway/router.py       → Route-Registrierung
  └── backend/api/routes/     → Alle Route-Handler

AI Layer
  ├── backend/api/orchestrator/orchestrator.py → Gemini
  └── backend/api/routes/ai_routes.py          → Endpoints
```

## Datenfluss — TX-Lifecycle

```
User-Wallet
    │ signiert TX (ECDSA secp256k1)
    ▼
API Gateway (:4000)
    │ validiert API-Key + ECDSA-Signatur
    │ prüft Rate-Limit (Token-Bucket)
    ▼
Blockchain Routes
    │ TX-Objekt erstellen (hash, from, to, amount, nonce)
    ▼
Mempool
    │ nonce-Prüfung (muss > letzter nonce)
    ▼
ShivaConsensus (Validator-Selection)
    │ Phase 1: PoH — VDF-Hash-Chain
    │ Phase 2: PoS — gewichtetes Random
    │ Phase 3: PoW — SHA3-ATC Target
    ▼
Block-Produktion
    │ Merkle-Tree aller TXs
    │ Block-Header (height, prev_hash, poh_hash, nonce)
    ▼
ECDSA-Signatur des Validators
    ▼
P2P-Propagation (ATCNet Gossip)
    ▼
Block confirmed ✅
```

## Gas-Fee Engine (EIP-1559)

```
Base Fee = vorheriger Base Fee × Anpassungsfaktor
  Block > 50% voll → × 1.125 (Base Fee steigt)
  Block < 50% voll → × 0.875 (Base Fee sinkt)
  Ziel: 50% Block-Auslastung

Gesamte Gebühr = min(Max Fee, Base Fee + Priority Fee)
Davon: 50% geburnt (deflationär) + 50% an Validator
```

## Netzwerk-Topologie

```
Bootstrap Node (Port 5005)
       │ initial peer discovery
  ┌────┴────┬──────────┐
Validator1  Validator2  FullNode   ArchiveNode
:4001       :4001       :4001      :4001
  │           │
 Peers       Peers
  (Kademlia DHT — K=20, Alpha=3)
```
