# ATC-98 — Testing Standard

> **Standard-ID:** ATC-98  
> **Kategorie:** Testing  
> **Tier:** 0 — Foundational  
> **Status:** 📝 DRAFT — Spezifikation in Arbeit  
> **Version:** 0.1.0  
> **Autor:** Michael Wroblewski  
> **Sprint:** 2.7  
> **Wiki-Ref:** Kapitel 69  
> **Abhängigkeiten:** ATC-95 (ATCLang Test Framework)  

---

## 1. Zweck

ATC-98 definiert die verbindlichen Test-Standards für das gesamte A-TownChain OS / KAI-OS Ökosystem. Es legt Mindest-Coverage, Naming-Conventions, CI/CD-Integration und Test-Typen fest.

## 2. Test-Typen und Coverage

### 2.1 Mindest-Coverage
| Komponente | Mindest-Coverage | Branch-Coverage |
|------------|-----------------|-----------------|
| Core (Konsens, Krypto) | 95% | 85% |
| Smart Contracts | 90% | 80% |
| Kernel | 90% | 80% |
| Backend/Gateway | 85% | 70% |
| UI/Frontend | 80% | 65% |

### 2.2 Naming-Conventions
- Unit: `test_<module>_<function>_<case>.atc`
- Integration: `test_int_<componentA>_<componentB>.atc`
- E2E: `test_e2e_<scenario>.atc`

## 3. CI/CD-Integration

| Stufe | Tests | Blockiert bei Fehler |
|-------|-------|---------------------|
| Pre-commit | Unit | ✅ |
| PR-Merge | Unit + Integration | ✅ |
| Pre-Release | Alle + E2E | ✅ |
| Post-Release | Smoke-Tests | ⚠️ Alert |

### 3.1 Pipeline-Anforderungen
- `atc test --ci --coverage --gas-report`
- XML-Report für CI/CD-System
- Coverage-Trend-Tracking
- Gas-Cost-Regression-Detection

## 4. Gas-Cost-Tabelle

Meta-Standard — Gas-Kosten für Test-Operationen sind in ATC-95 spezifiziert.

## 5. Testing-Strategie

| Test | Beschreibung |
|------|-------------|
| Coverage | Mindest-Coverage pro Komponente (Sektion 2.1) |
| Naming | Konventionen für alle Test-Dateien |
| CI/CD | Pipeline-Integration und Blocking-Rules |
| Regression | Gas-Cost und Performance-Trending |

## 6. Querverweise

| Referenz | Dokument |
|----------|---------|
| ATC-95 | ATCLang Test Framework |
| ATC-99 | ATCLang Universal Mandate |
| Regel 8 | Release-Blocker (AGENT_MASTERRULES) |
