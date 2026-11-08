# 🏛️ Architektur — atc-backend

> **Repo:** [atc-backend](https://github.com/A-TownChain-Okosystems/atc-backend)
> **Layer:** L7 | **Titel:** Backend Services
> **Stand:** 2026-08-06 | **Version:** v1.0.0

---

## Übersicht

FastAPI REST API, JSON-RPC Server & Backend Orchestrator für A-TownChain OS.

## Komponenten

### ATCLang Module (.atc)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `api/kai_routes.atc` | 229 | Kai Routes |
| `api/orchestrator/orchestrator.atc` | 259 | Orchestrator |
| `api/routes/ai_routes.atc` | 175 | Ai Routes |
| `api/routes/api_routes.atc` | 232 | Api Routes |
| `api/server.atc` | 68 | Server |
| `db/connection.atc` | 125 | Connection |
| `db/repository.atc` | 228 | Repository |
| `wallet/wallet.atc` | 139 | Wallet |

### Python Module (.py)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `api/orchestrator/orchestrator.py` | 130 | Orchestrator |
| `db/connection.py` | 40 | Connection |
| `db/repository.py` | 196 | Repository |

## Statistik

| Metrik | Wert |
|--------|------|
| .atc | 8 |
| .py | 3 |
| .rs | 0 |
| .ts | 0 |
| Total Zeilen | 1,821 |

---

*Auto-generiert 2026-08-06 · Aurora*
