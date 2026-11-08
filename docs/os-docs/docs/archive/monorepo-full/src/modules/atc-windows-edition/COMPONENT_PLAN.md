# 📋 Komponenten-Plan — atc-windows-edition

> **Erstellt:** 2026-08-06 | **Agent:** Aurora (MasterBrain · Base44)

## Übersicht

**Repo:** atc-windows-edition  
**Name:** ATC Windows Edition — Desktop Client  
**Beschreibung:** Windows-Desktop-Client für A-TownChain OS. egui-basiert, nativ, Wallet + Node + Explorer.  
**Layer:** L9 — User Apps  
**Sprint:** 3.0  
**ATC-Standards:** ATC-24

---

## Komponenten

### 1. src/main.rs

**Beschreibung:** Entry point: window, event loop, app state

**Status:** 📋 GEPLANT

**Schnittstellen:**
- Eingabe: —
- Ausgabe: —
- Abhängigkeiten: ATCLang Stdlib

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures
3. Modul ist im FILE_REGISTER.md eingetragen

---

### 2. src/app.rs

**Beschreibung:** App-Shell: tabs, navigation, theme, settings

**Status:** 📋 GEPLANT

**Schnittstellen:**
- Eingabe: —
- Ausgabe: —
- Abhängigkeiten: ATCLang Stdlib

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures
3. Modul ist im FILE_REGISTER.md eingetragen

---

### 3. src/wallet.rs

**Beschreibung:** Wallet-Tab: balance, send, receive, history

**Status:** 📋 GEPLANT

**Schnittstellen:**
- Eingabe: —
- Ausgabe: —
- Abhängigkeiten: ATCLang Stdlib

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures
3. Modul ist im FILE_REGISTER.md eingetragen

---

### 4. src/node.rs

**Beschreibung:** Node-Tab: status, peers, blocks, sync

**Status:** 📋 GEPLANT

**Schnittstellen:**
- Eingabe: —
- Ausgabe: —
- Abhängigkeiten: ATCLang Stdlib

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures
3. Modul ist im FILE_REGISTER.md eingetragen

---

### 5. src/explorer.rs

**Beschreibung:** Explorer-Tab: blocks, txs, addresses, search

**Status:** 📋 GEPLANT

**Schnittstellen:**
- Eingabe: —
- Ausgabe: —
- Abhängigkeiten: ATCLang Stdlib

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures
3. Modul ist im FILE_REGISTER.md eingetragen

---

## Implementierungs-Reihenfolge

1. `src/main.rs` — Entry point
2. `src/app.rs` — App-Shell
3. `src/wallet.rs` — Wallet-Tab
4. `src/node.rs` — Node-Tab
5. `src/explorer.rs` — Explorer-Tab

## Test-Strategie

1. Parse-Test: Jede .atc Datei muss mit ATCLang v0.3 Parser parsen
2. Unit-Tests: Mindestens 3 Tests pro Komponente
3. Integration-Test: Komponenten interagieren korrekt
4. Coverage-Ziel: >80%

## Dokumentations-Requirements

- ARCHITECTURE.md: Architektur-Baum + Komponenten-Übersicht ✅
- COMPONENT_PLAN.md: Dieser Plan ✅
- FILE_REGISTER.md: Datei-Liste ✅
- STATUS.md: Aktueller Status ✅
- ROADMAP.md: Sprint-Zuordnung ✅
- CHANGELOG.md: Änderungs-Historie ✅

---
*Auto-generiert 2026-08-06 · Aurora (MasterBrain · Base44)*
