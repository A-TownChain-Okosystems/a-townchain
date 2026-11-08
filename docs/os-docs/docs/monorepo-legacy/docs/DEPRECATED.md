# 📦 DEPRECATED — Veraltete Komponenten

> **Stand:** 05.07.2026 | A-TownChain OS v1.0.0
> **Policy:** Non-EVM, ATCLang Only (ATC-99)

---

## Entfernte Komponenten

| Komponente | Entfernt am | Grund |
|-----------|-------------|-------|
| Alle Solidity Contracts (.sol) | 2026-06-14 | Non-EVM (AD-007) |
| Ethereum Integration Docs | 05.07.2026 | Non-EVM (AD-007) |
| Solana Integration Docs | 05.07.2026 | Non-EVM (AD-007) |
| Alle Python Stubs (.py) | 05.07.2026 | ATCLang First (AD-006/ATC-99) |
| ATC-1000er IDs | 05.07.2026 | Konsolidiert zu ATC-01–99 |
| KIP/AIP/ATS Bezeichnungen | 05.07.2026 | Konsolidiert zu ATC-01–99 |
| atclang/programs/ Demos (10) | 05.07.2026 | v0.1 Syntax, durch v0.3 Module ersetzt |
| blockchain/contracts/ v0.1 Duplikate (10) | 05.07.2026 | Durch modules/ v0.3 Versionen ersetzt |
| shiva_consensus.py | 05.07.2026 | Durch shiva_consensus.atc ersetzt |

## Behalten (mit Begründung)

| Komponente | Grund |
|-----------|-------|
| ATCLang Compiler (19 .py) | Infrastruktur — compiliert ATCLang zu Bytecode |
| bridge_contract.atc | Bridge + API Layer bleiben erhalten (Non-EVM zu anderen Chains) |
| config/mainnet_genesis.json | Mainnet-Konfiguration (Sprint 4.0) |

---

*Deprecated v1.0.0 — Aurora · 05.07.2026*

## Archiviert am 03.08.2026 (ATCLang Archive)

| Komponente | Archiv-Pfad | Ursprünglicher Pfad | Grund |
|-----------|-------------|---------------------|-------|
| atcos_main.atc (v1.0 Showcase) | `archive/atclang-v01/atcos_main.atc` | `atclang/programs/atcos_main.atc` | v1.0 Syntax, nicht parsebar mit v0.3+ |
| 6× Consensus v0.1 (.atc) | `archive/atclang-v01/consensus/` | `blockchain/consensus/` | Superseded by _atc8X v0.3 Versionen |
| 4× Contracts v0.1 (.atc) | `archive/atclang-v01/contracts/` | `blockchain/contracts/` | Superseded by modules/contracts/ v0.3 |
| 4× src/ Duplikate (.atc) | `archive/duplicates/` | `src/blockchain/`, `src/core/` | K3/K4 Migration Duplikate |

**Siehe:** `archive/ATCLANG_ARCHIVE.md` für vollständige Dokumentation.

*Update: Aurora · 03.08.2026*
