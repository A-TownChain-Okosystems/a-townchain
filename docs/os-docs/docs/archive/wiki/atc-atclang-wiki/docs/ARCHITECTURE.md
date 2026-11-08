# 🏛️ Architektur — atc-atclang

> **Repo:** [atc-atclang](https://github.com/A-TownChain-Okosystems/atc-atclang)
> **Layer:** L2-L4 | **Titel:** ATCLang Sync
> **Stand:** 2026-08-06 | **Version:** v1.0.0

---

## Übersicht

ATCLang Compiler-Sync: v0.3 Features (lexer, parser, vm), v03-Module, atcos_main Showcase.

## Komponenten

### ATCLang Module (.atc)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `programs/atcos_main.atc` | 1161 | Atcos Main |

### Python Module (.py)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `compiler/compiler.py` | 561 | Compiler |
| `compiler/optimizer.py` | 558 | Optimizer |
| `compiler/type_checker.py` | 507 | Type Checker |
| `lexer/lexer.py` | 671 | Lexer |
| `parser/ast_nodes.py` | 392 | Ast Nodes |
| `parser/parser.py` | 1431 | Parser |
| `repl/repl.py` | 184 | Repl |
| `stdlib/atc_stdlib.py` | 69 | Atc Stdlib |
| `stdlib/chain.py` | 41 | Chain |
| `stdlib/collections.py` | 219 | Collections |
| `stdlib/collections_ext.py` | 143 | Collections Ext |
| `stdlib/crypto.py` | 155 | Crypto |
| `stdlib/crypto_ext.py` | 149 | Crypto Ext |
| `stdlib/encoding.py` | 210 | Encoding |
| `stdlib/io.py` | 107 | Io |
| `stdlib/io_ext.py` | 123 | Io Ext |
| `stdlib/math.py` | 154 | Math |
| `stdlib/primitives.py` | 244 | Primitives |
| `stdlib/string.py` | 99 | String |
| `stdlib/wallet.py` | 78 | Wallet |
| `v03/atclang_v03_features.py` | 352 | Atclang V03 Features |
| `vm/atcvm.py` | 997 | Atcvm |

## Statistik

| Metrik | Wert |
|--------|------|
| .atc | 1 |
| .py | 22 |
| .rs | 0 |
| .ts | 0 |
| Total Zeilen | 8,605 |

---

*Auto-generiert 2026-08-06 · Aurora*
