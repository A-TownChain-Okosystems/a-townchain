---
title: "ATC Org File Census 2026-10-09"
summary: "Exact-SHA Datei-Census ueber alle 33 Repos (26 aktiv, 7 archiviert) auf den Standard-Branches: 23.310 versionierte Dateien, aufgeschluesselt nach Code, Tests, Konfiguration, Dokumentation."
date: 2026-10-09
scr: SCR-0134
---

# ATC Org File Census — 2026-10-09

Automatisierter Census ueber die GitHub-API + git ls-tree. Gezaehlt wird der
Baumstand des Standard-Branches (nicht die Git-Historie). Exakter Branch und
Commit-SHA je Repository als Nachweis des Zaehlstands. Read-only — keine
Repositories veraendert.

## Kennzahlen gesamt

| Kennzahl | Wert |
|---|---|
| Sichtbare Repositories | 33 (26 aktiv, 7 archiviert) |
| Versionierte Dateien gesamt | **23,310** |
| davon aktive Repos | 22,872 Dateien (26 Repos) |
| davon archivierte Repos | 438 Dateien (7 Repos) |
| Quellcode-Dateien (aktiv) | 7,780 |
| Test-Dateien (aktiv) | 377 |
| Konfiguration/Build (aktiv) | 4,625 |
| Dokumentation (aktiv) | 9,532 |

## Aktive Repositories (26)

| Repository | Branch | SHA | Dateien | Code | Tests | Cfg/Build | Docs | Sonst. | Top-Sprachen |
|---|---|---|---:|---:|---:|---:|---:|---:|---|
| a-townchain-ecosystem | `main` | `32cfbcd2537d` | 11906 | 3859 | 177 | 2285 | 5324 | 261 | ATCLang 1580 / Rust 873 / Python 749 / TypeScript 698 |
| a-townchain-os-docs | `main` | `77e21b228280` | 5065 | 2422 | 119 | 240 | 2235 | 49 | ATCLang 1111 / Python 605 / TypeScript 447 / Rust 276 |
| atc-standards | `main` | `268d32be5f25` | 2154 | 43 | 4 | 1216 | 881 | 10 | Python 41 / ATCLang 5 / Rust 1 |
| a-townchain | `main` | `b9f13056fb62` | 1149 | 353 | 21 | 300 | 389 | 86 | ATCLang 188 / Rust 100 / Python 54 / TypeScript 27 |
| globus-os | `main` | `aad164fb21f8` | 460 | 267 | 0 | 74 | 106 | 13 | Rust 212 / ATCLang 51 / TypeScript 3 / Shell 1 |
| aurora-ai | `main` | `bfec6a5f0243` | 457 | 331 | 8 | 34 | 72 | 12 | TypeScript 200 / ATCLang 80 / JavaScript 28 / Rust 24 |
| genesis-engine | `main` | `f01c51698627` | 194 | 66 | 3 | 50 | 68 | 7 | Rust 34 / ATCLang 20 / Python 15 |
| .github | `main` | `eeacedf33fd4` | 154 | 12 | 1 | 93 | 42 | 6 | Python 12 / Shell 1 |
| atclang | `main` | `99e722c5cc75` | 130 | 36 | 3 | 25 | 42 | 24 | ATCLang 24 / Rust 13 / Python 2 |
| a-townchain-os | `main` | `8f5df13dba29` | 127 | 50 | 15 | 30 | 28 | 4 | Rust 65 |
| atc-contracts | `main` | `a7c8b417f482` | 124 | 37 | 14 | 18 | 25 | 30 | ATCLang 46 / JavaScript 2 / Shell 2 / Python 1 |
| atc-sdk | `main` | `34eef3459079` | 109 | 34 | 1 | 27 | 41 | 6 | ATCLang 23 / Rust 8 / Python 2 / TypeScript 2 |
| atc-shivacore | `main` | `7f93d3ff2573` | 101 | 25 | 1 | 23 | 47 | 5 | Rust 13 / ATCLang 12 / Python 1 |
| atc-marketplace | `main` | `5c3bab7da8a8` | 94 | 29 | 0 | 24 | 37 | 4 | ATCLang 12 / Rust 10 / TypeScript 7 |
| atc-engineering | `main` | `470e6a9f500f` | 91 | 34 | 0 | 22 | 31 | 4 | Rust 29 / Shell 3 / Python 2 |
| genesis-chronicles | `main` | `a702dc6710ec` | 90 | 36 | 0 | 19 | 31 | 4 | ATCLang 20 / Python 10 / Rust 6 |
| atc-ide | `main` | `2a17feda7ed1` | 84 | 73 | 0 | 8 | 2 | 1 | TypeScript 71 / HTML/CSS 2 |
| atc-zkp | `main` | `ffa8ec1098c5` | 60 | 8 | 0 | 25 | 17 | 10 | Rust 7 / Python 1 |
| genesis-franchise-factory | `main` | `ffe780b8ac3a` | 55 | 34 | 6 | 10 | 4 | 1 | ATCLang 28 / Python 12 |
| atc-algorithm | `main` | `9c5fe6aad473` | 52 | 6 | 0 | 17 | 25 | 4 | Rust 5 / Python 1 |
| atc-vm | `main` | `21132a49355e` | 49 | 7 | 1 | 19 | 16 | 6 | Rust 8 |
| atc-launchpad | `main` | `9bd5aeaf2dd9` | 46 | 5 | 1 | 18 | 20 | 2 | Rust 6 |
| atc-compute | `main` | `d0831dfb7259` | 45 | 2 | 0 | 18 | 23 | 2 | Rust 2 |
| atc-node | `main` | `1ee3a3aee6d6` | 44 | 10 | 1 | 16 | 14 | 3 | Rust 10 / Python 1 |
| demo-repository | `main` | `d9b0da9c966d` | 22 | 1 | 0 | 9 | 10 | 2 | HTML/CSS 1 |
| atc-toolchain | `main` | `82fe4a238603` | 10 | 0 | 1 | 5 | 2 | 2 | Python 1 |

## Archivierte Repositories (7)

| Repository | Branch | SHA | Dateien | Code | Tests | Cfg/Build | Docs | Sonst. | Top-Sprachen |
|---|---|---|---:|---:|---:|---:|---:|---:|---|
| atc-wallet | `main` | `812a31d29236` | 99 | 38 | 0 | 23 | 35 | 3 | Rust 15 / ATCLang 12 / Python 11 |
| atc-explorer | `main` | `c4da05304f21` | 70 | 27 | 0 | 18 | 22 | 3 | TypeScript 15 / ATCLang 12 |
| atc-interop | `main` | `5905fca02274` | 69 | 17 | 0 | 19 | 30 | 3 | ATCLang 9 / Rust 7 / Python 1 |
| atc-indexer | `main` | `7f3da49f3046` | 59 | 8 | 0 | 19 | 29 | 3 | ATCLang 6 / TypeScript 2 |
| atc-storage | `main` | `230746c1dc5e` | 49 | 3 | 0 | 19 | 25 | 2 | Rust 2 / Python 1 |
| atc-mining | `main` | `227f2d700307` | 48 | 5 | 0 | 19 | 22 | 2 | Rust 4 / Python 1 |
| atc-oracle | `main` | `8087d34ddb8c` | 44 | 2 | 0 | 17 | 23 | 2 | Rust 2 |

## Klassifizierungsregeln

- **Code**: Dateien mit Quell-Erweiterung (.rs, .py, .ts/.tsx, .js, .atc, .sh, .sql, .c/.h, .go, .html/.css, .java, .kt, .cpp)
- **Tests**: Pfade/Dateien mit Test-Signatur (/tests/, /test/, __tests__, test_*.py, *_test.rs, *.test.ts/.spec.ts, conftest.py, /e2e/) — vor Code priorisiert
- **Cfg/Build**: package*.json, Cargo.*, requirements*.txt, tsconfig, *.config.*, Dockerfile, Makefile, .github/workflows, *.yml/.yaml/.toml/.lock, Schemas
- **Docs**: .md, .txt, .rst, docs/-Pfade
- **Sonstige**: Assets, Bilder, Binaries, Lizenz- und Daten-Dateien

## Interpretations-Hinweise

1. **a-townchain-ecosystem (11.906 Dateien) ist der Umbrella** mit vendierten
   Komponenten-Kopien (git subtree). Die Zahlen dort duplizieren Inhalte der
   Quell-Repos und sind KEIN Mass fuer Unique-Code.
2. **.atc-Dateien** in os-docs/wiki (1.111) sind Spezifikations-/Beispiel-Dateien
   der KAI-OS-Wiki-Kanons und zaehlen als ATCLang-Code — es ist Doku-Code.
3. **atc-standards**: 1.216 "Cfg/Build"-Dateien sind ueberwiegend die Standard-
   und Registry-YAMLs (Governance-Daten, nicht Build-Konfiguration).
4. Die 7 archivierten Repos enthalten 0 Test-Dateien in Test-Signatur — konsistent
   mit dem SCR-0127-Stilllegungszustand; kanonischer Standort der Funktionen ist
   der Monorepo a-townchain/components/.

## Metadaten

- Ausfuehrung: 2026-10-09, automatisiert (git ls-tree je origin/<default-branch> nach fetch)
- Evidence: census.json (roh, inkl. voller SHAs) liegt im Ausfuehrungs-Arbeitsbereich
- scr: SCR-0134 - agent: ATC-AI-ARCH-001 - owner-auftrag: Michael Wroblewski
