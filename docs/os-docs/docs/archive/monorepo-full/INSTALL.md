# Installation — A-TownChain OS v1.0

> **Voraussetzungen:** Python 3.12+, Node.js 20+, Git, Docker (optional)

---

## Schnellstart (Docker)

```bash
# 1. Repository klonen
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os.git
cd a-townchain-os

# 2. Docker Compose starten (alle 7 Services)
docker compose -f docker/docker-compose.yml up -d

# 3. Services prüfen
curl http://localhost:4000/gateway/health    # Core
curl http://localhost:3000                   # Frontend
curl http://localhost:80/api/v1/health       # Gateway
```

Services:
| Service | Port | Beschreibung |
|---------|------|-------------|
| Core | 4000 | Blockchain Core |
| Blockchain | 5000 | Chain Node |
| Frontend | 3000 | React UI |
| Gateway | 80 | API Gateway |
| Contracts | 8002 | Smart Contract Engine |
| Franchise | 8003 | Franchise Factory |
| Game | 8001 | Game Engine |

---

## Installation (Development)

### Python Backend

```bash
# 1. Repository klonen
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os.git
cd a-townchain-os

# 2. Virtual Environment erstellen
python3.12 -m venv venv
source venv/bin/activate    # Linux/macOS
# venv\Scripts\activate     # Windows

# 3. Dependencies installieren
pip install -r requirements.txt
pip install -r modules/*/requirements.txt 2>/dev/null || true
pip install pytest pytest-cov pytest-asyncio  # Dev dependencies

# 4. Core starten
python -m src.gateway.main

# 5. Tests ausführen
./scripts/test.sh all
```

### Frontend (React + Vite)

```bash
# 1. Dependencies installieren
cd frontend
npm install

# 2. Dev Server starten
npm run dev

# 3. Production Build
npm run build

# 4. Frontend Tests
npm test
```

### ATCLang Compiler

```bash
# ATCLang ist im Monorepo enthalten
cd atclang
pip install -e .

# ATCLang ausführen
python -m atclang repl
```

---

## Build (Binary)

```bash
# Python Binary (PyInstaller)
pip install pyinstaller
pyinstaller --onefile --name a-townchain-os \
  --add-data "src:src" \
  --add-data "modules:modules" \
  --add-data "frontend:frontend" \
  src/gateway/main.py

# Frontend Build (Vite)
cd frontend && npm run build
```

---

## Konfiguration

```bash
# .env Datei erstellen
cat > .env << 'EOF'
ATC_CHAIN_ID=658467
ATC_ENV=development
ATC_VERSION=1.0.0
ATC_GATEWAY_PORT=4000
ATC_FRONTEND_PORT=3000
ATC_BLOCKCHAIN_PORT=5000
EOF
```

---

## Tests

```bash
# Alle Tests
./scripts/test.sh all

# Nur Unit Tests
./scripts/test.sh unit

# Integration Tests (braucht Docker)
./scripts/test.sh integration

# E2E Tests (braucht full stack)
./scripts/test.sh e2e

# Coverage Report
./scripts/test.sh coverage

# Test Report generieren
./scripts/test-report.sh
```

---

## Troubleshooting

| Problem | Lösung |
|---------|--------|
| `ModuleNotFoundError: src.gateway` | `export PYTHONPATH=/pfad/zum/repo` |
| Port bereits belegt | `docker compose down` dann `up` |
| Tests schlagen fehl | `pip install -r requirements.txt` erneut |
| Frontend lädt nicht | `cd frontend && npm install && npm run dev` |
| Permission denied | `chmod +x scripts/*.sh` |

---

**Agent:** Aurora #2 (6a275618) | **Datum:** 04.08.2026
