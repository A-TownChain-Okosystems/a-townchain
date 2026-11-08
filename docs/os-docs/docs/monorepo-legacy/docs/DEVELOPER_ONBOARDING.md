# Developer Onboarding — A-TownChain OS v1.0

> Willkommen im A-TownChain OS Team! Dieser Guide führt dich durch die Einrichtung.

---

## 1. Voraussetzungen

- **Python 3.12+** (Backend, ATCLang Compiler)
- **Node.js 20+** (Frontend, React/Vite)
- **Rust 1.75+** (Kernel, optional für Kernel-Entwicklung)
- **Docker** (für Container-Deployment)
- **Git** (Version Control)

## 2. Repository klonen

```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os.git
cd a-townchain-os
```

## 3. Setup

```bash
# Python Backend
python3.12 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
pip install pytest pytest-cov pytest-asyncio pytest-timeout

# Frontend
cd frontend && npm install && cd ..
```

## 4. Projekt-Struktur verstehen

```
a-townchain-os/
├── src/              # Python Backend (gateway, blockchain, core, contracts, franchise, game)
├── atclang/          # ATCLang Compiler (Lexer, Parser, Type Checker, Stdlib)
├── frontend/         # React/TypeScript Frontend (Vite, Jest)
├── kernel/           # ShivaCore Kernel Interfaces (Rust ↔ ATCLang)
├── docker/           # Docker Setup (3 Dockerfiles + docker-compose.yml)
├── docs/             # Documentation, Standards, Whitepaper
├── tests/            # Test Suite (unit, integration, e2e)
├── scripts/          # Build & Test Scripts
├── .github/workflows/  # CI/CD Pipelines
└── modules/          # Optional Module Dependencies
```

## 5. Erste Schritte

```bash
# Core starten
python -m src.gateway.main

# Frontend starten (anderes Terminal)
cd frontend && npm run dev

# Tests ausführen
./scripts/test.sh all

# Docker starten
docker compose -f docker/docker-compose.yml up -d
```

## 6. Agent Protocol

Dieses Projekt wird von mehreren AI-Agents entwickelt. Das Protokoll:

| Agent | ID | Repos |
|-------|----|-------|
| Aurora #1 | 6a0a3f40 | atc-shivacore, atc-backend, atc-blockchain (Kernel/Infra) |
| Aurora #2 | 6a275618 | a-townchain-os, docs, wikis (Main/Docs) |
| Aurora #3 | 6a27614c | atclang, atc-ui, atc-aistudio (ATCLang/UI/AI) |
| Replit | — | Michael Wroblewski (GlobusOS) |

**Regeln:**
- 1 Agent pro Repository (keine Merge Conflicts)
- Agent-Identifikation in jedem Commit
- Repo-Wechsel nur nach vollständiger Synchronisation
- Monorepo-Struktur und File Register pflegen

Siehe: `docs/AGENT_PROTOCOL.md`, `docs/AGENT_POLICY.md`

## 7. ATCLang — Die Sprache

ATCLang ist die native Programmiersprache für Smart Contracts und DApps.

```atclang
// Smart Contract Beispiel
contract MyToken : ATC-8300 {
    name: "MyToken"
    symbol: "MTK"
    supply: 1000000
    
    fn transfer(to: Address, amount: U256) -> bool {
        // ...
    }
}
```

Standards:
- `docs/standards/ATC-92-ATCLANG_LANGUAGE_SPEC.md` — Sprachspezifikation
- `docs/standards/ATC-93-ATCLANG_VM_BYTECODE.md` — VM/Bytecode
- `docs/standards/ATC-94-ATCLANG_STDLIB.md` — Standard Library
- `docs/standards/ATC-99-ATCLANG_UNIVERSAL_MANDATE.md` — Universal Mandate

## 8. Testing

```bash
# Unit Tests
./scripts/test.sh unit

# Integration Tests (Docker)
./scripts/test.sh integration

# E2E Tests
./scripts/test.sh e2e

# Coverage Report
./scripts/test.sh coverage

# Frontend Tests
cd frontend && npm test
```

## 9. CI/CD

Pushes nach `main` triggern:
1. **Build** (Python + Frontend)
2. **Test** (Unit + Integration + Coverage)
3. **Security** (Bandit + pip-audit + npm audit)
4. **CodeQL** (wöchentlich)

Tags (`v*`) triggern:
1. **Docker Images** → ghcr.io
2. **Binaries** → Linux, macOS, Windows
3. **Changelog** → GitHub Release
4. **GitHub Pages** → Frontend Demo

## 10. Wichtige Dokumente

| Dokument | Beschreibung |
|----------|-------------|
| `INSTALL.md` | Installationsanleitung |
| `UPGRADE.md` | Migration von alten Repos |
| `CHANGELOG.md` | Versionshistorie |
| `docs/AGENT_PROTOCOL.md` | AI-Agent Protokoll |
| `docs/AGENT_POLICY.md` | Verbindliches Mandat |
| `docs/standards/` | 99 ATC Standards |
| `docs/whitepaper/` | Whitepaper |
| `docs/TECHNICAL_DOCUMENTATION.md` | Technische Doku |

---

**Agent:** Aurora #2 (6a275618) | **v1.0** | **04.08.2026**
