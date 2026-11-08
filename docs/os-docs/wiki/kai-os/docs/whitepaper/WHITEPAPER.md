# A-TownChain Ökosystem
## Vollständiges Whitepaper — Version 2.1.0

**Erstellt:** 09. Juni 2026  
**Autor:** ShivaCore  
**Organisation:** A-TownChain-Okosystems  
**Chain-ID:** 9000  
**Status:** v1.0.0 RELEASE ✅

---

> *"We don't fork. We build."*  
> — ShivaCore, Gründer des A-TownChain Ökosystems

---

## Inhaltsverzeichnis

1. [Executive Summary](#1-executive-summary)
2. [Vision & Mission](#2-vision--mission)
3. [Gesamt-Architektur](#3-gesamt-architektur)
4. [ATCLang — Proprietäre Programmiersprache](#4-atclang--proprietäre-programmiersprache)
5. [ShivaOS — Dezentrales Betriebssystem](#5-shivaos--dezentrales-betriebssystem)
6. [ShivaConsensus — Hybrid PoH+PoS+PoW](#6-shivaconsensus--hybrid-pohposepow)
7. [Blockchain-Kern & ATCoin](#7-blockchain-kern--atcoin)
8. [Wallet & Kryptographie](#8-wallet--kryptographie)
9. [Smart Contract System](#9-smart-contract-system)
10. [ATC Token Standards (8300/9000/9900)](#10-atc-token-standards)
11. [API Gateway & Netzwerk](#11-api-gateway--netzwerk)
12. [Shivamon — NFT Gaming Ökosystem](#12-shivamon--nft-gaming-ökosystem)
13. [Franchise Factory](#13-atc-franchise)
14. [Gemini AI Integration](#14-gemini-ai-integration)
15. [Multi-Node Testnet](#15-multi-node-testnet)
16. [Security Audit v1.0.0](#16-security-audit-v210)
17. [ATCLang Security Analyzer](#17-atclang-security-analyzer)
18. [Tokenomics & Wirtschaft](#18-tokenomics--wirtschaft)
19. [Repository-Übersicht](#19-repository-übersicht)
20. [Protokoll-Standards ATC-0001–0008](#20-protokoll-standards-atc-0001-0008)
21. [ShivaOS Standards ATS-1000–1007](#21-shivaos-standards-ats-1000-1007)
22. [Entwicklungs-Issues v0.9–v1.0.0](#22-entwicklungs-issues)
23. [Roadmap](#23-roadmap)
24. [Rechtliches & Lizenz](#24-rechtliches--lizenz)

---

## 1. Executive Summary

A-TownChain ist ein **vollständig proprietäres, dezentrales Blockchain-Ökosystem** — entwickelt von ShivaCore / A-TownChain-Okosystems. Kein Fork. Kein Klon. Jede Zeile Code ist originär und proprietär entwickelt.

### Kern-Komponenten

| Komponente | Version | Beschreibung |
|------------|---------|-------------|
| **ATCLang** | v0.3.0 | Proprietäre Blockchain-Programmiersprache |
| **ShivaOS** | v1.0.0 | Dezentrales proprietäres Betriebssystem |
| **ShivaConsensus** | v1.0.0 | Hybrid PoH + PoS + PoW |
| **ATCoin** | v1.0.0 | Native Währung, Chain-ID 9000 |
| **Shivamon** | v1.0.0 | NFT Gaming Ökosystem |
| **Franchise Factory** | v1.0.0 | Dezentrales Business-Protokoll |
| **Gemini AI** | v1.0.0 | KI-Orchestrator (BYOK) |
| **API Gateway** | v1.0.0 | Port 4000, Rate-Limit, ECDSA-Auth |

### Zahlen (Stand 09.06.2026)
- **21 GitHub-Repositories** (10 Software + 10 Wiki + 1 Whitepaper)
- **300+ Quelldateien** in der Haupt-Codebase
- **47+ Wiki-Dokumentationsseiten**
- **23/23 Tests bestanden** ✅
- **10 Sicherheitslücken geschlossen** (v1.0.0 Security Audit)
- **15 ATCLang Security-Checks** implementiert

### Kern-Philosophie
Kein POSIX-Klon. Kein EVM-Fork. Vollständig proprietär, von Grund auf neu — unter Verwendung der ATC-0001–0008 Blockchain-Standards und ATS-1000–1007 Betriebssystem-Standards.

---

## 2. Vision & Mission

### Vision
Eine dezentrale, KI-gestützte Wirtschafts-Infrastruktur schaffen, die Blockchain, Gaming, Business und künstliche Intelligenz in einem vollständig proprietären Ökosystem vereint — ohne Abhängigkeit von bestehenden Chains oder Frameworks.

### Mission
- Vollständig eigenen Technologie-Stack entwickeln (ATC-0001–ATC-0008, ATS-1000–ATS-1007)
- **ATCLang** als erste blockchain-native Sprache außerhalb des EVM-Universums etablieren
- Dezentrale Franchise-Wirtschaft on-chain ermöglichen
- KI nahtlos in die Blockchain-Infrastruktur integrieren
- Open-Source-Transparenz via vollständiger GitHub-Dokumentation

### Kern-Prinzipien
1. **Proprietär**: Kein Fork. Jede Zeile Code ist original entwickelt.
2. **Sicher**: Reentrancy-Guards, Rate-Limiting, ECDSA, Input-Validation in allen Schichten.
3. **Modular**: Klare Trennung `/core`, `/ui`, `/plugins`, `/build`, `/export`, `/frontend`, `/backend`.
4. **KI-nativ**: Gemini AI direkt im Orchestrator integriert, BYOK-Strategie.
5. **Transparent**: Vollständige Open-Source-Dokumentation und Code via GitHub.
6. **Skalierbar**: Modulare 8-Repo-Struktur für unabhängige Entwicklung.

---

## 3. Gesamt-Architektur

```
┌─────────────────────────────────────────────────────────────────────┐
│                      A-TownChain Ökosystem v1.0.0                   │
├───────────────┬──────────────────┬─────────────────┬────────────────┤
│   ATCLang     │    ShivaOS       │   Blockchain    │   Services     │
│   v0.2.0      │    Kernel        │   Core          │                │
├───────────────┼──────────────────┼─────────────────┼────────────────┤
│ Lexer (67T)   │ Kernel           │ HybridConsensus │ Gemini AI      │
│ Parser (AST)  │ ATCFS            │ PoH (VDF)       │ Gateway :4000  │
│ Compiler      │ ATCNet P2P       │ PoS (Weighted)  │ REST API       │
│ VM (30+ Op)   │ ShivaConsensus   │ PoW (SHA3-ATC)  │ WebSocket :9944│
│ REPL          │ EventBus         │ ATCoin (Native) │ Rate-Limit     │
│ Stdlib (25+)  │ ModuleLoader     │ ECDSA Wallet    │ ECDSA-Auth     │
│ Security Anal │ ProcessManager   │ Smart Contracts │                │
│               │                  │ ATC-8300 Token  │ Shivamon NFT   │
│               │                  │ ATC-9000 NFT    │ Battle/Breed   │
│               │                  │ ATC-9900 DAO    │ Marketplace    │
│               │                  │ Bridge          │                │
│               │                  │                 │ Franchise Fac  │
├───────────────┴──────────────────┴─────────────────┴────────────────┤
│              Frontend Dashboard (Neon Dark Theme, Port 3000)        │
│              Vanilla HTML/CSS/JS — keine externen Frameworks        │
└─────────────────────────────────────────────────────────────────────┘
```

# ⚡ A-TownChain OS — v2.0

<div align="center">

![Version](https://img.shields.io/badge/version-2.0-a259ff?style=for-the-badge)
![Python](https://img.shields.io/badge/Python-3.10+-00d1ff?style=for-the-badge&logo=python)
![Flask](https://img.shields.io/badge/Flask-3.0-00ffb3?style=for-the-badge&logo=flask)
![Solidity](https://img.shields.io/badge/Solidity-0.8.20-7b61ff?style=for-the-badge&logo=solidity)
![Chain ID](https://img.shields.io/badge/Chain_ID-9000-ff6b35?style=for-the-badge)
![License](https://img.shields.io/badge/license-MIT-ff2d78?style=for-the-badge)

**Autonomous Franchise Factory — ShivaOS v2.0 — KAI-OS Layer Architecture**

*Blockchain × AI × Gaming × Operating System Ecosystem*

</div>

---

## 📋 Inhaltsverzeichnis

| # | Abschnitt | Tags |
|---|-----------|------|
| [1](#1-überblick) | Überblick | `overview` `ecosystem` |
| [2](#2-architektur) | Architektur | `architecture` `layers` |
| [3](#3-projektstruktur) | Projektstruktur | `structure` `tree` |
| [4](#4-installation--quick-start) | Installation | `install` `setup` `run` |
| [5](#5-api-dokumentation) | API Dokumentation | `api` `endpoints` |
| [6](#6-smart-contracts) | Smart Contracts | `contracts` `solidity` `python` |
| [7](#7-atc-token-standards) | ATC Token Standards | `standards` `atc8300` `atc9000` |
| [8](#8-frontend-dashboard) | Frontend Dashboard | `frontend` `ui` `shivaos` |
| [9](#9-entwicklung--tests) | Entwicklung & Tests | `dev` `tests` `contributing` |
| [10](#10-roadmap) | Roadmap | `roadmap` `milestones` `issues` |
| [11](#11-dokumentation) | Dokumentation | `docs` `wiki` `whitepaper` |

---

## 1. Überblick

A-TownChain OS ist ein vollständiges, modulares Technologie-Ökosystem:

| Komponente | Beschreibung | Standard | Status |
|-----------|-------------|---------|--------|
| 🏠 **ShivaOS** | Futuristisches Browser-OS Dashboard | ATS-1007 | ✅ v2.0 |
| ⛓ **A-TownChain** | Eigene Layer-1 Chain (PoI+PoS+PoH) | ATC-0004 | 🔨 Phase 1 |
| 🧠 **KAI-OS AI** | Gemini 2.0 + dezentrale KI-Agenten | ATS-1005 | 🔨 Phase 2 |
| 🎮 **Shivamon** | NFT-Battle-RPG (9.900 unique NFTs) | ATC-9000 | 🔨 Phase 2 |
| 🏛 **Governance** | Dezentrale DAO (ATC-9900) | ATC-9900 | ✅ Deployed |
| 🛒 **Marketplace** | NFT-Handelsplatz (2.5% Royalty) | — | ✅ Deployed |
| 💰 **ATC Wallet** | BIP-39 Wallet (24 Wörter, ECDSA) | ATC-0002 | ✅ v2.1 |
| 🌉 **Bridge** | Cross-Chain (Ethereum + Solana) | — | 📋 Phase 3 |
| 🔤 **ATCLang** | Native Smart-Contract-Sprache | ATS-VM | 🔨 Phase 2 |
| 🏭 **Franchise Factory** | Autonomes Deployment-System | — | 📋 Phase 4 |

---

## 2. Architektur

```
╔═══════════════════════════════════════════════════════╗
║          A-TownChain OS — KAI-OS Layer Stack          ║
╠═══════════════════════════════════════════════════════╣
║  L12  Gamification  Soul-Bound NFTs · Quests           ║
║  L11  DeFi          AMM · Lending · Oracle             ║
║  L10  dApps         Shivamon · Marketplace · Gov       ║
║  L9   Agenten       KI-Agents · Federated Learning     ║
║  L8   Governa

---

## 4. ATCLang — Proprietäre Programmiersprache

# ATCLang — Die Programmiersprache des A-TownChain Ökosystems
Version: 0.1.0-alpha
Datum: 2026-06-06

## Philosophie
ATCLang ist eine statisch typisierte, blockchain-native Sprache.
Keine Abhängigkeit von Python-Syntax. Eigene Grammatik. Eigene VM.

## Syntax-Beispiel

```atclang
// Wallet erstellen
wallet myWallet = ATC::Wallet::new("ShivaCore")
contract ShivaToken : ATC-8300 {
    state balance: Map<Address, UInt256>
    fn transfer(to: Address, amount: UInt256) -> Bool {
        require(balance[caller] >= amount)
        balance[caller] -= amount
        balance[to] += amount
        emit Transfer(caller, to, amount)
        return true
    }
}
```

## Token-Typen
- KEYWORD: wallet, contract, fn, state, emit, require, return
- TYPE: UInt256, Address, Bool, String, Map, List
- OPERATOR: +, -, *, /, >=, <=, ==, !=, ->, ::
- LITERAL: Integer, String, Bool
- SPECIAL: ATC:: (Namespace), @decorator


### Typ-System

| Typ | Bits | Wertebereich |
|-----|------|-------------|
| `u8` | 8 | 0 – 255 |
| `u16` | 16 | 0 – 65.535 |
| `u32` | 32 | 0 – 4.294.967.295 |
| `u64` | 64 | 0 – 18,4 · 10¹⁸ |
| `u128` | 128 | 0 – 3,4 · 10³⁸ |
| `bool` | — | true / false |
| `string` | — | UTF-8, max 64 KB |
| `bytes32` | 256 bit | Hash-Werte, Signaturen |
| `Address` | 35 Zeichen | "ATC" + 32 Hex |
| `Map<K,V>` | — | Hash-Map |
| `Vec<T>` | — | Dynamisches Array |

### Beispiel-Contract (vollständig)
```atclang
use ATC::Types::Address
use ATC::Crypto::sha256

contract SafeToken {
    state owner:         Address
    state paused:        bool    = false
    state name_:         string  = "MyToken"
    state symbol_:       string  = "MTK"
    state decimals_:     u8      = 18
    state total_supply_: u128    = 0
    state MAX_SUPPLY:    u128    = 21_000_000_000_000_000_000_000_000
    state balances:      Map<Address, u128>

    fn init(owner_addr: Address) {
        self.owner = owner_addr
        emit Initialized(owner_addr)
    }

    fn transfer(to: Address, amount: u128) -> bool {
        require(!self.paused, "Contract pausiert")
        require(is_valid_address(to_string(to)), "Ungültige Adresse")
        require(self.balances[caller] >= amount, "Unzureichendes Guthaben")
        self.balances[caller] = safe_sub(self.balances[caller], amount)
        self.balances[to]     = safe_add(self.balances[to], amount)
        emit Transfer(caller, to, amount)
        return true
    }

    fn mint(to: Address, amount: u128) -> bool {
        require(caller == self.owner, "Nur Owner")
        require(safe_add(self.total_supply_, amount) <= self.MAX_SUPPLY, "Max Supply erreicht")
        self.balances[to]   = safe_add(self.balances[to], amount)
        self.total_supply_  = safe_add(self.total_supply_, amount)
        emit Mint(to, amount)
        return true
    }

    fn pause()   { require(caller == self.owner, "Nur Owner"); self.paused = true  }
    fn unpause() { require(caller == self.owner, "Nur Owner"); self.paused = false }
}
```

### Compiler-Pipeline
```
Quellcode (.atc)
    ↓ [1] Lexer       → 67+ Token-Typen
    ↓ [2] Parser      → AST (Recursive Descent)
    ↓ [3] Type-Check  → Statische Typ-Validierung
    ↓ [4] Optimizer   → Konstanten-Faltung, Dead-Code
    ↓ [5] Code-Gen    → ATCLang Bytecode
    ↓ [6] VM          → Stack-basierte Ausführung
```

### VM Sicherheit (v1.0.0)
- `MAX_CALL_DEPTH = 128` — verhindert Stack-Overflow durch Rekursion
- Gas-Limit: 10.000.000/TX — verhindert DoS durch Endlosschleifen
- Stack-Underflow: Exception bei leerem Stack
- Input-Sanitization: `MAX_SOURCE_SIZE = 1MB`, Null-Byte-Erkennung

---

## 5. ShivaOS — Dezentrales Betriebssystem

### Kernel-Architektur (ATS-1000)
```
ShivaKernel v1.0.0
├── ProcessManager    PID-Vergabe, Spawn, Kill, State-Transitions
│   └── Types: AGENT | SERVICE | CONTRACT | DAEMON | USER
├── MemoryManager     Speicher-Isolation pro Prozess
├── IPC-Channels      Typisierte Inter-Prozess-Kommunikation
├── EventBus          Pub/Sub, Ringbuffer (500 max), Wildcard-Listener
├── ModuleLoader      Dynamisches Plugin-Loading
└── SecurityManager   Auth, ECDSA-Prüfung, Call-Validierung
```

### ATCFS — Dezentrales Dateisystem (ATS-1003)
Kein POSIX-Klon. Vollständig proprietäre Implementierung.

**INode-Typen:** `FILE` | `DIRECTORY` | `SYMLINK` | `CHAIN_STATE` | `CONTRACT`

**Berechtigungsmodell:**
```
OWNER: read | write | execute
GROUP: read | write | execute
OTHER: read | execute
```

### ATCNet P2P (ATS-1004 + ATC-0005)
Kademlia DHT-basiertes Peer-to-Peer Netzwerk.

| Port | Dienst |
|------|--------|
| 4001 | P2P Node Communication |
| 5005 | Bootstrap Node |
| 9944 | WebSocket (RPC) |

**Security (v1.0.0):**
- Rate-Limit: max 100 Nachrichten/60s pro Peer
- Message-Size-Limit: max 64 KB
- ECDSA-Signatur auf allen Block/TX-Messages

---

## 6. ShivaConsensus — Hybrid PoH+PoS+PoW

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

> **Dokument:** `docs/architecture/CONSENSUS.md`
> **Datum:** 2026-05-19 · **Autor:** A-TownChain-Okosystems × Aurora AI


---

## 7. Blockchain-Kern & ATCoin

### ATCoin
- **Max Supply:** 21.000.000 ATC
- **Dezimalen:** 18 (1 ATC = 10¹⁸ Wei)
- **Chain-ID:** 9000
- **Block-Reward:** halbiert alle 210.000 Blöcke
- **Genesis-Hash:** SHA3-256("A-TownChain Genesis Block 2026")

### Block-Struktur
```python
Block:
  height:      u64        # Block-Nummer
  hash:        bytes32    # SHA3-256 dieses Blocks
  prev_hash:   bytes32    # Hash des Vorgänger-Blocks
  timestamp:   u64        # Unix-Timestamp (ms)
  poh_hash:    bytes32    # Proof-of-History Hash
  pow_nonce:   u64        # Proof-of-Work Nonce
  validator:   Address    # Ausgewählter Validator
  tx_count:    u32        # Anzahl Transaktionen
  merkle_root: bytes32    # Merkle-Root der Transaktionen
  gas_used:    u64        # Verbrauchtes Gas
  gas_limit:   u64        # Gas-Limit des Blocks
```

### Transaktion
```python
Transaction:
  hash:        bytes32    # TX-Hash
  from_:       Address    # Sender
  to:          Address    # Empfänger
  amount:      u128       # Betrag in Wei
  gas_price:   u64
  gas_limit:   u64
  nonce:       u64        # Replay-Schutz
  chain_id:    u64        # 9000
  data:        bytes      # Contract-Aufruf-Daten
  signature:   bytes      # ECDSA secp256k1
```

---

## 8. Wallet & Kryptographie

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
> **Datum:** 2026-05-19 · **Autor:** A-TownChain-Okosystems × Aurora AI


### Adress-Generierung (ATC-0002)
```
256-bit Entropy (os.urandom — CSPRNG)
        ↓
24-Wort Mnemonic (BIP39-Wortliste, 2048 Wörter)
        ↓
512-bit Seed (PBKDF2-HMAC-SHA512, 2048 Iterationen)
        ↓
Private Key: SHA-256(seed) → 64 Hex-Zeichen
        ↓
Public Key:  SHA-256(private_key) → 64 Hex-Zeichen
        ↓
ATC-Adresse: "ATC" + SHA-256(public_key)[0:32] = 35 Zeichen
```

**Beispiel:**
```
Adresse: ATC8F3A2D1E9B4C7F0A5E2D8B3C6A9F1D4E7B2C5
Länge:   35 Zeichen (3 Prefix + 32 Hex)
```

### ECDSA
- Kurve: secp256k1
- Hash-Algorithmus: SHA3-256
- Replay-Schutz: Nonce (strikt monoton steigend) + Chain-ID 9000
- Library: Python `cryptography` (FIPS-konform)

---

## 9. Smart Contract System

### Deployed Contracts

| Contract | Standard | Beschreibung |
|----------|---------|-------------|
| ATCoin | ATC-8300 | Native Währung |
| GenesisToken | ATC-8300 | Initiales Token-Supply |
| ATC8300Token | ATC-8300 | Generischer Fungible Token |
| ShivamonNFT | ATC-9000 | NFT Gaming Token |
| GovernanceDAO | ATC-9900 | On-Chain Governance |
| MarketplaceContract | ATC-8300/9000 | NFT Marktplatz |
| BridgeContract | Custom | Cross-Chain Bridge |
| FranchiseRegistry | ATC-9000 | Franchise-Lizenzen als NFT |
| RevenueShare | ATC-8300 | Automatische Gewinnverteilung |
| FranchiseToken | ATC-8300 | FFT Utility Token |

### BaseContract — Basisklasse
```python
class BaseContract(ABC):
    owner:      str       # Contract-Eigentümer
    address:    str       # ATC_CONTRACT_<hash>
    paused:     bool      # Notfall-Pause
    events:     list      # Event-Log
    _locked:    bool      # Reentrancy-Guard

    def _nonreentrant_enter(self):
        if self._locked:
            raise RuntimeError("ReentrancyGuard: Kein rekursiver Aufruf")
        self._locked = True

    def _nonreentrant_exit(self):
        self._locked = False

    @abstractmethod
    def name(self) -> str: ...
```

---

## 10. ATC Token Standards

# 🪙 ATC Token Standard — Übersicht

> Technische Kurzreferenz aller ATC Token Standards

| Standard | Typ | Datei | Status |
|----------|-----|-------|--------|
| ATC-001 | Genesis Token | `blockchain/smart_contracts.py` | ✅ |
| ATC-8300 | Fungible Token | `blockchain/atcoin/atcoin.py` | ✅ |
| ATC-9000 | NFT (Shivamon) | `blockchain/contracts/shivamon/shivamon_contract.py` | ✅ |
| ATC-9900 | Governance/DAO | geplant | ⏳ v2.1 |

→ Vollständige Dokumentation: [SHIVAMON_NFT_CONTRACT.md](../contracts/SHIVAMON_NFT_CONTRACT.md)


### ATC-8300 Interface (vollständig)
```python
name()                              -> str
symbol()                            -> str
decimals()                          -> int      # Standard: 18
total_supply()                      -> int
balance_of(addr)                    -> int
transfer(caller, to, amount)        -> dict
approve(caller, spender, amount)    -> dict
allowance(owner, spender)           -> int
transfer_from(caller, from_, to, amount) -> dict
mint(caller, to, amount)            -> dict     # Nur Owner
burn(caller, amount)                -> dict
```

### ATC-9000 Interface (vollständig)
```python
name()                              -> str
symbol()                            -> str
total_supply()                      -> int
max_supply()                        -> int
balance_of(addr)                    -> int
owner_of(token_id)                  -> str
token_uri(token_id)                 -> str
transfer(caller, to, token_id)      -> dict
approve(caller, spender, token_id)  -> dict
mint(caller, to, **attributes)      -> dict     # DNA = sha256(owner+id+ts)
burn(caller, token_id)              -> dict
tokens_of(addr)                     -> list
```

### ATC-9900 Governance Interface
```python
create_proposal(caller, title, description, options) -> str   # proposal_id
vote(voter, proposal_id, option)                     -> dict
finalize_proposal(proposal_id)                       -> dict  # nach 7 Tagen
execute_proposal(caller, proposal_id)                -> dict  # nach 48h Timelock
get_proposals()                                      -> list
get_proposal(proposal_id)                            -> dict
```

**Governance-Parameter:**
- Min-Stake für Voting: 10.000 ATC
- Voting-Dauer: 7 Tage
- Quorum: 10% aller ATC
- Threshold: 51% Ja-Stimmen
- Timelock: 48h nach Voting-Ende

---

## 11. API Gateway & Netzwerk

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

> **Dokument:** `docs/architecture/GATEWAY.md`
> **Datum:** 2026-05-19 · **Autor:** A-TownChain-Okosystems × Aurora AI


### Vollständige API-Referenz

**System**
```
GET  /health                         System-Status
GET  /api/version                    Version Info (2.1.0)
GET  /api/nodes                      Verbundene P2P-Nodes
```

**Blockchain**
```
GET  /api/blockchain/status          Chain-Info (Höhe, Hash, TPS)
GET  /api/blockchain/blocks          Letzte 20 Blöcke
GET  /api/blockchain/block/:height   Block by Höhe
GET  /api/blockchain/tx/:hash        TX by Hash
POST /api/blockchain/tx              TX einreichen
```

**Wallet**
```
POST /api/wallet/generate            Neue Wallet (Mnemonic + ATC-Adresse)
GET  /api/wallet/balance/:addr       ATC-Balance
POST /api/wallet/send                ATC senden (ECDSA-signiert)
GET  /api/wallet/txs/:addr           TX-Historie
```

**Smart Contracts**
```
GET  /api/contracts                  Alle deployed Contracts
POST /api/contracts/call             Contract-Methode aufrufen
```

**AI / Gemini**
```
GET  /api/ai/status                  Orchestrator-Status
POST /api/ai/chat                    KI-Chat (BYOK)
POST /api/ai/generate-atclang        ATCLang-Code generieren
POST /api/ai/explain-contract        Contract erklären
POST /api/ai/analyze-tx              TX analysieren
```

**NFT / Shivamon**
```
POST /api/game/mint                  Shivamon minten
GET  /api/game/tokens/:addr          Tokens nach Adresse
POST /api/game/battle                Battle starten
GET  /api/marketplace/listings       Aktive Listings
POST /api/marketplace/buy            Kaufen (ECDSA-signiert)
```

**Governance**
```
GET  /api/governance/proposals       Alle Proposals
POST /api/governance/propose         Neues Proposal erstellen
POST /api/governance/vote            Abstimmen
```

---

## 12. Shivamon — NFT Gaming Ökosystem

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

---

## 13. Franchise Factory

### Konzept
Dezentrales Business-Ökosystem — Franchise-Lizenzen werden als NFTs (ATC-9000) on-chain registriert,
Revenue-Sharing läuft vollautomatisch per Smart Contract.

### Smart Contracts
| Contract | Standard | Funktion |
|----------|---------|---------|
| FranchiseRegistry | ATC-9000 | Lizenz-NFTs (mint, transfer, suspend) |
| RevenueShare | ATC-8300 | Automatische Gewinnverteilung |
| FranchiseToken (FFT) | ATC-8300 | Governance + Utility |
| FranchiseDAO | ATC-9900 | Franchise-Netzwerk-Governance |

### Revenue-Share Mechanismus
```atclang
fn record_revenue(amount: u128, memo: string) {
    require(caller == self.franchisee || caller == self.owner, "Nicht berechtigt")
    let franchisor_cut: u128 = amount * u128(self.share_pct) / 100
    let franchisee_cut: u128 = safe_sub(amount, franchisor_cut)
    self.pending_f_sor  = safe_add(self.pending_f_sor, franchisor_cut)
    self.pending_f_see  = safe_add(self.pending_f_see, franchisee_cut)
    emit RevenueRecorded(amount, franchisor_cut, franchisee_cut)
}
```

### Token Economy (FFT)
| Kategorie | Anteil | Vesting |
|-----------|--------|---------|
| Team | 15% | 4 Jahre, 1 Jahr Cliff |
| Community | 40% | Keine |
| Ökosystem-Fonds | 20% | 2 Jahre |
| Reserve | 15% | DAO-kontrolliert |
| Liquidität | 10% | Sofort |

**Max Supply: 100.000.000 FFT**

### Lizenz-Gebühren
| Paket | Gebühr | Revenue-Share | Laufzeit |
|-------|--------|--------------|---------|
| Starter | 500 ATC | 5% | 1 Jahr |
| Standard | 1.000 ATC | 10% | 2 Jahre |
| Premium | 2.500 ATC | 15% | 5 Jahre |
| Master | 10.000 ATC | 20% | Unbegrenzt |

---

## 14. Gemini AI Integration

### Architektur
- **GeminiOrchestrator** in `backend/api/orchestrator/orchestrator.py`
- **BYOK** (Bring Your Own Key) — kein zentraler API-Key, lokale Key-Verwaltung im Browser
- Retry-Logik: max 5 Versuche, exponentieller Backoff, Cap bei 5s
- Rate-Limit: 10 Requests/min auf `/api/ai/*`

### ATCLang Code-Generierung
```python
POST /api/ai/generate-atclang
Body: {"prompt": "Erstelle einen Fungible Token mit Mint-Funktion"}
→ Gemini generiert validen ATCLang-Contract-Code
```

### Funktionen
| Endpoint | Beschreibung |
|----------|-------------|
| `POST /api/ai/chat` | Freier KI-Chat über A-TownChain |
| `POST /api/ai/generate-atclang` | ATCLang-Code aus natürlicher Sprache |
| `POST /api/ai/explain-contract` | Contract-Code erklären |
| `POST /api/ai/analyze-tx` | Transaktion analysieren |

---

## 15. Multi-Node Testnet

# 🌐 A-TownChain Testnet — Technische Dokumentation

> **Milestone:** v1.0.0 · **Issues:** #8, #14–#19
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
        

---

## 16. Security Audit v1.0.0

### Audit-Ergebnis: 10/10 Schwachstellen geschlossen ✅

| ID | Schweregrad | Modul | Schwachstelle | Fix |
|----|------------|-------|--------------|-----|
| SEC-001 | MEDIUM | REPL | `os.system('clear')` Shell-Injection | ANSI-Escape `\033[2J` |
| SEC-002 | HIGH | ATCLang VM | Unbegrenzte Call-Depth → Stack-Overflow | `MAX_CALL_DEPTH = 128` |
| SEC-003 | MEDIUM | Compiler | Keine Input-Validierung | `MAX_SOURCE_SIZE=1MB`, Null-Byte-Check |
| SEC-004 | HIGH | BaseContract | Kein Reentrancy-Guard | `_nonreentrant_enter/exit()` |
| SEC-005 | MEDIUM | P2P | Kein Rate-Limiting, kein Message-Limit | 100/60s, max 64KB |
| SEC-006 | LOW | KeyGen | BIP39 Index-Overflow | `idx % len(BIP39_WORDLIST)` |
| SEC-007 | MEDIUM | GovernanceContract | `name()` Abstract Method fehlte | Implementiert |
| SEC-008 | MEDIUM | BridgeContract | `name()` Abstract Method fehlte | Implementiert |
| PERF-001 | LOW | Orchestrator | `sleep()` ohne Cap | Cap bei 5s |
| PERF-002 | LOW | Compiler | O(n²) Label-Lookup | `_label_cache` → O(1) |

### Test-Suite
```
✅ EventBus          ✅ Core-Kernel       ✅ ATCLang-Stack
✅ Consensus         ✅ Wallet+ECDSA      ✅ SmartContracts
✅ ATCoin            ✅ BackendServer      ✅ Orchestrator
✅ Gateway           ✅ ShivaOS           ✅ Reentrancy-Guard
✅ Compiler-Security

Ergebnis: 23/23 Tests bestanden ✅
```

---

## 17. ATCLang Security Analyzer

### ATC-SEC-0001 Standard — 15 Sicherheitsregeln

```python
from atclang.security.analyzer import generate_report

source = open("my_contract.atc").read()
print(generate_report(source, "my_contract.atc"))
```

### Regel-Übersicht

| Code | Schweregrad | Trigger |
|------|------------|---------|
| ATC-SEC-001 | 🟠 HIGH | `self.balance + amount` ohne `safe_add()` |
| ATC-SEC-002 | 🟠 HIGH | `fn transfer()` ohne `require(caller ==` |
| ATC-SEC-003 | 🟡 MEDIUM | `transfer()` Rückgabe ignoriert |
| ATC-SEC-004 | 🟡 MEDIUM | Hardcoded ATC-Adresse |
| ATC-SEC-005 | 🟠 HIGH | `emit X` vor `self.y =` (Reentrancy) |
| ATC-SEC-006 | 🟡 MEDIUM | Division ohne `require(divisor > 0)` |
| ATC-SEC-007 | 🟠 HIGH | `tx_origin` statt `caller` |
| ATC-SEC-008 | 🔴 CRITICAL | `random()` ohne Blockchain-Entropy |
| ATC-SEC-009 | 🔴 CRITICAL | `self.owner =` ohne `require()` |
| ATC-SEC-010 | 🟡 MEDIUM | Address-Parameter ohne `is_valid_address()` |
| ATC-SEC-011 | 🔴 CRITICAL | `terminate()` ohne `require()` |
| ATC-SEC-012 | 🟢 LOW | `fn transfer` ohne `require(!self.paused)` |
| ATC-SEC-013 | 🟡 MEDIUM | Kein `state owner:` im Contract |
| ATC-SEC-014 | 🟡 MEDIUM | Kein `fn init()` Constructor |
| ATC-SEC-015 | 🟠 HIGH | Arithmetik ohne `safe_add/sub` global |
| ATC-PERF-001 | 🟢 LOW | Unbegrenzte Schleife über Collection |

---

## 18. Tokenomics & Wirtschaft

### ATCoin (ATC)
- **Max Supply:** 21.000.000 ATC (analog Bitcoin)
- **Dezimalen:** 18
- **Block-Reward:** Halbiert alle 210.000 Blöcke
- **Chain-ID:** 9000

### Gebühren-Struktur
| Operation | Gebühr |
|-----------|--------|
| ATC senden | 0,001 ATC |
| Contract deployen | 1,0 ATC |
| NFT minten | 0,1 ATC |
| Shivamon Breeding | 500 ATC |
| Governance Proposal | 10 ATC |
| Franchise Starter | 500 ATC |
| Franchise Standard | 1.000 ATC |
| Franchise Premium | 2.500 ATC |

### Staking (PoS)
- Minimum: 10.000 ATC
- Validator-Auswahl: gewichtetes Random via SHA-256-Seed
- Slashing: 5% bei Doppel-Signatur, 1% bei Offline

---

## 19. Repository-Übersicht

### Software-Repositories (A-TownChain-Okosystems)
| Repository | Beschreibung | Dateien |
|-----------|-------------|---------|
| `a-townchain-os` | Haupt-Repo, Unified Start | 97+ |
| `atclang` | ATCLang Compiler-Stack | 27+ |
| `atc-kernel` | ShivaOS Kernel | 15+ |
| `atcnet` | P2P Stack | 6+ |
| `atc-standards` | ATC/ATS Protokoll-Standards | 5+ |
| `atc-contracts` | Smart Contracts | 22+ |
| `shivamon` | NFT Gaming | 8+ |
| `atc-gateway` | API Gateway | 9+ |
| `atc-ui` | Frontend Dashboard | 2+ |
| `atc-franchise` | Franchise Contracts | 8+ |
| `atownchain-whitepaper` | Dieses Whitepaper | 2+ |

### Wiki-Repositories (je Software-Repo)
| Wiki | Seiten | Inhalt |
|------|--------|--------|
| `atclang-wiki` | 10 | SPEC, Lexer, Parser, VM, Stdlib, Compiler, REPL, Security, Analyzer, Examples |
| `atc-kernel-wiki` | 7 | Kernel, ATCFS, ATCNet, Consensus, Security, Performance |
| `atcnet-wiki` | 4 | Protokoll, Messages, Bootstrap, Security |
| `atc-contracts-wiki` | 5 | ATC-8300, ATC-9000, ATC-9900, Security Audit, Deployment |
| `atc-standards-wiki` | 4 | ATC-Standards, ATS-Standards, Übersicht |
| `atc-gateway-wiki` | 4 | Routen, Middleware, Auth, Security |
| `shivamon-wiki` | 5 | NFT-Spec, Battle, Breeding, Marketplace, Roadmap |
| `atc-ui-wiki` | 4 | Design-System, Komponenten, API, Deployment |
| `a-townchain-os-wiki` | 7 | Quickstart, Architektur, Roadmap, Security, API, Whitepaper |
| `franchise-factory-wiki` | 7 | Konzept, Contracts, Security, API, Deployment, Token Economy |

**Links:**
- Haupt-Repo: https://github.com/A-TownChain-Okosystems/a-townchain-os
- Whitepaper: https://github.com/A-TownChain-Okosystems/atownchain-whitepaper
- Organisation: https://github.com/A-TownChain-Okosystems

---

## 20. Protokoll-Standards ATC-0001–0008

# ATC Standards — A-TownChain Core Standards
Version: 1.0.0 | Status: DRAFT | Datum: 2026-06-06
Autor: ShivaCore / Aurora

> Alle ATC-Standards sind eigenständig definiert.
> Keine Übernahme aus ERC/EIP oder anderen Blockchains.

---

## ATC-0001 — Core Identity Standard

Jede Entität im A-TownChain Ökosystem besitzt eine eindeutige Identität.

```
IDENTITY := {
    id:       Address,       // ATC-Adresse (35 Zeichen, Präfix "ATC")
    pubkey:   Bytes64,       // Öffentlicher Schlüssel (eigenes ATC-EC)
    created:  UInt64,        // Block-Nummer der Erstellung
    type:     IdentityType,  // WALLET | CONTRACT | NODE | AGENT
}
IdentityType := WALLET | CONTRACT | NODE | AGENT | VALIDATOR
```

---

## ATC-0002 — Wallet Address Format

```
PREFIX    := "ATC"
BODY      := SHA3_ATC(pubkey)[0:32]  // 32 Hex-Zeichen
CHECKSUM  := CRC8(PREFIX + BODY)[0:2]
ADDRESS   := PREFIX + BODY + CHECKSUM  // Gesamt: 37 Zeichen

Beispiel: ATC7a3f9e2b1c8d4a6e0f5b7c9d2e1f3a4b5c6d7e8f9a
```

---

## ATC-0003 — Transaction Schema

```
TX := {
    id:        Hash256,        // SHA3_ATC(Payload)
    from:      Address,        // Sender
    to:        Address,        // Empfänger
    value:     UInt256,        // Menge (in ATCoin, 18 Dezimalstellen)
    data:      Bytes,          // Contract-Calldata (optional)
    nonce:     UInt64,         // Anti-Replay
    gas_limit: UInt64,         // Max. Gas
    gas_price: UInt64,         // Gas-Preis in nano-ATC
    signature: ATCSig,         // ECDSA-Signatur (ATC-EC Kurve)
    timestamp: UInt64,         // Unix-Timestamp
    version:   UInt8 = 1,      // TX-Version
}

ATCSig := {
    r: Bytes32,
    s: Bytes32,
    v: UInt8,   // Recovery ID (0 oder 1)
}
```

---

## ATC-0004 — Block Format

```
BLOCK := {
    header:  BlockHeader,
    txs:     List<TX>,
    receipts: List<TXReceipt>,
}

BlockHeader := {
    version:      UInt8,
    height:       UInt64,
    timestamp:    UInt64,
    prev_hash:    Hash256,
    tx_root:      Hash256,    // Merkle-Root aller TXs
    state_root:   Hash256,    // State-Trie-Root
    receipt_root: Hash256,
    validator:    Address,    // Block-Produzent
    consensus:    ConsensusData,
    nonce:        UInt64,     // PoW-Nonce
    difficulty:   UInt256,
    gas_limit:    UInt64,
    gas_used:     UInt64,
    extra:        Bytes32,    // Freies Feld
}

TXReceipt := {
    tx_id:    Hash256,
    success:  Bool,
    gas_used: UInt64,
    logs:     List<EventLog>,
    error:    Optional<String>,
}
```

---

## ATC-0005 — ShivaConsensus Protocol

Hybrider Mechanismus: PoW + PoS + PoH (Proof of History)

```
Runde := {
    phase_1: PoH_Tick      // Zeitstempel-Kette (VDF-basiert)
    phase_2: PoS_Vote      // Validator-Voting (Stake-gewichtet)
    phase_3: PoW_Seal      // Finaler Hash-Beweis
}

BLOCK_TIME     := 3s
MIN_STAKE      := 1000 ATC
VALIDATOR_SET  := Top-100 nach Stake
FINALITY       := 2/3 Validator-Bestätigung
FORK_RULE      := Längste-gewichtete-Kette (Stake × Länge)
```

---

## ATC-0006 — Smart Contract Interface

```
CONTRACT := {
    address:   Address,
    code_hash: Hash256,     // Hash des Bytecodes
    abi:       List<FuncSpec>,
    state:     StateTree,   // Merkle-Patricia-Trie
    version:   UInt8,
    standard:  ATCStandard, // ATC-8300, ATC-9000, etc.
}

FuncSpec := {
    name:      String,
    inputs:    List<ParamSpec>,
    outputs:   List<ParamSpec>,
    mutates:   Bool,         // ändert State?
    payable:   Bool,         // akzeptiert ATC?
}
```

---

## ATC-0007 — P2P Message Protocol (ATCNet)

```
MSG := {
    version:   UInt8 = 1,
    type:      MsgType,
    from:      NodeID,      // Pubkey-Hash des Senders
    to:        NodeID,      // Ziel (oder broadcast = 0x00)
    payload:   Bytes,
    timestamp: UInt64,
    signature: ATCSig,
    ttl:       UInt8,       // Time-To-Live (Hops)
}

MsgType :=
    HELLO | PING | PONG |
    GET_BLOCKS | BLOCKS |
    GET_TX | TX |
    GET_STATE | STATE |
    CONSENSUS_VOTE | CONSENSUS_BLOCK |
    KI_QUERY | KI_RESPONSE |
    PEER_LIST | DISCONNECT
```

---

## ATC-0008 — KI-OS Process Standard

```
KI_PROCESS := {
    pid:       UInt32,
    name:      String,
    type:      ProcessType,    // AGENT | SERVICE | CONTRACT | SYSTEM
    model:     AIModelRef,     // Referenz auf KI-Modell
    memory:    MemoryRegion,   // isolierter Speicherbereich
    channels:  List<Channel>,  // IPC-Kanäle
    stake:     UInt256,        // ATC-Stake für Rechenrecht
    priority:  UInt8,          // 0=niedrig, 255=System
}

ProcessType := AGENT | SERVICE | CONTRACT | SYSTEM | VALIDATOR
```

---

## ATC-8300 — Fungible Token Standard

```
INTERFACE ATC8300 {
    fn total_supply() -> UInt256
    fn balance_of(owner: Address) -> UInt256
    fn transfer(to: Address, amount: UInt256) -> Bool
    fn approve(spender: Address, amount: UInt256) -> Bool
    fn allowance(owner: Address, spender: Address) -> UInt256
    fn transfer_from(from: Address, to: Address, amount: UInt256) -> Bool

    event Transfer(from: Address, to: Address, amount: UInt256)
    event Approval(owner: Address, spender: Address, amount: UInt256)
}
```

---

## ATC-9000 — NFT Standard (Shivamon)

```
INTERFACE ATC9000 {
    fn owner_of(token_id: UInt256) -> Address
    fn token_uri(token_id: UInt256) -> String
    fn transfer(to: Address, token_id: UInt256) -> Bool
    fn approve(to: Address, token_id: UInt256) -> Bool
    fn mint(to: Address, metadata: ATCMetadata) -> UInt256
    fn burn(token_id: UInt256) -> Bool
    fn total_supply() -> UInt256

    event Mint(to: Address, token_id: UInt256)
    event Burn(token_id: UInt256)
    event Transfer(from: Address, to: Address, token_id: UInt256)
}

ATCMetadata := {
    name:        String,
    description: String,
    image_uri:   String,        // ATCFS:// Pfad
    attributes:  Map<String, String>,
    generation:  UInt8,
    rarity:      RarityLevel,   // COMMON | RARE | EPIC | LEGENDARY
}
```


---

## 21. ShivaOS Standards ATS-1000–1007

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
    --atc-secondary: #7b61ff   // Neon-Violett
    --atc-bg:        #0a0a1a   // Tiefschwarz
    --atc-surface:   #1a1a3a   // Dunkelblau
    --atc-text:      #e0e0ff   // Hellblau-Weiß
    --atc-accent:    #ff6b35   // Neon-Orange
```


---

## 22. Entwicklungs-Issues

### v1.0.0 — ABGESCHLOSSEN ✅

# 📄 Issue #1 — Smart Contract Implementation (ATC Token Standards)

> **Labels:** enhancement · blockchain
> **Priorität:** 🔴 High · **Milestone:** v1.0.0
> **Referenz:** [GitHub Issue #1](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/1)

---

## Ziel

Implementierung der nativen ATC Smart Contracts für alle definierten Token-Standards des A-TownChain Ökosystems (ATC-001 bis ATC-9900) — zunächst als Python-Contracts, später als Solidity On-Chain Contracts (→ Issue #12).

---

## Hintergrund

Die aktuelle `blockchain/smart_contracts.py` ist ein Placeholder. Die vollständigen ATC Token Standards aus der ATC-Referenzmatrix müssen als vollwertige, testbare Contracts implementiert werden.

---

## Technische Spezifikation

### Verzeichnisstruktur (Ziel)

```
blockchain/contracts/
├── atc001/
│   └── genesis_token.py          # ATC-001: Genesis Block Token
├── atc8300/
│   └── atc_token.py              # ATC-8300: Fungible Token (ERC20)
├── atc9000/
│   └── shivamon_contract.py      # ATC-9000: NFT ✅ bereits implementiert
├── atc9900/
│   └── governance_contract.py    # ATC-9900: DAO/Governance (→ Issue #9)
└── base/
    └── base_contract.py          # Gemeinsame Basisklasse
```


---

# Issue #2 — Gemini AI Integration
**Status:** ✅ DONE | **Version:** v1.0.0 | **Abgeschlossen:** 09.06.2026

## Übersicht
Vollständige Integration der Gemini AI API als KI-Kern von A-TownChain OS.
Modell: `gemini-2.5-flash` | Proprietäre Orchestrierung ohne externe Frameworks.

## Implementierte Dateien

| Datei | LOC | Beschreibung |
|-------|-----|-------------|
| `backend/api/orchestrator/orchestrator.py` | 260 | GeminiOrchestrator — Hauptklasse |
| `backend/api/routes/ai_routes.py` | 75 | REST API-Endpunkte |

## API-Endpunkte

```http
POST /api/ai/chat
  Body: {"message": "...", "session_id": "optional"}
  → {"ok": true, "response": {text, tokens_in, tokens_out, latency_ms}}

POST /api/ai/analyze-tx
  Body: {"tx": {sender, receiver, amount, nonce}}
  → {"ok": true, "analysis": {text, ...}}

POST /api/ai/explain-contract
  Body: {"code": "contract ..."}
  → {"ok": true, "explanation": {text, ...}}

POST /api/ai/generate-atclang
  Body: {"description": "Zähler-Contract mit increment()"}
  → {"ok": true, "code": {text, ...}}

GET /api/ai/status  → Orchestrator-Stats
GET /api/ai/health  → Health Check
```

## Architektur

```
User Request
     │
     ▼
AI Routes (/api/ai/*)
     │

---

# 📄 Issue #6 — ECDSA Signatur (Sichere TX-Autorisierung)

> **Labels:** security · backend · priority:high
> **Priorität:** 🔴 High · **Milestone:** v1.0.0
> **Referenz:** [GitHub Issue #6](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/6)

---

## Ziel

Alle Transaktionen und NFT-Transfers müssen mit dem **Private Key des Senders signiert** werden. Ohne ECDSA-Verifikation kann aktuell jeder beliebige Transfers auslösen, wenn die Adresse bekannt ist — das ist ein kritisches Sicherheitsproblem.

---

## Sicherheitsanalyse — Aktueller Zustand

| Endpoint | Aktuell | Soll |
|----------|---------|------|
| `POST /api/wallet/send` | Keine Signatur-Prüfung | ECDSA Signatur required |
| `POST /api/game/shivamon/transfer` | Keine Signatur-Prüfung | ECDSA Signatur required |
| `POST /api/game/shivamon/battle` | Keine Auth | Owner-Signatur required |
| `POST /api/governance/vote` | — (noch nicht impl.) | ECDSA required |

---

## Technische Spezifikation

### ECDSA Implementation (secp256k1)

```python
# blockchain/wallet/ecdsa.py
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import hashes, serialization
import hashlib, json

class ECDSASi

---

# 🌐 Issue #14 — Bootstrap Node — P2P Discovery Service

## Status: ✅ ABGESCHLOSSEN

**Milestone:** v1.0.0  
**Abhängigkeit:** Keine  
**Abhängig von:** Issue #14 wird von #15, #16, #17 benötigt  

---

## 📋 Aufgabenbeschreibung

Bootstrap Node ist der **Einstiegspunkt für das P2P-Netzwerk**. Alle neuen Nodes verbinden sich zuerst mit dem Bootstrap-Node, um andere Peers zu entdecken und sich ins Netzwerk zu integrieren.

### Was wurde implementiert:

1. **Node Discovery Service** (`blockchain/nodes/discovery.py`)
   - UDP-basiertes Protokoll für Peer-Discovery
   - ANNOUNCE/PEER_LIST Handshake
   - Peer-Registry Verwaltung
   - Health-Check (entfernt offline Peers)

2. **P2P Network Base** (`blockchain/nodes/p2p_propagation.py`)
   - TCP-basiertes Messaging
   - Block- und TX-Propagation
   - Duplikat-Filter
   - Peer-Verbindungsverwaltung

---

## ✅ Implementierte Komponenten

### 1. NodeDiscovery Klasse

**Datei:** `blockchain/nodes/discovery.py`

```python
class NodeDiscovery:
    """UDP-basierter Node-Discovery Service"""
    
    def __init__(self, node_id: str, my_port: int, is_bootstrap: bool = False)
    def start()                          # Startet Discovery-Service
    de

---

### v1.0.0 — IN PLANUNG

# 📄 Issue #8 — Multi-Node Testnet

> **Labels:** enhancement · blockchain · networking · priority:high
> **Priorität:** 🔴 High · **Milestone:** v1.0.0
> **Referenz:** [GitHub Issue #8](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/8)

---

## Ziel

Mehrere A-TownChain Nodes zu einem echten P2P-Testnetzwerk verbinden — Blöcke werden propagiert, Konsens dezentral erreicht, neue Nodes können sich synchronisieren.

---

## Netzwerk-Architektur

```
Bootstrap Node (immer online)
  node-001:5005  ←── neue Nodes melden sich hier an

      ↕  P2P (TCP/UDP)

node-002:5105        node-003:5205
(VALIDATOR)          (MINER)
    ↕                    ↕
node-004:5305        node-005:5405
(FULL)               (LIGHT)
```

---

## Protokoll-Spezifikation

### Nachrichten-Typen

```python
MSG_TYPES = {
    "HANDSHAKE":   {"version": str, "chain_height": int, "node_type": str},
    "NEW_BLOCK":   {"block": dict},
    "NEW_TX":      {"tx": dict},
    "GET_BLOCKS":  {"from_height": int, "to_height": int},
    "BLOCKS":      {"blocks": list},
    "GET_PEERS":   {},
    "PEERS":       {"peers": list},
    "PING":        {"timestamp": int},
    "PONG":        {"timestamp": int},
}
```

### Node Discovery

```python
# blockchain/nodes/discovery.py
class NodeDiscovery:
    BOOTSTRAP_NODES = [
        "127.0.0.1:5005",   # lokaler Bootstrap
        # Mainnet: "node1.atownchain.io:5005"
    ]

    def announce(self, my_address: str):
        """Meldet sich bei Bootstrap-Nodes an."""

    def get

---

# 📄 Issue #10 — Cross-Chain Bridge (ATC ↔ EVM)

> **Labels:** enhancement · blockchain · bridge · priority:low
> **Priorität:** 🟢 Low · **Milestone:** v1.0.0
> **Referenz:** [GitHub Issue #10](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/10)

---

## Ziel

Eine sichere Bridge zwischen A-TownChain und EVM-kompatiblen Chains — ATC Token werden auf A-TownChain eingefroren und als **Wrapped ATC (wATC)** auf Ethereum/Polygon ausgegeben.

---

## Lock-and-Mint Mechanismus

```
A-TownChain                          Ethereum (Sepolia)
──────────                           ──────────────────
User locked 1000 ATC                 →  Relayer erkennt Lock-Event
  in Lock Contract                   →  Mint 1000 wATC an User-Adresse
                                         (ERC20 auf Ethereum)

User burnt 500 wATC                  ←  Relayer erkennt Burn-Event
  auf Ethereum                       ←  Release 500 ATC aus Lock Contract
```

---

## Sicherheits-Architektur

| Maßnahme | Beschreibu

---

### v1.0.0–v1.0.0 — Weitere Issues

# 🗺 A-TownChain OS × KAI-OS — Finaler Roadmap-Abgleich
> **Stand:** 09. Juni 2026 | Notion ↔ GitHub main (135 Dateien)
> **Erstellt von:** Aurora × ShivaCore
> **Letzter Notion-Export:** 07.06.2026 | Letzter GitHub-Sync: 09.06.2026

---

## ✅ v1.0.0 — Status-Abgleich (Notion vs. GitHub)

| # | Feature | Notion-Status | GitHub-Beweis | Sync-Status |
|---|---------|:---:|---|:---:|
| #1 | Smart Contract Implementation | 🔨 In Progress | `blockchain/contracts/base/base_contract.py` `blockchain/contracts/atc8300/atc8300_token.py` `blockchain/contracts/atc001/genesis_token.py` `blockchain/smart_contract_registry.py` `tests/test_smart_contracts.py` | ✅ **DONE** |
| #2 | Gemini AI Integration | 🔨 In Progress | `backend/api/routes/ai_routes.py` `backend/api/orchestrator/orchestrator.py` `docs/issues/ISSUE_02_GEMINI_AI.md` | ⚠️ **Routing vorhanden — API-Key-Integration fehlt** |
| #4 | NFT Persistenz / SQLite | 🔨 In Progress | `backend/db/schema.sql` `backend/db/connection.py` `backend/db/repository.py` `backend/db/migrate.py` `tests/test_persistence.py` | ✅ **DONE** |
| #6 | ECDSA Signaturverfahren | 🔨 In Progress | `blockchain/wallet/ecdsa.py` `blockchain/wallet/keygen.py` `ecdsa_final.py` `ecdsa_impl.py` `tests/test_ecdsa.py` | ✅ **DONE** |
| #8 | Multi-Node Testnet | 📋 Todo | `blockchain/nodes/node.py` `blockchain/nodes/p2p_propagation.py` `docs/architecture/Testnet.md` `tests/test_p2p_propagation.py` | ⚠️ **Infrastruktur da — Live-Launch offen** |
| #14 | Bootstrap Node (P2P Discovery) | ✅ Done | `blockchain/nodes/discovery.py` `tests/test_discovery.py` `shivaos/net/atcnet.py` | ✅ **DONE** |

**v1.0.0 Gesamtfortschritt: 4/6 Issues abgeschlossen (67%)**
Für Release fehlt nur noch: **#2 Gemini AI Key-Integration**

---

## 🟡 v1.0.0 — Status-Abgleich

| # | Feature | Notion-Status | GitHub-Beweis | Sync-Status |
|---|---------|:---:|---|:---:|
| #3 | Shivamon Battle UI | 📋 Todo | `blockchain/contracts/shivamon/shivamon_contract.py` `frontend/index.html` `backend/api/routes/game_routes.py` | ⚠️ **Contract + Backend da — UI fehlt** |
| #5 | Blockchain Explorer | 📋 Todo | `backend/api/routes/blockchain.py` `frontend/index.html` `docs/issues/ISSUE_05_EXPLORER.md` | ⚠️ **API vorhanden — Frontend-Dashboard fehlt** |
| #9 | ATC-9900 Governance (DAO) | 📋 Todo | `backend/api/routes/governance_routes.py` `docs/issues/ISSUE_09_GOVERNANCE.md` | ⚠️ **Route da — DAO-Logic fehlt** |
| #11 | Shivamon Breeding | 📋 Todo | `docs/issues/ISSUE_11_BREEDING.md` | ❌ **Nur Doku, kein Code** |
| #13 | NFT Marketplace | 📋 Todo | `backend/api/routes/marketplace_routes.py` `docs/issues/ISSUE_13_MARKETPLACE.md` | ⚠️ **Route da — Marketplace-Logic fehlt** |

---

## 🟢 v1.0.0 — Status-Abgleich

| # | Feature | Notion-Status | GitHub-Beweis | Sync-Status |
|---|---------|:---:|---|:---:|
| #7 | Build System (EXE Installer) | 📋 Todo | `build/build.py` `docs/issues/ISSUE_07_BUILD.md` | ⚠️ **Grundgerüst da** |
| #10 | Cross-Chain Bridge (ETH/BSC) | 📋 Todo | `docs/issues/ISSUE_10_BRIDGE.md`

---

## 23. Roadmap

# 🗺 A-TownChain OS — Roadmap

<div align="center">

![Sprint](https://img.shields.io/badge/Aktuell-Sprint_2.5-a259ff?style=for-the-badge)
![Phase](https://img.shields.io/badge/Phase-2_Expansion-00ffcc?style=for-the-badge)
![Stand](https://img.shields.io/badge/Stand-09.06.2026-7b61ff?style=for-the-badge)

</div>

---

## 📋 Inhaltsverzeichnis

| # | Abschnitt | Tags |
|---|-----------|------|
| [1](#1-überblick) | Überblick | `overview` `phases` |
| [2](#2-phase-1--foundation-v20v21) | Phase 1 — Foundation | `v2.0` `v2.1` `done` |
| [3](#3-phase-2--expansion-v22v23) | Phase 2 — Expansion | `v2.2` `v2.3` `active` |
| [4](#4-phase-3--cross-chain--mainnet-v30) | Phase 3 — Cross-Chain | `v3.0` `mainnet` `bridge` |
| [5](#5-phase-4--ecosystem-v40) | Phase 4 — Ecosystem | `v4.0` `defi` `gamification` |
| [6](#6-issue-übersicht--querverweise) | Issue-Übersicht | `issues` `references` |
| [7](#7-sprint-übersicht) | Sprint-Übersicht | `sprints` `timeline` |
| [8](#8-tech-baumstruktur) | Tech-Baumstruktur | `tech` `tree` `architecture` |
| [9](#9-abhängigkeits-graph) | Abhängigkeits-Graph | `dependencies` `graph` |

---

## 1. Überblick

```
ZEITLINIE
─────────────────────────────────────────────────────────────────
Mai 2026    Jun 2026      Sep 2026       Jan 2027     Okt 2027
    │            │             │              │             │
    ▼            ▼             ▼              ▼             ▼
[Phase 1]   [Phase 2]      [Phase 2]     [Phase 3]    [Phase 4]
 v2.0-2.1   v2.2 Sprint    v2.3 Sprint   v3.0 Cross    v4.0 Full
 Foundation  2.1-2.5        2.6-2.10      Chain+Mainnet  Ecosystem
```

| Phase | Version | Zeitraum | Status | Schwerpunkt |
|-------|---------|----------|--------|-------------|
| **Phase 1** | v2.0–v2.1 | Mai 2026 | ✅ Abgeschlossen | Foundation, Core Contracts |
| **Phase 2** | v2.2–v2.3 | Jun–Sep 2026 | 🔨 Aktiv | Testnet, Governance, Marketplace |
| **Phase 3** | v3.0 | Okt 2026–Jan 2027 | 📋 Geplant | Mainnet, Cross-Chain Bridge |
| **Phase 4** | v4.0 | Jan–Okt 2027 | 📋 Geplant | DeFi, Gamification, Franchise |

---

## 2. Phase 1 — Foundation (v2.0/v2.1)

> **Status:** ✅ Abgeschlossen | **Zeitraum:** Mai 2026

### Milestone v1.0.0 — Genesis Release

| Feature | Issue | Status | Doku |
|---------|-------|--------|------|
| ShivaOS Dashboard v2.0 | — | ✅ | [`frontend/README.md`](../../code/frontend/README.md) |
| A-TownChain Blockchain Kern | — | ✅ | [`CONSENSUS.md`](../architecture/CONSENSUS.md) |
| Python Smart Contract Basis | [#1](../issues/ISSUE_01_SMART_CONTRACTS.md) | ✅ | [`ISSUE_01`](../issues/ISSUE_01_SMART_CONTRACTS.md) |
| ATC-001 Genesis Token | [#1](../issues/ISSUE_01_SMART_CONTRACTS.md) | ✅ | [`genesis_token.py`](../../blockchain/contracts/atc001/genesis_token.atc) |
| ATC-8300 Fungible Token | [#1](../issues/ISSUE_01_SMART_CONTRACTS.md) | ✅ | [`atc8300_token.py`](../../modules/contracts/atc8300/atc8300_token.atc) |
| ATC-9000 Shivamon NFT | [#3](../issues/ISSUE_03_BATTLE_UI.md) | ✅ | [`SHIVAMON_NFT_CONTRACT.md`](../contracts/SHIVAMON_NFT_CONTRACT.md) |
| Shivamon Battle System | [#3](../issues/ISSUE_03_BATTLE_UI.md) | ✅ | [`ISSUE_03`](../issues/ISSUE_03_BATTLE_UI.md) |
| ECDSA Wallet Implementierung | [#6](../issues/ISSUE_06_ECDSA.md) | ✅ | [`WALLET_KEYGEN.md`](../architecture/WALLET_KEYGEN.md) |
| Blockchain Explorer UI | [#5](../issues/ISSUE_05_EXPLORER.md) | ✅ | [`ISSUE_05`](../issues/ISSUE_05_EXPLORER.md) |
| NFT Persistenz (SQLite) | [#4](../issues/ISSUE_04_PERSISTENZ.md) | ✅ | [`ISSUE_04`](../issues/ISSUE_04_PERSISTENZ.md) |
| Gemini AI Integration | [#2](../issues/ISSUE_02_GEMINI_AI.md) | ✅ | [`ISSUE_02`](../issues/ISSUE_02_GEMINI_AI.md) |
| API Gateway + Backend | — | ✅ | [`GATEWAY.md`](../architecture/GATEWAY.md) |
| ATC-Standards Dokumente | — | ✅ | [`ATC_STANDARDS.md`](../standards/ATC_STANDARDS.md) |
| ATS-Standards Dokumente | — | ✅ | [`ATS_STANDARDS.md`](../standards/ATC_STANDARDS.md) |
| ATCLang Spec v1.0 | — | ✅ | [`ATCLANG_SPEC.md`](../../atclang/ATCLANG_SPEC.md) |

---

## 3. Phase 2 — Expansion (v2.2/v2.3)

> **Status:** 🔨 Aktiv | **Zeitraum:** Jun–Sep 2026

### Sprint 2.1–2.3 (abgeschlossen)

| Feature | Issue | Status | Doku |
|---------|-------|--------|------|
| Governance Contract (Python) | [#9](../issues/ISSUE_09_GOVERNANCE.md) | ✅ | [`governance_contract.py`](../../blockchain/contracts/governance/governance_contract.atc) |
| Marketplace Contract (Python) | [#13](../issues/ISSUE_13_MARKETPLACE.md) | ✅ | [`marketplace_contract.py`](../../modules/contracts/marketplace/marketplace_contract.atc) |
| Bridge Contract (Python) | [#10](../issues/ISSUE_10_BRIDGE.md) | ✅ | [`bridge_contract.py`](../../modules/contracts/bridge/bridge_contract.atc) |
| Solidity ATC Token.sol | [#12](../issues/ISSUE_12_SOLIDITY.md) | ✅ | [`Governance Contract (ATCLang)`](../../blockchain/contracts/governance/governance_contract.atc) |
| Solidity ShivamonNFT.sol | [#12](../issues/ISSUE_12_SOLIDITY.md) | ✅ | [`ISSUE_12`](../issues/ISSUE_12_SOLIDITY.md) |
| Solidity KAIGovernance.sol | [#12](../issues/ISSUE_12_SOLIDITY.md) | ✅ | [`governance_contract.atc`](../../blockchain/contracts/governance/governance_contract.atc) |
| Solidity KAIMarketplace.sol | [#12](../issues/ISSUE_12_SOLIDITY.md) | ✅ | [`marketplace_contract.atc`](../../modules/contracts/marketplace/marketplace_contract.atc) |
| Solidity KAIBridge.sol | [#10](../issues/ISSUE_10_BRIDGE.md) | ✅ | [`bridge_contract.atc`](../../modules/contracts/bridge/bridge_contract.atc) |
| Solidity GenesisToken.sol | [#1](../issues/ISSUE_01_SMART_CONTRACTS.md) | ✅ | [`genesis_token.atc`](../../blockchain/contracts/atc001/genesis_token.atc) |
| Hardhat Deploy-Script | [#12](../issues/ISSUE_12_SOLIDITY.md) | ✅ | [`scripts/generate_validators.atc`](../../scripts/generate_validators.atc) |
| Smart Contract Tests (Chai) | [#12](../issues/ISSUE_12_SOLIDITY.md) | ✅ | [`test/`](../../code/blockchain/contracts/solidity/test/) |
| Backend Routes Refactoring | — | ✅ | [`backend/api/routes/`](../../code/backend/api/routes/) |
| Gateway Port-Fix (:5000) | — | ✅ | [`gateway/router.py`](../../code/gateway/router.py) |

### Sprint 2.4–2.5 (a

---

## 24. Rechtliches & Lizenz

### Proprietäre Technologie
Alle in diesem Whitepaper beschriebenen Technologien — einschließlich ATCLang, ShivaOS, ShivaConsensus,
das ATC-Protokoll-Framework (ATC-0001–ATC-0008), das ShivaOS-Standards-Framework (ATS-1000–ATS-1007),
der ATCLang Security Analyzer (ATC-SEC-0001) sowie alle Smart Contracts — sind proprietäre Entwicklungen
von **ShivaCore / A-TownChain-Okosystems**.

Kein Bestandteil dieses Ökosystems ist ein Fork, Klon oder Ableitung eines bestehenden Blockchain-Projekts.

### Externe Abhängigkeiten
Das gesamte Ökosystem verwendet ausschließlich:
- Python `cryptography`-Library für ECDSA (FIPS-konform, OpenSSL-basiert)
- Python-Standardbibliothek

Keine weiteren externen Blockchain-Abhängigkeiten.

### Lizenz
Dieses Whitepaper und der zugehörige Code stehen unter der proprietären Lizenz von
A-TownChain-Okosystems / ShivaCore. Alle Rechte vorbehalten.

### Haftungsausschluss
Dieses Whitepaper dient ausschließlich zu Informations- und Dokumentationszwecken.
Es stellt weder eine Finanzberatung noch eine Investitionsempfehlung dar.
Die Nutzung des A-TownChain-Ökosystems und seiner Smart Contracts erfolgt auf eigenes Risiko.

### Kontakt
- GitHub Organisation: https://github.com/A-TownChain-Okosystems
- Haupt-Repository: https://github.com/A-TownChain-Okosystems/a-townchain-os

---

*© 2026 ShivaCore | A-TownChain-Okosystems | Whitepaper v1.0.0 | Stand: 09. Juni 2026*

*"We don't fork. We build."*
