# A-TownChain OS — Architecture

> Monorepo (Code) des A-TownChain Ökosystems — gepaart mit dem
> Dokumentations-Hub [a-townchain-os-docs](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs).
> Lizenz: All Rights Reserved, Michael Wroblewski / ShivaCore / A-TownChain-Okosystems.

## Repository-Struktur (Stand: Konsolidierung Sept 2026)

```
a-townchain-os/
├── src/
│   ├── modules/          # 60 konsolidierte Modul-Repos (atc-*, atclang, atcnet …)
│   ├── core/             # Kern-Bibliotheken (kernel, crypto, …)
│   ├── blockchain/       # Blockchain-Kern (ATCLang + Python)
│   ├── wallet/ zkp/ bridge/  # K3-Subsysteme (Rust/Python)
│   ├── atclang/          # ATCLang-Spezifikation & K3-Modul
│   ├── kernel/ shivacore/    # Kernel-Komponenten
│   └── legacy/           # Übernommene Unique-Dateien aus der Parallel-Struktur
├── tests/                 # unit / integration / e2e
├── docker/                # Container & nginx
├── scripts/               # Build- & Automatisierungs-Skripte
├── config/                # Laufzeit-Konfiguration
├── .github/               # CI/CD-Workflows
└── Meta-Files             # README, CHANGELOG, VERSION, Makefile, Dockerfile, …
```

## Module
- 60 Module unter `src/modules/` — jedes mit vollständigem Meta-File-Satz
  (README, LICENSE, ARCHITECTURE.md, COMPONENT_PLAN.md, CHANGELOG.md, STATUS.md,
  ROADMAP.md, FILE_REGISTER.md, .gitignore)
- Flagship-Kernel: `src/modules/atc-shivacore/` (Rust, 30+ Module, 367 Tests)
- ATCLang-Compiler: `src/modules/atclang/` (Python, Lexer→Parser→Compiler→VM)
- ATC-Atclang-Mirror: `src/modules/atc-atclang/` (v0.3-Feature-Set inkl. VM)

## Programmiersprachen
- **Python** — ATCLang-Compiler, Blockchain-Logik, K3-Subsysteme
- **Rust** — ShivaCore-Kernel, Aurora AI, ZKP, Wallet, Bridge, Treiber
- **TypeScript/React** — Frontend, Explorer, IDE, Analytics
- **ATCLang** — proprietäre Sprache (.atc), CORE-Logik & Standards

## Build System
- Makefile, Dockerfile, docker-compose.yml (10 Dienste, Kernel im CI-Profil)
- **Unified Cargo Workspace** (Root-`Cargo.toml`): alle 19 echten Rust-Crates in einem Build-System — `cargo test --workspace` läuft 731 Tests auf der Stable-Toolchain (Boot-Teile des Kernels hinter dem optionalen Feature `x86-boot`)
- **Modul-Registry** (`src/modules/registry.py`): inventarisiert alle 60 Module beim Systemstart, importiert Python-Pakete live, meldet Workspace-Status; in `scripts/start.sh` integriert
- pytest (Python), tsc/npm (TypeScript)

## Dependencies
- Rust 1.97+, Python 3.11+, Node.js/TypeScript
- ed25519-dalek, hashlib/ECDSA (secp256k1)

## Dokumentation
- Vollständige Doku im Hub: [a-townchain-os-docs](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs)
- Standards: [docs/standards/](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/standards) (ATC-01 bis ATC-35)
- Wiki: [docs/kai-os-wiki.md](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/archive/kai-os-legacy/docs/kai-os-wiki.md)

## Status
Active — konsolidiertes Monorepo, VERSION 1.0.0, Mainnet-Target: 15.09.2026
