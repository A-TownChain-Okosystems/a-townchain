# Upgrade Guide — A-TownChain OS v1.0

> **Für Entwickler, die von den alten 24 Repositories auf das Monorepo migrieren.**

---

## Was hat sich geändert?

A-TownChain OS war bisher über 24 separate Repositories verteilt. Mit v1.0
wurden alle in ein einziges Monorepo (`a-townchain-os`) konsolidiert.

### Alte → Neue Struktur

| Altes Repository | Neuer Pfad | Status |
|-----------------|-----------|--------|
| `atc-backend` | `src/gateway/` + `src/core/` | ✅ migriert |
| `atc-blockchain` | `src/blockchain/` | ✅ migriert |
| `atc-gateway` | `src/gateway/` | ✅ migriert |
| `atc-frontend` | `frontend/` | ✅ migriert |
| `atc-contracts` | `src/contracts/` | ✅ migriert |
| `atc-franchise` | `src/franchise/` | ✅ migriert |
| `atc-ui` | `frontend/` (komponenten) | ✅ migriert |
| `atclang` | `atclang/` | ✅ migriert |
| `atc-kernel` | `kernel/` (interfaces) | ✅ migriert |
| `atc-shivamon` | `src/core/shivamon/` | ✅ migriert |
| `atcnet` | `src/core/network/` | ✅ migriert |
| `atc-standards` | `docs/standards/` | ✅ migriert |
| `atc-whitepaper` | `docs/whitepaper/` | ✅ migriert |

### Archivierte Repositories (12)

Folgende Repos wurden archiviert (`📦ARCHIVED`):
- `atc-contracts`, `atc-contracts-wiki`, `atc-franchise`, `atc-franchise-wiki`
- `atc-gateway-wiki`, `atc-kernel`, `atc-kernel-wiki`
- `atc-shivamon`, `atc-shivamon-wiki`, `atc-standards`, `atc-standards-wiki`
- `atc-ui`, `atc-ui-wiki`, `atclang`, `atclang-wiki`
- `atcnet`, `atcnet-wiki`, `franchise-factory-wiki`

---

## Migration: Schritt-für-Schritt

### 1. Altes Repository sichern

```bash
# Backup der alten Repos
mkdir -p ~/atc-backup
cp -r ~/pfad/zu/atc-backend ~/atc-backup/
cp -r ~/pfad/zu/atc-blockchain ~/atc-backup/
# ... für alle alten Repos
```

### 2. Neues Monorepo klonen

```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os.git
cd a-townchain-os
```

### 3. Dependencies installieren

```bash
# Python
python3.12 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
pip install -r modules/*/requirements.txt 2>/dev/null || true

# Frontend
cd frontend && npm install && cd ..

# Dev dependencies
pip install pytest pytest-cov pytest-asyncio pytest-timeout
```

### 4. Import-Pfade anpassen

**Alt:**
```python
from blockchain import Blockchain
from gateway import Gateway
```

**Neu:**
```python
from src.blockchain import Blockchain
from src.gateway import Gateway
```

### 5. Konfiguration migrieren

```bash
# Alte .env Werte übertragen
cat > .env << 'EOF'
ATC_CHAIN_ID=658467
ATC_ENV=development
ATC_VERSION=1.0.0
EOF
```

### 6. Tests ausführen

```bash
./scripts/test.sh all
```

### 7. Docker (optional)

```bash
docker compose -f docker/docker-compose.yml up -d
```

---

## Breaking Changes

1. **Import-Pfade:** Alle Python-Imports verwenden jetzt `src.` Prefix
2. **ATCLang First:** Solidity entfernt, alle Contracts in ATCLang
3. **SHA-256:** PoH verwendet SHA-256 statt SHA-3
4. **Token Standards:** ATC-001, ATC-8300, ATC-9900 (keine ERC-20 Kompatibilität)
5. **Kernel:** ShivaCore (Rust) als separate Komponente, Interfaces in `kernel/`

---

**Agent:** Aurora #2 (6a275618) | **Datum:** 04.08.2026
