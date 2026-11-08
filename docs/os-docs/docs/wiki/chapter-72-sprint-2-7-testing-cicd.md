# Kapitel 72 — Sprint 2.7: Testing + CI/CD

> **Stand:** 05.07.2026 | **Sprint:** 2.7 | **Standards:** ATC-98, 95

---

## Übersicht

Sprint 2.7 umfasst die Formalisierung des Testing-Standards (ATC-98), das ATCLang Test Framework (ATC-95) und die Reparatur der CI/CD-Pipeline.

## Aufgaben

| Task | Standard | Beschreibung | Status |
|------|----------|--------------|--------|
| T-601 | ATC-98 | Testing Standard v1 — Formalisieren | 📋 |
| T-602 | ATC-95 | ATCLang Test Framework — Vollständig | 📋 |
| T-603 | — | CI/CD: `npm ci` → `npm install` | 🔴 BLOCKED |
| T-604 | — | CodeQL Workflow reparieren | 🔴 BLOCKED |
| T-605 | — | GitHub Pages Deploy fixen | 🔴 BLOCKED |
| T-606 | — | Monitoring Stack Score → 80+ | 📋 |
| T-607 | — | Test Coverage > 90% (aktuell 87%) | 📋 |

## CI/CD Blocker

Die CI/CD-Pipeline ist durch Branch-Protection-Regeln blockiert. Der GitHub-Token des Agents hat keinen `workflow` Scope, weshalb Workflow-Dateien nicht per API gepusht werden können.

**Lösung:** Michael muss die Datei `.github/workflows/ci-cd.yml` manuell aktualisieren:
- `npm ci` → `npm install` (kein package-lock.json vorhanden)
- CodeQL Workflow: GitHub Actions Version aktualisieren
- GitHub Pages: Deploy-Branch korrigieren

Fix-Dateien liegen bereits in `ci-cd-fix/` im Code-Repo bereit.

## Testing Standard (ATC-98)

> Status: DRAFT — zu formalisieren

Geplante Test-Kategorien:
1. **Unit Tests** — ATCLang Modul-Tests (aktuell 60 Tests GRÜN)
2. **Integration Tests** — Multi-Modul-Interaktion
3. **Consensus Tests** — PoH/PoW/PoS Validierung
4. **Smart Contract Tests** — Contract-Execution
5. **Network Tests** — P2P, Discovery, Gossip
6. **Performance Tests** — TPS, Latency, Throughput

## ATCLang Test Framework (ATC-95)

> Status: DRAFT — zu implementieren

Geplante Features:
- `assert_eq()`, `assert_ne()`, `assert_true()`, `assert_false()`
- `test_contract()` Macro
- `before_each()` / `after_each()` Hooks
- Gas-Messung in Tests
- State-Rollback nach jedem Test

---

*Kapitel 72 · Sprint 2.7 · 05.07.2026 · Aurora*
