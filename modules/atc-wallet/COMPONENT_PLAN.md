# 📋 Komponenten-Plan — atc-wallet

> **Erstellt:** 2026-08-06 | **Agent:** Aurora (MasterBrain · Base44)

## Übersicht

**Repo:** atc-wallet  
**Name:** ATC Wallet — Wallet Core  
**Beschreibung:** Wallet-Core-Logik. Key-Management, Transaction-Signing, Balance-Tracking, NFT-Management.  
**Layer:** L9 — User Apps  
**Sprint:** 2.5  
**ATC-Standards:** ATC-86, ATC-90

---

## Komponenten

### 1. keymanager.atc

**Beschreibung:** Key-Manager: keypair gen, import, export, encrypt, store

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

### 2. tx_signer.atc

**Beschreibung:** Transaction-Signer: build, sign, broadcast, RLP encoding

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

### 3. balance.atc

**Beschreibung:** Balance-Tracker: query chain, UTXO set, staking balance

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

### 4. nft_viewer.atc

**Beschreibung:** NFT-Viewer: list, metadata, transfer, history

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

### 5. history.atc

**Beschreibung:** Transaction-History: filter, paginate, export

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

### 6. faucet.atc

**Beschreibung:** Faucet-Integration: request testnet tokens

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

1. `keymanager.atc` — Key-Manager
2. `tx_signer.atc` — Transaction-Signer
3. `balance.atc` — Balance-Tracker
4. `nft_viewer.atc` — NFT-Viewer
5. `history.atc` — Transaction-History
6. `faucet.atc` — Faucet-Integration

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
