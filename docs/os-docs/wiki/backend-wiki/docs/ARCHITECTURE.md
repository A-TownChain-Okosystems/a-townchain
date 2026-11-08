# 🏗️ System-Architektur — atc-backend

> **Repo:** `atc-backend` · **KAI-OS Standard:** ATS-1000

---

## 1. Überblick

`atc-backend` bildet die zentrale API- und Orchestrierungs-Schicht im A-TownChain OS Ökosystem. Es vermittelt zwischen den Frontend-Clients (wie `atc-ui` oder `atc-cli`) und den tiefer liegenden System-Komponenten (`atc-shivacore`, `atc-kernel`, `atcnet`).

---

## 2. Komponenten-Architektur

### 2.1 API Server (`api/server.atc`)
Der API-Server läuft auf Port 5000 und nimmt eingehende Anfragen entgegen. Er parst ATCLang-native Datentypen und leitet Requests an die entsprechenden Modul-Routen weiter.

### 2.2 Route Modules
- **`api/routes/api_routes.atc`**: Konsolidierte REST-Endpunkte für Wallet-Verwaltung, Node-Informationen, Blockchain-Status und Governance-Anfragen.
- **`api/kai_routes.atc`**: Agenten-Verwaltung für KAI-OS (Lifecycle, Task Assignment, IPFS State Hash Sync).
- **`api/routes/ai_routes.atc`**: High-Performance Endpunkte für KI-Inferenz, Prompt-Streaming und Modell-Status-Checks.

### 2.3 Task Queue Orchestrator (`api/orchestrator/orchestrator.py` / `.atc`)
Ein asynchroner Multi-Threaded Worker-Pool verarbeitet rechenintensive Aufgaben (wie Transaction Signing, KI-Inferenz und Blockchain-State Queries) entkoppelt vom HTTP-Request-Loop.

### 2.4 Data Layer (`db/`)
Verwaltet persistente Zustände mit SQL-Schema (`schema.sql`), Connection Pool (`connection.py`) und Abstraktions-Repository (`repository.py`).

---

## 3. Sicherheits- & Rate-Limiting-Konzept

- **JWT / Bearer Token Authentifizierung** für geschützte Agenten- und Wallet-Routen.
- **Circuit Breaker** im Orchestrator zur Vermeidung von Überlastung bei hoher Anfragenzahl.
- **Rate-Limiting** auf IP- und Key-Ebene gem. ATS-1000 Standard.
