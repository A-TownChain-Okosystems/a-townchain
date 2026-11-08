# 🏛️ ATCLang Modular System Architecture

> **Repository:** [atc-atclang](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-atclang)  
> **Status:** Active / Production-Ready  
> **Stand:** 2026-08-05  

---

## 📌 Übersicht

`atc-atclang` stellt das erweiterte, modulare Fundament für die Programmiersprache **ATCLang** bereit. Es ergänzt den Kern-Compiler um eine mehrstufige Pipeline mit statischer Typenprüfung, AST-Optimierung, Version 0.3.0 Spracherweiterungen und eine umfangreiche Standardbibliothek.

---

## 🔄 Compilation & Execution Pipeline

```
 Source Code (.atc)
        |
        v
  [ Lexer Engine ]  ---> Token Stream
        |
        v
 [ Parser Engine ]  ---> Abstract Syntax Tree (AST)
        |
        v
 [ Type Checker ]   ---> Validated AST (Type Safe)
        |
        v
 [ AST Optimizer ]  ---> Optimized AST (Constant Folding, Dead Code Elimination)
        |
        v
[ Compiler Stack ]  ---> ATVM Bytecode
        |
        v
  [ ATVM Engine ]   ---> Execution & State Mutating Operations
```

---

## 🧩 Pipeline-Komponenten

### 1. Lexer (`lexer/lexer.py`)
- Unterstützt Keyword-Recognition (`fn`, `let`, `mut`, `struct`, `policy`, `contract`, `emit`)
- Behandelt Zahlenliterale, Strings, Identifikatoren und Kommentare
- Zeilen- und Spalten-Tracking für präzise Fehlermeldungen

### 2. Parser & AST (`parser/parser.py`, `parser/ast_nodes.py`)
- Rekursiver Abstiegsparser (Recursive Descent Parser)
- Strukturierte AST-Knoten für Funktionen, Variablen, Kontrollstrukturen und Verträge

### 3. Type Checker (`compiler/type_checker.py`)
- Verifiziert Typkompatibilität vor der Kompilierung
- Unterstützt primitive Typen (`int`, `float`, `string`, `bool`) und erweiterte Typen (`address`, `bytes32`)

### 4. AST Optimizer (`compiler/optimizer.py`)
- Constant Folding (z.B. `10 + 20` -> `30` zur Kompilierzeit)
- Dead Code Elimination und Loop Unrolling
- Inlining von einfachen Hilfsfunktionen

### 5. Compiler (`compiler/compiler.py`)
- Generierung von ATVM Opcodes
- Erzeugung von Symbol- und Sprungtabellen
- Ausfallfreie Bytecode-Emitierung

### 6. ATVM Runtime (`vm/atcvm.py`)
- Stack-basierte Ausführungsumgebung
- Unterstützung von Frame Pointern, lokalen Variablen und Return-Werten
- Durchsetzung der ATC-99 Lizenz- und Compliance-Regeln
