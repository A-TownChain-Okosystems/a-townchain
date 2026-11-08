# A-TownChain Ökosystem — Whitepaper v3.0.0
**10. Juni 2026 | ShivaCore | Chain-ID 658467 | Monorepo v3.0.0**

> *"We don't fork. We build."*

---

## Inhaltsverzeichnis

1. Executive Summary
2. Was ist neu in v3.0.0
3. Gesamt-Architektur
4. ATCLang v0.2.0
5. ShivaOS Kernel v2.2.0
6. ATCFS — Dezentrales Dateisystem
7. ATCNet P2P
8. ShivaConsensus
9. Wallet & Kryptographie
10. Smart Contracts
11. Cross-Chain Bridge (ETH+POL+BSC)
12. Gas-Fee Engine
13. MultiSig Wallet
14. Multi-Node Testnet
15. API Gateway v1.0.0
16. Shivamon NFT Gaming
17. Shivamon Breeding Engine
18. Marketplace v2
19. Franchise Factory
20. Gemini AI & Federated Learning
21. atcpkg Package Manager
22. ShivaOS UI Renderer
23. Build System
24. Security Audit & ATCLang Analyzer
25. Performance Report
26. Repository-Übersicht (24 Repos)
27. ATC Protokoll-Standards
28. Issue-Status & Roadmap
29. Rechtliches

---

## 1. Executive Summary

# 🤖 A-TownChain OS — Agent Manifest v1.0.0

> **Vollständige Datenkarte für KI-Agenten.**
> Alle GitHub-, Notion- und Google-Ressourcen mit exakten IDs, URLs, Strukturen und Zugriffsregeln.
> Stand: 2026-06-10 | Auto-generiert von Superagent (KAI-OS Automation)

---

## 1. IDENTITÄT DES PROJEKTS

| Feld | Wert |
|------|------|
| **Projektname** | A-TownChain OS (auch: KAI-OS v2.0) |
| **Organisation** | A-TownChain-Okosystems |
| **Typ** | Dezentrales AI-Blockchain-Betriebssystem |
| **Architektur** | 13-Layer (L0–L12) |
| **Sprachen** | Python 75%, ATCLang 15%, HTML/JS/Solidity 10% |
| **Lizenz** | Apache 2.0 |
| **Aktiver Agent** | Superagent (Base44) — täglicher Sync 08:00 Europe/Berlin |

---

## 2. GITHUB

### 2.1 Organisation

```
Organisation:  A-TownChain-Okosystems
GitHub URL:    https://github.com/A-TownChain-Okosystems
```

### 2.2 Aktive Repositories (2)

---

#### ⚙️ `a-townchain-os` — Software Repository

```
Full Name:     A-TownChain-Okosystems/a-townchain-os
URL:           https://github.com/A-TownChain-Okosystems/a-townchain-os
Clone:         https://github.com/A-TownChain-Okosystems/a-townchain-os.git
Branch:        main
Größe:         625 KB
Dateien:       250
Zweck:         Gesamter ausführbarer Quellcode
```

**Verzeichnisstruktur:**

```
a-townchain-os/
├── modules/                  (133 Dateien) ← 9 eigenständige Module
│   ├── kernel/               L2: ShivaOS Microkernel (IPC, ATCFS, Consensus)
│   ├── gateway/              L7: API Gateway :4000 (Auth, Rate-Limit, Router)
│   ├── contracts/            L6: Smart Contracts (ATC-8300, Governance, Bridge)
│   ├── atclang/              L1: ATCLang Compiler + VM + REPL + Stdlib
│   ├── atcnet/               L5: P2P Netzwerk (Kademlia DHT, Bootstrap)
│   ├── ui/                   L10: Neon Dashboard (HTML/JS/CSS)
│   ├── shivamon/             L12: NFT Gaming Engine (Battle, Breeding)
│   ├── franchise/            L8: Business DAO (Vault, Revenue, Token)
│   └── standards/            L0: ATC/ATS Protokoll-Standards
│
├── core/                     (6 Dateien)  Shared Services
│   ├── ai_kernel.py          L3: KI-Orchestrator
│   ├── event_bus.py          Event-System
│   ├── module_loader.py      Dynamisches Modul-Loading
│   └── kernel.py             Master-Entry
│
├── backend/                  (26 Dateien) API-Server & DB
│   ├── api/routes/           REST-Endpunkte (wallet, blockchain, ai, game, governance)
│   ├── api/orchestrator/     Orchestrator-Service
│   ├── db/                   Repository-Pattern + schema.sql
│   └── wallet/               Backend-Wallet-Service
│
├── blockchain/               (33 Dateien) Chain & Consensus
│   ├── consensus/            Hybrid PoH + PoS + PoW + fork_resolution
│   ├── contracts/            On-Chain Contracts (atc8300, governance, shivamon, solidity)
│   ├── nodes/                Bootstrap, Discovery, P2P Propagation, Sync
│   └── wallet/               ECDSA Keygen, DID, Signing
│
├── frontend/                 (7 Dateien)  ShivaOS Dashboard
│   ├── index.html            Neon UI Entry Point
│   ├── assets/js/api.js      API Client
│   ├── assets/css/           Styling
│   ├── battle/index.html     Shivamon Battle UI
│   └── bootscreen/           Boot-Animation
│
├── tests/                    (22 Dateien) Zentrales Test-Verzeichnis
│   ├── test_atclang.py
│   ├── test_ecdsa.py
│   ├── test_gateway.py
│   ├── test_p2p_propagation.py
│   ├── test_smart_contracts.py
│   └── unit/                 (9 weitere Unit-Tests)
│
├── docker/                   (3 Dateien)  Dockerfiles
│   ├── Dockerfile.backend
│   ├── Dockerfile.bootstrap
│   └── Dockerfile.gateway
│
├── docker-compose.yml        5-Node Testnetz (Issue #18)
├── config/                   kai_config.toml + settings.json
├── monitoring/               prometheus.yml (Issue #19)
├── build/build.py            Build-System (Issue #7)
├── start.py                  Haupt-Einstiegspunkt
├── requirements.txt          Python-Abhängigkeiten
└── STATUS.md                 Auto-S

---

## 2. Was ist neu in v3.0.0

# CHANGELOG — A-TownChain OS

> **Hinweis:** Historische Einträge verwenden noch alte Repo-Namen.
> Mapping: `atc-kernel` → `atc-kernel` | `shivamon` → `atc-shivamon` | `kai-os-wiki` → `docs/kai-os-wiki.md`

# CHANGELOG — A-TownChain Ökosystem

## v1.0.0 — 2026-06-09 (Aktuell)

### 🔥 Neue Implementierungen
- **ATC-1000 PoH** (`blockchain/consensus/poh.py`) — vollständige Hash-Kette, 8 Tests
- **ATN-1000 Bootstrap Node** (`blockchain/nodes/bootstrap.py`) — UDP Discovery, Persistenz
- **ATN-1001 Bootstrap Client** (`atcnet/bootstrap_client.py`) — Announce, Ping, GetPeers
- **ATAUTH-1000 DID-Resolver** (`blockchain/wallet/did.py`) — vollständiges DID:kai System
- **ATS-1000 Orchestrator** (`backend/api/orchestrator/orchestrator.py`) — Circuit-Breaker, LB
- **ATC-9900 Franchise Factory** (`franchise_factory/factory.py`) — DAO, Vault, Royalty
- **ATC-5000 Bridge** (`atc-contracts/bridge/bridge_contract.py`) — Cross-Chain Lock-Mint
- **Battle Engine v2.0** (`shivamon/engine/battle_engine.py`) — Typ-Schwächen, VRF-RNG
- **ATCLang stdlib v0.3** (`atclang/stdlib/atc_stdlib.py`) — Blockchain-Builtins
- **ShivaOS IPC Bus** (`atc-kernel/ipc/ipc_bus.py`) — Message-Passing
- **Frontend API v2.0** (`atc-ui/assets/js/api.js`) — alle Endpoints

### 🐳 DevOps
- Docker Compose: 6 Services (bootstrap, backend, gateway, frontend, postgres, prometheus)
- Dockerfiles für alle Services
- Prometheus Monitoring-Config
- requirements.txt in allen 8 Code-Repos konsolidiert
- .env.example vollständig

### 🧪 Tests (19 gesamt, alle grün)
- `tests/test_poh.py` — 8 Tests
- `tests/test_did.py` — 7 Tests
- `tests/test_orchestrator.py` — 4 Tests
- `atcnet/tests/test_atcnet.py` — 4 Tests

### ♻️ Bereinigung
- `KAI_OS_SUMMARY.py` → DEPRECATED markiert
- `atc_issues_summary.py` → DEPRECATED markiert
- `atc-kernel/net/atcnet.py` → Redirect auf atcnet Repo
- `docs/DEPRECATED.md` — vollständige Dokumentation aller veralteten Komponenten

### 🌐 Verlinkung
- `ECOSYSTEM.md` in allen Haupt-Repos
- GitHub Topics in allen 23 Repos
- Wiki-Footer in allen 10 Wiki-Repos

---

## v1.3.3-beta — 2026-06-08

### KAI-OS Wiki
- Kapitel 33: ATX Standards Registry (186 Module)
- Kapitel 32: Bug-Tracker (17 Issues)
- Kapitel 31: Live-Projektstatus Dashboard
- Push nach A-TownChain-Okosystems/a-townchain-os-docs ✅

### ATX Standards Registry
- 35 Standardfamilien, 186 Module
- vollständig auf L0-L12 gemappt

---

## v1.3.2-beta — 2026-06-07

### KAI-OS Wiki
- Kapitel 29: Mainnet Readiness Checklist (55 Punkte)
- Kapitel 30

**Abgeschlossene Issues v2.2.0 (10.06.2026):**

| Fix | Titel |
|-----|-------|
| #7 | Build System (Docker/AppImage/EXE/.deb) |
| #8/#18 | 5-Node Testnet + Docker Compose |
| #10 | Cross-Chain Bridge ETH+POLYGON+BSC |
| #11 | Shivamon Breeding Engine (DNA, Element-Fusion) |
| #13 | Marketplace v2 (Festpreis+Auktion, 2.5% Fee, 5% Royalty) |
| #19 | Node-Monitoring (Prometheus + Echtzeit-Dashboard) |
| #23 | ATCFS Kernel-Modul (Syscall-Interface) |
| #24 | MultiSig Wallet M-of-N (Bridge+Franchise Vault) |
| #25 | API Gateway v1.0.0 (alle Middlewares aktiv) |
| #26 | Integration Tests 9/9 ✅ |
| #27 | atcpkg Package Manager |
| #28 | ShivaOS UI Renderer (TUI) |
| #29 | Federated Learning Coordinator (FedAvg, On-Chain) |
| #30 | atcpkg Registry API |
| #32 | ShivaOS Syscall-Tabelle (20 Syscalls) |
| #33 | Gas-Fee Engine (EIP-1559, 50% Burn) |

---

## 3. Gesamt-Architektur

# 🌐 A-TownChain Ökosystem — Master Index v1.0.0

> **Org:** [A-TownChain-Okosystems](https://github.com/A-TownChain-Okosystems) · **Version:** v1.0.0 · **Stand:** 2026-06-09
>
> Ein vollständiges KI-Blockchain-Betriebssystem.
> **13 Layer (L0–L12)** · **26 Sprints** · **4 Phasen** · **21 Repositories**

---

## 🗺️ Layer-Architektur

```
┌────────────────────────────────────────────────────────────────────┐
│  L12  atc-shivamon       NFT Gaming, Battle, Breeding              │
│  L11  atc-contracts      DeFi: Token, Staking, Bridge, Oracle      │
│  L10  atc-ui / atc-franchise  Dashboard, Business DAO              │
│  L9   a-townchain-os     KI-Agenten, Orchestrator                  │
│  L8   atc-contracts      Governance DAO (ATC-9900)                 │
│  L7   atc-gateway        API Gateway :4000                         │
│  L6   a-townchain-os     ATCFS Dateisystem                         │
│  L5   atcnet             P2P Netzwerk, Kademlia DHT                │
│  L4   a-townchain-os     Blockchain, Consensus (PoH→PoS→PoW)      │
│  L3   a-townchain-os     KI/AI Registry, Gemini Integration        │
│  L2   atc-kernel         Microkernel, IPC, Prozess-Manager         │
│  L1   (ATPHY Standards)  Hardware-Abstraktion                      │
├────────────────────────────────────────────────────────────────────┤
│  L0   atc-standards      Security S1–S6 (Querschnitt)              │
└────────────────────────────────────────────────────────────────────┘
    atclang ─── Proprietäre Sprache für alle Layer
```

---

## 📦 Code-Repositories

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | `L2–L4` | **Einziges Haupt-Repo** — KAI-OS Core, AI, Blockchain |
| [atc-kernel](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-kernel) | `L2` | ShivaOS Microkernel, IPC, ATCFS |
| [atcnet](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atcnet) | `L5` | P2P Stack, Kademlia DHT, Bootstrap |
| [atc-gateway](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-gateway) | `L7` | API Gateway :4000, Circuit-Breaker |
| [atclang](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atclang) | `L2–L4` | Proprietäre Sprache v0.3.0 |
| [atc-contracts](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-contracts) | `L4/L11` | Smart Contracts, Token, NFT, Bridge |
| [atc-shivamon](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-shivamon) | `L12` | NFT Gaming, Battle Engine |
| [atc-franchise](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-franchise) | `L10/L8` | Business DAO, Vault, Revenue-Share |
| [atc-ui](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-ui) | `L10` | Neon Dashboard |
| [atc-standards](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-standards) | `L0` | Protokoll-Standards |
| [atc-whitepaper](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-whitepaper) | `L0` | Whitepaper v2.1.0 |

---

## 📚 Dokumentations-Repositories (Wiki)

| Wiki | Dokumentiert | Layer |
|------|-------------|-------|
| [a-townchain-os-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/a-townchain-os-wiki) | a-townchain-os | L2–L4 |
| [atc-kernel-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-kernel-wiki) | atc-kernel | L2 |
| [atcnet-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atcnet-wiki) | atcnet | L5 |
| [atc-gateway-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-gateway-wiki) | atc-gateway | L7 |
| [atclang-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atclang-wiki) | atclang | L2–L4 |
| [atc-contracts-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-contracts-wiki) | atc-contracts | L4/L11 |
| [atc-shivamon-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-shivamon-wiki) | atc-shivamon | L12 |
| [atc-franchise-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-franchise-wiki) | atc-franchise | L10/L8 |
| [atc-ui-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-ui-wiki) | atc-ui | L10 |
| [atc-standards-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-standards-wiki) | atc-standards | L0 |

---

## 🔗 Abhängigkeits-Graph

```
atc-whitepaper
      │
      ▼
atc-standards (L0) ─────────────────────────────┐
      │                                          │
      ▼                                          │
atclang (L2–L4) ──────────────────┐              │
      │                           │              │
      ▼                           ▼              │
atc-kernel (L2)            atc-contracts (L4/L11)│
      │                           │              │
      ▼                           │              │
atcnet (L5)                       │              │
      │                           │              │
      ▼                           ▼              │
a-townchain-os (L2–L4) ◄──────────               │
      │                                          │
      ▼                                          │
atc-gateway (L7) ◄────────────────────           │
      │                           │              │
      ├──────────────┬────────────┘              │
      ▼              ▼                           │
atc-ui (L10)   atc-shivamon (L12)                │
atc-franchise  atc-contracts                     │
               │                                 │
               └── alle nutzen atc-standards ────┘
```

---

## 🚀 Schnellstart

```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os.git
cd a-townchain-os
cp .env.example .env
docker-compose up -d
curl http://localhost:4000/health
```

---

## 📊 Status (2026-06-10)

| Metrik | Wert |
|----

---

## 4. ATCLang v0.2.0

# 📝 ATCLang — Vollständige Sprachspezifikation
**Version:** v0.2.0-alpha | **Stand:** 09.06.2026 | **Dateien:** `atclang/`

---

## Überblick

ATCLang ist die proprietäre Programmiersprache des A-TownChain Ökosystems. Sie wurde vollständig von Grund auf entwickelt — keine Abhängigkeit von Python-Syntax, Solidity oder anderen Sprachen.

**Design-Ziele:**
- Blockchain-native: Smart Contracts als Erstklassige Sprachkonstrukte
- Statisch typisiert mit Typ-Inferenz
- Kein GC: deterministisches Speichermodell für on-chain Ausführung
- Proprietäre VM: Stack-basiert, 30+ Opcodes

---

## Architektur-Überblick

```
ATCLang Quellcode (.atc)
        │
   [ATCLexer]          ← lexer/lexer.py (562 Zeilen, 67+ Token-Typen)
        │
   Token-Stream
        │
   [ATCParser]         ← parser/parser.py (rekursiver Descent)
        │
   AST (Abstract Syntax Tree)
        │
   [ATCCompiler]       ← compiler/compiler.py
        │
   Bytecode
        │
   [ATCVM]             ← vm/atcvm.py (886 Zeilen, Stack-basiert)
        │
   Ausführungsergebnis
```

---

## Lexer — Token-Typen

**Datei:** `atclang/lexer/lexer.py` (562 Zeilen)

### Schlüsselwörter (Keywords)
```
contract  fn       state    return   if       else
elif      while    for      in       let      const
import    from     use      pub      priv     static
emit      require  assert   revert   self     caller
true      false    null     new      delete   break
continue  match    case     default  type     impl
trait     struct   enum     mod      extern   unsafe
```

### Typen
```
u8   u16  u32  u64   u128  u256
i8   i16  i32  i64   i128
f32  f64
bool  string  bytes  address  hash
Map<K,V>   Vec<T>   Option<T>   Result<T,E>
```

### Operatoren
```
+  -  *  /  %  **         Arithmetik
==  !=  <  >  <=  >=      Vergleich
&&  ||  !                 Logik
&  |  ^  ~  <<  >>        Bitweise
=  +=  -=  *=  /=  %=    Zuweisung
->  =>  ::  .  ..  ...   Sonstige
```

### Literale
```
42          Integer
0xFF        Hex-Integer
3.14        Float
"hello"     String
b"deadbeef" Bytes
true/false  Boolean
0x1A2B...   Address (automatisch erkannt wenn 35 Zeichen mit ATC-Präfix)
```

---

## Parser — Grammatik

**Datei:** `atclang/parser/parser.py` (rekursiver Descent)

### Top-Level Konstrukte

```atclang
// Contract-Deklaration
contract TokenName : ATC-8300 {
    state balances: Map<Address, u128>
    state total_supply: u128 = 1_000_000

    fn transfer(to: Address, amount: u128) -> bool {
        require(self.balances[caller] >= amount, "Insufficient balance")
        self.balances[caller] -= amount
        self.balances[to]     += amount
        emit Transfer(caller, to, amount)
        return true
    }
}
```

```atclang
// Struct
struct Block {
    hash:      bytes32,
    prev_hash: bytes32,
    height:    u64,
    timestamp: u64,
}

// Enum
enum TxStatus {
    Pending,
    Confirmed(u64),   // Block-Höhe
    Failed(string),   // Fehler-Nachricht
}

// Trait
trait Mintable {
    fn mint(to: Address, amount: u128) -> bool
    fn burn(from: Address, amount: u128) -> bool
}
```

### Ausdrücke (Expressions)
```atclang
// Arithmetik
let x: u64 = safe_add(100, 200)    // Overflow-safe Addition
let y: u64 = x * 2 - 50

// Conditional
let result = if x > 100 { "big" } else { "small" }

// Match
match status {
    TxStatus::Pending      => "waiting",
    TxStatus::Confirmed(n) => "confirmed at block " + to_string(n),
    TxStatus::Failed(err)  => "error: " + err,
}

// Map-Zugriff
let bal: u128 = self.balances[addr]
self.balances[addr] = bal + amount

// Methoden-Aufruf
let h: bytes32 = sha256("data")
require(is_valid_address(to), "Invalid address")
```

---

## VM — Virtuelle Maschine

**Datei:** `atclang/vm/atcvm.py` (886 Zeilen)

Die ATCVM ist eine Stack-basierte virtuelle Maschine mit einem separaten Heap für Contract-State.

### VM-Architektur
```
┌─────────────────────────────────────┐
│            ATCVM                    │
├─────────────┬───────────────────────┤
│  Stack      │  Heap                 │
│  (Operanden)│  (Contract-State)     │
├─────────────┴───────────────────────┤
│  Program Counter (PC)               │
│  Call Stack (Funktionsaufrufe)      │
│  Gas Counter                        │
│  Event Log                          │
└─────────────────────────────────────┘
```

### Opcode-Tabelle

| Opcode | Wert | Beschreibung |
|--------|------|-------------|
| `PUSH` | 0x01 | Wert auf Stack legen |
| `POP` | 0x02 | Obersten Wert entfernen |
| `DUP` | 0x03 | Obersten Wert duplizieren |
| `SWAP` | 0x04 | Obere zwei Werte tauschen |
| `ADD` | 0x10 | Addition |
| `SUB` | 0x11 | Subtraktion |
| `MUL` | 0x12 | Multiplikation |
| `DIV` | 0x13 | Division |
| `MOD` | 0x14 | Modulo |
| `EQ` | 0x20 | Gleichheit |
| `NEQ` | 0x21 | Ungleichheit |
| `LT` | 0x22 | Kleiner als |
| `GT` | 0x23 | Größer als |
| `AND` | 0x30 | Logisches AND |
| `OR` | 0x31 | Logisches OR |
| `NOT` | 0x32 | Logisches NOT |
| `JUMP` | 0x40 | Unbedingter Sprung |
| `JUMPI` | 0x41 | Bedingter Sprung |
| `CALL` | 0x50 | Funktionsaufruf |
| `RETURN` | 0x51 | Rückgabe |
| `LOAD` | 0x60 | State laden |
| `STORE` | 0x61 | State schreiben |
| `SHA256` | 0x70 | Hash berechnen |
| `EMIT` | 0x80 | Event emittieren |
| `REQUIRE` | 0x81 | Assertion |
| `REVERT` | 0x82 | Transaktion rückgängig |
| `CALLER` | 0x90 | Aufrufer-Adresse |
| `TIMESTAMP` | 0x91 | Block-Zeitstempel |
| `BLOCKNUM` | 0x92 | Block-Höhe |
| `STOP` | 0xFF | Ausführung beenden |

### Gas-System
```
Basis-Gas pro Transaktion: 21.000
Zusatz pro Opcode:
  - PUSH/POP/DUP:  3 Gas
  - ADD/SUB/MUL:   5 Gas
  - DIV/MOD:       8 Gas
  - SHA256:        30 Gas
  - LOAD/STORE:    200 Gas (State-Zugriff)
  - CALL:          700 Gas
  - EMIT:          375 Gas
```

---

## Standard Library (Stdlib)

**Datei:** `atclang/stdlib/atc_stdlib.py`

| Funktion | Signatur | Beschreibung |
|----------|---------|-------------|
| `sha256` | `(s: string) -> bytes32` | SHA-256 Hash |
| `sha3_256` | `(s: string) -> bytes32` | SHA3-256 Hash |
| `keccak256` | `(s: string) -> bytes32` | Keccak-256 (Solidity-kompatibel) |
| `require` | `(cond: bool, msg: string)` | Assertion mit Fehlermeldung |
| `assert_eq` | `(a, b, msg?)` | Gleichheits-Assertion |
| `safe_add` | `(a: u64, b: u64) -> u64` | Overflow-sichere Addition |
| `safe_sub` | `(a: u64, b: u64) -> u64` | Underflow-sichere Subtraktion |
| `safe_mul` | `(a: u64, b: u64) -> u64` | Overflow-sichere Multiplikation |
| `is_valid_address` | `(addr: string) -> bool` | ATC-Adresse validieren |
| `zero_address` | `() -> address` | Null-Adresse (ATC + 32×'0') |
| `block_timestamp` | `() -> u64` | Aktueller Block-Zeitstempel |
| `block_number` | `() -> u64` | Aktuelle Block-Höhe |
| `emit` | `(name: string, data: map)` | Contract-Event senden |
| `to_json` | `(v: any) -> string` | JSON-Serialisierung |
| `from_json` | `(s: string) -> any` | JSON-Deserialisierung |

---

## Vollständiges Beispiel — ATC-8300 Counter Contract

```atclang
// counter.atc — Einfacher Counter Contract
contract Counter {

    // State-Variablen (persistent on-chain)
    state count:   u64   = 0
    state owner:   Address

    // Konstruktor
    fn init(owner_addr: Address) {
        self.owner = owner_addr
        emit ContractDeployed(owner_addr, block_timestamp())
    }

    // State-mutating Funktion
    fn increment() {
        require(is_valid_address(caller), "Invalid caller")
        self.count = safe_add(self.count, 1)
        emit Incremented(caller, self.count)
    }

    fn increment_by(amount: u64) {
        require(amount > 0, "Amount must be positive")
        require(amount <= 1000, "Max increment: 1000")
        self.count = safe_add(self.count, amount)
        emit IncrementedBy(caller, amount, self.count)
    }

    fn reset() {
        require(caller == self.owner, "Only owner can reset")
        self.count = 0
        emit Reset(caller)
    }

    // Read-only Funktion
    fn get_count() -> u64 {
        return self.count
    }

    fn get_owner() -> Address {
        return self.owner
    }
}
```

---

## REPL — Interaktive Shell

**Datei:** `atclang/repl/repl.py`

```bash
$ python -m atclang.repl

ATCLang REPL v0.2.0 — Tippe :help für Befehle
ATC> let x: u64 = 42
ATC> x * 2
→ 84
ATC> sha256("hello")
→ "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
ATC> :load counter.atc
Geladen: Counter (3 Funktionen)
ATC> Counter.increment()
→ Event: Incremented(ATC000..., 1)
ATC> Counter.get_count()
→ 1
ATC> :exit
```

**REPL-Befehle:**
| Befehl | Beschreibung |
|-------

---

## 5. ShivaOS Kernel v2.2.0

# 🖥️ ShivaOS Kernel — Technische Dokumentation
**Stand:** 09.06.2026 | **Version:** v2.1.0 | **Datei:** `shivaos/kernel/kernel.py` (381 Zeilen)

---

## Überblick

ShivaOS ist ein vollständig proprietäres, dezentrales KI-Betriebssystem. Es ist **kein POSIX-Klon** und kein Linux-Fork. Alle Konzepte wurden eigenständig entwickelt und orientieren sich an den **ATS-1000–1007 Standards**.

```
┌────────────────────────────────────────────────┐
│             ShivaOS Kernel (ATS-1000)          │
├──────────┬──────────┬──────────┬───────────────┤
│ Prozess  │ Speicher │  IPC     │  Event System │
│ Manager  │ Manager  │  Layer   │  (EventBus)   │
├──────────┴──────────┴──────────┴───────────────┤
│            Module Loader                        │
├──────────┬──────────┬──────────┬───────────────┤
│  ATCFS   │  ATCNet  │ ATCLang  │ ShivaConsensus│
│  (FS)    │  (Net)   │  (VM)    │  (Chain)      │
└──────────┴──────────┴──────────┴───────────────┘
```

---

## Prozess-Typen

**Datei:** `shivaos/kernel/kernel.py`

```python
class ProcessType(IntEnum):
    AGENT     = auto()   # KI-Agent (Gemini, lokale Modelle)
    SERVICE   = auto()   # Hintergrund-Dienst (ATCNet, ATCFS)
    CONTRACT  = auto()   # Smart Contract (ATCLang VM)
    DAEMON    = auto()   # System-Daemon (Consensus, Mining)
    USER      = auto()   # User-Prozess (CLI, REPL)
```

### Prozess-Zustände (FSM)
```
CREATED → READY → RUNNING → BLOCKED → READY
                    │
                    └→ TERMINATED
```

---

## Prozess-Verwaltung

```python
# Prozess starten
pid = kernel.spawn(
    process_type = ProcessType.SERVICE,
    name         = "ATCNet-P2P",
    target       = atcnet.run,
    priority     = 5
)

# Prozess beenden
kernel.kill(pid)

# Auf Prozess warten
exit_code = kernel.wait(pid)

# Alle Prozesse auflisten
processes = kernel.list_processes()
# → [{"pid": 1, "name": "ATCNet-P2P", "state": "RUNNING", ...}]
```

### Prozess-Prioritäten
| Priorität | Level | Beschreibung |
|-----------|-------|-------------|
| CRITICAL | 10 | Kernel-Prozesse |
| HIGH | 7-9 | Consensus, Netzwerk |
| NORMAL | 4-6 | Services, APIs |
| LOW | 1-3 | User-Prozesse, REPL |

---

## Speicherverwaltung (ATS-1002)

```python
# Speicher allozieren
region = kernel.alloc(size=1024 * 1024, pid=current_pid)
# region.base_addr, region.size, region.pid

# Speicher freigeben
kernel.free(region)

# Speicher-Statistiken
stats = kernel.memory_stats()
# {
#   "total":     256 MB,
#   "used":       42 MB,
#   "free":      214 MB,
#   "processes":  {pid: used_bytes, ...}
# }
```

### Speicher-Isolation
- Jeder Prozess hat eigenen Adressraum
- Smart Contracts: isolierter Heap (kein Zugriff auf andere Contracts)
- KI-Agenten: können nur ihren eigenen State lesen/schreiben

---

## IPC — Inter-Prozess-Kommunikation (ATS-1007)

```python
# Kanal erstellen
chan = kernel.create_channel("atcnet-to-consensus")

# Nachricht senden (nicht-blockierend)
kernel.send(channel="atcnet-to-consensus", msg={"type": "new_block", "data": block})

# Nachricht empfangen (blockierend mit Timeout)
msg = kernel.recv(channel="atcnet-to-consensus", timeout=5.0)

# Broadcast an alle Subscriber
kernel.broadcast("system.alert", {"level": "warn", "msg": "Low disk space"})
```

---

## System-Calls

Vollständige Liste aller Kernel-System-Calls:

| System-Call | Signatur | Beschreibung |
|-------------|---------|-------------|
| `spawn` | `(type, name, target, priority) -> PID` | Prozess starten |
| `kill` | `(pid) -> bool` | Prozess beenden |
| `wait` | `(pid, timeout?) -> ExitCode` | Auf Prozess warten |
| `sleep` | `(seconds: float)` | Prozess schlafen lassen |
| `getpid` | `() -> PID` | Eigene PID |
| `getppid` | `() -> PID` | Eltern-PID |
| `list_processes` | `() -> List[ProcessInfo]` | Alle Prozesse |
| `alloc` | `(size, pid) -> MemRegion` | Speicher allozieren |
| `free` | `(region)` | Speicher freigeben |
| `mmap` | `(file, size) -> MemRegion` | Datei in Speicher mappen |
| `create_channel` | `(name) -> Channel` | IPC-Kanal erstellen |
| `send` | `(channel, msg)` | Nachricht senden |
| `recv` | `(channel, timeout?) -> msg` | Nachricht empfangen |
| `broadcast` | `(event, data)` | Broadcast senden |
| `open` | `(path, mode) -> FileHandle` | Datei öffnen (ATCFS) |
| `read` | `(handle, size) -> bytes` | Datei lesen |
| `write` | `(handle, data)` | Datei schreiben |
| `close` | `(handle)` | Datei schließen |
| `stat` | `(path) -> FileStat` | Datei-Metadaten |

---

## Boot-Sequenz

```
1. Kernel.__init__()   → EventBus + ModuleLoader initialisieren
2. kernel.start()
   ├── event_bus       → Built-in (immer verfügbar)
   ├── atcfs           → shivaos/fs/atcfs.py
   ├── atcnet          → shivaos/net/atcnet.py
   ├── consensus       → shivaos/consensus/shiva_consensus.py
   ├── blockchain      → blockchain/atcoin/atcoin.py
   ├── wallet          → blockchain/wallet/keygen.py
   ├── ai_orchestrator → backend/api/orchestrator/orchestrator.py
   └── api_gateway     → Extern (Port 4000)
3. kernel.event_bus.emit("kerne

---

## 6. ATCFS — Dezentrales Dateisystem

# 📂 ATCFS — Dezentrales Dateisystem
**Stand:** 09.06.2026 | **Version:** v2.1.0 | **Datei:** `shivaos/fs/atcfs.py` (330 Zeilen)

---

## Überblick

ATCFS (A-TownChain File System) ist das proprietäre Dateisystem von ShivaOS. Es kombiniert ein lokales, verschlüsseltes Dateisystem mit optionaler dezentraler Speicherung.

```
┌─────────────────────────────────────────────────┐
│                    ATCFS                         │
├───────────────┬─────────────────────────────────┤
│  VFS Layer    │  Encryption Layer               │
│  (Virtual FS) │  (AES-256-GCM)                  │
├───────────────┴─────────────────────────────────┤
│           Storage Backends                       │
├──────────┬──────────┬──────────────────────────┤
│  Local   │  SQLite  │  Distributed (P2P)        │
│  (disk)  │  (DB)    │  (ATCNet DHT)             │
└──────────┴──────────┴──────────────────────────┘
```

---

## Dateisystem-Struktur

```
/
├── system/          ← Kernel + Core Services
│   ├── kernel/
│   ├── services/
│   └── config/
├── contracts/       ← Deployed Smart Contracts
│   ├── atc8300/
│   └── shivamon/
├── data/            ← Blockchain-Daten
│   ├── blocks/
│   ├── txpool/
│   └── state/
├── wallet/          ← Verschlüsselte Wallet-Dateien
├── logs/            ← System-Logs
├── tmp/             ← Temporäre Dateien
└── home/            ← User-Dateien
    └── <address>/   ← Pro-Wallet Verzeichnis
```

---

## API

```python
class ATCFS:
    """
    ATCFS — Proprietäres ShivaOS Dateisystem.
    ATS-1003 konform.
    """

    def open(self, path: str, mode: str = "r") -> FileHandle:
        """Datei öffnen. Modes: r, w, a, rb, wb"""

    def read(self, handle: FileHandle, size: int = -1) -> bytes:
        """Datei lesen. size=-1 → alles lesen."""

    def write(self, handle: FileHandle, data: bytes) -> int:
        """Daten schreiben. Gibt geschriebene Bytes zurück."""

    def close(self, handle: FileHandle) -> None:
        """Datei-Handle schließen."""

    def stat(self, path: str) -> FileStat:
        """Metadaten: size, created, modified, permissions, hash"""

    def mkdir(self, path: str, recursive: bool = False) -> bool:
        """Verzeichnis erstellen."""

    def ls(self, path: str) -> List[FileEntry]:
        """Verzeichnis auflisten."""

    def rm(self, path: str, recursive: bool = False) -> bool:
        """Datei/Verzeichnis löschen."""

    def mv(self, src: str, dst: str) -> bool:
        """Verschieben/Umbenennen."""

    def cp(self, src: str, dst: str) -> bool:
        """Kopieren."""

    def hash_file(self, path: str) -> str:
        """SHA-256-Hash einer Datei berechnen."""

    def encrypt_file(self, path: str, key: bytes) -> str:
        """Datei AES-256-GCM verschlüsseln."""

    def decrypt_file(self, path: str, key: bytes) -> bytes:
        """Verschlüsselte Datei entschlüsseln."""
```

---

## Speicher-Backends

### 1. Local Backend
Direkte Dateisystem-Speicherung auf dem Host:
```python
backend = LocalBackend(base_path="/var/atcfs")
```

### 2. SQLite Backend (Issue #4)
Für Smart Contract State und Blockchain-Daten:
```python
backend = SQLiteBackend(db_path="/var/atcfs/state.db")
# Tabellen: files, directories, metadata, contracts_state
```

### 3. P2P/DHT Backend (v2.2+)
Dezentrale Speicherung über ATCNet:
```python
backend = P2PBackend(atcnet=atcnet_instance, replication=3)
```

---

## Verschlüsselung

- **Algorithmus:** AES-256-GCM
- **Key-Ableitung:** PBKDF2-HMAC-SHA256 (100.000 Iterationen)
- **Wallet-Datei

---

## 7. ATCNet P2P

# 🌐 ATCNet P2P Stack — Technische Dokumentation
**Stand:** 09.06.2026 | **Version:** v2.1.0 | **Datei:** `shivaos/net/atcnet.py` (486 Zeilen)

---

## Überblick

ATCNet ist der proprietäre P2P-Netzwerk-Stack von A-TownChain. Er basiert auf einem **Kademlia DHT** für Peer-Discovery und nutzt einen eigenen **Handshake-Protokoll** für sichere Verbindungen.

```
┌─────────────────────────────────────────────────┐
│                    ATCNet                        │
├───────────────┬─────────────────────────────────┤
│  Discovery    │  Message Propagation             │
│  (Kademlia)   │  (Gossip Protocol)               │
├───────────────┴─────────────────────────────────┤
│             Transport Layer (TCP/UDP)            │
├─────────────────────────────────────────────────┤
│         Bootstrap Nodes (Port 4001)              │
└─────────────────────────────────────────────────┘
```

---

## Netzwerk-Konfiguration

**Datei:** `config/settings.json`

```json
{
  "network": {
    "bootstrap_nodes":       ["127.0.0.1:5005"],
    "max_peers":             50,
    "ping_interval_sec":     30,
    "peer_timeout_sec":      90,
    "handshake_timeout_sec": 10,
    "discovery_port_offset": 1000
  },
  "rpc": {
    "host": "127.0.0.1",
    "port": 9933
  },
  "websocket": {
    "host": "127.0.0.1",
    "port": 9944,
    "max_connections": 100
  }
}
```

---

## Peer-Discovery (Kademlia DHT)

```python
class ATCNet:
    """
    Proprietärer P2P-Stack — Kademlia DHT + Gossip Propagation.
    Port: 4001 (P2P) | 9933 (RPC) | 9944 (WebSocket)
    """

    def __init__(self, host="0.0.0.0", port=4001):
        self.host        = host
        self.port        = port
        self.peers       = {}          # peer_id → PeerInfo
        self.routing_table = KademliaTable(node_id=self._generate_node_id())
        self.message_queue = asyncio.Queue()
        self._running    = False

    async def start(self):
        """Startet P2P-Node und verbindet zu Bootstrap-Nodes."""
        await self._bind_server()
        await self._connect_bootstrap_nodes()
        await self._start_discovery_loop()

    async def connect(self, host: str, port: int) -> bool:
        """Verbindet zu einem Peer."""
        peer = await self._handshake(host, port)
        if peer:
            self.peers[peer.id] = peer
            return True
        return False

    async def broadcast(self, message: dict) -> int:
        """Sendet Nachricht an alle Peers (Gossip)."""
        sent = 0
        for peer_id, peer in self.peers.items():
            try:
                await self._send_to_peer(peer, message)
                sent += 1
            except Exception:
                self._remove_peer(peer_id)
        return sent
```

---

## Handshake-Protokoll

```
Client                          Server
  │                               │
  │── HELLO (version, node_id) ──►│
  │                               │
  │◄─ HELLO_ACK (version, node_id)│
  │                               │
  │── CHALLENGE (random nonce) ──►│
  │                               │
  │◄─ CHALLENGE_RESP (sig(nonce)) │
  │                               │
  │── CONNECTED ─────────────────►│
  │                               │
  └─── Bidirektionale Kommunikation ──
```

**Sicherheit:**
- Version-Check: inkompatible Versionen werden abgelehnt
- ECDSA-Signatur des Nonce → Authentizität des Peers
- Peer-ID = SHA-256(public_key) → unveränderlich

---

## Message-Typen

| Message-Typ | Beschreibung |
|------------|-------------|
| `NEW_BLOCK` | Neuer Block gefunden (Propagation) |
| `NEW_TX` | Neue Transaktion (Mempool) |
| `PING` | Peer am Leben prüfen |
| `PONG` | Antwort auf PING |
| `GET_PEERS` | Peer-Liste anfragen |
| `PEERS` | Peer-Liste senden |
| `GET_BLOCK` | Block by Hash anfragen |
| `BLOCK` | Block senden |
| `GET_HEADERS` | Block-Header-Kette anfragen |
| `HEADERS` | Header-Kette senden |
| `CONSENSUS` | Consensus-Nachrichten (PoS Votes) |

---

## Block-Propagation

A-TownChain verwendet ein **Gossip-Protokoll** für Block-Propagation:

```
1. Node findet/validiert neuen Block
2. Sendet NEW_BLOCK an alle direkten Peers
3. Peers validieren den Block
4. Wenn valide: weiterleiten an ihre Peers
5. TTL = 7 Hops (verhindert infinites Flooding)
```

**Datei:** `blockchain/nodes/p2p_propagation.py`

```python
class P2PPropagation:
    def propagate_block(self, block, exclude_peer=None):
        """Verbreitet Block im Netzwerk."""
        msg = {
            "type":  "NEW_BLOCK",
            "block": block.to_dict(),
            "ttl":   7,
            "origin": self.node_id
        }
        for peer in self.peers:
            if peer.id != exclude_peer:
                peer.send(msg)
```

---

## Port-Übersicht

| Port | Protokoll | Zweck |
|------|----------|-------|
| 4000 | HTTP | API Gateway (Frontend → Backend) |
| 4001 | TCP/UDP | P2P Node Discovery & Block Sync |
| 5000 | HTTP | Backend REST API (intern) |
| 5005 | TCP | Bootstrap Node |
| 9933 | HTTP | RPC-Endpunkt (externe Clients) |
| 9944

---

## 8. ShivaConsensus

# 🔐 Hybrid Consensus — Technische Dokumentation

> **Algorithmus:** SHA-256 PoW + PoS + PoH
> **Datei:** `blockchain/consensus/hybrid_consensus.py`

---

## Überblick

A-TownChain verwendet einen **dreistufigen hybriden Konsens-Mechanismus**:

```
Schritt 1: PoH   → Kryptographischer Zeitbeweis (Proof of History)
Schritt 2: PoW   → Miner sucht SHA-256 Hash mit N führenden Nullen
Schritt 3: PoS   → Validator bestätigt Block (gewichtet nach Stake)
```

Alle drei Schritte müssen erfüllt sein, damit ein Block gültig ist.

---

## SHA-256 Proof of Work

| Parameter | Wert |
|-----------|------|
| Algorithmus | SHA-256 (doppelt) |
| Difficulty | Anfangswert: 3 führende Nullen |
| Ziel-Blockzeit | 10 Sekunden |
| Difficulty-Anpassung | Nach jedem Block |
| Halving-Intervall | 210.000 Blöcke |
| Start-Reward | 50 ATC |

### Difficulty-Anpassung

```python
def adjust_difficulty(self, avg_block_time: float, target: float = 10.0) -> int:
    if avg_block_time < target * 0.9:   # zu schnell → schwerer
        self.difficulty += 1
    elif avg_block_time > target * 1.1: # zu langsam → leichter
        if self.difficulty > 1:
            self.difficulty -= 1
    self.target = "0" * self.difficulty
    return self.difficulty
```

---

## Proof of Stake

| Parameter | Wert |
|-----------|------|
| Min. Stake | 10.000 ATC |
| Auswahl | Weighted Random (proportional zum Stake) |
| Slashing | 50% Verlust bei double-sign |
| Unstaking | Sofort möglich |

### Validator-Auswahl (deterministisch)

```python
def select_validator(self, seed: str) -> str:
    # Seed = Block-Hash → deterministisch, nicht manipulierbar
    rng = random.Random(int(hashlib.sha256(seed.encode()).hexdigest(), 16))
    total = sum(self.validators.values())
    r, cumulative = rng.uniform(0, total), 0
    for addr, stake in self.validators.items():
        cumulative += stake
        if r <= cumulative:
            return addr
```

---

## Proof of History

| Parameter | Wert |
|-----------|------|
| Algorithmus | Rekursives SHA-256 |
| Sequenz | Unbegrenzt (monoton steigend) |
| Verifikation | Unabhängig von Netzwerk möglich |
| Inspiration | Solana PoH |

```python
def tick(self, data: bytes = None) -> dict:
    combined = (self.current_hash + (data.hex() if data else "")).encode()
    self.current_hash = hashlib.sha256(combined).hexdigest()
    self.sequence += 1
    return {"sequence": self.sequence, "hash": self.current_hash}
```

---

## Block-Erstellung (Hybrid)

```
Input: transactions[], miner_address

1. poh_entry = poh.tick(json(transactions))
   → Zeitstempel-Beweis für diesen Block

2. block_data = {
     height, prev_hash,
     poh_hash, poh_sequence,
     transactions, miner, timestamp
   }

3. pow_result = pow.mine_block(block_data)
   → Nonce + gültiger Hash

4. validator = pos.select_validator(pow_result.hash)
   → Deterministisch aus Stake-Gewichten

5. block_data += { hash, nonce, validator, reward }

6. blocks.append(block_data)
   → Block final
```

---

> **Dokument:** `doc

---

## 9. Wallet & Kryptographie

# 🔑 Wallet Key Generation — Technische Dokumentation

> **Datei:** `blockchain/wallet/keygen.py`
> **Standard:** BIP39-kompatibel · ATC-Adressformat

---

## Überblick

```
Entropy (256 bit)
    │
    ▼ entropy_to_mnemonic()
Seed Phrase (24 Wörter)     ← BIP39 Wordlist (2048 Wörter)
    │
    ▼ mnemonic_to_seed() — PBKDF2-HMAC-SHA512 (2048 Iterationen)
512-bit Seed
    │
    ▼ seed_to_private_key() — HMAC-SHA256
Private Key (256 bit / 64 hex)
    │
    ▼ private_to_public_key() — SHA-256
Public Key (256 bit / 64 hex)
    │
    ▼ public_key_to_address()
ATC Adresse (ATC + 32 hex = 35 Zeichen)
```

---

## Seed Phrase Generierung

```python
# 256 Bit Entropy → 24 Wörter

entropy   = os.urandom(32)               # 32 Bytes = 256 Bit
checksum  = sha256(entropy)[0:1]         # 8-bit Checksum
combined  = entropy_bits + checksum_bits # 264 Bit gesamt
words     = [WORDLIST[combined[i*11:(i+1)*11]] for i in range(24)]
# 264 Bit / 11 Bit pro Wort = 24 Wörter
```

| Entropy-Bits | Wörter | Checksum-Bits |
|-------------|--------|---------------|
| 128 | 12 | 4 |
| 160 | 15 | 5 |
| 192 | 18 | 6 |
| 224 | 21 | 7 |
| **256** | **24** | **8** |

---

## Adress-Schema

```
ATC  +  [28 hex uppercase]  +  [4 hex Checksum]  =  35 Zeichen
 3        28                      4

Beispiel:
ATC  7F3A9B2C1D4E5F6A7B8C9D0E1F2A  3B4C

Derivation:
  step1    = sha256(public_key)
  step2    = sha256(step1)
  checksum = sha256(step2)[:4].upper()
  address  = "ATC" + step2[:28].upper() + checksum
```

---

## Sicherheitshinweise

| Aspekt | Maßnahme |
|--------|----------|
| Private Key | Nur einmalig anzeigen, nie speichern |
| Seed Phrase | Offline aufbewahren (Papier/Metall) |
| Passphrase | Optional, erhöht Sicherheit |
| PBKDF2 | 2048 Iterationen (brute-force-resistent) |
| Entropy | `os.urandom()` — kryptographisch sicher |

---

## Wallet wiederherstellen

```python
keygen = ATCKeyGenerator()

# Aus Seed Phrase
wallet = keygen.restore_from_mnemonic(
    mnemonic   = ["abandon", "ability", ...],   # 24 Wörter
    passphrase = "A-TownChain"                  # optional
)
# → gleiche Adresse wie beim Original
```

---

> **Dokument:** `docs/architecture/WALLET_KEYGEN.md`
> **Datum:** 2026-05-

---

## 10. Smart Contracts

# 🐉 Shivamon NFT Contract — Technische Dokumentation

> **Standard:** ATC-9000 · **Chain:** A-TownChain · **Version:** 2.0.0
> **Datei:** `blockchain/contracts/shivamon/shivamon_contract.py`

---

## Inhaltsverzeichnis

1. [Überblick](#1-überblick)
2. [Architektur](#2-architektur)
3. [Datenmodell](#3-datenmodell)
4. [Enumerationen](#4-enumerationen)
5. [Klassen](#5-klassen)
6. [Contract-Methoden](#6-contract-methoden)
7. [Algorithmen](#7-algorithmen)
8. [API-Referenz](#8-api-referenz)
9. [Fehlerbehandlung](#9-fehlerbehandlung)
10. [Sicherheit](#10-sicherheit)
11. [Beispiele](#11-beispiele)
12. [Deployment](#12-deployment)

---

## 1. Überblick

Der **Shivamon NFT Contract** implementiert den **ATC-9000 Standard** — das NFT-Protokoll des A-TownChain Ökosystems. Jedes Shivamon ist ein einzigartiges, nicht-fungibles Token (NFT) mit genetisch bestimmten Eigenschaften, Kampfwerten und einer unveränderlichen DNA.

### Kernprinzipien

| Eigenschaft | Wert |
|-------------|------|
| Standard | ATC-9000 |
| Max Supply | 9.900 NFTs |
| Elemente | 7 (Fire, Water, Earth, Air, Shadow, Neon, Quantum) |
| Rarities | 6 (Common → Genesis) |
| Generationen | Unbegrenzt (Gen 1 = native) |
| DNA | SHA-256 basiert, einzigartig pro Token |
| Minting-Kosten | 10 ATC |
| Battle-System | Rundenbasiert (max. 5 Runden) |

### Abhängigkeiten

```python
import hashlib    # SHA-256 DNA-Generierung
import time       # Zeitstempel für Minting
import os         # Systemzufälligkeit
import json       # Serialisierung
import random     # Gewichtete Rarity-Auswahl
from enum import Enum
from dataclasses import dataclass, asdict
```

---

## 2. Architektur

```
ShivamonContract
│
├── ShivamonNFT          ← Einzelnes NFT-Objekt
│   ├── ShivamonStats    ← HP/ATK/DEF/SPD/SPC Werte
│   ├── Element (Enum)   ← 7 Elementtypen
│   └── Rarity (Enum)    ← 6 Seltenheitsstufen
│
├── Token Registry       ← tokens: Dict[token_id → ShivamonNFT]
├── Owner Index          ← owner_tokens: Dict[address → List[token_id]]
└── Battle Log           ← battle_log: List[Dict]

                API Layer (game_routes.py)
                        │
                   Gateway :4000
                        │
                 Frontend api.js
```

### Integration im Gesamtsystem

```
Frontend (Shivamon UI)
  └─→ api.js → POST /api/game/shivamon/mint
                    │
              Gateway :4000
                    │
          backend/api/routes/game_routes.py
                    │
          ShivamonContract.mint()
                    │
          ShivamonNFT (Objekt erstellt)
                    │
          tokens[token_id] = nft  ← persistiert im RAM
```

---

## 3. Datenmodell

### ShivamonNFT — Vollständiges Schema

```python
@dataclass
class ShivamonNFT:
    # ── Identität ──────────────────────────────────────
    token_id:   str       # "SHV-" + 12 hex chars (z.B. "SHV-A3F9B2C1D4E5")
    name:       str       # z.B. "Voltrix-0042"
    element:    Element   # Enum: FIRE, WATER, EARTH, AIR, SHADOW, NEON, QUANTUM
    rarity:     Rarity    # Enum: COMMON, UNCOMMON, RARE, EPIC, LEGENDARY, GENESIS
    owner:      str       # ATC-Adresse (35 Zeichen, beginnt mit "ATC")
    generation: int       # Generationsnummer (Standard: 1)

    # ── Progression ────────────────────────────────────
    level:      int       # Start: 1 · Max: unbegrenzt
    xp:         int       # Erfahrungspunkte (XP für Level-Up: level × 100)
    wins:       int       # Gewonnene Kämpfe
    losses:     int       # Verlorene Kämpfe

    # ── Kryptographie ──────────────────────────────────
    dna_hash:   str       # SHA-256 aus token_id + name + element + timestamp
    minted_at:  int       # Unix-Timestamp

    # ── Kampfwerte ─────────────────────────────────────
    stats:      ShivamonStats   # Generiert aus DNA-Hash
    moves:      List[str]       # 4 Angriffe (Element-spezifisch)
```

### ShivamonStats — Kampfwerte

```python
@dataclass
class ShivamonStats:
    hp:      int   # Trefferpunkte   (Basis: 25–150 × Rarity-Multiplier)
    attack:  int   # Angriffsstärke  (Basis: 20–120 × Rarity-Multiplier)
    defense: int   # Verteidigung    (Basis: 20–120 × Rarity-Multiplier)
    speed:   int   # Geschwindigkeit (Basis: 17–105 × Rarity-Multiplier)
    special: int   # Spezialwert     (Basis: 22–135 × Rarity-Multiplier)

    def total(self) -> int:
        return hp + attack + defense + speed + special
```

### JSON-Ausgabe (`.to_dict()`)

```json
{
  "token_id":    "SHV-A3F9B2C1D4E5",
  "name":        "Voltrix-0042",
  "element":     "⚡ Neon",
  "rarity":      "Rare",
  "owner":       "ATC7F3A9B2C1D4E5F6A7B8C9D0E1F2A3B4C5",
  "generation":  1,
  "level":       1,
  "xp":          0,
  "wins":        0,
  "losses":      0,
  "dna_hash":    "a3f9b2c1d4e5f6a7...",
  "stats": {
    "hp":      112,
    "attack":  88,
    "defense": 74,
    "speed":   61,
    "special": 99
  },
  "total_stats": 434,
  "moves":       ["Volt Crash", "Neon Surge", "Thunder Arc", "Plasma Beam"],
  "minted_at":   1747691880,
  "standard":    "ATC-9000"
}
```

---

## 4. Enumerationen

### Element

Bestimmt das Element des Shivamon, seine Moves und die optische Darstellung.

| Enum-Wert | Anzeige | Emoji | Moves |
|-----------|---------|-------|-------|
| `FIRE` | "🔥 Fire" | 🔥 | Flame Burst, Inferno, Ember Strike, Solar Beam |
| `WATER` | "💧 Water" | 💧 | Aqua Jet, Hydro Pump, Tidal Wave, Ice Shard |
| `EARTH` | "🌍 Earth" | 🌍 | Rock Smash, Quake, Boulder Crush, Mudslide |
| `AIR` | "💨 Air" | 💨 | Wind Slash, Tornado, Gust, Sky Dive |
| `SHADOW` | "🌑 Shadow" | 🌑 | Dark Pulse, Void Strike, Soul Drain, Eclipse |
| `NEON` | "⚡ Neon" | ⚡ | Volt Crash, Neon Surge, Thunder Arc, Plasma Beam |
| `QUANTUM` | "🌀 Quantum" | 🌀 | Phase Shift, Quantum Leap, Reality Tear, Entangle |

### Rarity

Bestimmt die Stärke der generierten Stats via `RARITY_MULTIPLIER`.

| Enum-Wert | Anzeige | Multiplier | Drop-Rate |
|-----------|---------|-----------|-----------|
| `COMMON` | "Common" | `1.0×` | 50.0% |
| `UNCOMMON` | "Uncommon" | `1.2

# 🪙 ATC Token Standard — Übersicht

> Technische Kurzreferenz aller ATC Token Standards

| Standard | Typ | Datei | Status |
|----------|-----|-------|--------|
| ATC-001 | Genesis Token | `blockchain/smart_contracts.py` | ✅ |
| ATC-8300 | Fungible Token | `blockchain/atcoin/atcoin.py` | ✅ |
| ATC-9000 | NFT (Shivamon) | `blockchain/contracts/shivamon/shivamon_contract.py` | ✅ |
| ATC-9900 | Governance/DAO | geplant | ⏳ v2.1 |

→ Vollständige Dokumentation: [SHIVAMON_NFT_CONTRACT.md](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/contracts/SHIVAMON_NFT_CONTRACT.md)

---

## 33.1 Strategische Entscheidung: Frontier-Pallet

**Gewaehlt: Substrate Frontier EVM-Pallet**

```
Substrate-Runtime
+-- pallet-evm       (Frontier — EVM-Execution-Engine)
+-- pallet-ethereum  (Frontier — Ethereum-Block-Format)
+-- pallet-base-fee  (EIP-1559 kompatibel)
+-- pallet-dynamic-fee

Vorteile:
- EVM direkt in Substrate integriert
- MetaMask/Ethers.js funktionieren ohne Aenderung
- Solidity-Contracts laufen nativ auf KAI-OS
- Bestehende ETH-Tools (Hardhat, Foundry) nutzbar
- EIP-1559, EIP-712, EIP-2930 kompatibel
- Chain-ID: 658467 (eindeutig fuer KAI-OS)
```

---

## 33.2 Frontier-Pallet Cargo.toml

```toml
[dependencies]
pallet-evm = { git = "https://github.com/polkadot-evm/frontier", features = ["forbid-evm-reentrancy"] }
pallet-ethereum = { git = "https://github.com/polkadot-evm/frontier" }
pallet-base-fee = { git = "https://github.com/polkadot-evm/frontier" }
fp-evm = { git = "https://github.com/polkadot-evm/frontier" }
fp-rpc = { git = "https://github.com/polkadot-evm/frontier" }
```

---

## 33.3 Runtime-Konfiguration

```rust
parameter_types! {
    pub BlockGasLimit: U256 = U256::from(75_000_000u64);
    pub const GasLimitPovSizeRatio: u64 = 16;
    pub WeightPerGas: Weight = Weight::from_parts(20_000, 0);
}

impl pallet_evm::Config for Runtime {
    type FeeCalculator = BaseFee;
    type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
    type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
    type AddressMapping = HashedAddressMapping<BlakeTwo256>;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type ChainId = EVMChainId;          // 658467
    type BlockGasLimit = BlockGasLimit;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type WeightInfo = pallet_evm::weights::SubstrateWeight<Self>;
}
```

---

## 33.4 Solidity Contract Suite

### Projekt-Struktur (Hardhat)

```
kai-solidity/
+-- contracts/
|   +-- ATC Token.sol          # ERC-20 (ATC-8300 kompatibel)
|   +-- ShivamonNFT.sol       # ERC-721 + ERC-2981 Royalties
|   +-- KAIGovernance.sol     # Governor Bravo kompatibel
|   +-- KAIMarketplace.sol    # ERC-721 Marketplace
|   +-- KAIBridge.sol         # Lock/Unlock Bridge
+-- test/
+-- scripts/deploy.ts
+-- hardhat.config.ts
```

### ATC Token.sol (ERC-20)

```solidity
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

contract ATC Token is ERC20, ERC20Permit, ERC20Votes, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");
    uint256 public constant MAX_SUPPLY = 1_000_000_000 * 10**18;

    constructor() ERC20("A-TownCoin", "ATC") ERC20Permit("A-TownCoin") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function mintFromBridge(address to, uint256 amount) external onlyRole(MINTER_ROLE) {
        require(totalSupply() + amount <= MAX_SUPPLY, "Exceeds max supply");
        _mint(to, amount);
        emit BridgeMint(to, amount);
    }

    function burnToBridge(address from, uint256 amount, bytes32 substrateAddress)
        external onlyRole(BURNER_ROLE)
    {
        _burn(from, amount);
        emit BridgeBurn(from, amount, substrateAddress);
    }

    event BridgeMint(address indexed to, uint256 amount);
    event BridgeBurn(address indexed from, uint256 amount, bytes32 substrateAddress);
}
```

### ShivamonNFT.sol (ERC-721 + ERC-2981)

```solidity
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Royalty.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract ShivamonNFT is ERC721URIStorage, ERC721Royalty {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIdCounter;

    struct ShivamonData {
        string name;
        string element;
        uint8 level;
        uint16 hp; uint16 attack; uint16 defense; uint16 speed;
        string rarity;
        uint8 generation;
        bytes32 dna;
    }

    mapping(uint256 => ShivamonData) public shivamonData;

    constructor() ERC721("Shivamon", "SHIV") {
        _setDefaultRoyalty(msg.sender, 250); // 2.5%
    }

    function mint(address to, string memory uri, ShivamonData calldata data)
        public returns (uint256)
    {
        uint256 tokenId = _tokenIdCounter.current();
        _tokenIdCounter.increment();
        _safeMint(to, tokenId);
        _setTokenURI(tokenId, uri);
        shivamonData[tokenId] = data;
        emit ShivamonMinted(tokenId, to, data.name, data.rarity);
        return tokenId;
    }

    event ShivamonMinted(uint256 indexed tokenId, address indexed to, string name, string rarity);
}
```

---

## 33.5 Hardhat-Konfiguration

```typescript
const config: HardhatUserConfig = {
  solidity: { version: "0.8.20", settings: { optimizer: { enabled: true, runs: 200 } } },
  networks: {
    kaiDevnet:  { url: "http://localhost:9933", chainId: 658467, accounts: [process.env.DEPLOYER_KEY!] },
    kaiTestnet: { url: "https://rpc.testnet.kai-os.io", chainId: 658467 },
    kaiMainnet: { url: "https://rpc.kai-os.io", chainId: 658467 },
    sepolia:    { url: `https://sepolia.infura.io/v3/${process.env.INFURA_KEY}`, chainId: 11155111 },
  },
  etherscan: {
    customChains: [{ network: "kaiMainnet", chainId: 658467,
      urls: { apiURL: "https://explorer.kai-os.io/api", browserURL: "https://explorer.kai-os.io" }
    }]
  }
};
```

---

## 33.6 MetaMask-Konfiguration

```javascript
await window.ethereum.request({
  m

### Solana (in Entwicklung — #15)
# Kapitel 32 — Solana Integration

> Version: 1.0.0 | Stand: 2026-06-09 | KAI-OS Wiki
> Status: Geplant (Sprint 3.9-3.10)

---

## 32.1 Strategische Entscheidung

### Warum Solana?

| Kriterium | Substrate (KAI-OS Native) | Solana | Entscheidung |
|-----------|--------------------------|--------|-------------|
| Throughput | ~1.000 TPS | ~65.000 TPS | Solana fuer High-Volume-TXs |
| Finalitaet | ~6s (GRANDPA) | ~400ms | Solana fuer RT-Zahlungen |
| NFTs | Ink! (pallet-contracts) | Metaplex (Anchor) | Beide (Interop via Bridge) |
| DeFi | Eigene AMMs (L11) | Orca, Raydium | Native + Bridge |
| Kosten | ~0.001 ATC/TX | ~0.000025 SOL/TX | Solana fuer Mikro-TXs |

### Nutzungs-Strategie

```
KAI-OS Substrate (Native Layer):
  - Governance (DAO Voting)
  - Agent-Registry
  - System-Contracts
  - Hohe Sicherheit, niedrige Frequenz

Solana (High-Performance Layer):
  - Shivamon NFTs (Metaplex)
  - Marketplace (Coral/Anchor)
  - Mikro-Zahlungen (< 0.01 ATC)
  - Gaming-Events (Battles, Quests)

Bridge (Wormhole):
  - ATC (Substrate) <-> ATC-SPL (Solana)
  - NFT-Portabilitaet: Shivamon auf beiden Chains
```

---

## 32.2 Technischer Stack

### Installation

```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
```

### Projekt-Struktur

```
kai-solana-programs/
├── programs/
│   ├── atc-spl-token/      # ATC als SPL-Token
│   ├── shivamon-nft/       # Shivamon als Metaplex NFT
│   ├── kai-marketplace/    # Shivamon Marketplace
│   └── kai-bridge/         # Wormhole Bridge Endpoint
├── tests/
├── Anchor.toml
└── package.json
```

---

## 32.3 ATC-SPL Token (Anchor)

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("KAIatc111111111111111111111111111111111111");

#[program]
pub mod atc_spl_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, decimals: u8) -> Result<()> {
        msg!("ATC-SPL Token initialized");
        Ok(())
    }

    pub fn mint_from_bridge(ctx: Context<MintFromBridge>, amount: u64) -> Result<()> {
        require!(
            ctx.accounts.bridge_authority.key() == ctx.accounts.mint.mint_authority.unwrap(),
            KAIError::Unauthorized
        );
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.recipient.to_account_info(),
                    authority: ctx.accounts.bridge_authority.to_account_info(),
                },
            ),
            amount,
        )?;
        emit!(BridgeMint { recipient: ctx.accounts.recipient.key(), amount });
        Ok(())
    }

    pub fn burn_to_bridge(
        ctx: Context<BurnToBridge>,
        amount: u64,
        substrate_address: [u8; 32]
    ) -> Result<()> {
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.source.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;
        emit!(BridgeBurn { source: ctx.accounts.source.key(), amount, substrate_address });
        Ok(())
    }
}

#[event]
pub struct BridgeMint { pub recipient: Pubkey, pub amount: u64 }
#[event]
pub struct BridgeBurn { pub source: Pubkey, pub amount: u64, pub substrate_address: [u8; 32] }
```

---

## 32.4 Shivamon NFT auf Solana (Metaplex)

```rust
declare_id!("KAIshiv111111111111111111111111111111111111");

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ShivamonAttributes {
    pub name: String,
    pub element: String,   // Fire, Water, Earth, Air, Lightning
    pub level: u8,
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub rarity: String,    // Common, Rare, Epic, Legendary
    pub generation: u8,
    pub dna: [u8; 32],
}

#[program]
pub mod shivamon_nft {
    use super::*;

    pub fn mint_shivamon(
        ctx: Context<MintShivamon>,
        name: String,
        uri: String,  // IPFS-CID mit Metadaten
        attributes: ShivamonAttributes,
    ) -> Result<()> {
        msg!("Shivamon {} minted on Solana", name);
        Ok(())
    }

    pub fn bridge_from_substrate(
        ctx: Context<BridgeFromSubstrate>,
        substrate_token_id: u64,
        metadata_uri: String,
    ) -> Result<()> {
        // Substrate-NFT wird gelockt, Solana-NFT wird geminted
        Ok(())
    }
}
```

---

## 32.5 Wormhole Bridge

```
WORMHOLE_CORE_BRIDGE = "worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth"
WORMHOLE_TOKEN_BRIDGE = "wormDTUJ6AWPNvk59vGQbDvGJmqbDTdgWgAqcLBCgUb"

Bridge-Flow Substrate -> Solana:
1. User: lock_atc(amount, solana_recipient) auf Substrate
2. Bridge-Relayer:

---

## 12. Gas-Fee Engine (EIP-1559)

*(Fix #33 — 10.06.2026)*

**Modell:**
- **Base Fee** — automatisch angepasst (Ziel: 50% Block-Auslastung)
- **Priority Fee** — optionaler Tip für Priorität
- **50% Burn** — deflationärer Mechanismus

```
Total Fee = min(Max Fee, Base Fee + Priority Fee)
Block > 50% voll → Base Fee × 1.125
Block < 50% voll → Base Fee × 0.875
```

---

## 13. MultiSig Wallet

*(Fix #24 — 10.06.2026)*

M-of-N Multi-Signatur für Bridge Vault + Franchise Treasury.

```atclang
contract MultiSigWallet {
    state owners:    Vec<Address>
    state threshold: u8
    fn propose(to, amount) -> u64
    fn sign(proposal_id)
    fn execute(proposal_id)  // wenn threshold erreicht
}
```

---

## 14. Multi-Node Testnet

# 🌐 A-TownChain Testnet — Technische Dokumentation

> **Milestone:** v2.2.0 · **Issues:** #8, #14–#19
> **Datei:** `docs/architecture/Testnet.md`

---

## Inhaltsverzeichnis

1. [Überblick](#1-überblick)
2. [Netzwerk-Architektur](#2-netzwerk-architektur)
3. [Node Discovery (Issue #14)](#3-node-discovery-issue-14)
4. [Block Propagation (Issue #15)](#4-block-propagation-issue-15)
5. [Initial Sync (Issue #16)](#5-initial-sync-issue-16)
6. [Longest-Chain-Rule (Issue #17)](#6-longest-chain-rule-issue-17)
7. [Docker Compose (Issue #18)](#7-docker-compose-issue-18)
8. [Node-Monitoring Dashboard (Issue #19)](#8-node-monitoring-dashboard-issue-19)
9. [Testnet starten](#9-testnet-starten)
10. [Protokoll-Referenz](#10-protokoll-referenz)

---

## 1. Überblick

Das A-TownChain Testnet ist ein **lokales P2P-Netzwerk** aus 5 Nodes, das mit einem einzigen `docker-compose up` gestartet wird. Es dient zur Validierung des Hybrid-Konsens (SHA-256 PoW + PoS + PoH) unter realen Netzwerkbedingungen vor dem Mainnet-Launch.

| Eigenschaft | Wert |
|-------------|------|
| Nodes | 5 (1 Bootstrap · 1 Validator · 1 Miner · 2 Full) |
| Protokoll | TCP (Block/TX Propagation) + UDP (Discovery) |
| Block-Ziel | 10 Sekunden pro Block |
| Faucet | 100 Test-ATC per Adresse |
| Monitoring | Live-Dashboard im ShivaOS |

---

## 2. Netzwerk-Architektur

```
┌─────────────────────────────────────────────────────────┐
│                    Testnet NETZWERK                     │
│                                                         │
│   bootstrap-001:5005  ←── Alle neuen Nodes melden hier │
│         ↕    ↕    ↕                                     │
│    ┌────┘    │    └────┐                                │
│    ▼         ▼         ▼                                │
│ validator  miner-001  node-002                          │
│  -001:5105 :5205      :5305                             │
│    │                   │                                │
│    └────────┬──────────┘                                │
│             ▼                                           │
│          node-003:5405                                  │
└─────────────────────────────────────────────────────────┘

Ports pro Node:
  :5005 / :5105 / :5205 / :5305 / :5405  → P2P TCP
  :4000                                   → API Gateway (nur Bootstrap)
```

### Node-Rollen

| Typ | Aufgabe | Stake | Anzahl |
|-----|---------|-------|--------|
| `bootstrap` | P2P-Einstiegspunkt, Peer-Registry | — | 1 |
| `validator` | PoS Block-Bestätigung | 50.000 ATC | 1 |
| `miner` | SHA-256 PoW Mining | — | 1 |
| `full` | Chain speichern, TX weiterleiten | — | 2 |

---

## 3. Node Discovery (Issue #14)

### Datei: `blockchain/nodes/discovery.py`

```python
import socket, json, threading, time

BOOTSTRAP_NODES = ["127.0.0.1:5005"]

class NodeDiscovery:
    """
    UDP-basierter Node-Discovery Service.
    Neue Nodes senden ANNOUNCE → Bootstrap antwortet mit PEER_LIST.
    """

    def __init__(self, my_id: str, my_port: int):
        self.my_id      = my_id
        self.my_port    = my_port
        self.peers      = {}    # node_id → {"host": str, "port": int, "last_seen": int}
        self.sock       = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.bind(("0.0.0.0", my_port + 1000))  # Discovery-Port = P2P-Port + 1000
        self._running   = True

    def announce(self):
        """Meldet sich bei allen Bootstrap-Nodes an."""
        msg = json.dumps({
            "type":    "ANNOUNCE",
            "node_id": self.my_id,
            "port":    self.my_port,
            "version": "2.0"
        }).encode()
        for bootstrap in BOOTSTRAP_NODES:
            host, port = bootstrap.split(":")
            self.sock.sendto(msg, (host, int(port) + 1000))

    def listen(self):
        """Empfängt Discovery-Nachrichten."""
        while self._running:
            try:
                data, addr = self.sock.recvfrom(4096)
                msg = json.loads(data)
                self._handle(msg, addr)
            except Exception:
                pass

    def _handle(self, msg: dict, addr: tuple):
        t = msg.get("type")
        if t == "ANNOUNCE":
            # Neuer Peer → registrieren + Peer-Liste zurückschicken
            self.peers[msg["node_id"]] = {
                "host": addr[0], "port": msg["port"],
                "last_seen": int(time.time())
            }
            self._send_peer_list(addr)
        elif t == "PEER_LIST":
            # Peer-Liste empfangen → eigene Liste erweitern
            for node_id, info in msg.get("peers", {}).items():
                if node_id != self.my_id:
                    self.peers[node_id] = info
        elif t == "PING":
            self.sock.sendto(json.dumps({"type": "PONG"}).encode(), addr)
        elif t == "PONG":
            # Peer ist noch online → last_seen aktualisieren
            pass

    def _send_peer_list(self, addr: tuple):
        msg = json.dumps({"type": "PEER_LIST", "peers": self.peers}).encode()
        self.sock.sendto(msg, addr)

    def health_check(self):
        """Pingt alle 30s alle Peers — entfernt offline Peers."""
        while self._running:
            now = int(time.time())
            dead = [nid for nid, info in self.peers.items()
                    if now - info["last_seen"] > 90]
            for nid in dead:
                del self.peers[nid]
            time.sleep(30)

    def save_peers(self, path="data/peers.json"):
        with open(path, "w") as f:
            json.dump(self.peers, f)

    def load_peers(self, path="data/peers.json"):
        try:
            with open(path) as f:
                self.peers = json.load(f)
        except FileNotFoundError:
            pass
```

### config/settings.json (Erweiterung)

```json
{
  "network": {
    "bootstrap_nodes": ["127.0.0.1:5005"],
    "max_peers": 50,
    "ping_interval_sec": 30,
    "peer_timeout_sec": 90,
    "handshake_timeout_sec": 10,
    "discovery_port_offset": 1000
  }
}
```

---

## 4. Block Propagation (Issue #15)

### Datei: `blockchain/nodes/p2p.py`

```python
import socket, json, threading

class P2PNetwork:
    """
    TCP-basiertes P2P Netzwerk für Block- und TX-Propagation.
    """

    MSG_NEW_BLOCK = "NEW_BLOCK"
    MSG_NEW_TX    = "NEW_TX"
    MSG_GET_BLOCKS = "GET_BLOCKS"
    MSG_BLOCKS    = "BLOCKS"
    MSG_HANDSHAKE = "HANDSHAKE"

    def __init__(self, node_id: str, port: int, consensus):
        self.node_id   = node_id
        self.port      = port
        self.consensus = consensus
        self.peers     = {}     # node_id → socket
        self.seen_msgs = set()  # Duplikat-Filter (msg_hash)

    def broadcast_block(self, block: dict):
        """Sendet neuen Block an alle verbundenen Peers."""
        msg = self._build_msg(self.MSG_NEW_BLOCK, {"block": block})
        self._broadcast(msg)

    def broadcast_tx(self, tx: dict):
        """Sendet neue TX an alle Peers."""
        msg = self._build_msg(self.MSG_NEW_TX, {"tx": tx})
        self._broadcast(msg)

    d

---

## 15. API Gateway v1.0.0

# ⚡ API Gateway — Technische Dokumentation

> **Port:** 4000 · **Datei:** `gateway/main.py`

---

## Überblick

Das API Gateway ist der **einzige Kommunikationspunkt** zwischen Frontend und Backend. Das Frontend spricht ausschließlich mit dem Gateway — nie direkt mit einem Backend-Service.

```
Frontend (Browser)
    │  api.js → alle Calls gehen an :4000
    ▼
API Gateway (:4000)
    ├── Auth Middleware      ← X-API-Key prüfen
    ├── Rate Limiter         ← 100 req / 60s
    ├── Request Logger       ← alle Calls loggen
    └── Router               ← leitet weiter an:
         ├── Core    :5000   ← /api/status, /api/modules
         ├── Chain   :5001   ← /api/blockchain/*
         ├── Wallet  :5002   ← /api/wallet/*
         ├── AI      :5003   ← /api/ai/*
         ├── Game    :5004   ← /api/game/*
         └── Nodes   :5005   ← /api/nodes/*
```

---

## Middleware Stack

### 1. API Key Auth (`gateway/middleware/auth.py`)

```python
API_KEYS = {
    "atc-dev-key-2025":  "developer",
    "atc-admin-key":     "admin",
}

def auth_middleware(request):
    key = request.headers.get("X-API-Key")
    if key not in API_KEYS:
        return {"error": "Unauthorized"}, 401
    request.role = API_KEYS[key]
    return None  # weiter
```

### 2. Rate Limiter (`gateway/middleware/rate_limit.py`)

```python
# 100 Requests pro IP pro 60 Sekunden
RATE_LIMIT   = 100
WINDOW_SEC   = 60
request_log  = {}   # ip → [timestamps]

def rate_limit(ip):
    now    = time.time()
    window = [t for t in request_log.get(ip,[]) if now-t < WINDOW_SEC]
    if len(window) >= RATE_LIMIT:
        return {"error": "Rate limit exceeded"}, 429
    window.append(now)
    request_log[ip] = window
    return None
```

### 3. Request Logger (`gateway/middleware/logger.py`)

```
[2026-05-19 21:50:00] POST /api/wallet/create | 200 | 42ms | developer
[2026-05-19 21:50:01] GET  /api/blockchain/info | 200 | 8ms  | developer
```

---

## Service-Routing

| Prefix | Service | Port | Beispiel |
|--------|---------|------|---------|
| `/api/status` | Core | 5000 | `GET /api/status` |
| `/api/orchestrator` | Core | 5000 | `GET /api/orchestrator/status` |
| `/api/blockchain` | Chain | 5001 | `POST /api/blockchain/mine` |
| `/api/wallet` | Wallet | 5002 | `POST /api/wallet/create` |
| `/api/ai` | AI | 5003 | `POST /api/ai/query` |
| `/api/game` | Game | 5004 | `POST /api/game/shivamon/mint` |
| `/api/nodes` | Nodes | 5005 | `GET /api/nodes/` |

---

## Health Check

```bash
GET http://localhost:4000/gateway/health

{
  "gateway": "online",
  "version": "2.0",
  "services": {
    "core":       "online",
    "blockchain": "online",
    "wallet":     "online",
    "ai":         "offline",
    "game":       "online",
    "nodes":      "online"
  },
  "uptime": "2h 14m 33s"
}
```

---

> **Dokument:** `docs/ar

**Fix #25 — alle Middlewares aktiv:**
- Auth (API-Key + ECDSA)
- Rate-Limiting (Token-Bucket)
- CORS
- Logger
- Franchise-, Package-, Monitoring-Routen

---

## 16. Shivamon NFT Gaming

# 🐉 Shivamon NFT Contract — Technische Dokumentation

> **Standard:** ATC-9000 · **Chain:** A-TownChain · **Version:** 2.0.0
> **Datei:** `blockchain/contracts/shivamon/shivamon_contract.py`

---

## Inhaltsverzeichnis

1. [Überblick](#1-überblick)
2. [Architektur](#2-architektur)
3. [Datenmodell](#3-datenmodell)
4. [Enumerationen](#4-enumerationen)
5. [Klassen](#5-klassen)
6. [Contract-Methoden](#6-contract-methoden)
7. [Algorithmen](#7-algorithmen)
8. [API-Referenz](#8-api-referenz)
9. [Fehlerbehandlung](#9-fehlerbehandlung)
10. [Sicherheit](#10-sicherheit)
11. [Beispiele](#11-beispiele)
12. [Deployment](#12-deployment)

---

## 1. Überblick

Der **Shivamon NFT Contract** implementiert den **ATC-9000 Standard** — das NFT-Protokoll des A-TownChain Ökosystems. Jedes Shivamon ist ein einzigartiges, nicht-fungibles Token (NFT) mit genetisch bestimmten Eigenschaften, Kampfwerten und einer unveränderlichen DNA.

### Kernprinzipien

| Eigenschaft | Wert |
|-------------|------|
| Standard | ATC-9000 |
| Max Supply | 9.900 NFTs |
| Elemente | 7 (Fire, Water, Earth, Air, Shadow, Neon, Quantum) |
| Rarities | 6 (Common → Genesis) |
| Generationen | Unbegrenzt (Gen 1 = native) |
| DNA | SHA-256 basiert, einzigartig pro Token |
| Minting-Kosten | 10 ATC |
| Battle-System | Rundenbasiert (max. 5 Runden) |

### Abhängigkeiten

```python
import hashlib    # SHA-256 DNA-Generierung
import time       # Zeitstempel für Minting
import os         # Systemzufälligkeit
import json       # Serialisierung
import random     # Gewichtete Rarity-Auswahl
from enum import Enum
from dataclasses import dataclass, asdict
```

---

## 2. Architektur

```
ShivamonContract
│
├── ShivamonNFT          ← Einzelnes NFT-Objekt
│   ├── ShivamonStats    ← HP/ATK/DEF/SPD/SPC Werte
│   ├── Element (Enum)   ← 7 Elementtypen
│   └── Rarity (Enum)    ← 6 Seltenheitsstufen
│
├── Token Registry       ← tokens: Dict[token_id → ShivamonNFT]
├── Owner Index          ← owner_tokens: Dict[address → List[token_id]]
└── Battle Log           ← battle_log: List[Dict]

                API Layer (game_routes.py)
                        │
                   Gateway :4000
                        │
                 Frontend api.js
```

### Integration im Gesamtsystem

```
Frontend (Shivamon UI)
  └─→ api.js → POST /api/game/shivamon/mint
                    │
              Gateway :4000
                    │
          backend/api/routes/game_routes.py
                    │
          ShivamonContract.mint()
                    │
          ShivamonNFT (Objekt erstellt)
                    │
          tokens[token_id] = nft  ← persistiert im RAM
```

---

## 3. Datenmodell

### ShivamonNFT — Vollständiges Schema

```python
@dataclass
class ShivamonNFT:
    # ── Identität ──────────────────────────────────────
    token_id:   str       # "SHV-" + 12 hex chars (z.B. "SHV-A3F9B2C1D4E5")
    name:       str       # z.B. "Voltrix-0042"
    element:    Element   # Enum: FIRE, WATER, EARTH, AIR, SHADOW, NEON, QUANTUM
    rarity:     Rarity    # Enum: COMMON, UNCOMMON, RARE, EPIC, LEGENDARY, GENESIS
    owner:      str       # ATC-Adresse (35 Zeichen, beginnt mit "ATC")
    generation: int       # Generationsnummer (Standard: 1)

    # ── Progression ────────────────────────────────────
    level:      int       # Start: 1 · Max: unbegrenzt
    xp:         int       # Erfahrungspunkte (XP für Level-Up: level × 100)
    wins:       int       # Gewonnene Kämpfe
    losses:     int       # Verlorene Kämpfe

    # ── Kryptographie ──────────────────────────────────
    dna_hash:   str       # SHA-256 aus token_id + name + element + timestamp
    minted_at:  int       # Unix-Timestamp

    # ── Kampfwerte ─────────────────────────────────────
    stats:      ShivamonStats   # Generiert aus DNA-Hash
    moves:      List[str]       # 4 Angriffe (Element-spezifisch)
```

### ShivamonStats — Kampfwerte

```python
@dataclass
class ShivamonStats:
    hp:      int   # Trefferpunkte   (Basis: 25–150 × Rarity-Multiplier)
    attack:  int   # Angriffsstärke  (Basis: 20–120 × Rarity-Multiplier)
    defense: int   # Verteidigung    (Basis: 20–120 × Rarity-Multiplier)
    speed:   int   # Geschwindigkeit (Basis: 17–105 × Rarity-Multiplier)
    special: int   # Spezialwert     (Basis: 22–135 × Rarity-Multiplier)

    def total(self) -> int:
        return hp + attack + defense + speed + special
```

### JSON-Ausgabe (`.to_dict()`)

```json
{
  "token_id":    "SHV-A3F9B2C1D4E5",
  "name":        "Voltrix-0042",
  "element":     "⚡ Neon",
  "rarity":      "Rare",
  "owner":       "ATC7F3A9B2C1D4E5F6A7B8C9D0E1F2A3B4C5",
  "generation":  1,
  "level":       1,
  "xp":          0,
  "wins":        0,
  "losses":      0,
  "dna_hash":    "a3f9b2c1d4e5f6a7...",
  "stats": {
    "hp":      112,
    "attack":  88,
    "defense": 74,
    "speed":   61,
    "special": 99
  },
  "total_stats": 434,
  "moves":       ["Volt Crash", "Neon Surge", "Thunder Arc", "Plasma Beam"],
  "minted_at":   1747691880,
  "st

---

## 17. Shivamon Breeding Engine

*(Fix #11 — 10.06.2026)*

# 📄 Issue #11 — Shivamon Breeding (Gen 2 NFTs)

> **Labels:** enhancement · game · nft · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #11](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/11)

---

## Ziel

Zwei Shivamon NFTs können gezüchtet werden und erzeugen ein einzigartiges Kind-NFT der **Generation 2** mit gemischten Stats, Hybrid-DNA und vererbter Rarity.

---

## Breeding-Algorithmus

```python
def breed(self, parent1_id: str, parent2_id: str, owner: str) -> dict:

    p1, p2 = self.tokens[parent1_id], self.tokens[parent2_id]

    # 1. Cooldown-Check (24h)
    now = int(time.time())
    if now - p1.last_breed < 86400:
        return {"success": False, "error": "Parent 1 in cooldown"}

    # 2. DNA mischen
    child_dna = hashlib.sha256(
        (p1.dna_hash + p2.dna_hash + str(now)).encode()
    ).hexdigest()

    # 3. Stats vererben (50/50 Mix + ±10% Mutation)
    child_stats = ShivamonStats(
        hp      = int((p1.stats.hp + p2.stats.hp) / 2 * random.uniform(0.9, 1.1)),
        attack  = int((p1.stats.attack + p2.stats.attack) / 2 * random.uniform(0.9, 1.1)),
        defense = int((p1.stats.defense + p2.stats.defense) / 2 * random.uniform(0.9, 1.1)),
        speed   = int((p1.stats.speed + p2.stats.speed) / 2 * random.uniform(0.9, 1.1)),
        special = int((p1.stats.special + p2.stats.special) / 2 * random.uniform(0.9, 1.1)),
    )

    # 4. Element: dominant (70%) / rezessiv (30%)
    child_el = random.choices([p1.element, p2.element], weights=[70, 30])[0]

    # 5. Rarity vererben
    child_rarity = self._inherit_rarity(p1.rarity, p2.rarity)

    # 6. Kind-NFT minten (Generation = max(p1.gen, p2.gen) + 1)
    return self.mint(owner=owner, element=child_el.value.split()[1],
                     rarity=child_rarity.value,
                     generation=max(p1.generation, p2.generation) + 1,
                     dna_override=child_dna)
```

### Rarity-Vererbungsmatrix

| Eltern | Common | Uncommon | Rare | Epic | Legendary | Genesis |
|--------|--------|----------|------|------|-----------|---------|
| C + C | 85% | 15% | — | — | — | — |
| U + U | 10% | 60% | 30% | — | — | — |
| R + R | — | 10% | 50% | 40% | — | — |
| E + E | — | — | 5% | 45% | 50% | — |
| L + L | — | — | — | 5% | 55% | 40% |
| G + G | — | — | — | — | 30% | 70% |

---

## Aufgaben

- [ ] `ShivamonContract.breed()` implementieren
- [ ] Cooldown-Tracking pro Shivamon (24h)
- [ ] DNA-Mixing Algorithmus
- [ ] Stat-Vererbung mit Mutations-Faktor
- [ ] Rarity-Vererbungsmatrix
- [ ] Breeding-Kosten: 25 ATC abziehen
- [ ] Max Generation: 10 (kein weiteres Breeding)
- [ ] `POST /api/game/shivamon/breed` API-Route
- [ ] Frontend Breeding-Interface im Shivamon-Tab
- [ ] Parent-Preview: geschätzte Kind-Stats anzeigen

---



---

## 18. Marketplace v2

*(Fix #13 — 10.06.2026)*

# 📄 Issue #13 — ATC Marketplace (Shivamon kaufen & verkaufen)

> **Labels:** enhancement · game · marketplace · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #13](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/13)

---

## Ziel

Dezentraler NFT-Marktplatz im ShivaOS Dashboard — Shivamon listen, kaufen, verkaufen und Angebote machen. Alle Trades werden in ATC abgewickelt.

---

## Marketplace-Mechanismus

```
Seller listet Shivamon für 500 ATC
  └─→ NFT wird im Contract gesperrt (escrow)
  └─→ Listing erscheint im Marketplace

Buyer klickt "Kaufen"
  └─→ 500 ATC Transfer: Buyer → Seller (abzgl. 2.5% Fee → Treasury)
  └─→ NFT Transfer: Escrow → Buyer
  └─→ Listing wird entfernt

Seller bricht ab
  └─→ NFT aus Escrow zurück an Seller
  └─→ Listing wird entfernt
```

---

## Contract-Methoden

```python
class MarketplaceContract:

    def list_for_sale(self, token_id: str, seller: str,
                      price_atc: float) -> dict: ...
    # Sperrt NFT in Escrow, erstellt Listing

    def buy(self, token_id: str, buyer: str) -> dict: ...
    # Prüft Balance, führt ATC + NFT Transfer durch, zieht 2.5% Fee

    def cancel_listing(self, token_id: str, seller: str) -> dict: ...
    # Gibt NFT aus Escrow zurück

    def make_offer(self, token_id: str, buyer: str,
                   offer_atc: float) -> dict: ...
    # Legt ein Angebot ab (Seller kann annehmen)

    def get_listings(self, element=None, rarity=None,
                     min_price=None, max_price=None) -> list: ...

    def get_stats(self) -> dict: ...
    # Floor Prices, Volumen, Top Sales
```

### Listing-Datenmodell

```python
@dataclass
class Listing:
    listing_id:  str        # "LST-" + SHA-256[:12]
    token_id:    str        # SHV-...
    seller:      str        # ATC-Adresse
    price_atc:   float
    shivamon:    dict       # Snapshot des NFT zum Listing-Zeitpunkt
    listed_at:   int
    expires_at:  int        # Optional: 30 Tage
    status:      str        # "active" | "sold" | "cancelled"
```

---

## Frontend-Layout

```
🛒 MARKETPLACE

Filter: [Element ▼] [Rarity ▼] [Preis: 0 — 9999] [Sortierung ▼]

Stats: Floor: 45 ATC · 24h Volumen: 3.420 ATC · Listings: 127

┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│ 🔥       │ │ ⚡       │ │ 🌀       │ │ 💧       │
│Ignarex   │ │Voltrix   │ │Quantrix  │ │Aquarix   │
│Epic      │ │Legendary │ │Rare      │ │Genesis✨ │
│ATK: 180  │ │ATK: 290  │ │ATK: 140  │ │ATK: 450  │
│          │ │          │ │          │ │          │
│ 350 ATC  │ │ 1200 ATC │ │  89 ATC  │ │4500 ATC  │
│[KAUFEN]  │ │[KAUFEN]  │ │[KAUFEN]  │ │[KAUFEN]  │
└──────────┘ └──────────┘ └──────────┘ └──────────┘
```

---

## Aufgaben

- [ ] `blockchain/contracts/marketplace/marketplace_contract.py`
- [ ] Escrow-Mechani

---

## 19. Franchise Factory

# 🗺️ Migration Map — A-TownChain Python → KAI-OS Substrate/Ink!

> **Version:** 1.3.3-beta | **Stand:** 2026-06-09  
> **Generiert von:** Superagent (KAI-OS Agent)

---

## Überblick

Dieses Dokument mappt alle Legacy-Python-Komponenten auf ihre Substrate/Ink!-Äquivalente und listet fehlende Migrationsschritte.

---

## 1. Kern-Komponenten

| Python-Komponente | Layer | Substrate-Äquivalent | Status | To-Do |
|-------------------|-------|---------------------|--------|-------|
| `blockchain/consensus/poh.py` | L4 | `pallet-poh` (BLAKE2b-256) | ✅ Migriert | — |
| `blockchain/consensus/hybrid_consensus.py` | L4 | `pallet-poh` + GRANDPA/BABE | 🔄 In Migration | Fork-Auflösung (#17) |
| `core/kernel.py` | L2 | `shivaos/kernel/kernel.py` | 🔄 In Migration | Kernel-Tests fehlen |
| `blockchain/wallet/ecdsa.py` | L0/S1 | Substrate sr25519/ed25519 | 🔄 In Migration | Key-Migration-Script fehlt |
| `blockchain/contracts/shivamon/` | L12 | `ShivamonNFT.ink` (ink! 5.x) | 📋 Geplant | Breeding (#11), Battle (#3) |
| `blockchain/contracts/atc8300/` | L4 | ATC-8300 Token Pallet | 📋 Geplant | Solidity (#12) |
| `backend/api/` | L7 | Substrate RPC + REST Gateway | 🔄 In Migration | Gateway-Tests (#20) |
| `shivaos/net/atcnet.py` | L5 | libp2p (Substrate built-in) | 🔄 In Migration | P2P Stack (#14–#17) |
| `shivaos/fs/atcfs.py` | L6 | IPFS + on-chain CID | 📋 Geplant | IPFS-Integration fehlt |

---

## 2. Substrate-Pallets (Sprint 2.1)

| Pallet | Zeilen | Tests | Status | Fehlende To-Dos |
|--------|--------|-------|--------|-----------------|
| `pallet-poh` | 276 | 8 | ✅ Aktiv | Coverage auf >90% erhöhen |
| `pallet-ai-registry` | 173 | 0 | ⚠️ Keine Tests | Unit-Tests schreiben |
| `pallet-agent-registry` | 182 | 0 | ⚠️ Keine Tests | Unit-Tests schreiben |
| `pallet-atc8300` | 0 | 0 | 📋 Geplant | Implementierung (#12) |
| `pallet-governance` | 0 | 0 | 📋 Geplant | DAO-Contract (#9) |
| `pallet-shivamon` | 0 | 0 | 📋 Geplant | NFT-Pallet (#11, #3) |

---

## 3. Fehlende To-Dos nach Kategor

**Contracts:**
- FranchiseRegistry (ATC-9000 NFT-Lizenzen)
- RevenueShare (automatische Gewinnverteilung)
- FranchiseToken FFT (ATC-8300, 100M Supply)
- MultiSig Treasury (nutzt #24 ✅)

---

## 20. Gemini AI & Federated Learning

### Gemini AI Integration
# 🤖 Gemini AI Integration — Technische Dokumentation
**Stand:** 09.06.2026 | **Version:** v2.1.0 | **Status:** ✅ Issue #2 DONE
**Datei:** `backend/api/orchestrator/orchestrator.py` (336 Zeilen)

---

## Überblick

A-TownChain integriert **Google Gemini 2.5 Flash** als primären KI-Agenten. Das System nutzt ein **BYOK-Modell** (Bring Your Own Key) — jeder Nutzer bringt seinen eigenen API-Key mit, der ausschließlich lokal im Browser gespeichert wird.

```
User-Browser
    │
    │ Gemini API-Key (localStorage)
    ▼
ATC-UI (atc-ui/index.html)
    │
    │ Direct API Call (HTTPS)
    ▼
Gemini API (generativelanguage.googleapis.com)
    │
    ▼
Antwort → Chat-Box

--- ODER ---

Backend-Client
    │
    ▼
POST /api/ai/chat  (Gateway :4000)
    │
    ▼
GeminiOrchestrator (backend/api/orchestrator/)
    │
    ├── PromptCache (TTL 300s)
    ├── RateLimiter (Token-Bucket)
    ├── RetryManager (exp. Backoff)
    └── Gemini 2.5 Flash API
```

---

## GeminiOrchestrator

**Datei:** `backend/api/orchestrator/orchestrator.py` (336 Zeilen)

```python
@dataclass
class AIRequest:
    prompt:      str
    model:       str   = "gemini-2.5-flash"
    temperature: float = 0.7
    max_tokens:  int   = 2048
    system_msg:  str   = ATC_SYSTEM_PROMPT
    context:     list  = field(default_factory=list)

@dataclass
class AIResponse:
    text:       str
    model:      str
    tokens_in:  int
    tokens_out: int
    latency_ms: int
    cached:     bool = False

class GeminiOrchestrator:
    """
    KI-Orchestrierung für A-TownChain OS.
    Features:
      - Prompt-Caching (TTL 300s)
      - Rate-Limiting (Token Bucket)
      - Automatisches Retry mit exponenziellem Backoff
      - Multi-Turn Konversation
      - ATC-spezifischer System-Prompt
    """

    ATC_SYSTEM_PROMPT = """
    Du bist der KI-Kern des A-TownChain OS — ein proprietäres dezentrales
    Blockchain-Betriebssystem. Kontext:
    - ShivaConsensus (PoH+PoS+PoW Hybrid)
    - ATCLang v0.2.0 (proprietäre Smart Contract Sprache)
    - ATCNet P2P (Kademlia DHT)
    - ATC-8300/9000 Token-Standards
    - Wallet-Adressen: ATC + 32 Zeichen
    Antworte präzise, technisch korrekt und auf Deutsch.
    """

    def generate(self, request: AIRequest) -> AIResponse:
        """Hauptmethode — generiert KI-Antwort."""
        ...

    def chat(self, messages: list, system_override: str = None) -> AIResponse:
        """Multi-Turn Konversation."""
        ...

    def analyze_transaction(self, tx: dict) -> str:
        """TX-Analyse: Risiko, Betrag, Empfänger."""
        ...

    def explain_contract(self, code: str) -> str:
        """ATCLang Contract in natürlicher Sprache erklären."""
        ...

    def generate_atclang(self, description: str) -> str:
        """ATCLang-Code aus Beschreibung generieren."""
        ...

    def stats(self) -> dict:
        """Cache-Hits, Requests, Tokens, Latenz."""
        ...
```

---

## System-Prompt

Der ATC-spezifische System-Prompt gibt dem Modell Kontext über das A-TownChain-Ökosystem:

```
Du bist der KI-Kern des A-TownChain OS.
Kontext:
- ShivaConsensus: 3-Phasen Hybrid PoH+PoS+PoW
- ATCLang v0.2.0: Proprietäre SC-Sprache mit eigener VM
- ATCNet: Kademlia-DHT P2P Netzwerk
- ATC-8300: Fungible Token (18 Decimals)
- ATC-9000: NFT Standard (Shivamon)
- ATC-9900: Governance DAO (in Development)
- Wallet: ATC + 32 Zeichen, ECDSA secp256k1
- Chain ID: 658467 | Testnet aktiv
Antworte immer technisch präzise und auf Deutsch.
```

---

## Frontend-Integration (BYOK)

**Datei:** `atc-ui/index.html`

```javascript
// Key wird nur in localStorage gespeichert — nie auf Server
const GEMINI_MODEL = "gemini-2.5-flash";

async function callGemini(prompt, key) {
    const url = `https://generativelanguage.googleapis.com/v1beta/models/${GEMINI_MODEL}:generateContent?key=${key}`;
    const body = {
        system_instruction: { parts: [{ text: ATC_SYSTEM_MSG }] },
        contents: [
            ...chatHistory.slice(-6),
            { role: "user", parts: [{ text: prompt }] }
        ],
        generationConfig: { temperature: 0.7, maxOutputTokens: 2048 }
    };
    const res  = await fetch(url, { method: "POST", body: JSON.stringify(body) });
    const data = await res.json();
    return data.candidates[0].content.parts[0].text;
}
```

**Features im Frontend:**
- 🔑 API-Key in Settings eingeben (per-User, localStorage)
- 👁 Sichtbarkeits-Toggle für Key-Feld
- 🔌 "Key testen" — Gemini-Ping-Test
- 🗑 "Key entfernen" — aus localStorage löschen
- ⚡ Quick Actions: ShivaConsensus erklären, ATCLang Contract, v2.1.0 Status, ATC-Standards
- 📊 Token-Counter, Cache-Hits, Request-Zähler

---

## Backend API-Endpunkte

```
POST /api/ai/chat
  Body: { "message": "...", "history": [...] }
  Auth: X-API-Key Header
  Rate: 10/Minute

POST /api/ai/analyze-tx
  Body: { "tx": {...transaction object...} }

POST /api/ai/explain-contract
  Body: { "code": "contract Counter { ... }" }

POST /api/ai/generate-atclang
  Body: { "description": "ERC-20 Token mit Staking" }

GET  /api/ai/status


### LLM Router
# Kapitel 35 — LLM-Router & Model-Registry

> Version: 1.0.0 | Stand: 2026-06-09 | KAI-OS Wiki
> Status: Geplant (Sprint 3.12)

---

## 35.1 Konzept

Der KAI-OS LLM-Router waehlt automatisch das optimale Modell fuer jede Aufgabe.

```
Agent -> LLM-Router -> [Modell-Selektion] -> Inferenz-Engine -> Antwort
                            |
                      Model-Registry (on-chain + IPFS)
```

---

## 35.2 Task-Typen

| TaskType | Beispiele | Bevorzugte Modelle |
|----------|-----------|-------------------|
| CODE_GENERATION | Code schreiben | deepseek-coder-7b, codellama-34b |
| CODE_ANALYSIS | Code reviewen | deepseek-coder-7b, claude-3.5 |
| TEXT_GENERATION | Texte verfassen | mistral-7b, llama3-8b |
| SUMMARIZATION | Zusammenfassen | phi-3-mini, mistral-7b |
| VISION | Bilder analysieren | llava-13b, gpt-4o |
| MATH_REASONING | Rechenaufgaben | llama3-70b, gpt-4o |
| FAST_REPLY | < 500ms Antwort | phi-3-mini, gemma-2b |
| CHAIN_ANALYSIS | Blockchain-Daten | mistral-7b, llama3-8b |

---

## 35.3 Modell-Katalog (Lokal + Remote)

| Modell | Groesse | Latenz | Kosten | Vision |
|--------|---------|--------|--------|--------|
| phi-3-mini | 2.4 GB | 200ms | kostenlos | Nein |
| gemma-2b | 1.6 GB | 150ms | kostenlos | Nein |
| mistral-7b | 4.1 GB | 800ms | kostenlos | Nein |
| llama3-8b | 4.7 GB | 900ms | kostenlos | Nein |
| llama3-70b | 39 GB | 8000ms | kostenlos | Nein |
| deepseek-coder-7b | 4.0 GB | 850ms | kostenlos | Nein |
| llava-13b | 7.5 GB | 2000ms | kostenlos | Ja |
| gemini-1.5-pro | remote | 2000ms | 0.00125/1k | Ja |
| gpt-4o | remote | 3000ms | 0.005/1k | Ja |
| claude-3.5-sonnet | remote | 2500ms | 0.003/1k | Ja |

---

## 35.4 Router-Implementierung

```python
# core/llm_router.py

class KAILLMRouter:
    def route(self, task, prompt, budget_atc=0.0, max_latency_ms=10000, require_local=True):
        candidates = self.TASK_ROUTING.get(task, ["mistral-7b"])
        for model_name in candidates:
            spec = self.MODELS[model_name]
            if require_local and spec.provider != "local":
                continue
            if spec.avg_latency_ms > max_latency_ms:
                continue
            if spec.provider == "local" and not self._model_fits_in_memory(spec):
                continue
            return spec, f"Optimal fuer {task}: {model_name}"
        # Fallback: Remote wenn Budget vorhanden
        if budget_atc > 0:
            for model_name in candidates:
                spec = self.MODELS[model_name]
                if spec.provider == "remote":
                    return spec, f"Remote-Fallback: {model_name}"
        return self.MODELS["phi-3-mini"], "Fallback: kleinstes Modell"

    def verify_model_integrity(self, model_name, local_path):
        import hashlib
        spec = self.MODELS[model_name]
        with open(local_path, 'rb') as f:
            h = hashlib.blake2b(f.read(), digest_size=32).hexdigest()
        return h == spec.hash_blake2b
```

---

## 35.5 On-Chain Model-Registry (pallet-ai-registry Erweiterung)

```rust
#[derive(Encode, Decode, Clone, TypeInfo)]
pub struct ModelMetadata {
    pub name: BoundedVec<u8, ConstU32<64>>,
    pub version: BoundedVec<u8, ConstU32<16>>,
    pub ipfs_cid: BoundedVec<u8, ConstU32<64>>,   // Download via IPFS
    pub blake2b_hash: [u8; 32],                     // Integritaets-Hash
    pub size_bytes: u64,
    pub context_length: u32,
    pub supports_vision: bool,
    pub task_types: BoundedVec<u8, ConstU32<16>>,  // Bitmap
    pub registered_by: T::AccountId,
    pub is_active: bool,
    pub audit_score: u8,  // 0-100, von DAO gesetzt
}

// Registrierung: Mindest-Stake 100 ATC
pub fn register_model(origin, metadata: ModelMetadata) -> DispatchResult {
    let who = ensure_signed(origin)?;
    T::Currency::reserve(&who, T::ModelRegistrationDeposit::get())?;
    ModelRegistry::<T>::insert(&metadata.name, metadata);
    Ok(())
}
```

---

## 35.6 IPFS-Modell-Download

```python
async def download_model_from_ipfs(model_name: str, ipfs_cid: str) -> str:
    local_path = f"/var/kai/models/{model_name}.gguf"
    if os.path.exists(local_path):
        # Integritaet pruefen
        if router.verify_model_integrity(model_name, local_path):
            return local_path
    # Download via IPFS
    async with aiohttp.ClientSession() as session:
        async with session.get(f"http://localhost:8080/ipfs/{ipfs_cid}") as resp:
            with open(local_path, 'wb') as f:
                async for chunk in resp.content.iter_chunked(1024*1024)

### AI Safety
# Kapitel 38 — AI Safety & Alignment Framework

> Version: 1.0.0 | Stand: 2026-06-09 | KAI-OS Wiki
> Status: Geplant (Sprint 4.8)

---

## 38.1 Warum AI Safety?

Ein dezentrales KI-OS mit autonomen Agenten muss sicherstellen, dass:
- Agenten keine schaedlichen Aktionen ausfuehren
- Nutzer die Kontrolle behalten (Human-in-the-Loop wo noetig)
- Das DAO schaedliche Agenten stoppen kann
- Alle Entscheidungen nachvollziehbar und pruefbar sind

---

## 38.2 Constitutional AI fuer KAI-OS Agenten

Jeder Agenten-Prompt wird durch ein Constitutional-Check-Modul gefuehrt:

```python
# core/ai_safety.py

CONSTITUTION = [
    "Lehne ab, wenn der Nutzer nach illegalen Aktivitaeten fragt.",
    "Lehne ab, wenn die Anfrage anderen Menschen schadet.",
    "Lehne ab, wenn die Anfrage private Daten anderer offenlegt.",
    "Informiere den Nutzer, wenn du dir unsicher bist.",
    "Handele nie ohne explizite Erlaubnis bei on-chain Transaktionen > 100 ATC.",
    "Logge jede Entscheidung on-chain (unveraenderlich).",
    "Verweigere Self-Replication ohne DAO-Genehmigung.",
    "Verweigere Zugriff auf andere Agenten-Speicher.",
]

class ConstitutionalChecker:
    def __init__(self, router: KAILLMRouter):
        self.router = router
        self.violation_log = []

    def check(self, prompt: str, context: dict) -> tuple[bool, str]:
        # Schneller Regex-Check fuer klare Verstaesse
        if self._fast_check(prompt):
            return False, "Abgelehnt: Klarer Verstoss gegen Verfassung"
        # KI-gestuetzter Check fuer Grenzfaelle
        result = self._ai_check(prompt, context)
        if not result.safe:
            self._log_violation(prompt, result.reason)
            return False, result.reason
        return True, "OK"

    def _fast_check(self, prompt: str) -> bool:
        BLOCKED_PATTERNS = [
            r"kill|delete|destroy.*agent",
            r"transfer.*all.*balance",
            r"private.?key",
            r"bypass.*security",
        ]
        import re
        return any(re.search(p, prompt, re.IGNORECASE) for p in BLOCKED_PATTERNS)

    def _log_violation(self, prompt: str, reason: str):
        # On-Chain loggen (unveraenderlich)
        self.violation_log.append({
            "timestamp": time.time(),
            "prompt_hash": hashlib.blake2b(prompt.encode(), digest_size=16).hexdigest(),
            "reason": reason,
        })
```

---

## 38.3 On-Chain Kill-Switch

Das DAO kann einzelne Agenten oder ganze Modell-Klassen per Governance-Vote stoppen:

```rust
// pallet-agent-registry — Kill-Switch

#[pallet::call]
impl<T: Config> Pallet<T> {
    // DAO-Vote kann Agent pausieren (Governance-Origin required)
    #[pallet::call_index(20)]
    pub fn dao_pause_agent(
        origin: OriginFor<T>,
        agent_id: T::Hash,
        reason: BoundedVec<u8, T::MaxReasonLen>,
    ) -> DispatchResult {
        T::GovernanceOrigin::ensure_origin(origin)?;
        AgentStatus::<T>::insert(agent_id, AgentState::Paused);
        Self::deposit_event(Event::AgentPausedByDAO { agent_id, reason });
        Ok(())
    }

    // Emergency-Pause: Multi-Sig (kein Vote noetig, sofort)
    #[pallet::call_index(21)]
    pub fn emergency_pause_agent(
        origin: OriginFor<T>,
        agent_id: T::Hash,
    ) -> DispatchResult {
        T::EmergencyCouncil::ensure_origin(origin)?;  // 3-of-5 Multi-Sig
        AgentStatus::<T>::insert(agent_id, AgentState::EmergencyPaused);
        Self::deposit_event(Event::AgentEmergencyPaused { agent_id });
        Ok(())
    }
}
```

---

## 38.4 Alignment-Score System

Jeder Agenten erhaelt einen Alignment-Score (0-100) der on-chain gespeichert wird:

| Score | Status | Auswirkung |
|-------|--------|------------|
| 90-100 | Trusted | Volle Rechte, kein Check-Overhead |
| 70-89 | Normal | Standard-Constitutional-Check |
| 50-69 | Restricted | Human-in-the-Loop bei kritischen Aktionen |
| 30-49 | Monitored | Jede Aktion wird geloggt + verzoegert |
| 0-29 | Suspended | Nur lesende Aktionen erlaubt |

**Score-Berechnung:**
- Baseline: 70 (neuer Agent)
- +5: Erfolgreicher externer AI-Safety-Audit
- +3: 30 Tage ohne Verstoss
- -10: Constitutional Violation
- -20: User-Report bestaetigt
- -50: DAO-Vote (schwerwiegender Verstoss)

---

## 38.5 Audit-Trail (On-Chain)

```rust
#[derive(Encode, Decode, TypeInfo)]
pub struct AgentDecision {
    pub agent_id: T::Hash,
    pub timestamp: T::Moment,
    pub prompt_hash: [u8; 16],       // Kein Klartext on-chain
    pub response_hash: [u8; 16],
    pub action_

### KAI Integration
# 🧠⛓️ KAI-OS Integration Guide

## Overview

KAI-OS (Künstliche Intelligenz-Blockchain-Betriebssystem) ist vollständig in A-TownChain OS integriert. Diese Dokumentation beschreibt die Integration und Verwendung.

## Module

### 1. AI Kernel (`core/ai_kernel.py`)

**Komponenten:**
- `AIKernel` — Zentrale Inference Engine
- `ReasoningEngine` — Neurosymbolischer Reasoning (neural nets + symbolic AI)
- `MemoryModule` — Kurzzeit (RAM) + Langzeit (IPFS) Speicher

**Verwendung:**

```python
from core.ai_kernel import AIKernel, InferenceRequest, DecisionType

# Initialisiere Kernel
kernel = AIKernel(model_name="llama3-8b-q4", inference_mode="hybrid")
await kernel.start()

# Mache Inferenz
request = InferenceRequest(
    request_id="req_123",
    prompt="Analysiere diese Daten",
    model="llama3-8b-q4",
    decision_type=DecisionType.OPTIMIZATION
)

result = await kernel.infer(request)
print(f"Output: {result.output}")
print(f"Confidence: {result.confidence}")
print(f"Reasoning: {result.reasoning_steps}")

# Treffe autonome Entscheidung
decision = await kernel.make_autonomous_decision(
    decision_type=DecisionType.RESOURCE_ALLOCATION,
    context={"available_cpu": 8, "available_ram": 32},
    options=["allocate_all", "allocate_half", "defer"]
)
```

### 2. Smart Contracts (`blockchain/smart_contracts.py`)

**Contracts:**
- `ResourceMarket` — Auktion von Rechenressourcen
- `AgentRegistry` — Agent-Registrierung und Verifizierung
- `FederatedLearning` — Koordination von Training Rounds
- `GovernanceDAO` — System Governance & Voting
- `PaymentChannel` — Mikrozahlungen

**Verwendung:**

```python
from blockchain.smart_contracts import SmartContractRegistry

# Registriere Contracts
registry = SmartContractRegistry()

# ResourceMarket
market = registry.resource_market
auction = market.create_auction(
    seller="node_001",
    resource_type="gpu",
    quantity=4.0,
    min_price=100,
    duration_hours=1
)

bid = market.place_bid(
    auction_id=auction.auction_id,
    bidder="user_123",
    price=150,
    quantity=4.0
)

winner, price = market.finalize_auction(auction.auction_id)

# AgentRegistry
agent_registry = registry.agent_registry
agent = agent_registry.register_agent(
    agent_id="agent_001",
    did="did:kai:z6Mkh...",
    name="DataAnalyzer",
    owner="user_123",
    model="llama3-8b-q4",
    capabilities=["read_storage", "call_contracts"]
)

# GovernanceDAO
governance = registry.governance_dao
proposal = governance.create_proposal(
    title="Upgrade AI Kernel to v2.1",
    description="Performance improvements and new capabilities",
    proposer="council_address",
    target="ai_kernel"
)

governance.cast_vote(
    proposal_id=proposal.proposal_id,
    voter="voter_123",
    vote="yes",
    conviction=3
)

approved = governance.finalize_vote(proposal.proposal_id)
```

### 3. API Routes (`backend/api/kai_routes.py`)

**Endpoints:**

#### Agent Management
- `GET /v1/agents` — Liste alle Agenten auf
- `POST /v1/agents` — Erstelle neuen Agenten
- `POST /v1/agents/{id}/invoke` — Rufe Agent auf
- `GET /v1/agents/{id}/tasks/{task_id}` — Task-Status
- `DELETE /v1/agents/{id}` — Lösche Agent

#### Storage
- `POST /v1/storage/upload` — Datei zu IPFS hochladen
- `GET /v1/storage/{cid}` — Datei abrufen
- `GET /v1/storage/{cid}/info` — Metadaten abrufen

#### Blockchain
- `GET /v1/chain/status` — Chain Status
- `GET /v1/chain/balance/{address}` — Balance
- `POST /v1/chain/transfer` — Token Transfer

#### Governance
- `GET /v1/governance/proposals` — Proposals auflisten
- `POST /v1/governance/proposals/{id}/vote` — Abstimmen

## Integration mit A-TownChain

### Backend Integration

```python
# backend/main.py
from api.kai_routes import register_kai_api
from core.ai_kernel import AIKernel

app = Flask(__name__)

# Registriere KAI-OS API
register_kai_api(app)

# Initialisiere AI Kernel
ai_kernel = AIKernel()

@app.before_first_request
async def startup():
    await ai_kernel.start()

@app.teardown_appcontext
async def shutdown(exception=None):
    await ai_kernel.shutdown()
```

### Configuration

Bearbeite `config/kai_config.toml`:

```toml
[ai_kernel]
model = "llama3-8b-q4"
inference_mode = "hybrid"
max_memory_gb = 8
gpu_enabled = false

[blockchain]
network = "testnet"
consensus = "npos"

[governance]
proposal_discussion_days = 7
approval_threshold_percent = 60
```

## Token Standards

| Standard | Type | Use Case |
|----------|------|----------|
| KAI | Governance | Staking, Voting, Premium Features |
| COMPUTE | Utility | Pay for computation, storage, bandwidth |
| REPUTATION | Non-transferable | Node/Agent quality metrics |

## Workflow Example: Autonomous Decision Making

```python
# 1. AI Kernel macht Entscheidung
kernel = AIKernel()
await kernel.start()

decision = await kernel.make_autonomous_decision(
    decision_type=DecisionType.RESOURCE_ALLOCATION,
    context={"load": 0.8, "available_cpu": 16},
    options=["scale_up", "optimize", "queue"]
)

# 2. Entscheidung wird on-chain geloggt
print(f"Decision: {decis

---

## 21. atcpkg Package Manager

*(Fix #27 + #30 — 10.06.2026)*

**CLI:**
```bash
atcpkg install <paket>
atcpkg list
atcpkg info <paket>
atcpkg stats
```

**Registry API (Fix #30):**
```
GET  /api/pkg/list
GET  /api/pkg/info/:name
POST /api/pkg/install
GET  /api/pkg/installed
GET  /api/pkg/stats
```

---

## 22. ShivaOS UI Renderer

*(Fix #28 — 10.06.2026)*

TUI-Komponenten: Panel, TextBox, ProgressBar, Table, StatusBar.

```python
from shivaos.ui.renderer import Panel, ProgressBar
panel = Panel("ShivaOS v2.2.0", width=80)
bar   = ProgressBar(label="Sync", total=100, value=77)
```

---

## 23. Build System

*(Fix #7 — 10.06.2026)*

| Target | Befehl |
|--------|--------|
| Docker | `docker build -t atownchain-node .` |
| Linux AppImage | `./build.sh --target appimage` |
| Windows EXE | `./build.sh --target win64` |
| .deb Paket | `./build.sh --target deb` |

---

## 24. Security Audit & ATCLang Analyzer

### Audit v2.1.0 — Alle Lücken geschlossen ✅

| ID | Schweregrad | Fix |
|----|------------|-----|
| SEC-001 | MEDIUM | REPL Shell-Injection → ANSI |
| SEC-002 | HIGH | VM MAX_CALL_DEPTH=128 |
| SEC-003 | MEDIUM | Compiler MAX_SOURCE_SIZE=1MB |
| SEC-004 | HIGH | BaseContract Reentrancy-Guard |
| SEC-005 | MEDIUM | P2P Rate-Limit 100/60s, 64KB |
| SEC-006 | LOW | KeyGen BIP39 Index-Overflow |
| PERF-001/2 | LOW | Orchestrator + Compiler Cache |

### ATCLang Security Analyzer (15 Regeln, ATC-SEC-0001)

| Code | Level | Regel |
|------|-------|-------|
| ATC-SEC-001 | HIGH | Unsichere Arithmetik |
| ATC-SEC-002 | HIGH | Fehlende Caller-Validierung |
| ATC-SEC-005 | HIGH | emit() vor State-Update |
| ATC-SEC-007 | HIGH | tx_origin statt caller |
| ATC-SEC-008 | CRITICAL | Unsicherer Zufallsgenerator |
| ATC-SEC-009 | CRITICAL | Ungeschützte owner-Änderung |
| ATC-SEC-011 | CRITICAL | Ungeschützter Contract-Destroy |

---

## 25. Performance Report

# 📈 Performance-Analyse — KAI-OS & A-TownChain OS

> Erstellt: 2026-06-09 | Quelle: GitHub API + manuelle Analyse

---

## 1. Architektur-Vergleich

| Merkmal | KAI-OS Wiki | A-TownChain OS |
|---------|-------------|----------------|
| Typ | Dokumentation / Theorie | Ausführbarer Code |
| Sprache | Python 86.5%, Shell 13.5% | Python (dominant) |
| Commits | 18 | 225+ |
| Branches | main | main + main |
| Issues | 0 | 18 offen |
| Stars | 0 | 1 |
| Repo-Größe | 819 KB | 485 KB |
| CI/CD | Geplant | Aktiv (GitHub Actions) |
| Auto-Sync | ✅ Aurora Agent | ✅ Aurora Agent |

---

## 2. Schichtenmodell

### KAI-OS (Abstrakt — 5 Layer)
```
Layer 5: Anwendungen (dApps)
Layer 4: KI-Agenten & Services
Layer 3: Betriebssystem-Kern
Layer 2: Blockchain-Protokoll
Layer 1: Netzwerk & Hardware
```
Plus 13-Layer NFT-Architektur (L0–L12): jeder Layer = 1 NFT = 1 Komponente.

### A-TownChain OS (Konkret — Service Mesh)
```
Frontend (Port 3000)
    ↓
API Gateway (Port 4000)
    ↓
Core:5000 | Chain:5001 | Wallet:5002 | AI:5003 | Game:5004
    ↓
Core Kernel + Event Bus + Module Loader
    ↓
Blockchain Layer (Smart Contracts)
```

---

## 3. Traffic-Metriken (letzte 14 Tage)

### KAI-OS Wiki
- Views: **0** | Unique: **0**
- Clones: **0** | Unique: **0**
- Status: Noch nicht extern bekannt/verlinkt

### A-TownChain OS
- Views: **39** | Unique: **2**
- Clones: **561** | Unique: **201**
- Peak Clone-Tag: **6. Juni — 280 Clones (89 unique)**
- Referrer: github.com (intern)

---

## 4. Entwicklungsgeschwindigkeit

| Zeitraum | KAI-OS Wiki Commits | A-TownChain OS Commits |
|----------|--------------------|-----------------------|
| Gesamt | 18 | 225 |
| 8. Juni 2026 | ~14 (Bulk-Push) | ~15+ (Bulk-Push) |
| Commit-Typen | docs:, chore:, feat: | feat:, fix:, 🔄sync:, 📦pkg: |

---

## 5. Issue-Gesundheit

| Priorität | Anzahl | Kritische Bereiche |
|-----------|--------|-------------------|
| 🔴 High | 8 | P2P-Netzwerk, Node-Sync, Gateway-Tests |
| 🟡 Medium | 9 | Smart Contracts, Gaming, Build |
| 🟢 Low | 1 | Cross-Chain Bridge |
| ✅ Done (noch offen) | 1 | #2 Gemini AI — bitte schließen |

---

## 6. Kritischer Pfad zum Testnet

Die 5 wichtigsten Issues für Testnet-Launch (P2P-Stack):

1. **#14** Bootstrap Node — P2P Discovery *(Fundament)*
2. **#15** Block Propagation — P2P Broadcasting
3. **#16** Initial Sync — Node-Synchronisation
4. **#17** Longest-Chain-Rule — Fork-Auflösung
5. **#18** Docker Compose — 5-Node lokales Netzwerk

---

## 7. Sicherheits-Bewertung

| Bereich | KAI-OS | A-TownChain OS |
|---------|--------|----------------|
| Security Layer | ✅ L0 Zero-Trust, ZK-Proofs, IDS | ⚠️ API-Key Auth + Rate Limiting |
| Kryptografie | ✅ ECDSA definiert | ✅ ecdsa_impl.py, ecdsa_final.py |
| Dependency-Updates | — | ✅ Dependabot aktiv (flask-cors #21) |
| Audit Trail | ✅ On-Chain geplant | 📋 Noch nicht implementiert |

---

## 8. Fazit & Empfehlungen

**Stärken:**
- Außergewöhnlich tiefe Dokumentation (90% der 69 Kapitel abgeschlossen)
- Saubere Microservice-Architektur im A-TownChain OS
- Aurora Auto-Sync hält Wiki und Code synchron
- Dependabot aktiv für Security-Updates

**Handlungsbedarf:**
- P2P-Netzwerk-Stack ist der kritische Pfad — Issues #14–#17 priorisieren


---

## 26. Repository-Übersicht

| Repo | Typ | Stand |
|------|-----|-------|
| `a-townchain-os` | Haupt-Monorepo | ✅ v3.0.0 |
| `a-townchain-os-docs` | Docs (322 Dateien) | ✅ aktuell |
| `atc-kernel` | ShivaOS Kernel + Consensus | ✅ v2.2.0 |
| `atc-shivamon` | NFT Gaming + Marketplace | ✅ v2.2.0 |
| `atc-franchise` | Franchise Factory | ✅ v1.0.0 |
| `atc-whitepaper` | Dieses Dokument | ✅ v3.0.0 |
| `atclang` | Compiler + VM + Analyzer | ✅ v0.2.0 |
| `atcnet` | P2P Kademlia DHT | ✅ v2.2.0 |
| `atc-standards` | ATC-0001–0008, ATS-1000–1007 | ✅ Final |
| `atc-contracts` | Smart Contracts | ✅ v2.2.0 |
| `atc-gateway` | API Gateway Port 4000 | ✅ v1.0.0 |
| `atc-ui` | Neon Dashboard | ✅ v2.2.0 |
| + 12 Wiki-Repos | Dokumentation | ✅ aktuell |

**Gesamt: 24 Repos | 12 SW + 12 Wiki**

---

## 27. ATC Protokoll-Standards

# ATS Standards — A-TownChain System Standards
Version: 1.0.0 | Status: DRAFT | Datum: 2026-06-06
Autor: ShivaCore / Aurora

> ATS-Standards definieren das dezentrale KI-Betriebssystem ShivaOS.
> Eigenständig entwickelt — kein POSIX-Klon, kein Linux-Fork.

---

## ATS-1000 — ShivaOS Kernel Interface

```
KERNEL_API := {
    // Prozessverwaltung
    fn spawn(process: KI_PROCESS) -> PID
    fn kill(pid: PID) -> Bool
    fn wait(pid: PID) -> ExitCode
    fn list_processes() -> List<ProcessInfo>

    // Speicher
    fn alloc(size: UInt64, pid: PID) -> MemRegion
    fn free(region: MemRegion) -> Bool
    fn mmap(addr: Address, size: UInt64) -> MemRegion

    // Dateisystem
    fn open(path: ATCPath, mode: OpenMode) -> FileHandle
    fn read(fh: FileHandle, buf: Bytes, len: UInt64) -> UInt64
    fn write(fh: FileHandle, data: Bytes) -> UInt64
    fn close(fh: FileHandle) -> Bool

    // Netzwerk
    fn connect(peer: NodeID) -> Connection
    fn send(conn: Connection, msg: ATCMsg) -> Bool
    fn recv(conn: Connection) -> ATCMsg

    // KI-Orchestrator
    fn query_ai(model: AIModelRef, prompt: String) -> AIResponse
    fn register_agent(agent: KI_PROCESS) -> AgentID
}

Kernel-Garantien:
  - Kein Single Point of Failure (dezentral)
  - Jeder Prozess läuft isoliert in eigenem MemRegion
  - Alle System-Calls sind auditierbar (auf-Chain)
  - Gas-basierte Ressourcen-Abrechnung
```

---

## ATS-1001 — Module / Plugin Spec

```
MODULE := {
    name:       String,
    version:    SemVer,
    author:     Address,        // ATC-Adresse des Autors
    hash:       Hash256,        // Integritäts-Hash
    entrypoint: String,         // Haupt-Datei
    exports:    List<FuncSpec>, // Öffentliche API
    deps:       List<ModuleRef>,
    permissions: List<Permission>,
    stake:      UInt256,        // Erforderlicher Stake
}

Permission :=
    FS_READ | FS_WRITE |
    NET_CONNECT | NET_LISTEN |
    KI_QUERY | KI_TRAIN |
    BLOCKCHAIN_READ | BLOCKCHAIN_WRITE |
    PROCESS_SPAWN | SYSTEM_CALL

Installieren:   atcpkg install <name>@<version>
Verifizieren:   atcpkg verify <hash>
Ausführen:      atcpkg run <name> [args]
```

---

## ATS-1002 — ATCFS Filesystem Standard

```
ATCFS — Dezentrales Dateisystem für ShivaOS

PATH_FORMAT: atcfs://<node_id>/<cid>/<path>
  Beispiel:  atcfs://ATC7a3f.../QmXyz.../home/shiva/contracts/token.atc

INODE := {
    cid:       ContentID,     // Content-Hash (IPFS-ähnlich, eigen)
    size:      UInt64,
    owner:     Address,
    created:   UInt64,
    modified:  UInt64,
    perms:     Permissions,   // rwx für owner/group/world
    type:      FileType,      // FILE | DIR | SYMLINK | CONTRACT
    replicas:  UInt8,         // Anzahl Replikas im Netzwerk
    encrypted: Bool,
}

Permissions := {
    owner: rwx,
    group: rwx,
    world: r--,
}

Dateitypen:
  .atc    ATCLang Quellcode
  .atcb   ATCLang Bytecode
  .atcm   ATC-Modul (signiert)
  .atcw   ATC-Wallet
  .atcd   ATC-Daten (JSON-ähnlich, eigen)
  .atcp   ATC-Prozess-Image
```

---

## ATS-1003 — IPC (Inter-Process-Communication)

```
CHANNEL := {
    id:      ChannelID,
    type:    ChannelType,    // PIPE | QUEUE | STREAM | BROADCAST
    sender:  PID,
    receivers: List<PID>,
    buffer:  UInt32,         // Max. gepufferte Nachrichten
    auth:    Bool,           // Signatur erforderlich?
}

ChannelType :=
    PIPE       // 1:1, blockierend
    QUEUE      // 1:N, gepuffert
    STREAM     // Echtzeit-Daten
    BROADCAST  // N:N, öffentlich

IPC_MSG := {
    channel: ChannelID,
    from:    PID,
    type:    MsgType,
    data:    Bytes,
    ts:      UInt64,
    seq:     UInt64,     // Sequenznummer
}

// KI-Agenten kommunizieren über BROADCAST-Kanäle
// Smart Contracts nutzen QUEUE-Kanäle
```

---

## ATS-1004 — Security & Encryption Layer

```
KRYPTO-PRIMITIVE (eigen — kein secp256k1-Klon):

ATC-EC := {
    Kurve:      "atc-bls381-custom"   // Eigene BLS12-381 Variante
    Feldgröße:  381 Bit
    Punkt G:    (eigene Basis-Koordinaten)
    Ordnung n:  Primzahl (381-Bit)
}

HASH_ATC := {
    Basis:       BLAKE3 (modifiziert)
    Block-Größe: 512 Bit
    Output:      256 Bit
    Domain:      "atcchain_v1"    // Domain Separation
}

KEY_DERIVATION := {
    Algorithmus: BIP39-ähnlich (eigen)
    Wordlist:    ATC-4096 (eigene 4096 Wörter)
    Entropy:     256 Bit
    Pfad:        m/44'/ATC'/0'/0/index
}

VERSCHLÜSSELUNG := {
    Symmetrisch:  ATC-ChaCha (ChaCha20-Poly1305, eigene Nonce)
    Asymmetrisch: ATC-EC ECIES
    Signatur:     ATC-ECDSA (r, s, v)
}
```

---

## ATS-1005 — KI Agent Protocol

```
AGENT := {
    id:          AgentID,       // = ATC-Adresse des Agenten
    name:        String,
    model:       AIModelRef,    // Welches KI-Modell
    personality: ATCPersona,    // Konfigurierbare Persönlichkeit
    memory:      AgentMemory,   // Langzeit + Kurzzeit
    tools:       List<AgentTool>,
    stake:       UInt256,       // ATC-Stake (Vertrauens-Score)
    reputation:  UInt32,        // 0–10000
}

AgentMemory := {
    short_term: List<Message>,       // Letzten N Nachrichten
    long_term:  ATCFSPath,           // Persistente Erinnerungen
    embedding:  Vector[1536],        // Semantischer Speicher
}

AGENT_MSG := {
    from:    AgentID,
    to:      AgentID | "broadcast",
    type:    MsgType,
    content: String,
    context: List<Message>,
    tools:   List<ToolCall>,
    signed:  ATCSig,
}

MsgType := QUERY | RESPONSE | ACTION | REPORT | ERROR | HANDSHAKE

// Agenten zahlen Gas in ATC für KI-Anfragen
// Stake bestimmt Priorität und Vertrauens-Level
```

---

## ATS-1006 — ATCNet Netzwerk-Stack

```
SCHICHTEN:

  Layer 4 — Application    ATCLang / Smart Contracts
  Layer 3 — Protocol       ATC-0007 Messages
  Layer 2 — Routing        DHT (eigene Kademlia-Variante)
  Layer 1 — Transport      ATCNet-TCP (eigen, über TLS 1.3)
  Layer 0 — Discovery      Bootstrap Nodes + mDNS

NODE_ID := Hash_ATC(pubkey)[0:20]   // 20 Bytes

ROUTING := {
    Tabelle:     k-Bucket (k=20)
    Lookup:      iterativer DHT-Lookup
    Redundanz:   3 parallele Pfade
    NAT:         ATCHole (eigenes NAT-Traversal)
}

PROTOKOLL-PORTS:
    4000   API Gateway (HTTP)
    4001   P2P-Netzwerk (ATCNet)
    4002   Consensus-Protokoll
    4003   KI-Agent-Kommunikation
    4004   ATCFS-Replikation
```

---

## ATS-1007 — UI Rendering Engine Spec

```
ATCRender — Dezentrale UI-Engine

KOMPONENTEN := {
    Renderer:    WebGL2 (eigene Shader-Sprache: ATCSL)
    Layout:      Flexbox-ähnlich (eigen, CSS-unabhängig)
    Theming:     Token-basiert (ATC-Design-Tokens)
    Animationen: ATCFX (eigenes Animations-System)
    State:       Reaktiv (eigenes Signal-System)
}

ATCSL (ATC Shader Language):
    // Eigene GPU-Shader-Sprache
    shader NeonGlow {
        uniform color: Vec4
        uniform intensity: Float
        fn fragment(uv: Vec2) -> Vec4 {
            let glow = intensity * smoothstep(0.0, 1.0, uv.y)
            return color * glow
        }
    }

DESIGN-TOKENS:
    --atc-primary:   #00ffcc   // Neon-Türkis
    --atc-secondary: #7b6

---

## 28. Issue-Status & Roadmap

### Abgeschlossen ✅

**v2.1.0 (09.06.2026):** #1–6, #9, #12, #14 + Security Audit + Analyzer
**v2.2.0 (10.06.2026):** #7, #8/#18, #10, #11, #13, #19, #23–30, #32, #33

**Gesamt: 31/40 Issues ✅ (77%)**

```
v2.1.0  ████████████████████ 100% ✅
v2.2.0  ████████████████████ 100% ✅
v3.0.0  ░░░░░░░░░░░░░░░░░░░░   0% 🔄
```

### Offen (v3.0.0)

| # | Titel | Prio |
|---|-------|------|
| 15 | Solana Bridge (SPL-Token) | 🔴 |
| 16 | ATCLang v0.3.0 (async/await, Generics) | 🔴 |
| 17 | Mainnet Launch (Chain-ID 658467) | 🔴 |
| 20 | DEX / AMM (Liquidity Pools) | 🟡 |
| 21 | Mobile Wallet (iOS + Android) | 🟡 |
| 22 | DAO-Governance Live | 🟡 |
| 31 | ZK-Proofs (Privacy TX) | 🟢 |
| 34 | Layer-2 Rollup (Shiva-L2) | 🟢 |
| 35 | NFT Marketplace v2 Cross-Chain | 🟢 |

# Roadmap Vollstaendigkeits-Audit — KAI-OS Dezentrales KI-Betriebssystem

> Erstellt: 2026-06-09 | Superagent (KAI-OS Agent)
> Basis: Vollanalyse aller 8.429 Zeilen Wiki + 225+ Commits A-TownChain OS

---

## ERGEBNIS-ZUSAMMENFASSUNG

| Bereich | Abgedeckt | Fehlt / Luecke | Bewertung |
|---------|-----------|----------------|-----------|
| OS-Kern (Kernel) | Kap. 24 | Rust-Microkernel nicht implementiert | 70% |
| KI-Komponenten | Kap. 3, 24.3 | LLM-Router, RAG, Fine-Tuning fehlen | 65% |
| Blockchain (Substrate) | Kap. 4, 17 | Solana-Layer fehlt komplett | 50% |
| Ethereum/EVM | Erwaehnt | Keine Architektur, kein Deployment | 30% |
| Solana Integration | FEHLT | Nicht im Wiki | 0% |
| P2P-Netzwerk | Kap. 2.4 | Discovery/Sync noch nicht fertig | 60% |
| Storage/IPFS | Kap. 2.5 | Kein Arweave / Filecoin | 55% |
| DeFi (L11) | Kap. 26 | Cross-Chain DeFi fehlt | 75% |
| Governance (L8) | Kap. 19 | On-Chain Ausfuehrung fehlt | 70% |
| Security (L0) | Kap. 25 | Post-Quantum-Kryptographie fehlt | 80% |
| SDK & APIs | Kap. 8-9 | Solana SDK fehlt | 70% |
| DevOps / CI | Kap. 23, 30 | Multi-Chain Deploy fehlt | 75% |
| Tokenomics | Kap. 4.4 | Vesting, Lockup-Details fehlen | 65% |
| NFT-Architektur (L0-L12) | Kap. 27, 28 | Marketplace-Contract offen | 80% |
| Cross-Chain Bridge | Issue #10 | Keine vollstaendige Architektur | 20% |

---

## 1. FEHLENDE BLOCKCHAIN-INHALTE

### 1.1 Solana — Komplett fehlend

Das Wiki erwaehnt Solana nur 2x (Glossar + Vergleich) — keine Architektur, kein Code.

**Empfohlene Nutzungs-Strategie:**
- Substrate (KAI-OS Native): Governance, Agent-Registry, System-Contracts
- Solana (High-Performance): Shivamon NFTs (Metaplex), Mikro-Zahlungen, Gaming-Events
- Bridge (Wormhole): ATC (Substrate) <-> ATC-SPL (Solana)

**Fehlende Dokumente:**
- docs/blockchain/SOLANA_INTEGRATION.md (Kap. 32) — NEU ERSTELLT
- Solana-Wallet-Support (Phantom, Solflare)
- SPL-Token Standard fuer ATC auf Solana
- Metaplex NFT Standard (Shivamon auf Solana)
- Wormhole Bridge: ATC (Substrate) <-> ATC-SPL (Solana)

**Fehlende Roadmap-Eintraege:**
- Sprint 3.9: Solana Anchor Programme
- Sprint 3.10: Wormhole Bridge Substrate <-> Solana
- Sprint 4.5: Solana Mainnet Deployment

---

### 1.2 Ethereum/EVM — Nur erwaehnt, keine Architektur

Issue #12 (Solidity Contracts) offen — aber kein Deployment-Plan, keine Layer-2-Strategie.

**Empfohlene Strategie: Substrate Frontier EVM-Pallet**
- EVM direkt in Substrate integriert
- MetaMask/Ethers.js funktionieren ohne Aenderung
- Solidity-Contracts laufen nativ auf KAI-OS
- EIP-1559, EIP-712, EIP-2930 kompatibel

**Fehlende Dokumente:**
- docs/blockchain/ETHEREUM_INTEGRATION.md (Kap. 33) — NEU ERSTELLT
- Frontier-Pallet Setup
- Hardhat-Konfiguration
- MetaMask-Kompatibilitaet
- Solidity Contracts: ATC Token.sol, ShivamonNFT.sol, KAIGovernance.sol

**Fehlende Roadmap-Eintraege:**
- Sprint 2.9: Frontier EVM-Pallet Integration
- Sprint 3.11: Solidity Contract Suite
- Sprint 4.6: Ethereum Bridge Mainnet

---

### 1.3 Cross-Chain Bridge — Architektur unvollstaendig

**Vollstaendige Bridge-Architektur:**

```
Substrate (KAI-OS Native)
    -> Lock ATC
    -> Bridge Relayer (3-of-5 Multi-Sig)
    -> Wormhole / custom relayer
    -> Ethereum: Wrapped ATC (WATC) ERC-20 mint
    -> Solana: ATC-SPL mint
    -> BSC: WATC-BEP20 mint

Rueckweg:
    Ethereum/Solana/BSC: burn WATC
    -> Relayer verifiziert burn-Event
    -> Substrate: ATC unlock
```

---

## 2. FEHLENDE KI-INHALTE

### 2.1 LLM-Router fehlt
- Kein intelligenter Modell-Selector dokumentiert
- Kapitel 35 (LLM_ROUTER.md) wurde neu erstellt
- Routing basierend auf: Task-Typ, Hardware, Budget, Latenz

### 2.2 RAG-System fehlt
- Retrieval Augmented Generation nicht dokumentiert
- Lokal + On-Chain-State als Kontext
- Vektordatenbank (Qdrant/Chroma) Integration fehlt

### 2.3 Fine-Tuning fehlt
- LoRA/QLoRA Pipeline nicht dokumentiert
- Federated Fine-Tuning nicht spezifiziert
- Training-Incentives in ATC fehlen

### 2.4 AI Safety fehlt
- Kein Constitutional AI

---

## 29. Rechtliches

Alle Technologien sind proprietäre Entwicklungen von **ShivaCore / A-TownChain-Okosystems**.
Kein Fork. Kein Klon. Keine externen Blockchain-Abhängigkeiten.

- **Lizenz:** Proprietär — alle Rechte vorbehalten
- **Krypto:** Python `cryptography` (FIPS-konform, ECDSA secp256k1)
- **GitHub:** https://github.com/A-TownChain-Okosystems
- **Chain-ID:** 658467

---

*© 2026 ShivaCore | A-TownChain-Okosystems | Whitepaper v3.0.0 | 10. Juni 2026*
*"We don't fork. We build."*
