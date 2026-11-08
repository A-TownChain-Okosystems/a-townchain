# Gesamt-Architektur v2.1.0

```
A-TownChain OS
├── ATCLang v0.2.0          Proprietäre Programmiersprache
│   ├── Lexer               67+ Token-Typen
│   ├── Parser              Rekursiver Descent, AST
│   ├── Compiler            AST → Bytecode
│   ├── VM                  Stack-basiert, 30+ Opcodes, 128-Depth-Limit
│   ├── REPL                Interaktive Shell
│   └── Stdlib              25+ Built-in Funktionen
│
├── ShivaOS Kernel v2.1.0   Proprietäres Betriebssystem
│   ├── ProcessManager      PID, Spawn, Kill
│   ├── ATCFS               Dezentrales Dateisystem
│   ├── ATCNet              P2P Kademlia DHT
│   └── ShivaConsensus      PoH + PoS + PoW hybrid
│
├── Blockchain Core          
│   ├── HybridConsensus     Kombiniert alle 3 Proof-Typen
│   ├── Smart Contracts     ATC-8300/9000/9900
│   ├── Wallet/ECDSA        secp256k1, ATC-Prefix
│   └── ATCoin              Native Währung
│
├── Backend API              Flask, Port 5000
│   ├── GeminiOrchestrator  KI-Integration (BYOK)
│   └── Routes              Wallet, Blockchain, Game, Governance
│
├── API Gateway              Port 4000
│   ├── Rate-Limiting       Token-Bucket
│   ├── Auth                API-Key
│   └── Signature-Verify    ECDSA
│
└── Frontend                 Neon Dashboard
    └── index.html           Wallet, Explorer, Shivamon, AI
```
