# REPO_ARCHITECTURE.md — A-TownChain-Ökosystem Repository-Architektur

> **Stand:** 05.08.2026 | **Total Repos:** 70 | **Org:** A-TownChain-Okosystems
> **Copyright:** Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0 lizenziert — siehe LICENSE

---

## 1. Ökosystem-Baum (Top-Level)

```
A-TownChain-Okosystems/                          # GitHub Organization (70 Repos)
│
├── 🟦 KERNEL & BARE-METAL (1 Repo)
│   └── atc-shivacore/                            # Rust no_std Kernel (K0-K40, 51 Module, 1304 Tests)
│       ├── kernel/src/*.rs                       # 51 Rust-Module (lib.rs + 50 pub mod)
│       ├── kernel/Cargo.toml                     # no_std, x86_64-unknown-none
│       ├── README.md                             # K-Sprint Doku (3-38)
│       └── .gitignore
│
├── 🟨 KERNEL-NAH (5 Repos)
│   ├── atc-bootloader/                           # UEFI Bootloader (Rust no_std)
│   │   ├── src/main.rs                           # UEFI entry point
│   │   ├── src/gpt.rs                            # GPT-Partition-Parser
│   │   ├── src/secure_boot.rs                    # Signature-Verification
│   │   ├── src/kernel_handoff.rs                 # Memory-Map → Kernel
│   │   ├── Cargo.toml                            # x86_64-unknown-uefi
│   │   └── README.md
│   ├── atc-stdlib/                               # Userspace Standard Library (Rust no_std)
│   │   ├── src/lib.rs                            # Public API
│   │   ├── src/string.rs                         # String-Operations
│   │   ├── src/collections.rs                    # Vec, HashMap, BTreeMap
│   │   ├── src/io.rs                             # printf, scanf, read, write
│   │   ├── src/alloc.rs                          # malloc, free, realloc
│   │   ├── src/errno.rs                          # Error numbers
│   │   ├── src/syscall.rs                        # ATC-96 Syscall-Wrapper
│   │   ├── Cargo.toml                            # x86_64-unknown-none
│   │   └── README.md
│   ├── atc-drivers/                              # Hardware-Treiber (Rust no_std)
│   │   ├── src/lib.rs
│   │   ├── src/usb/mod.rs                        # USB-Stack (xHCI)
│   │   ├── src/gpu/mod.rs                        # GPU (VBE, GOP)
│   │   ├── src/audio/mod.rs                      # Audio (Intel HD Audio)
│   │   ├── src/storage/mod.rs                    # SATA/NVMe (AHCI)
│   │   ├── src/hid/mod.rs                        # HID (Keyboard, Mouse)
│   │   ├── src/net/mod.rs                        # Intel e1000, Realtek r8169
│   │   ├── Cargo.toml                            # x86_64-unknown-none
│   │   └── README.md
│   ├── atc-vm/                                   # ShivaVM Smart Contract VM (Rust no_std)
│   │   ├── src/lib.rs                            # VM entry point
│   │   ├── src/opcodes.rs                        # 27 Opcodes
│   │   ├── src/stack.rs                          # Stack-Interpreter
│   │   ├── src/gas.rs                            # Gas-Metering
│   │   ├── src/storage.rs                        # Contract-Storage
│   │   ├── Cargo.toml                            # x86_64-unknown-none
│   │   └── README.md
│   └── atc-dns/                                  # Dezentraler DNS-Resolver (Rust no_std)
│       ├── src/lib.rs
│       ├── src/resolver.rs                       # DID → IP:Port
│       ├── src/cache.rs                          # DNS-Caching mit TTL
│       ├── src/records.rs                        # A, AAAA, CNAME, TXT, SRV
│       ├── Cargo.toml                            # x86_64-unknown-none
│       └── README.md
│
├── 🟩 BLOCKCHAIN & CONTRACTS (5 Repos)
│   ├── atc-blockchain/                           # Blockchain-Kern (Python)
│   │   ├── consensus/                            # PoH, PoW, PoS, Finality
│   │   ├── contracts/                            # Smart Contract Runtime
│   │   ├── p2p/                                  # P2P-Networking
│   │   ├── genesis/                              # Genesis Block
│   │   ├── requirements.txt
│   │   └── README.md
│   ├── atc-contracts/                            # Smart Contracts (Python, migriert)
│   ├── atc-franchise/                            # Franchise-Modul (Python, migriert)
│   ├── atc-genesis-engine/                       # Vision-Dokument (Python, kein Code)
│   └── atc-atcpkg/                               # On-Chain Package Manager (Konzept)
│
├── 🟧 BACKEND & API (2 Repos)
│   ├── atc-backend/                              # REST-API, DB, Wallet (Python)
│   │   ├── src/
│   │   │   ├── routes/                           # API-Endpunkte
│   │   │   ├── models/                           # Datenbank-Modelle
│   │   │   ├── services/                        # Business-Logic
│   │   │   └── middleware/                      # Auth, CORS, Rate-Limit
│   │   ├── tests/
│   │   ├── requirements.txt
│   │   └── README.md
│   └── atc-gateway/                              # API-Gateway (Python)
│       ├── python/                               # Stabile Python-Implementierung
│       │   ├── routes/
│       │   ├── middleware/
│       │   └── service_discovery/
│       ├── atclang/                              # Experimenteller ATCLang-Port
│       └── README.md
│
├── 🟪 FRONTEND & UI (5 Repos)
│   ├── atc-frontend/                             # Web-Frontend (HTML/JS)
│   │   ├── index.html
│   │   ├── css/
│   │   ├── js/
│   │   └── assets/
│   ├── atc-aistudio/                             # AI Studio (TypeScript)
│   │   ├── src/
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   └── README.md
│   ├── atc-explorer/                             # Blockchain Explorer (TypeScript)
│   │   ├── src/
│   │   │   ├── components/
│   │   │   ├── pages/
│   │   │   └── api/
│   │   ├── package.json
│   │   └── README.md
│   ├── atc-ide/                                  # ATCLang IDE/Playground (TypeScript)
│   │   ├── src/
│   │   │   ├── editor/                          # Monaco Editor
│   │   │   ├── debugger/                       # Inline-Debugger
│   │   │   └── repl/                           # REPL
│   │   ├── package.json
│   │   └── README.md
│   └── atc-ui/                                   # UI-Assets (HTML, migriert)
│
├── 🟫 DESKTOP & MOBILE (4 Repos)
│   ├── atc-windows-edition/                      # Windows Desktop-Client (Rust std + egui)
│   │   ├── src/main.rs
│   │   ├── src/gui/                              # egui Windows
│   │   ├── Cargo.toml                            # x86_64-pc-windows-msvc
│   │   └── README.md
│   ├── atc-linux-edition/                        # Linux Desktop-Client (Rust std + egui)
│   │   ├── src/main.rs
│   │   ├── src/gui/                              # egui Windows
│   │   ├── Cargo.toml                            # x86_64-unknown-linux-gnu
│   │   └── README.md
│   ├── atc-wallet/                               # Desktop-Wallet (Rust std + egui)
│   │   ├── src/main.rs
│   │   ├── src/keys/                             # Key-Management
│   │   ├── src/tx/                               # TX-Signing
│   │   ├── Cargo.toml                            # x86_64-unknown-linux-gnu
│   │   └── README.md
│   └── atc-mobile/                               # Mobile Wallet (Python)
│       ├── src/
│       ├── requirements.txt
│       └── README.md
│
├── 🟥 SPRACHE & TOOLS (8 Repos)
│   ├── atc-atclang/                              # ATCLang Compiler (Python)
│   │   ├── lexer/
│   │   ├── parser/
│   │   ├── compiler/
│   │   ├── tests/
│   │   └── README.md
│   ├── atclang/                                  # Alt ATCLang (Python, migriert)
│   ├── atc-cli/                                  # CLI-Tool (Rust std)
│   │   ├── src/main.rs                           # atc status/deploy/call/query
│   │   ├── src/commands/
│   │   ├── Cargo.toml                            # x86_64-unknown-linux-gnu
│   │   └── README.md
│   ├── atc-sdk/                                  # SDK (Rust + TypeScript)
│   │   ├── rust/                                 # Rust-Crate
│   │   │   ├── src/lib.rs
│   │   │   └── Cargo.toml
│   │   ├── typescript/                           # TypeScript-Package
│   │   │   ├── src/
│   │   │   └── package.json
│   │   └── README.md
│   ├── atc-shivacore-tools/                      # Build-Tooling (Shell-Skripte)
│   ├── atc-ci/                                   # CI/CD-Pipeline (YAML)
│   │   ├── .github/workflows/
│   │   │   ├── kernel-test.yml
│   │   │   ├── python-test.yml
│   │   │   ├── frontend-build.yml
│   │   │   └── release.yml
│   │   └── README.md
│   ├── atc-shivamon/                             # Monitoring (Python, migriert)
│   └── atc-standards/                            # ATC-Standards (Docs)
│
├── 🟦 DOKU & MAIN (4 Repos)
│   ├── a-townchain-os/                           # Haupt-Repo / Monorepo (TypeScript)
│   │   ├── REALITY_STATUS.md                     # ✅ EINZIGE kanonische Statusquelle
│   │   ├── README.md
│   │   ├── docs/
│   │   │   ├── kai-os-wiki.md                    # Wiki (7.735 Zeilen, v1.3.4)
│   │   │   └── standards/                        # ATC-1 bis ATC-40
│   │   └── package.json
│   ├── a-townchain-os-docs/                      # Offizielle Doku-Seite (TypeScript)
│   ├── atc-whitepaper/                           # Whitepaper (Docs)
│   └── a-townchain-os-wiki/                      # Wiki-Archiv-Snapshot
│
└── 📖 WIKI-REPOS (34 Repos)
    ├── atc-shivacore-wiki/                       # Kernel Wiki
    ├── atc-bootloader-wiki/
    ├── atc-stdlib-wiki/
    ├── atc-drivers-wiki/
    ├── atc-vm-wiki/
    ├── atc-dns-wiki/
    ├── atc-blockchain-wiki/
    ├── atc-contracts-wiki/
    ├── atc-franchise-wiki/
    ├── atc-genesis-engine-wiki/
    ├── atc-atcpkg-wiki/
    ├── atc-backend-wiki/
    ├── atc-gateway-wiki/
    ├── atc-frontend-wiki/
    ├── atc-aistudio-wiki/
    ├── atc-explorer-wiki/
    ├── atc-ide-wiki/
    ├── atc-ui-wiki/
    ├── atc-windows-edition-wiki/
    ├── atc-linux-edition-wiki/
    ├── atc-wallet-wiki/
    ├── atc-mobile-wiki/
    ├── atc-atclang-wiki/
    ├── atclang-wiki/
    ├── atc-cli-wiki/
    ├── atc-sdk-wiki/
    ├── atc-shivacore-tools-wiki/
    ├── atc-ci-wiki/
    ├── atc-shivamon-wiki/
    ├── atc-standards-wiki/
    ├── atc-kernel-wiki/
    ├── atcnet-wiki/
    ├── atc-kernel/                               # (Legacy, migriert)
    ├── atcnet/                                   # (Legacy, migriert)
    └── franchise-factory-wiki/                   # (Legacy)
```

---

## 2. Datei-Platzierungs-Regeln

### Kernel-Repo (atc-shivacore)
```
ERLAUBT:
  ✅ kernel/src/*.rs        — Rust no_std Module
  ✅ kernel/Cargo.toml      — Cargo-Konfiguration (no_std)
  ✅ README.md              — K-Sprint-Dokumentation
  ✅ .gitignore

VERBOTEN:
  ❌ *.py                   — Keine Python-Dateien
  ❌ *.ts / *.js            — Keine TypeScript/JavaScript
  ❌ *.html                 — Keine HTML-Dateien
  ❌ docs/                  — Doku gehört in a-townchain-os/docs/
  ❌ wiki/                  — Wiki gehört in *-wiki Repo
```

### Kernel-Nah Repos (atc-bootloader, atc-stdlib, atc-drivers, atc-vm, atc-dns)
```
ERLAUBT:
  ✅ src/*.rs               — Rust no_std Module
  ✅ Cargo.toml             — Cargo-Konfiguration (no_std/uefi)
  ✅ README.md
  ✅ .gitignore

VERBOTEN:
  ❌ *.py, *.ts, *.js, *.html
  ❌ Kernel-Dateien (gehören in atc-shivacore)
```

### Python Repos (atc-backend, atc-gateway, atc-blockchain, atc-atclang, etc.)
```
ERLAUBT:
  ✅ *.py                   — Python-Module
  ✅ requirements.txt       — Dependencies
  ✅ tests/                 — Python-Tests
  ✅ README.md

VERBOTEN:
  ❌ *.rs                   — Keine Rust-Dateien
  ❌ kernel/                — Kernel-Code gehört in atc-shivacore
```

### TypeScript Repos (atc-aistudio, atc-explorer, atc-ide, atc-sdk)
```
ERLAUBT:
  ✅ src/*.ts, *.tsx        — TypeScript-Module
  ✅ package.json           — npm-Konfiguration
  ✅ tsconfig.json          — TypeScript-Konfiguration
  ✅ README.md

VERBOTEN:
  ❌ *.rs                   — Keine Rust-Dateien
  ❌ *.py                   — Keine Python-Dateien
```

### Desktop Repos (atc-windows-edition, atc-linux-edition, atc-wallet)
```
ERLAUBT:
  ✅ src/*.rs               — Rust std Module
  ✅ Cargo.toml             — Cargo-Konfiguration (std, egui)
  ✅ README.md

VERBOTEN:
  ❌ no_std                 — Diese Repos nutzen std, nicht no_std
  ❌ kernel/                — Kernel-Code gehört in atc-shivacore
  ❌ *.py, *.ts, *.html
```

### Wiki Repos (*-wiki)
```
ERLAUBT:
  ✅ README.md              — Wiki-Inhalt
  ✅ *.md                   — Markdown-Dokumentation

VERBOTEN:
  ❌ *.rs, *.py, *.ts, *.js, *.html  — Kein Code in Wiki-Repos
  ❌ Cargo.toml, package.json        — Keine Build-Files
```

---

## 3. Sprachen-Matrix

| Kategorie | Repo | Sprache | Target |
|-----------|------|---------|--------|
| Kernel | atc-shivacore | Rust (no_std) | x86_64-unknown-none |
| Bootloader | atc-bootloader | Rust (no_std) | x86_64-unknown-uefi |
| StdLib | atc-stdlib | Rust (no_std) | x86_64-unknown-none |
| Drivers | atc-drivers | Rust (no_std) | x86_64-unknown-none |
| VM | atc-vm | Rust (no_std) | x86_64-unknown-none |
| DNS | atc-dns | Rust (no_std) | x86_64-unknown-none |
| CLI | atc-cli | Rust (std) | x86_64-unknown-linux-gnu |
| SDK | atc-sdk | Rust (std) + TS | multi |
| Desktop | atc-windows-edition | Rust (std) + egui | x86_64-pc-windows-msvc |
| Desktop | atc-linux-edition | Rust (std) + egui | x86_64-unknown-linux-gnu |
| Wallet | atc-wallet | Rust (std) + egui | x86_64-unknown-linux-gnu |
| Backend | atc-backend | Python | — |
| Gateway | atc-gateway | Python | — |
| Blockchain | atc-blockchain | Python | — |
| Compiler | atc-atclang | Python | — |
| Frontend | atc-frontend | HTML/JS | — |
| AI Studio | atc-aistudio | TypeScript | — |
| Explorer | atc-explorer | TypeScript | — |
| IDE | atc-ide | TypeScript | — |
| CI/CD | atc-ci | YAML + Rust | — |

---

## 4. Abhängigkeits-Graph

```
atc-bootloader ──→ atc-shivacore (lädt Kernel)
atc-stdlib ──→ atc-shivacore (Syscall-Interface)
atc-drivers ──→ atc-shivacore (Driver-Framework K35)
atc-vm ──→ atc-shivacore (Kernel-Integration)
atc-dns ──→ atc-shivacore (TCP/IP, DID)

atc-cli ──→ atc-backend, atc-blockchain (API-Calls)
atc-sdk ──→ atc-backend, atc-blockchain (API-Calls)
atc-explorer ──→ atc-backend (REST-API)
atc-ide ──→ atc-atclang (Compiler)
atc-wallet ──→ atc-blockchain (TX-Broadcast)

atc-backend ──→ atc-gateway (Routing)
atc-gateway ──→ atc-backend, atc-blockchain (Service Discovery)
atc-aistudio ──→ atc-backend (API)
atc-frontend ──→ atc-backend, atc-gateway (API)
atc-mobile ──→ atc-backend, atc-gateway (API)

atc-ci ──→ alle Repos (CI/CD)
```

---

## 5. Namenskonvention

| Präfix | Bedeutung | Beispiele |
|--------|-----------|-----------|
| `atc-` | A-TownChain Komponente | atc-shivacore, atc-backend, atc-cli |
| `*-wiki` | Wiki-Repo zu Source-Repo | atc-shivacore-wiki, atc-backend-wiki |
| `atc-*-edition` | Desktop-Edition | atc-windows-edition, atc-linux-edition |
| `a-townchain-*` | Haupt-Repo / Doku | a-townchain-os, a-townchain-os-docs |

**Regel:** Neue Repos immer mit `atc-`-Präfix. Wiki-Repos = `<source-repo>-wiki`.

---

Copyright © Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0 lizenziert — siehe LICENSE
