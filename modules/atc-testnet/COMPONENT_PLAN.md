# 📋 Komponenten-Plan — atc-testnet

> **Erstellt:** 2026-08-06 | **Agent:** Aurora (MasterBrain · Base44)

## Übersicht

**Repo:** `atc-testnet`
**Name:** ATC Testnet — Testnet & Devnet Infrastructure
**Beschreibung:** Testnet- und Devnet-Infrastruktur. Launcher, Genesis-Konfiguration, Node-Bootstrap, Validator-Setup, Snapshot-Management. Entlastet die Blockchain-Repo von infrastruktureller Logik.
**Layer:** L5 — Infrastructure
**Sprint:** 2.8
**ATC-Standards:** ATC-01
**Komponenten:** 8

---

## Komponenten-Liste

| # | Datei | Zeilen | Typ | Beschreibung |
|---|-------|--------|-----|-------------|
| 1 | `config/genesis_config.atc` | 22 | .atc | ATCLang v0.3 — Genesis Configuration |
| 2 | `config/mainnet_config.atc` | 27 | .atc | ATCLang v0.3 — Mainnet Configuration |
| 3 | `devnet/snapshot_manager.atc` | 17 | .atc | ATCLang v0.3 — Devnet Snapshot Manager |
| 4 | `launcher/devnet_launcher.atc` | 17 | .atc | ATCLang v0.3 — Devnet Launcher |
| 5 | `launcher/testnet_launcher.atc` | 30 | .atc | ATCLang v0.3 — Testnet Launcher |
| 6 | `nodes/node_bootstrap.atc` | 21 | .atc | ATCLang v0.3 — Node Bootstrap |
| 7 | `nodes/validator_setup.atc` | 21 | .atc | ATCLang v0.3 — Validator Setup |
| 8 | `scripts/deploy_testnet.atc` | 17 | .atc | ATCLang v0.3 — Testnet Deploy Script |

---

## Detaillierte Komponenten

### 1. `config/genesis_config.atc`

**Zeilen:** 22
**Typ:** .atc
**Beschreibung:** ATCLang v0.3 — Genesis Configuration
**Funktionen/Structs:** struct GenesisBlock, build_genesis, validate_genesis
**Status:** 🔄 STUB

---

### 2. `config/mainnet_config.atc`

**Zeilen:** 27
**Typ:** .atc
**Beschreibung:** ATCLang v0.3 — Mainnet Configuration
**Funktionen/Structs:** struct ChainConfig, get_mainnet_config, get_testnet_config, get_devnet_config
**Status:** 🔄 STUB

---

### 3. `devnet/snapshot_manager.atc`

**Zeilen:** 17
**Typ:** .atc
**Beschreibung:** ATCLang v0.3 — Devnet Snapshot Manager
**Funktionen/Structs:** create_snapshot, restore_snapshot, list_snapshots
**Status:** 🔄 STUB

---

### 4. `launcher/devnet_launcher.atc`

**Zeilen:** 17
**Typ:** .atc
**Beschreibung:** ATCLang v0.3 — Devnet Launcher
**Funktionen/Structs:** quick_start, reset, fund_account
**Status:** 🔄 STUB

---

### 5. `launcher/testnet_launcher.atc`

**Zeilen:** 30
**Typ:** .atc
**Beschreibung:** ATCLang v0.3 — Testnet Launcher
**Funktionen/Structs:** struct TestnetConfig, launch, launch_local, launch_docker, stop
**Status:** 🔄 STUB

---

### 6. `nodes/node_bootstrap.atc`

**Zeilen:** 21
**Typ:** .atc
**Beschreibung:** ATCLang v0.3 — Node Bootstrap
**Funktionen/Structs:** bootstrap_node, initial_sync, register_validator, health_bootstrap
**Status:** 🔄 STUB

---

### 7. `nodes/validator_setup.atc`

**Zeilen:** 21
**Typ:** .atc
**Beschreibung:** ATCLang v0.3 — Validator Setup
**Funktionen/Structs:** generate_validator_key, register_stake, configure_validator, activate_validator
**Status:** 🔄 STUB

---

### 8. `scripts/deploy_testnet.atc`

**Zeilen:** 17
**Typ:** .atc
**Beschreibung:** ATCLang v0.3 — Testnet Deploy Script
**Funktionen/Structs:** deploy, teardown, status
**Status:** 🔄 STUB

---

## Test-Strategie

1. Parse-Test: Jede .atc Datei muss mit ATCLang v0.3 Parser parsen
2. Unit-Tests: Mindestens 3 Tests pro Komponente
3. Integration-Test: Komponenten interagieren korrekt
4. Coverage-Ziel: >80%

---
*Auto-generiert 2026-08-06 · Aurora*
