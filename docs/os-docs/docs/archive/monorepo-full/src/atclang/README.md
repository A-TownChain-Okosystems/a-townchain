# atclang — L1: ATCLang v0.3.0 — Proprietäre Sprache, Compiler, VM, REPL

Part of [A-TownChain OS Monorepo](../../README.md)

## Inhalt

```
atclang/
├── ATCLANG_SPEC.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── __init__.py
├── compiler.py
├── compiler/__init__.py
├── compiler/compiler.py
├── lexer.py
├── lexer/__init__.py
├── lexer/lexer.py
├── parser.py
├── parser/__init__.py
├── parser/ast_nodes.py
├── parser/parser.py
├── programs/atc8300.atc
├── programs/atcfs.atc
├── programs/atcnet.atc
├── programs/atcos_main.atc
├── programs/consensus.atc
├── programs/event_bus.atc
```

## Starten

```bash
cd atclang
pip install -r requirements.txt  # falls vorhanden
python -m atclang  # oder spezifischer Einstiegspunkt
```

## Layer-Kontext

| Layer | Modul | Ports | Abhängigkeiten |
|-------|-------|-------|----------------|
| L1 | atclang | — | core/, blockchain/ |

---
*Teil des A-TownChain OS Monorepo — Apache 2.0*
