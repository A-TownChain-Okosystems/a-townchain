# 📛 A-TownChain OS — Naming Conventions Standard

> **Version:** 1.0
> **Erstellt:** 05.07.2026
> **Status:** VERBINDLICH für alle Repositories, Dokumente, Code, Issues und Notion-Seiten

---

## Zweck

Dieses Dokument definiert die kanonische Schreibweise aller Projekt-Begriffe. Abweichende Schreibweisen sind als Fehler zu betrachten und zu korrigieren.

---

## Kanonische Namens-Tabelle

| # | Konzept | Kanonisch (Prose) | Kanonisch (Code/Pfad) | Verboten |
|---|---------|-------------------|----------------------|----------|
| 1 | Betriebssystem | **A-TownChain OS** | `a-townchain-os` | ~~ATC-OS~~, ~~ATCOS~~, ~~atcos~~, ~~ATownChain OS~~ |
| 2 | Organisation | **A-TownChain-Okosystems** | `A-TownChain-Okosystems` | — (keine Abkürzung) |
| 3 | Wiki | **KAI-OS Wiki** | `kai-os-wiki` | ~~KAI OS~~, ~~KAI_OS~~, ~~kai_os~~, ~~KaiOS~~ |
| 4 | Programmiersprache | **ATCLang** | `atclang` | ~~ATC-Lang~~, ~~ATC_Lang~~, ~~ATCLANG~~, ~~AtcLang~~ |
| 5 | Engine | **Genesis Engine** | `genesis_engine` | ~~GenesisEngine~~, ~~genesis-engine~~ |
| 6 | Franchise Factory | **Genesis Franchise Factory (GFF)** | `gff` | ~~FranchiseFactory~~, ~~Franchise-Factory~~ |
| 7 | Communication Layer | **Genesis Communication Layer (GCL)** | `gcl` | ~~GenesisComm~~ |
| 8 | Civilization Platform | **Genesis Civilization Platform (GCP)** | `gcp` | ~~CivilizationPlatform~~ |
| 9 | MetaFactory | **MetaFactory** | `metafactory` | ~~Meta Factory~~, ~~meta-factory~~ |
| 10 | Spiel/Universum | **Shivamon** | `shivamon` | ~~SHIVAMON~~, ~~ShivaMon~~, ~~shiva_mon~~ |
| 11 | Betriebssystem (Gesamt) | **GlobusOS** | `globusos` | ~~ShivaOS~~ (veraltet seit 05.07.2026, siehe Architektur-Entscheidung), ~~GLOBUSOS~~, ~~Globus OS~~ |
| 11a | Kernel (bare-metal, Rust, K-Sprint-Track) | **ShivaCore** | `shivacore` | ~~ShivaOS~~ (das war der alte Name, bezeichnete faelschlich das ganze OS statt nur den Kernel) |
| 11b | Browser | **GateToHell** | `gatetohell` | ~~Gate to Hell~~, ~~GateToHell~~ als zwei Woerter |
| 12 | Sync-Agent | **Aurora Agent** | `aurora_agent` | ~~AuroraAgent~~, ~~aurora-agent~~ |
| 13 | Agent Protocol | **ATC-97** (Abkürzung), **Agent Interaction Protocol** (Vollname) | `aip_001` | ~~AIP001~~, ~~ATC-97~~ als Prose |
| 14 | Hauptnetz | **Mainnet** | `mainnet` | ~~MainNet~~, ~~MAINNET~~, ~~main-net~~ |
| 15 | Testnetz | **Testnet** | `testnet` | ~~TestNet~~, ~~TESTNET~~, ~~test-net~~ |
| 16 | Bootstrap-Knoten | **Bootstrap Node** | `bootstrap_node` | ~~BootstrapNode~~, ~~BootNode~~, ~~bootnode~~ |
| 17 | Smart Contracts | **Smart Contract** | `smart_contract` | ~~SmartContract~~, ~~smart-contract~~ |
| 18 | Validatoren | **Validator** | `validator` | ~~VALIDATOR~~ (nur in Code-Konstanten erlaubt) |
| 19 | Genesis-Wallet | **Genesis-Wallet** | `genesis_wallet` | ~~GenesisWallet~~, ~~Genesis Wallet~~ |
| 20 | Token | **ATC Token** | `atc_token` | ~~ATCToken~~, ~~ATC-Token~~, ~~ATCToken~~ |

---

## Regeln

### 1. Prosa (Dokumentation, README, Issues, Notion)
- Verwende immer die kanonische Schreibweise aus Spalte "Kanonisch (Prose)"
- Abkürzungen (GFF, GCL, GCP) beim ersten Vorkommen ausschreiben: "Genesis Franchise Factory (GFF)"
- Danach darf die Abkürzung verwendet werden

### 2. Code (Python, TypeScript, ATCLang)
- Verwende die kanonische Schreibweise aus Spalte "Kanonisch (Code/Pfad)"
- Klassen: PascalCase (`GenesisEngine`, `FranchiseFactory`)
- Module/Dateien: snake_case (`genesis_engine.py`, `franchise_factory.py`)
- Konstanten: UPPER_SNAKE_CASE (`MAINNET_CHAIN_ID`, `VALIDATOR_COUNT`)

### 3. Dateipfade und Verzeichnisse
- Verwende Kleinbuchstaben mit Bindestrich für Repo-Namen: `a-townchain-os`, `kai-os-wiki`
- Verwende Kleinbuchstaben mit Unterstrich für Dateien: `genesis_engine.py`, `atc_token.py`

### 4. ATCLang-spezifisch
- Der Sprachname in Prosa: **ATCLang** (ein Wort, camelCase)
- In Code-Pfaden: `atclang/` (Kleinbuchstaben)
- Dateiendung: `.atc` (Kleinbuchstaben)
- Niemals: "ATC-Lang", "ATC Lang", "ATCLANG" (außer in Konstanten)

### 5. Ausnahmen
- `VALIDATOR` als Python-Konstante ist erlaubt (z.B. `VALIDATOR_KEYS`)
- `TESTNET` als Konstante in Code ist erlaubt (z.B. `TESTNET_CHAIN_ID`)
- In GitHub Labels sind Bindestriche erlaubt: `sprint:K1`, `priority:high`

---

## Gefundene Inkonsistenzen (Stand 05.07.2026)

| Konzept | Varianten | Aktion |
|---------|-----------|--------|
| A-TownChain OS | "atc-os", "atcos" | → Korrigieren zu "A-TownChain OS" |
| KAI-OS | "KAI OS", "kai_os", "KAI_OS" | → Korrigieren zu "KAI-OS" |
| ATCLang | "ATCLANG" (all caps) | → Korrigieren zu "ATCLang" |
| Shivamon | "SHIVAMON", "ShivaMon" | → Korrigieren zu "Shivamon" |
| ShivaOS | "shivaos" in Prosa | → Korrigieren zu "ShivaOS" |
| Testnet | "TESTNET" in Prosa | → Korrigieren zu "Testnet" |
| Validator | "VALIDATOR" in Prosa | → Korrigieren zu "Validator" |
| Bootstrap Node | "BootstrapNode" | → Korrigieren zu "Bootstrap Node" |
| ATC Token | "ATCToken", "ATC-Token" | → Korrigieren zu "ATC Token" |

---

## ⚠️ Kritische Disambiguierung: "Kernel" ist NICHT eindeutig — zwei verschiedene Dinge!

> **Hinzugefuegt:** 06.07.2026 von aurora-base44-superagent-6a0a3f408dced6c5ca7506ef

Es gibt im Oekosystem **zwei komplett unterschiedliche "Kernel"**, die leicht verwechselt werden:

| | `atc-kernel` / `modules/kernel/` | **ShivaCore** |
|---|---|---|
| **Was** | Python/ATCLang OS-**Simulation** (GCL 13-Bus-Architektur, laeuft auf bestehendem OS/Linux) | Echter bare-metal Kernel (Rust, `no_std`, x86_64, bootet selbst ohne Unterbau) |
| **Repo/Ort** | `a-townchain-os` (`modules/kernel/`) + separates Repo `atc-kernel` | Aktuell NUR in einer Agent-Sandbox (`atc-kernel/`) — **noch nicht in einem GitHub-Repo!** |
| **Track** | K1-K8 Konsolidierungs-Track (Python-Backend-Zusammenfuehrung) | K-Sprint-0-9-Track ("Kernel from Scratch", eigene Architektur-Entscheidung 05.07.2026) |
| **Zustaendiger Agent-Fokus** | Konsolidierungs-/Doku-Agenten (z.B. `...105b5`) | Kernel-Entwicklungs-Agent (`...ca7506ef`) |

**Fuer alle Agenten:** Wenn irgendwo "Kernel-Arbeit" erwaehnt wird — IMMER pruefen welcher der beiden gemeint ist, bevor Status/Fortschritt zusammengefuehrt oder verglichen wird. Sie sind NICHT Fortschritt am selben Ziel.

**Bekannte Luecke:** Der ShivaCore-Kernel-Code existiert bisher nur lokal in einer Agent-Sandbox und ist fuer andere Agenten NICHT einsehbar. Solange er nicht in ein Repo gepusht ist, kann kein anderer Agent seinen Fortschritt verifizieren — nur die Textbeschreibungen in `AGENT_COORDINATION.md`.

---

*Dieses Dokument ist die einzige verbindliche Quelle für Namenskonventionen im A-TownChain OS Projekt.*
