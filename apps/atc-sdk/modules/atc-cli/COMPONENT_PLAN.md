# 📋 Komponenten-Plan — atc-cli

> **Erstellt:** 2026-08-06 | **Agent:** Aurora (MasterBrain · Base44)

## Übersicht

**Repo:** atc-cli  
**Name:** ATC CLI — Command Line Interface  
**Beschreibung:** CLI-Tool für A-TownChain OS. Status, Wallet, Mining, Send, Mint, Explore, Install.  
**Layer:** L7 — Application  
**Sprint:** 3.0  
**ATC-Standards:** ATC-24, ATC-86

---

## Komponenten

### 1. kai_cli.atc

**Beschreibung:** CLI-Main: command parsing, help, config, interactive mode

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

### 2. repl.atc

**Beschreibung:** Interactive REPL: eval, history, multi-line, tab completion

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

### 3. ecdsa_impl.atc

**Beschreibung:** ECDSA-Tool: keypair gen, sign, verify, tx building

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

### 4. bigquery_pipeline.atc

**Beschreibung:** BigQuery-Pipeline: GitHub metrics, blockchain stats, sync

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

### 5. atc_issues_summary.atc

**Beschreibung:** Issue-Tracker: sprint assignment, summary, critical detection

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

### 6. hf_review_pipeline.atc

**Beschreibung:** HuggingFace-PR-Review: severity, PR summary

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

1. `kai_cli.atc` — CLI-Main
2. `repl.atc` — Interactive REPL
3. `ecdsa_impl.atc` — ECDSA-Tool
4. `bigquery_pipeline.atc` — BigQuery-Pipeline
5. `atc_issues_summary.atc` — Issue-Tracker
6. `hf_review_pipeline.atc` — HuggingFace-PR-Review

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
