# 📐 Dateinamen-Konventionen — A-TownChain Ökosystem (36 Repos)

> **Zweck:** Verbindliche Dateinamen-Konventionen für jedes Repository.
> Definiert Namensschema, Verzeichnisstruktur und Dateityp-Regeln pro Repo.
>
> **Autor:** Aurora (MasterBrain · Base44) | **Stand:** 2026-08-05
> **Version:** v1.0.0 | **Gültig für:** Alle 36 Repositories

---

## Globale Regeln (für alle Repos)

| Regel | Konvention | Beispiel |
|-------|-----------|----------|
| **Sprache** | ATCLang (`.atc`) für Produktion | `kernel.atc`, `p2p_node.atc` |
| **Python** | `.py` nur für Compiler-Module & Tests | `parser.py`, `test_kernel.py` |
| **Markdown** | `.md` für alle Dokumente | `README.md`, `STATUS.md` |
| **Namen** | `snake_case` für Dateien | `p2p_propagation.atc` |
| **Module** | Verzeichnis = Modulname, `snake_case` | `modules/contracts/` |
| **Tests** | `test_<name>.py` oder `test_<name>.atc` | `test_parser.py` |
| **Wiki** | `chapter-XX-name.md` (2-stellige Nummer) | `chapter-01-overview.md` |
| **Standards** | `ATC-XX-NAME.md` (2-stellige Nummer) | `ATC-01-P2P.md` |
| **Config** | Kleinbuchstaben, `.`-prefix für dotfiles | `.env.example`, `.gitignore` |
| **Version** | `v1.0.0` in allen Referenzen | nicht `v1.0.0` |

---

## Per-Repo Konventionen


### a-townchain-os

**Typ:** Haupt-Code-Repository (Monorepo)

**Verzeichnisstruktur:**
- `Root: Systemdateien (README, ROADMAP, STATUS, CHANGELOG, AGENT_MASTERRULES)`
- `blockchain/ — Konsens, Validierung, Wallet, Governance`
- `backend/ — API, DB, Services`
- `gateway/ — API-Gateway, Middleware`
- `modules/ — Feature-Module (contracts, atcnet, kernel, franchise, assets)`
- `core/ — Kern-Module (ai, kai_cli)`
- `tools/ — Werkzeuge (ecdsa, bigquery, atc_issues)`
- `monitoring/ — Prometheus, Grafana, Health-Checks`
- `shivaos/ — OS-Komponenten (UI, Shell)`
- `mobile/ — Mobile Apps (wallet, biometric)`
- `tests/ — Test-Suite`
- `docs/ — Lokale Doku-Kopien (gespiegelt aus a-townchain-os-docs)`
- `scripts/ — Build & Deploy-Scripts`
- `atcpkg/ — Package Manager`

**Namenskonvention:** snake_case für alle .atc/.py Dateien | Kapitel: chapter-XX-name.md | Standards: ATC-XX-NAME.md

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, AGENT_MASTERRULES.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### a-townchain-os-docs

**Typ:** Haupt-Wiki & Dokumentation

**Verzeichnisstruktur:**
- `Root: Wiki-Hauptdateien (README, STATUS, CHANGELOG, ROADMAP)`
- `docs/ — Alle Dokumentation`
- `docs/wiki/ — 69 Wiki-Kapitel (chapter-01 bis chapter-69)`
- `docs/standards/ — 99 ATC-Standards (ATC-01 bis ATC-99)`
- `docs/issues/ — Issue-Registry`
- `docs/whitepaper/ — Whitepaper & Architektur-Docs`
- `docs/file_registers/ — Per-Repo Datei-Register (22 Code-Repos)`
- `docs/FILE_REGISTER.md — Globales Datei-Register (alle 36 Repos)`
- `docs/FILE_NAMING_CONVENTIONS.md — Dieses Dokument`

**Namenskonvention:** chapter-XX-name.md (Wiki) | ATC-XX-NAME.md (Standards) | ISSUE_XX_NAME.md (Issues)

**Pflichtdateien:** README.md, STATUS.md, CHANGELOG.md, .gitignore, LICENSE

---


### a-townchain-os-wiki

**Typ:** Wiki-Repo (Archiviert/gespiegelt)

**Verzeichnisstruktur:**
- `Root: README.md, FILE_REGISTER.md, .gitignore, LICENSE`

**Namenskonvention:** Spiegel von a-townchain-os-docs; nur Referenz-Dateien

**Pflichtdateien:** README.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-aistudio

**Typ:** AI Studio-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `src/ — AI-Studio-Komponenten`
- `TypeScript-basiert`

**Namenskonvention:** PascalCase: App.tsx, Component.tsx | snake_case für utilities

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-atclang

**Typ:** ATCLang Sync-Repo

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `src/ — ATCLang-Quellcode-Spiegel`

**Namenskonvention:** snake_case: lexer.py, parser.py, tokens.py

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-atcpkg

**Typ:** Package Manager-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `manager.atc — Package Manager`

**Namenskonvention:** snake_case: manager.atc

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-backend

**Typ:** Backend-Service-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `api/ — REST-Routen`
- `db/ — Datenbank, Repository`

**Namenskonvention:** snake_case: api_routes.atc, repository.atc, connection.atc

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-blockchain

**Typ:** Blockchain-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `consensus/ — PoH, PoW, PoS, Hybrid`
- `nodes/ — Node-Management`
- `wallet/ — Wallet-Logik`

**Namenskonvention:** snake_case: hybrid_consensus.atc, poh.atc, pos.atc

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-contracts

**Typ:** Smart Contract-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `contracts/ — Contract-Implementierungen`

**Namenskonvention:** snake_case: base_contract.atc, atcoin.atc, governance_contract.atc

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-contracts-wiki

**Typ:** Contracts-Wiki

**Verzeichnisstruktur:**
- `Root: README, FILE_REGISTER`

**Namenskonvention:** contract-XX-name.md

**Pflichtdateien:** README.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-franchise

**Typ:** Franchise-Modul (Dezentrale Business-DAOs)

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `factory.atc — Franchise-Factory`

**Namenskonvention:** snake_case: factory.atc, routes.atc

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-franchise-wiki

**Typ:** Franchise-Wiki

**Verzeichnisstruktur:**
- `Root: README, FILE_REGISTER`

**Namenskonvention:** franchise-XX-name.md

**Pflichtdateien:** README.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-frontend

**Typ:** Frontend-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `src/ — Frontend-Komponenten`

**Namenskonvention:** PascalCase für Komponenten: Renderer.tsx, Dashboard.tsx

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-gateway

**Typ:** API-Gateway-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `main.atc — Gateway-Haupteinstieg`
- `middleware/ — Auth, Rate-Limit, Logger`

**Namenskonvention:** snake_case: gateway.atc, rate_limit.atc, auth.atc

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-gateway-wiki

**Typ:** Gateway-Wiki

**Verzeichnisstruktur:**
- `Root: README, FILE_REGISTER`

**Namenskonvention:** gateway-XX-name.md

**Pflichtdateien:** README.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-genesis-engine

**Typ:** Genesis-Engine-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `engine/ — Genesis-Logik`

**Namenskonvention:** snake_case: genesis.atc, validator_genesis.atc

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-kernel

**Typ:** Kernel-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS, CHANGELOG`
- `src/ — Kernel-Quellcode (.atc/.py)`
- `tests/ — Kernel-Tests`

**Namenskonvention:** snake_case: kernel.atc, syscalls.atc, scheduler.atc

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-kernel-wiki

**Typ:** Kernel-Wiki

**Verzeichnisstruktur:**
- `Root: README, FILE_REGISTER`
- `wiki/ — Kernel-Dokumentation`

**Namenskonvention:** kernel-XX-name.md

**Pflichtdateien:** README.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-linux-edition

**Typ:** Linux-Plattform-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `src/ — Linux-spezifischer Code (.rs)`

**Namenskonvention:** snake_case: main.rs

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-mobile

**Typ:** Mobile-Modul (Wallet, Biometric)

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `wallet_api.atc`
- `biometric_auth.atc`

**Namenskonvention:** snake_case: wallet_api.atc, biometric_auth.atc

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-shivacore

**Typ:** ShivaCore Kernel (Rust-Implementierung)

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `src/ — Rust-Quellcode`
- `Cargo.toml — Rust-Projekt`

**Namenskonvention:** snake_case: main.rs, kernel.rs, scheduler.rs (Rust-Konvention)

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-shivacore-tools

**Typ:** ShivaCore Tools

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`

**Namenskonvention:** snake_case

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### genesis-chronicles

**Typ:** Genesis Chronicles NFT-Game-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `contracts/ — Genesis-Chronicles-Contract`

**Namenskonvention:** snake_case: genesis_chronicles_contract.atc, marketplace_contract.atc

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### genesis-chronicles-wiki

**Typ:** Genesis-Chronicles-Wiki

**Verzeichnisstruktur:**
- `Root: README, FILE_REGISTER`

**Namenskonvention:** genesis-chronicles-XX-name.md

**Pflichtdateien:** README.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-standards

**Typ:** Standards-Repository

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `standards/ — ATC-01 bis ATC-99`

**Namenskonvention:** ATC-XX-NAME.md (2-stellig, Bindestrich)

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-standards-wiki

**Typ:** Standards-Wiki

**Verzeichnisstruktur:**
- `Root: README, FILE_REGISTER`

**Namenskonvention:** standard-XX-name.md

**Pflichtdateien:** README.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-ui

**Typ:** UI-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`

**Namenskonvention:** PascalCase für Komponenten, snake_case für Logik

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-ui-wiki

**Typ:** UI-Wiki

**Verzeichnisstruktur:**
- `Root: README, FILE_REGISTER`

**Namenskonvention:** ui-XX-name.md

**Pflichtdateien:** README.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-whitepaper

**Typ:** Whitepaper-Repository

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS, CHANGELOG`
- `whitepaper.md — Hauptdokument`

**Namenskonvention:** whitepaper.md, appendix-XX.md

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atc-windows-edition

**Typ:** Windows-Plattform-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `src/ — Windows-spezifischer Code (.rs)`

**Namenskonvention:** snake_case: main.rs

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atclang

**Typ:** ATCLang Compiler & VM

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `lexer/ — Lexer-Module`
- `parser/ — Parser-Module`
- `vm/ — Virtual Machine`
- `stdlib/ — Standard Library`
- `compiler/ — Compiler-Kern`

**Namenskonvention:** snake_case: lexer.py, parser.py, vm.py, stdlib.atc

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atclang-wiki

**Typ:** ATCLang-Wiki

**Verzeichnisstruktur:**
- `Root: README, FILE_REGISTER`
- `wiki/ — ATCLang-Dokumentation`

**Namenskonvention:** atclang-XX-name.md

**Pflichtdateien:** README.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atcnet

**Typ:** P2P-Netzwerk-Modul

**Verzeichnisstruktur:**
- `Root: README, ROADMAP, STATUS`
- `p2p/ — P2P-Protokoll`

**Namenskonvention:** snake_case: p2p_propagation.atc, discovery.atc, nat_traversal.atc

**Pflichtdateien:** README.md, ROADMAP.md, STATUS.md, CHANGELOG.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### atcnet-wiki

**Typ:** ATCNet-Wiki

**Verzeichnisstruktur:**
- `Root: README, FILE_REGISTER`

**Namenskonvention:** atcnet-XX-name.md

**Pflichtdateien:** README.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### franchise-factory-wiki

**Typ:** Archiviertes Franchise-Wiki

**Verzeichnisstruktur:**
- `Root: README, FILE_REGISTER`

**Namenskonvention:** franchise-XX-name.md (archiviert)

**Pflichtdateien:** README.md, FILE_REGISTER.md, .gitignore, LICENSE

---


### kai-os-wiki

**Typ:** Legacy Wiki (Archiviert)

**Verzeichnisstruktur:**
- `Root: Vollständiges Legacy-Wiki (kai-os-wiki.md, 15K+ Zeilen)`
- `docs/ — Roadmap, Status, ChangeLog`

**Namenskonvention:** kai-os-wiki.md (Einzeldatei-Wiki) | ROADMAP.md, STATUS.md, CHANGELOG.md

**Pflichtdateien:** README.md, FILE_REGISTER.md, ROADMAP.md, STATUS.md, CHANGELOG.md, .gitignore, LICENSE

---

## Vollständigkeits-Matrix

| Repo | Typ | README | FILE_REG | ROADMAP | STATUS | CHANGELOG | .gitignore | LICENSE |
|------|------|--------|----------|---------|--------|-----------|------------|---------|
| a-townchain-os | Haupt-Code-Repository | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| a-townchain-os-docs | Haupt-Wiki & Dokumentation | ✅ | ✅ | ➖ | ✅ | ✅ | ✅ | ✅ |
| a-townchain-os-wiki | Wiki-Repo | ✅ | ✅ | ➖ | ➖ | ➖ | ✅ | ✅ |
| atc-aistudio | AI Studio-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-atclang | ATCLang Sync-Repo | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-atcpkg | Package Manager-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-backend | Backend-Service-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-blockchain | Blockchain-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-contracts | Smart Contract-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-contracts-wiki | Contracts-Wiki | ✅ | ✅ | ➖ | ➖ | ➖ | ✅ | ✅ |
| atc-franchise | Franchise-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-franchise-wiki | Franchise-Wiki | ✅ | ✅ | ➖ | ➖ | ➖ | ✅ | ✅ |
| atc-frontend | Frontend-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-gateway | API-Gateway-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-gateway-wiki | Gateway-Wiki | ✅ | ✅ | ➖ | ➖ | ➖ | ✅ | ✅ |
| atc-genesis-engine | Genesis-Engine-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-kernel | Kernel-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-kernel-wiki | Kernel-Wiki | ✅ | ✅ | ➖ | ➖ | ➖ | ✅ | ✅ |
| atc-linux-edition | Linux-Plattform-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-mobile | Mobile-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-shivacore | ShivaCore Kernel | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-shivacore-tools | ShivaCore Tools | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| genesis-chronicles | Genesis Chronicles NFT-Game-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| genesis-chronicles-wiki | Genesis-Chronicles-Wiki | ✅ | ✅ | ➖ | ➖ | ➖ | ✅ | ✅ |
| atc-standards | Standards-Repository | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-standards-wiki | Standards-Wiki | ✅ | ✅ | ➖ | ➖ | ➖ | ✅ | ✅ |
| atc-ui | UI-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-ui-wiki | UI-Wiki | ✅ | ✅ | ➖ | ➖ | ➖ | ✅ | ✅ |
| atc-whitepaper | Whitepaper-Repository | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atc-windows-edition | Windows-Plattform-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atclang | ATCLang Compiler & VM | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atclang-wiki | ATCLang-Wiki | ✅ | ✅ | ➖ | ➖ | ➖ | ✅ | ✅ |
| atcnet | P2P-Netzwerk-Modul | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| atcnet-wiki | ATCNet-Wiki | ✅ | ✅ | ➖ | ➖ | ➖ | ✅ | ✅ |
| franchise-factory-wiki | Archiviertes Franchise-Wiki | ✅ | ✅ | ➖ | ➖ | ➖ | ✅ | ✅ |
| kai-os-wiki | Legacy Wiki | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

---

*Auto-generiert 2026-08-05 · Aurora (MasterBrain · Base44) · v1.0.0*
