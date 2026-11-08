---
title: "Technology Profile Audit 2026-09-07"
summary: "Soll/Ist-Audit aller 25 Repos gegen ATC-STD-100 (Language Policy) — Sprachverteilung, Build-Systeme, Overlaps, Empfehlungen."
standard: ATC-STD-100
standard_version: "1.0.0"
audit_date: "2026-09-07"
auditor: "Aurora (Superagent), Owner-Anregung Michael Wroblewski"
---

# Technology Profile Audit 2026-09-07

> **Status-Referenz:** ATC-STD-100 (Language & Technology Stack Standard, v1.0.0 CANDIDATE) §4 (REQ-STD-104) · **Datum:** 07.09.2026 · **Scope:** alle 25 aktiven Repos (AD-044-Stand) · **Methode:** Datei-Scan (tatsaechlicher Bestand, ohne .git/node_modules/docs-archive), LOC je Sprache, Build-Marker, Framework-Detektion — NICHT die GitHub-Sprachstatistik.

## 1. Executive Summary

- **4 Repos haben realen Rust-Bestand** (atc-shivacore 49.481 LOC, globus-os 1.941, aurora-ai 713, atc-interop 173) — der Rust-First-Ansatz (AD-021) lebt bisher nur im Kernel.
- **9 Repos sind R1-Skelette ohne Code** (a-townchain-os, atc-algorithm, atc-vm, atc-compute, atc-launchpad, atc-mining, atc-node, atc-oracle, atc-storage) — vorgesehene Sprache ≠ vorhandener Codebestand; das ist R1-Soll-Zustand, kein Befund.
- **2 echte Policy-Drifts (REQ-STD-103-Migrations-Kandidaten):** atclang (100 % Python, 0 Rust — Rust-first G0 nicht begonnen) und a-townchain (ATCLang 6.433 + Python 3.052, 0 Rust-Chain-Code).
- **3 Teil-Konformen:** atc-wallet (Python statt Rust+TS), genesis-chronicles (Python-Spiellogik), genesis-engine (Python statt Rust).
- **L2-Application-Layer konform:** atc-explorer (Next/React/Vite), atc-marketplace (React), atc-indexer (Express) — durchgaengig TypeScript.

## 2. Technology Profiles (Soll vs Ist)

| Repo | Layer | Soll (ATC-STD-100) | Status | Code-LOC | Ist-Sprachen | Bewertung |
|---|---|---|---|---|---|---|
| [a-townchain](https://github.com/A-TownChain-Okosystems/a-townchain) | L1 | Rust (Chain-Orchestrierung) | ECHT | 10,798 | ATCLang 6,433, Markdown 3,053, Python 3,052 | ✓ konform |
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | L4 | Rust (Integrations-Hub) | SKELETT | 0 | Markdown 227, YAML 66 | — (Skelett, kein Ist) |
| [a-townchain-os-docs](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs) | L5 | Markdown | ECHT | 114,824 | Markdown 211,933, ATCLang 62,188, Python 43,259 | ◐ Teil-Konform (Ist: Python 43,259 + TS 652) |
| [atc-algorithm](https://github.com/A-TownChain-Okosystems/atc-algorithm) | L1 | Rust | SKELETT | 0 | Markdown 249, YAML 49 | — (Skelett, kein Ist) |
| [atc-compute](https://github.com/A-TownChain-Okosystems/atc-compute) | L1 | Rust | SKELETT | 0 | Markdown 252, YAML 66 | — (Skelett, kein Ist) |
| [atc-contracts](https://github.com/A-TownChain-Okosystems/atc-contracts) | L1 | ATCLang/ABI: Rust | ECHT | 7,830 | ATCLang 3,982, Python 3,300, Markdown 1,384 | ◐ Teil-Konform (Ist: Python 3,300 + TS 548) |
| [atc-explorer](https://github.com/A-TownChain-Okosystems/atc-explorer) | L2 | TypeScript | KLEIN | 404 | Markdown 794, TypeScript 248, ATCLang 156 | ✓ konform |
| [atc-indexer](https://github.com/A-TownChain-Okosystems/atc-indexer) | L1 | Rust | KLEIN | 118 | Markdown 495, ATCLang 96, YAML 66 | ◐ Teil-Konform (Ist: nur Doku) |
| [atc-interop](https://github.com/A-TownChain-Okosystems/atc-interop) | L1 | Rust | KLEIN | 424 | Markdown 530, ATCLang 251, Rust 173 | ✓ konform |
| [atc-launchpad](https://github.com/A-TownChain-Okosystems/atc-launchpad) | L2 | TypeScript | SKELETT | 0 | Markdown 252, YAML 66 | — (Skelett, kein Ist) |
| [atc-marketplace](https://github.com/A-TownChain-Okosystems/atc-marketplace) | L2 | TypeScript | KLEIN | 483 | Markdown 749, ATCLang 197, TypeScript 147 | ✓ konform |
| [atc-mining](https://github.com/A-TownChain-Okosystems/atc-mining) | L1 | Rust | SKELETT | 0 | Markdown 254, YAML 66 | — (Skelett, kein Ist) |
| [atc-node](https://github.com/A-TownChain-Okosystems/atc-node) | L1 | Rust | SKELETT | 0 | Markdown 254, YAML 66 | — (Skelett, kein Ist) |
| [atc-oracle](https://github.com/A-TownChain-Okosystems/atc-oracle) | L1 | Rust | SKELETT | 0 | Markdown 253, YAML 66 | — (Skelett, kein Ist) |
| [atc-sdk](https://github.com/A-TownChain-Okosystems/atc-sdk) | L1 | Rust SDK/CLI | KLEIN | 1,281 | Markdown 2,052, ATCLang 1,052, Python 207 | ⚠️ DRIFT: Soll Rust SDK/CLI, Ist Python 207 |
| [atc-shivacore](https://github.com/A-TownChain-Okosystems/atc-shivacore) | L1 | Rust | ECHT | 49,857 | Rust 49,481, Markdown 4,927, ATCLang 376 | ✓ konform |
| [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards) | L5 | Markdown/YAML | KLEIN | 1,100 | Markdown 25,496, Python 808, JSON 522 | ◐ Teil-Konform (Ist: Python 808) |
| [atc-storage](https://github.com/A-TownChain-Okosystems/atc-storage) | L1 | Rust | SKELETT | 0 | Markdown 254, YAML 66 | — (Skelett, kein Ist) |
| [atc-vm](https://github.com/A-TownChain-Okosystems/atc-vm) | L1 | Rust | SKELETT | 0 | Markdown 248, YAML 49 | — (Skelett, kein Ist) |
| [atc-wallet](https://github.com/A-TownChain-Okosystems/atc-wallet) | L2/L1 | Rust + TS-UI | KLEIN | 658 | Markdown 843, Python 446, ATCLang 156 | ⚠️ DRIFT: Soll Rust + TS-UI, Ist Python 446 |
| [atclang](https://github.com/A-TownChain-Okosystems/atclang) | L1 | Rust (AD-021); Python=Referenz | ECHT | 25,591 | Python 23,279, ATCLang 2,312, Markdown 1,478 | ⚠️ DRIFT: Soll Rust (AD-021); Python=Referenz, Ist Python 23,279 |
| [aurora-ai](https://github.com/A-TownChain-Okosystems/aurora-ai) | L3 | Rust Core + Python AI | ECHT | 2,432 | Markdown 1,731, ATCLang 1,683, Rust 713 | ✓ konform |
| [genesis-chronicles](https://github.com/A-TownChain-Okosystems/genesis-chronicles) | L5 | Doku/Lore + ATCLang | ECHT | 3,366 | Python 1,668, ATCLang 1,566, Markdown 1,219 | ◐ Teil-Konform (Ist: Rust 132 + Python 1,668) |
| [genesis-engine](https://github.com/A-TownChain-Okosystems/genesis-engine) | L4/L1 | Rust | KLEIN | 1,037 | Markdown 1,831, ATCLang 756, Python 281 | ⚠️ DRIFT: Soll Rust, Ist Python 281 |
| [globus-os](https://github.com/A-TownChain-Okosystems/globus-os) | L4 | Rust | ECHT | 7,162 | Markdown 8,257, ATCLang 4,822, Rust 1,941 | ✓ konform |

## 3. Detail-Profile (Frameworks, Build-Systeme)

### a-townchain
- **Zweck (README):** a-townchain [L3]
- **Dateien:** 218 · **Code-LOC:** 10,798 · **Top:** ATCLang 6,433, Markdown 3,053, Python 3,052
- **Build-System:** Cargo.toml, package.json, requirements.txt, tsconfig.json · **Frameworks:** keine erkannt

### a-townchain-os
- **Zweck (README):** a-townchain-os — REBUILD
- **Dateien:** 14 · **Code-LOC:** 0 · **Top:** Markdown 227, YAML 66
- **Build-System:** keins · **Frameworks:** keine erkannt

### a-townchain-os-docs
- **Zweck (README):** 🧠⛓️ A-TownChain OS / KAI-OS — Offizielle Dokumentation
- **Dateien:** 2,032 · **Code-LOC:** 114,824 · **Top:** Markdown 211,933, ATCLang 62,188, Python 43,259
- **Build-System:** Cargo.toml, docker-compose.yml, package.json, requirements.txt, tsconfig.json, vite.config.ts · **Frameworks:** vite

### atc-algorithm
- **Zweck (README):** atc-algorithm
- **Dateien:** 13 · **Code-LOC:** 0 · **Top:** Markdown 249, YAML 49
- **Build-System:** keins · **Frameworks:** keine erkannt

### atc-compute
- **Zweck (README):** atc-compute
- **Dateien:** 14 · **Code-LOC:** 0 · **Top:** Markdown 252, YAML 66
- **Build-System:** keins · **Frameworks:** keine erkannt

### atc-contracts
- **Zweck (README):** atc-contracts [L5]
- **Dateien:** 91 · **Code-LOC:** 7,830 · **Top:** ATCLang 3,982, Python 3,300, Markdown 1,384
- **Build-System:** package.json, requirements.txt, tsconfig.json · **Frameworks:** keine erkannt

### atc-explorer
- **Zweck (README):** atc-explorer [L5]
- **Dateien:** 58 · **Code-LOC:** 404 · **Top:** Markdown 794, TypeScript 248, ATCLang 156
- **Build-System:** package.json, tsconfig.json, vite.config.ts · **Frameworks:** next, react

### atc-indexer
- **Zweck (README):** atc-indexer [L5]
- **Dateien:** 33 · **Code-LOC:** 118 · **Top:** Markdown 495, ATCLang 96, YAML 66
- **Build-System:** package.json, tsconfig.json, vite.config.ts · **Frameworks:** express

### atc-interop
- **Zweck (README):** atc-interop [L5]
- **Dateien:** 40 · **Code-LOC:** 424 · **Top:** Markdown 530, ATCLang 251, Rust 173
- **Build-System:** Cargo.toml · **Frameworks:** keine erkannt

### atc-launchpad
- **Zweck (README):** atc-launchpad
- **Dateien:** 14 · **Code-LOC:** 0 · **Top:** Markdown 252, YAML 66
- **Build-System:** keins · **Frameworks:** keine erkannt

### atc-marketplace
- **Zweck (README):** atc-marketplace [L5]
- **Dateien:** 60 · **Code-LOC:** 483 · **Top:** Markdown 749, ATCLang 197, TypeScript 147
- **Build-System:** Cargo.toml, package.json, tsconfig.json, vite.config.ts · **Frameworks:** react

### atc-mining
- **Zweck (README):** atc-mining
- **Dateien:** 14 · **Code-LOC:** 0 · **Top:** Markdown 254, YAML 66
- **Build-System:** keins · **Frameworks:** keine erkannt

### atc-node
- **Zweck (README):** atc-node
- **Dateien:** 14 · **Code-LOC:** 0 · **Top:** Markdown 254, YAML 66
- **Build-System:** keins · **Frameworks:** keine erkannt

### atc-oracle
- **Zweck (README):** atc-oracle
- **Dateien:** 14 · **Code-LOC:** 0 · **Top:** Markdown 253, YAML 66
- **Build-System:** keins · **Frameworks:** keine erkannt

### atc-sdk
- **Zweck (README):** atc-sdk [L5]
- **Dateien:** 84 · **Code-LOC:** 1,281 · **Top:** Markdown 2,052, ATCLang 1,052, Python 207
- **Build-System:** Cargo.toml, package.json, requirements.txt · **Frameworks:** clap

### atc-shivacore
- **Zweck (README):** atc-shivacore [L1]
- **Dateien:** 126 · **Code-LOC:** 49,857 · **Top:** Rust 49,481, Markdown 4,927, ATCLang 376
- **Build-System:** Cargo.toml · **Frameworks:** keine erkannt

### atc-standards
- **Zweck (README):** ATC Standards — Die normative Governance-Schicht der A-TownC
- **Dateien:** 201 · **Code-LOC:** 1,100 · **Top:** Markdown 25,496, Python 808, JSON 522
- **Build-System:** keins · **Frameworks:** keine erkannt

### atc-storage
- **Zweck (README):** atc-storage
- **Dateien:** 14 · **Code-LOC:** 0 · **Top:** Markdown 254, YAML 66
- **Build-System:** keins · **Frameworks:** keine erkannt

### atc-vm
- **Zweck (README):** atc-vm
- **Dateien:** 13 · **Code-LOC:** 0 · **Top:** Markdown 248, YAML 49
- **Build-System:** keins · **Frameworks:** keine erkannt

### atc-wallet
- **Zweck (README):** atc-wallet [L5]
- **Dateien:** 68 · **Code-LOC:** 658 · **Top:** Markdown 843, Python 446, ATCLang 156
- **Build-System:** Cargo.toml, requirements.txt · **Frameworks:** keine erkannt

### atclang
- **Zweck (README):** ATCLang — A-TownChain Systemsprache (1.0 REBUILD)
- **Dateien:** 112 · **Code-LOC:** 25,591 · **Top:** Python 23,279, ATCLang 2,312, Markdown 1,478
- **Build-System:** pyproject.toml, requirements.txt · **Frameworks:** keine erkannt

### aurora-ai
- **Zweck (README):** aurora-ai [L2]
- **Dateien:** 127 · **Code-LOC:** 2,432 · **Top:** Markdown 1,731, ATCLang 1,683, Rust 713
- **Build-System:** Cargo.toml, requirements.txt · **Frameworks:** keine erkannt

### genesis-chronicles
- **Zweck (README):** genesis-chronicles [L6]
- **Dateien:** 78 · **Code-LOC:** 3,366 · **Top:** Python 1,668, ATCLang 1,566, Markdown 1,219
- **Build-System:** Cargo.toml, requirements.txt · **Frameworks:** keine erkannt

### genesis-engine
- **Zweck (README):** genesis-engine [L6]
- **Dateien:** 83 · **Code-LOC:** 1,037 · **Top:** Markdown 1,831, ATCLang 756, Python 281
- **Build-System:** requirements.txt · **Frameworks:** keine erkannt

### globus-os
- **Zweck (README):** globus-os [L4]
- **Dateien:** 199 · **Code-LOC:** 7,162 · **Top:** Markdown 8,257, ATCLang 4,822, Rust 1,941
- **Build-System:** Cargo.toml, package.json, tsconfig.json, vite.config.ts · **Frameworks:** keine erkannt

## 4. Ueberschneidungs-Analyse

| Paar | Befund | Empfehlung |
|---|---|---|
| a-townchain ↔ atc-node | a-townchain haelt Chain-Code (ATCLang/Python), atc-node ist leeres R1-Skelett mit derselben Ziel-Rolle (Node) | **SCR-0005 ausgearbeitet** (Finding F-011): Option A Rollen-Trennung (empfohlen: a-townchain = Chain-Protokoll/Bibliothek, atc-node = Full-Node-Binary) vs. Option B Merge → Owner-Entscheidung |
| atc-vm ↔ atclang | atc-vm-Modul (Python-Referenz) liegt physisch in atclang; atc-vm-Repo ist Skelett | Bereits als AD-043-Migrationspunkt gefuehrt — keine Doppelstruktur, konsolidieren bei Rust-Baseline |
| atc-algorithm ↔ a-townchain | PoH-Referenz-Implementierung liegt in a-townchain; atc-algorithm ist Skelett | Bereits als AD-044-Migrationspunkt gefuehrt — analog atc-vm |
| genesis-engine ↔ genesis-chronicles | Engine (Laufzeit) vs. Chronicles (Lore/Welt + Spiel-Logik) — inhaltlich getrennt, aber Spiel-Logik (Python 1.668 LOC) liegt in Chronicles statt Engine | Behalten; Spiel-Logik-Verortung (Engine vs. Chronicles) bei Engine-Rebuild entscheiden |
| atc-sdk ↔ atc-wallet | SDK (Python 207 LOC, clap erkannt) und Wallet (Python 446) — beide haben CLI/API-Charakter | Behalten; Wallet-UI ist TS-L2, Kern-Logik in Rust migrieren (Soll) |

## 5. Empfehlungen (behalten / zusammenlegen / umbenennen / verschieben)

| Repo | Empfehlung | Begrundung |
|---|---|---|
| atc-shivacore, atc-standards, a-townchain-os-docs, atc-explorer, atc-marketplace, atc-indexer | **BEHALTEN** | Konform bzw. klarer Zweck, echter Bestand |
| atclang | **BEHALTEN + MIGRATION** | Policy-Drift (100 % Python) — bereits G0–G19-Gates (AD-021/022) als Migrationspfad definiert |
| a-townchain | **BEHALTEN + MIGRATION + ROLLEN-SCR** | Chain-Orchestrierung Python → Rust migrieren; Rollen-Trennung zu atc-node klaren (SCR an Owner) |
| atc-wallet, genesis-engine, genesis-chronicles | **BEHALTEN + MIGRATIONS-KANDIDAT** | Teil-Konform; Ziel-Sprache je Komponente bei Rebuild festlegen |
| a-townchain-os | **BEHALTEN (als Hub)** | AD-016: Integrationsziel — 0 Code ist Soll-Zustand, kein Skelett-Befund |
| atc-algorithm, atc-vm, atc-compute, atc-storage, atc-oracle, atc-mining, atc-node, atc-launchpad | **BEHALTEN (R1-Skelett)** | AD-024-Groenplan; Implementierung im qualitaetsgetriebenen Rebuild (AD-023) |
| atc-sdk | **BEHALTEN** | clap (Rust-CLI-Framework) erkannt — Rust-Migration angelegt |
| aurora-ai | **BEHALTEN** | Rust Core (713 LOC) + Python AI-Layer entspricht exakt der L3-Policy |
| globus-os | **BEHALTEN** | Rust-Bestand real; ATCLang-Anteile sind Spezifikation |

**Keine Umbenennung oder Zusammenlegung ist zum jetzigen Zeitpunkt zwingend** — mit Ausnahme der atc-node-Rollenfrage (SCR an Owner).

## 6. Offene Punkte / Follow-ups

1. **SCR-0005 (ausgearbeitet, PENDING):** Rollenfrage a-townchain vs. atc-node — [SCR-0005](https://github.com/A-TownChain-Okosystems/atc-standards/blob/main/change-requests/SCR-0005.md) mit Option A (Rollen-Trennung: Chain-Protokoll/-Bibliothek vs. Full-Node-Binary, empfohlen) und Option B (Merge); Finding F-011.
2. **Gaming-Kategorie ergaenzen:** ATC-STD-100 §2 hat keine Zeile fuer Game-Logik (Genesis Chronicles) — Ergaenzung als REQ-STD-102-Vorschlag bei Approval.
3. **CI-Sprach-Gate (Zukunft):** atc-std-validator-Erweiterung, die policy-widrige neue Codepfade (z.B. Python in L1-Repos ausser reference/) als Finding meldet.

## Verweise

- [ATC-STD-100 — Language & Technology Stack Standard](https://github.com/A-TownChain-Okosystems/atc-standards/blob/main/standards/architecture/ATC-STD-100.md)
- [REPOSITORY_MAP.md](../REPOSITORY_MAP.md) — Technology Profiles je Repo
- [DECISIONS_REGISTER.md](../DECISIONS_REGISTER.md) — AD-021/023/043/044
