# 🏗️ A-TownChain OS — Konsolidierungs-Roadmap: 24 Repos → 1 Software

> **Erstellt:** 05.07.2026 19:29 (Europe/Berlin)
> **Autor:** Aurora Agent
> **Ziel:** Aus 24 verteilten Repositories eine einzelne, baubare Software erstellen

---

## Ausgangslage

### Aktuelle Repositories (24)

| Repo | Sprache | Größe | Inhalt |
|------|---------|-------|--------|
| `a-townchain-os` | TS/Python | 2,312 KB | Hauptcode (868 Dateien) |
| `a-townchain-os-docs` | TS/MD | 1,942 KB | Dokumentation (921 Dateien) |
| `kai-os-wiki` | Python/MD | 765 KB | Wiki-Mirror (285 Dateien) |
| `atclang` | Python | 81 KB | ATCLang Compiler |
| `atc-kernel` | Python | 49 KB | OS-Kernel |
| `atc-contracts` | Python | 46 KB | Smart Contracts |
| `atc-ui` | HTML | 38 KB | Web-UI |
| `atc-shivamon` | Python | 22 KB | Shivamon Game |
| `atcnet` | Python | 29 KB | P2P Netzwerk |
| `atc-franchise` | Python | 17 KB | Franchise System |
| `atc-gateway` | Python | 18 KB | API Gateway |
| `atc-standards` | — | 19 KB | ATC/ATS Standards |
| `atc-whitepaper` | — | 168 KB | Whitepaper |
| `A-TownChain-Okosystems/*` | TS/Python | — | AI Studio App (378 Dateien) |
| 11× `*-wiki` Repos | — | je 7-23 KB | Wiki pro Modul |

### Probleme
1. **Code verstreut:** Kernmodule in 10 separaten Repos, schwer zu bauen
2. **Kein einheitliches Build-System:** Jedes Repo hat eigene Struktur
3. **Abhängigkeiten unklar:** Module referenzieren sich gegenseitig ohne Package-Manager
4. **Wiki-Fragmentierung:** 12 Wiki-Repos statt einer Doku
5. **Keine Release-Pipeline:** Kein `./build.sh` oder `make` für das Gesamtprojekt

### Ziel
**Eine einzelne Software**, die mit einem Befehl gebaut und gestartet werden kann:
```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os
cd a-townchain-os
./build.sh        # Buildet alles: Python + TypeScript + Docker
./start.sh        # Startet: Gateway + Core + Chain + Wallet + AI + Game
```

---

## Konsolidierungs-Sprints (K1–K8)

### Sprint K1 — Repository Audit & Mapping (Woche 1)

**Ziel:** Vollständiges Inventar aller 24 Repos mit Abhängigkeitsgraph

#### Todos
- [ ] **K1.1** — Jedes Repo klonen und Dateiinventar erstellen (JSON-Schema)
- [ ] **K1.2** — Duplikate identifizieren (gleiche Dateien in mehreren Repos)
- [ ] **K1.3** — Abhängigkeitsgraph erstellen (welches Modul importiert welches)
- [ ] **K1.4** — Python-Imports analysieren (`import atc.*`, `from modules.*`)
- [ ] **K1.5** — TypeScript-Imports analysieren (`import { ... } from './...'`)
- [ ] **K1.6** — Dead Code identifizieren (ungenutzte Module/Dateien)
- [ ] **K1.7** — Konflikt-Liste erstellen (Dateien mit gleichem Pfad, unterschiedlichem Inhalt)
- [ ] **K1.8** — Konsolidierungs-Matrix erstellen (Quelle → Ziel-Mapping)

**Deliverable:** `KONSOLIDIERUNGS_MATRIX.md` mit vollständigem Datei-Mapping

---

### Sprint K2 — Monorepo-Struktur erstellen (Woche 2)

**Ziel:** Neue Verzeichnisstruktur im `a-townchain-os` Repo definieren

#### Ziel-Struktur
```
a-townchain-os/
├── build.sh                    # Ein-Klick-Build
├── start.sh                    # Ein-Klick-Start
├── docker-compose.yml          # Multi-Service Docker
├── Makefile                    # GNU Make Targets
├── setup.py                    # Python Package
├── package.json                # TypeScript/Frontend
├── pyproject.toml              # Python Build Config
├── requirements.txt            # Python Dependencies (vereint)
├── tsconfig.json               # TypeScript Config
├── vite.config.ts              # Frontend Build
│
├── src/                        # Python-Backend (vereint)
│   ├── core/                   # ← atc-kernel + core/
│   ├── blockchain/             # ← blockchain/ + atc-contracts
│   ├── network/                # ← atcnet
│   ├── gateway/                # ← atc-gateway
│   ├── ai/                     # ← ai_kernel + AI Studio backend
│   ├── contracts/              # ← atc-contracts
│   ├── game/                   # ← atc-shivamon
│   ├── franchise/              # ← atc-franchise
│   └── atclang/                # ← atclang
│
├── frontend/                   # TypeScript Desktop App
│   ├── src/                    # ← AI Studio (Michael Wroblewski)
│   ├── package.json
│   └── vite.config.ts
│
├── modules/                    # Kernel-Module (bestehend)
│   ├── kernel/
│   ├── blockchain/
│   ├── ai/
│   └── ...
│
├── config/                     # Alle Konfigurationen
│   ├── mainnet_genesis.json
│   ├── validator.yaml
│   └── settings.json
│
├── docker/                     # Docker-Setup
│   ├── Dockerfile.core
│   ├── Dockerfile.gateway
│   ├── Dockerfile.frontend
│   └── docker-compose.yml
│
├── docs/                       # Vereinte Dokumentation
│   ├── wiki/                   # ← kai-os-wiki
│   ├── standards/              # ← atc-standards
│   ├── whitepaper/             # ← atc-whitepaper
│   ├── sprints/                # ← Sprint-Todos
│   └── issues/                 # ← Issue-Dokumentation
│
├── scripts/                    # Build & Deploy Scripts
│   ├── build.sh
│   ├── start.sh
│   ├── stop.sh
│   ├── test.sh
│   └── deploy.sh
│
├── tests/                      # Vereinigte Test-Suite
│   ├── unit/
│   ├── integration/
│   └── e2e/
│
└── .github/                    # CI/CD
    └── workflows/
        ├── build.yml
        ├── test.yml
        └── release.yml
```

#### Todos
- [ ] **K2.1** — Verzeichnisstruktur im `a-townchain-os` Repo erstellen
- [ ] **K2.2** — `src/` Verzeichnis mit Python-Modulen aufsetzen
- [ ] **K2.3** — `frontend/` Verzeichnis für TypeScript-App aufsetzen
- [ ] **K2.4** — `docs/` Verzeichnis mit Wiki-Content füllen
- [ ] **K2.5** — `docker/` Verzeichnis mit Multi-Service-Setup erstellen
- [ ] **K2.6** — `scripts/` Verzeichnis mit Build/Start/Stop-Scripts erstellen
- [ ] **K2.7** — `tests/` Verzeichnis-Struktur definieren
- [ ] **K2.8** — `.github/workflows/` CI/CD Pipeline Templates erstellen

**Deliverable:** Leere Monorepo-Struktur mit Build-Scripts und Docker-Setup

---

### Sprint K3 — Python-Backend konsolidieren (Woche 3-4)

**Ziel:** Alle Python-Module aus 10 Repos in `src/` zusammenführen

#### Todos
- [ ] **K3.1** — `atc-kernel` → `src/core/` migrieren
- [ ] **K3.2** — `atcnet` → `src/network/` migrieren
- [ ] **K3.3** — `atc-gateway` → `src/gateway/` migrieren
- [ ] **K3.4** — `atc-contracts` → `src/contracts/` migrieren
- [ ] **K3.5** — `atc-shivamon` → `src/game/` migrieren
- [ ] **K3.6** — `atc-franchise` → `src/franchise/` migrieren
- [ ] **K3.7** — `atclang` → `src/atclang/` migrieren
- [ ] **K3.8** — `core/` bestehend → `src/core/` mergen (Konflikte auflösen)
- [ ] **K3.9** — `blockchain/` bestehend → `src/blockchain/` mergen
- [ ] **K3.10** — `modules/` bestehend → `src/modules/` mergen
- [ ] **K3.11** — AI Studio `temp_repo/` Python → `src/` migrieren (133 Dateien)
- [ ] **K3.12** — Alle Python-Imports aktualisieren (`from src.core import ...`)
- [ ] **K3.13** — `__init__.py` Dateien in allen `src/` Unterverzeichnissen erstellen
- [ ] **K3.14** — `setup.py` / `pyproject.toml` mit allen Dependencies erstellen
- [ ] **K3.15** — `requirements.txt` konsolidieren (alle Deps aus allen Repos)
- [ ] **K3.16** — Python-Import-Test: `python -c "from src.core import kernel"` muss funktionieren

**Deliverable:** Alle Python-Module in `src/`, einheitliche Imports, `pip install -e .` funktioniert

---

### Sprint K4 — TypeScript Frontend konsolidieren (Woche 5)

**Ziel:** AI Studio Desktop App + bestehendes Frontend in `frontend/` zusammenführen

#### Todos
- [ ] **K4.1** — Michael Wroblewski AI Studio → `frontend/src/` migrieren (190 TS-Dateien)
- [ ] **K4.2** — Bestehendes `frontend/` aus a-townchain-os → `frontend/src/legacy/` migrieren
- [ ] **K4.3** — `package.json` konsolidieren (alle npm Dependencies)
- [ ] **K4.4** — `vite.config.ts` anpassen (Build-Output, Proxy auf Backend)
- [ ] **K4.5** — `tsconfig.json` mit Path-Aliases erstellen (`@/` → `frontend/src/`)
- [ ] **K4.6** — Backend API-Client generieren (TypeScript Types aus Python Endpoints)
- [ ] **K4.7** — Environment Variables standardisieren (`.env.example`)
- [ ] **K4.8** — Frontend Build-Test: `cd frontend && npm ci && npm run build`
- [ ] **K4.9** — Desktop App Wrapper (Electron / Tauri) konfigurieren
- [ ] **K4.10** — Wallpaper-Picker und Login-Overlay aus AI Studio integrieren

**Deliverable:** Frontend baut mit `npm run build`, Desktop App startet mit `npm run electron`

---

### Sprint K5 — Build-System & Docker (Woche 6)

**Ziel:** Ein-Klick-Build und Docker-Compose für alle Services

#### Todos
- [ ] **K5.1** — `build.sh` Script erstellen:
  ```
  1. pip install -r requirements.txt
  2. cd frontend && npm ci && npm run build
  3. python -m pytest tests/unit/
  4. docker build -t atc-core docker/Dockerfile.core
  5. docker build -t atc-frontend docker/Dockerfile.frontend
  ```
- [ ] **K5.2** — `start.sh` Script erstellen:
  ```
  1. Start Gateway (Port 4000)
  2. Start Core (Port 5000)
  3. Start Chain (Port 5001)
  4. Start Wallet (Port 5002)
  5. Start AI (Port 5003)
  6. Start Game (Port 5004)
  7. Start Frontend (Port 3000)
  ```
- [ ] **K5.3** — `stop.sh` Script (alle Services beenden)
- [ ] **K5.4** — `Makefile` mit Targets: `build`, `start`, `stop`, `test`, `clean`, `docker`
- [ ] **K5.5** — `docker-compose.yml` mit 7 Services:
  - `atc-gateway` (Port 4000)
  - `atc-core` (Port 5000)
  - `atc-chain` (Port 5001)
  - `atc-wallet` (Port 5002)
  - `atc-ai` (Port 5003)
  - `atc-game` (Port 5004)
  - `atc-frontend` (Port 3000)
- [ ] **K5.6** — `Dockerfile.core` (Python-Backend)
- [ ] **K5.7** — `Dockerfile.frontend` (TypeScript + Nginx)
- [ ] **K5.8** — `Dockerfile.gateway` (Nginx Reverse Proxy)
- [ ] **K5.9** — Docker Build-Test: `docker-compose up --build`
- [ ] **K5.10** — Health-Checks für alle Docker-Services

**Deliverable:** `./build.sh && ./start.sh` startet das gesamte System

---

### Sprint K6 — CI/CD Pipeline (Woche 7)

**Ziel:** Einheitliche GitHub Actions Pipeline für das Monorepo

#### Todos
- [ ] **K6.1** — `.github/workflows/build.yml` — Build auf Push (Python + TypeScript)
- [ ] **K6.2** — `.github/workflows/test.yml` — Unit + Integration Tests
- [ ] **K6.3** — `.github/workflows/release.yml` — Release auf Tag (Docker Images + Binaries)
- [ ] **K6.4** — CodeQL Security Scan (wöchentlich)
- [ ] **K6.5** — Bandit Python Security Scan (bei Push)
- [ ] **K6.6** — npm audit + pip audit (bei Push)
- [ ] **K6.7** — Docker Image Publishing (ghcr.io)
- [ ] **K6.8** — Release-Artifacts: Linux Binary, macOS Binary, Windows Binary
- [ ] **K6.9** — GitHub Pages Deployment (Frontend Demo)
- [ ] **K6.10** — Automated Changelog Generation

**Deliverable:** Push → Build → Test → Docker Image → Release

---

### Sprint K7 — Tests & QA (Woche 8)

**Ziel:** Vereinigte Test-Suite mit ≥80% Coverage

#### Todos
- [ ] **K7.1** — Bestehende 113 ATCLang-Tests in `tests/unit/atclang/` migrieren
- [ ] **K7.2** — Kernel-Tests in `tests/unit/core/` migrieren
- [ ] **K7.3** — Blockchain-Tests in `tests/unit/blockchain/` migrieren
- [ ] **K7.4** — Network-Tests in `tests/unit/network/` migrieren
- [ ] **K7.5** — Smart Contract Tests in `tests/unit/contracts/` migrieren
- [ ] **K7.6** — Integration-Tests: Gateway ↔ Core ↔ Chain (in `tests/integration/`)
- [ ] **K7.7** — E2E-Tests: Frontend → Backend → Blockchain (in `tests/e2e/`)
- [ ] **K7.8** — Docker-Compose Integration Test (alle 7 Services)
- [ ] **K7.9** — Test-Report: Coverage, Pass-Rate, Performance
- [ ] **K7.10** — `pytest --cov=src --cov-report=html` einrichten
- [ ] **K7.11** — Frontend Tests: Jest + React Testing Library
- [ ] **K7.12** — Test-Threshold in CI: ≥80% Coverage, 0 failing tests

**Deliverable:** `./test.sh` läuft alle Tests, Coverage ≥80%

---

### Sprint K8 — Release v1.0 (Woche 9-10)

**Ziel:** Erste baubare, installierbare Software-Version

#### Todos
- [ ] **K8.1** — `CHANGELOG.md` erstellen (alle Änderungen seit v0.1)
- [ ] **K8.2** — `INSTALL.md` mit Schritt-für-Schritt Anleitung
- [ ] **K8.3** — `UPGRADE.md` für bestehende Entwickler
- [ ] **K8.4** — Binary Builds (PyInstaller für Python, Vite für Frontend)
- [ ] **K8.5** — Docker Images publishen (ghcr.io/a-townchain-os/core:1.0)
- [ ] **K8.6** — GitHub Release mit Assets (Linux, macOS, Windows)
- [ ] **K8.7** — Release Notes v1.0
- [ ] **K8.8** — Alte Repos als `archived` markieren (mit Weiterleitung)
- [ ] **K8.9** — Wiki-Repos in `docs/` migrieren (alle 12 Wikis konsolidieren)
- [ ] **K8.10** — `atc-standards` → `docs/standards/` migrieren
- [ ] **K8.11** — `atc-whitepaper` → `docs/whitepaper/` migrieren
- [ ] **K8.12** — Alte Wiki-Repos archivieren
- [ ] **K8.13** — Developer Onboarding Guide aktualisieren
- [ ] **K8.14** — Landing Page (a-townchain-os.io) mit Download-Links
- [ ] **K8.15** — Community Announcement (Discord, Twitter, Blog)

**Deliverable:** v1.0 Release — eine Software, ein Befehl, ein Download

---

## Konsolidierungs-Matrix

| Quell-Repo | Ziel-Pfad | Sprint | Dateien |
|------------|-----------|--------|---------|
| `atc-kernel` | `src/core/` | K3 | ~15 |
| `atcnet` | `src/network/` | K3 | ~20 |
| `atc-gateway` | `src/gateway/` | K3 | ~12 |
| `atc-contracts` | `src/contracts/` | K3 | ~18 |
| `atc-shivamon` | `src/game/` | K3 | ~25 |
| `atc-franchise` | `src/franchise/` | K3 | ~10 |
| `atclang` | `src/atclang/` | K3 | ~40 |
| `A-TownChain-Okosystems/*` | `frontend/src/` | K4 | 190 |
| `kai-os-wiki` | `docs/wiki/` | K8 | 285 |
| `atc-standards` | `docs/standards/` | K8 | ~15 |
| `atc-whitepaper` | `docs/whitepaper/` | K8 | ~20 |
| 11× `*-wiki` | `docs/wiki/<module>/` | K8 | ~100 |

## Timeline

```
Woche 1:  K1 — Audit & Mapping
Woche 2:  K2 — Monorepo-Struktur
Woche 3-4: K3 — Python-Backend
Woche 5:  K4 — Frontend
Woche 6:  K5 — Build & Docker
Woche 7:  K6 — CI/CD
Woche 8:  K7 — Tests & QA
Woche 9-10: K8 — Release v1.0
```

## Erfolgskriterien

- [ ] `git clone` + `./build.sh` funktioniert auf Ubuntu 22.04 / macOS 14 / Windows 11
- [ ] `./start.sh` startet alle 7 Services
- [ ] `docker-compose up` startet das gesamte System
- [ ] Frontend erreichbar auf `localhost:3000`
- [ ] API Gateway erreichbar auf `localhost:4000`
- [ ] Blockchain synchronisiert zwischen Services
- [ ] Test-Coverage ≥80%
- [ ] Release v1.0 auf GitHub mit Binaries für Linux/macOS/Windows
- [ ] Alle alten Repos als `archived` markiert

---

*Erstellt von Aurora Agent · 05.07.2026 19:29 (Europe/Berlin)*
