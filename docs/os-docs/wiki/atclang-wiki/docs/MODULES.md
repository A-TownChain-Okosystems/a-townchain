# 🧩 ATCLang Extended Modules & Standard Library Reference

> **Repository:** [atc-atclang](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-atclang)  
> **Stand:** 2026-08-05  

---

## 📂 Verzeichnis- & Modulstruktur

`atc-atclang` ist in saubere, eigenständige Submodule gegliedert:

---

## 1. Compiler Stack (`compiler/`)

- **`compiler.py`**: Transformation von AST-Knoten in Bytecode. Generiert Bytecode-Instructions mit Argumenten.
- **`optimizer.py`**: Optimierungsdurchläufe vor der Bytecode-Generierung. Minimiert Instruktionen und Ausführungszeiten.
- **`type_checker.py`**: Statische Analyse von Ausdrücken und Deklarationen. Erzwingt Typensicherheit.

---

## 2. Standard Library Stack (`stdlib/`)

- **`primitives.py`**: Basis-Typkonvertierungen (`to_int`, `to_string`, `is_bool`, `type_of`).
- **`math.py`**: Erweiterte Arithmetik (`abs`, `pow`, `sqrt`, `min`, `max`, `random`, `clamp`).
- **`string.py`**: String-Manipulation (`substring`, `replace`, `to_upper`, `to_lower`, `split`, `join`).
- **`collections.py` & `collections_ext.py`**: Listen-, Map-, Stack-, Queue- und Set-Implementierungen.
- **`crypto.py` & `crypto_ext.py`**: Kryptographische Hashing- und Signatur-Algorithmen (`sha256`, `keccak256`, `verify_ed25519`).
- **`chain.py`**: Blockchain-Interaktion (`get_block_number`, `get_balance`, `emit_event`).
- **`wallet.py`**: Wallet-Verwaltung (`get_address`, `sign_hash`).
- **`io.py` & `io_ext.py`**: Standard I/O, File-Access und Formatierte Ausgaben.

---

## 3. Sprach-Features v0.3.0 (`v03/`)

- **`atclang_v03_features.py`**: Implementiert neuartige v0.3.0 Syntaxkonstrukte, einschließlich erweiterter Pattern-Matching-Klauseln und Asynchroner Operations-Hooks.

---

## 4. Virtual Machine (`vm/`)

- **`atcvm.py`**: Vollständige Execution Engine mit Stack, Memory, Register-Handling und Metering.
