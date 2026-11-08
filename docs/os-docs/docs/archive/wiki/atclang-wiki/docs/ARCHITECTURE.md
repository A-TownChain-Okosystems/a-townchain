# 🏛️ Architektur — atclang

> **Repo:** [atclang](https://github.com/A-TownChain-Okosystems/atclang)
> **Layer:** L2-L4 | **Titel:** ATCLang Compiler
> **Stand:** 2026-08-06 | **Version:** v1.0.0

---

## Übersicht

ATCLang Compiler: Lexer, Parser, VM, Stdlib, Test-Framework.

## Komponenten

### ATCLang Module (.atc)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `programs/atc8300.atc` | 96 | Atc8300 |
| `programs/atcfs.atc` | 142 | Atcfs |
| `programs/atcnet.atc` | 135 | Atcnet |
| `programs/atcos_main.atc` | 1154 | Atcos Main |
| `programs/consensus.atc` | 144 | Consensus |
| `programs/event_bus.atc` | 75 | Event Bus |
| `programs/gateway.atc` | 138 | Gateway |
| `programs/governance.atc` | 113 | Governance |
| `programs/kernel.atc` | 148 | Kernel |
| `programs/shivamon.atc` | 162 | Shivamon |
| `programs/wallet.atc` | 124 | Wallet |

### Python Module (.py)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `compiler.py` | 102 | Compiler |
| `compiler/compiler.py` | 471 | Compiler |
| `lexer.py` | 115 | Lexer |
| `lexer/lexer.py` | 563 | Lexer |
| `parser.py` | 95 | Parser |
| `parser/ast_nodes.py` | 265 | Ast Nodes |
| `parser/parser.py` | 399 | Parser |
| `repl/repl.py` | 185 | Repl |
| `stdlib/atc_stdlib.py` | 69 | Atc Stdlib |
| `vm.py` | 98 | Vm |
| `vm/atcvm.py` | 887 | Atcvm |

## Abhängigkeiten

Dieses Repo ist Teil des A-TownChain Ökosystems und nutzt:
- [ATCLang Compiler](https://github.com/A-TownChain-Okosystems/atclang) für .atc Module
- [ATC Standards](https://github.com/A-TownChain-Okosystems/atc-standards) für Spezifikationen
- [Haupt-Wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs) für Governance

## Statistik

| Metrik | Wert |
|--------|------|
| Code-Dateien | 22 |
| .atc | 11 |
| .py | 11 |
| .rs | 0 |
| .ts | 0 |
| Total Zeilen | 5,680 |

---

*Auto-generiert 2026-08-06 · Aurora (MasterBrain · Base44)*
