# 🌐 A-TownChain OS — Devnet Setup v1.0

> **Dezentrales Entwicklungsnetz für lokale Entwicklung und Testen**

**Stand:** 2026-07-02 | **Version:** 1.0.0 | **Status:** ✅ Production-Ready

---

## 📋 Inhaltsverzeichnis

1. [Quick Start](#quick-start)
2. [Systemanforderungen](#systemanforderungen)
3. [Installation](#installation)
4. [Netzwerk-Topologie](#netzwerk-topologie)
5. [Services & Ports](#services--ports)
6. [Monitoring](#monitoring)
7. [Verwendung](#verwendung)
8. [Debugging](#debugging)
9. [Maintenance](#maintenance)

---

## 🚀 Quick Start

### 1️⃣ Clone & Setup (2 Minuten)

```bash
# Repo klonen
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os.git
cd a-townchain-os

# Devnet-Verzeichnis vorbereiten
mkdir -p devnet/{config,data,logs,monitoring}

# Skripte ausführbar machen
chmod +x devnet/*.sh
```

### 2️⃣ Devnet starten (1 Minute)

```bash
# Komplettes Devnet mit 5 Nodes + Monitoring
bash devnet/start_devnet.sh

# Output:
# ✓ Building Docker images...
# ✓ Starting Bootstrap Node (atc-bootstrap)...
# ✓ Starting 3 Validator Nodes...
# ✓ Starting Full Node...
# ✓ Starting Monitoring Stack...
# 
# 🎉 Devnet Ready!
#    API Gateway:     http://localhost:4000
#    RPC Endpoint:    http://localhost:9933
#    WebSocket:       ws://localhost:9944
#    Monitoring:      http://localhost:3001 (grafana/atcadmin2026!)
#    Explorer:        http://localhost:8080 (coming soon)
```

### 3️⃣ Test-Transaktion (1 Minute)

```bash
# Wallet erstellen
source devnet/.env
kai wallet create --name "dev-wallet" --network devnet

# Tokens von Faucet holen
kai faucet --address $(kai wallet list | grep dev-wallet | awk '{print $NF}') --amount 1000

# Transaktion senden
kai tx send --to 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --amount 100 --network devnet
```

---

## 💻 Systemanforderungen

| Komponente | Minimum | Empfohlen |
|-----------|---------|-----------|
| **OS** | Ubuntu 22.04 / macOS 13+ | Ubuntu 24.04 LTS |
| **CPU** | 4 Kerne | 8+ Kerne |
| **RAM** | 8 GB | 16+ GB |
| **Disk** | 50 GB SSD | 100 GB NVMe |
| **Docker** | 20.10+ | 24+ |
| **Docker Compose** | 2.0+ | 2.20+ |
| **Python** | 3.9+ | 3.11+ |
| **Node.js** | 18+ | 20+ |

### ✅ Abhängigkeiten installieren

```bash
# Ubuntu/Debian
sudo apt-get update && sudo apt-get install -y docker.io docker-compose python3 nodejs

# macOS
brew install docker docker-compose python3 node

# Docker Daemon starten
sudo systemctl start docker
sudo usermod -aG docker $USER
newgrp docker
```

---

## 📦 Installation

### Option A: Automatische Installation (Empfohlen)

```bash
bash devnet/install_devnet.sh
```

### Option B: Manuelle Installation

```bash
# 1. Devnet-Verzeichnis erstellen
mkdir -p devnet/{config,data,logs,monitoring}
cd devnet

# 2. docker-compose.yml kopieren
cp ../docker-compose.devnet.yml docker-compose.yml

# 3. Konfiguration generieren
python3 generate_config.py --nodes 5 --testnet devnet

# 4. Starten
docker-compose up -d
```

---

## 🌐 Netzwerk-Topologie

```
┌─────────────────────────────────────────────────────┐
│          Devnet Network (172.21.0.0/16)             │
└─────────────────────────────────────────────────────┘
     │
     ├─── 🔷 Bootstrap Node (172.21.0.10)
     │        ├─ P2P: 30333
     │        ├─ RPC: 9933
     │        └─ WebSocket: 9944
     │
     ├─── 🔶 Validator-1 (172.21.0.11)
     │        ├─ P2P: 30333
     │        └─ API Gateway: 4000
     │
     ├─── 🔶 Validator-2 (172.21.0.12)
     │        └─ P2P: 30333
     │
     ├─── 🔶 Validator-3 (172.21.0.13)
     │        └─ P2P: 30333
     │
     ├─── 🟢 Full Node (172.21.0.14)
     │        └─ Archival Sync
     │
     ├─── 📊 Prometheus (172.21.0.20)
     │
     ├─── 📈 Grafana (172.21.0.21)
     │        └─ Dashboard: :3001
     │
     └─── 💾 Redis (172.21.0.30)
          └─ Cache: 6379
```

---

## 🔌 Services & Ports

| Service | Container | Host Port | Internal Port | Zweck |
|---------|-----------|-----------|---------------|-------|
| **Bootstrap** | atc-bootstrap | 9933 | 9933 | RPC |
| | | 9944 | 9944 | WebSocket |
| | | 30333 | 30333 | P2P |
| **Validator-1** | atc-validator-1 | 4000 | 4000 | API Gateway |
| | | 4001 | 4001 | Metrics |
| **Validator-2** | atc-validator-2 | (internal) | 30333 | P2P |
| **Validator-3** | atc-validator-3 | (internal) | 30333 | P2P |
| **Full Node** | atc-fullnode | (internal) | 30333 | P2P |
| **Prometheus** | atc-prometheus | 9090 | 9090 | Monitoring |
| **Grafana** | atc-grafana | 3001 | 3000 | Dashboard |
| **Redis** | atc-redis | 6379 | 6379 | Cache |
| **IPFS** | atc-ipfs | 5001 | 5001 | File Storage |

---

## 📊 Monitoring

### Grafana Dashboard

```
URL: http://localhost:3001
User: admin
Pass: atcadmin2026!

Dashboards:
├─ Network Health
│  ├─ Block Height (alle Nodes)
│  ├─ Block Time
│  ├─ Transaction Throughput
│  ├─ Network Peers
│  └─ Validator Set
│
├─ Node Performance
│  ├─ CPU Usage
│  ├─ Memory Usage
│  ├─ Disk I/O
│  └─ Network Bandwidth
│
├─ Blockchain Metrics
│  ├─ Active Accounts
│  ├─ Total Transactions
│  ├─ Smart Contract Calls
│  └─ Gas Usage
│
└─ System Health
   ├─ Container Status
   ├─ Log Errors
   ├─ Failed Health Checks
   └─ Uptime
```

### Prometheus Queries

```promql
# Block Height pro Node
blockchain_block_height{node=~"validator.*"}

# Transaction Rate
rate(blockchain_transactions_total[1m])

# P2P Peer Count
libp2p_peers_count

# CPU Utilization
process_resident_memory_bytes
```

---

## 💻 Verwendung

### Agenten deployen

```bash
# Agent-Verzeichnis vorbereiten
mkdir -p devnet/agents/my-agent
cd devnet/agents/my-agent

# agent.toml erstellen
cat > agent.toml << 'EOF'
[agent]
name = "my-agent"
version = "1.0.0"
description = "Mein erster KAI-OS Agent"

[model]
name = "llama3-8b-q4"
inference = "local"
max_tokens = 2048
temperature = 0.7

[capabilities]
read_storage = true
write_storage = true
call_contracts = true
network_access = true

[budget]
compute = 1000
storage_gb = 10
max_runtime_minutes = 60
EOF

# Agent deployen
kai agent deploy --config agent.toml --network devnet

# Status abrufen
kai agent status my-agent --network devnet
```

### Smart Contract testen

```bash
# Verzeichnis vorbereiten
mkdir -p devnet/contracts
cd devnet/contracts

# Contract in ATCLang schreiben
cat > hello.atc << 'EOF'
contract HelloKAI {
  state {
    message: String = "Hello, KAI-OS!"
    counter: Int = 0
  }

  function greet() -> String {
    self.counter = self.counter + 1
    return self.message
  }

  function setMessage(msg: String) {
    self.message = msg
  }
}
EOF

# Kompilieren
kai contract compile hello.atc --output hello.wasm

# Deployen
kai contract deploy hello.wasm \
  --name "HelloKAI" \
  --network devnet \
  --from $(kai wallet address dev-wallet)

# Aufrufen
kai contract call HelloKAI.greet() --network devnet
```

### Multi-Node Testing

```bash
# Transaction von Node 1 zu Node 2 senden
kai tx send \
  --to 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  --amount 100 \
  --node http://validator-2.devnet:4000

# Alle Nodes synchronisiert?
for i in 1 2 3; do
  echo "=== Validator $i ==="
  kai status --node http://validator-$i.devnet:4000 | jq '.chain.height'
done
```

---

## 🐛 Debugging

### Logs anschauen

```bash
# Alle Services
docker-compose -f devnet/docker-compose.yml logs -f

# Einzelner Service
docker-compose -f devnet/docker-compose.yml logs -f atc-validator-1

# Nur Fehler
docker-compose -f devnet/docker-compose.yml logs atc-validator-1 | grep -i error
```

### Health Check

```bash
# Devnet-Gesundheit überprüfen
bash devnet/health_check.sh

# Output:
# ✓ Bootstrap Node:   HEALTHY (height: 1234)
# ✓ Validator-1:      HEALTHY (height: 1234)
# ✓ Validator-2:      HEALTHY (height: 1234)
# ✓ Validator-3:      HEALTHY (height: 1234)
# ✓ Full Node:        HEALTHY (height: 1234, archival: 100%)
# ✓ Prometheus:       HEALTHY
# ✓ Grafana:          HEALTHY
# ✓ Redis:            HEALTHY
# ✓ Network Peers:    8/8 connected
# 
# 🎉 All systems operational!
```

### Node Connection Test

```bash
# Nodes können sich gegenseitig sehen?
docker-compose -f devnet/docker-compose.yml exec atc-bootstrap \
  kai net peers

# Output:
# /ip4/172.21.0.11/tcp/30333/p2p/12D3KooW...
# /ip4/172.21.0.12/tcp/30333/p2p/12D3KooW...
# /ip4/172.21.0.13/tcp/30333/p2p/12D3KooW...
# /ip4/172.21.0.14/tcp/30333/p2p/12D3KooW...
# (4 peers connected)
```

### Blockchain State prüfen

```bash
# Genesis Block abrufen
curl http://localhost:9933/api/v1/block/0

# Aktuelle Chain Height
curl http://localhost:9933/api/v1/status | jq '.chain.height'

# Account Balance
curl http://localhost:9933/api/v1/account/5GrwvaEF... | jq '.balance'

# Alle Validator abrufen
curl http://localhost:9933/api/v1/validators
```

---

## 🔧 Maintenance

### Neustart

```bash
# Sanfter Neustart (graceful)
bash devnet/restart_devnet.sh

# Hard Reset (alle Daten gelöscht!)
bash devnet/reset_devnet.sh --hard
```

### Backup & Restore

```bash
# Backup erstellen
bash devnet/backup_devnet.sh --output backups/devnet-$(date +%Y%m%d).tar.gz

# Wiederherstellen
bash devnet/restore_devnet.sh --from backups/devnet-20260702.tar.gz

# Nur Blockchain State
bash devnet/backup_devnet.sh --only-blockchain
```

### Upgrade Node

```bash
# Neue Version bauen
docker build -t kaios/node:latest .

# Staging: 1 Node upgraden
docker-compose -f devnet/docker-compose.yml up -d --build atc-validator-3

# Status überprüfen (sollte weiter syncer mit anderen)
docker-compose -f devnet/docker-compose.yml logs -f atc-validator-3

# Alle upgraden
docker-compose -f devnet/docker-compose.yml up -d --build
```

### Metriken exportieren

```bash
# Prometheus Daten exportieren (letzte 7 Tage)
bash devnet/export_metrics.sh --days 7 --output metrics/

# Grafana Dashboards exportieren
bash devnet/export_dashboards.sh --output dashboards/
```

---

## 📋 Checkliste für Devnet-Verwendung

### Vor Entwicklung starten

- [ ] Docker & Docker Compose installiert
- [ ] Systemressourcen ausreichend (RAM, Disk)
- [ ] Git Repo geklont
- [ ] `start_devnet.sh` ausgeführt
- [ ] Alle 5 Nodes healthy (Health Check bestanden)
- [ ] API Gateway antwortet auf `:4000`
- [ ] Grafana accessible auf `:3001`

### Während Entwicklung

- [ ] Logs kontinuierlich monitoren
- [ ] Prometheus/Grafana Dashboard im Auge behalten
- [ ] Health Check regelmäßig ausführen
- [ ] Backups vor großen Tests machen

### Nach Entwicklung

- [ ] Tests ausführen
- [ ] Metriken exportieren
- [ ] Logs archivieren
- [ ] Devnet stoppen: `docker-compose down`
- [ ] Optional: Backup erstellen

---

## 🆘 FAQ & Troubleshooting

### Q: Devnet startet nicht
**A:** 
```bash
# 1. Docker daemon läuft?
sudo systemctl status docker

# 2. Ports frei?
lsof -i :9933,4000,3001

# 3. Logs lesen
bash devnet/health_check.sh
docker-compose logs --tail 100
```

### Q: Node syncen sich nicht
**A:**
```bash
# Bootstrap Node Logs
docker-compose logs atc-bootstrap | grep -i "peer\|sync"

# P2P Verbindungen prüfen
docker-compose exec atc-validator-1 kai net peers
```

### Q: Transaktion wird nicht bestätigt
**A:**
```bash
# Nonce zu hoch?
curl http://localhost:9933/api/v1/account/YOUR_ADDR | jq '.nonce'

# Gas ausreichend?
curl http://localhost:9933/api/v1/gas-price

# Validator aktiv?
curl http://localhost:9933/api/v1/validators
```

### Q: Grafana zeigt keine Daten
**A:**
```bash
# Prometheus läuft?
docker-compose ps atc-prometheus

# Data Sources in Grafana
# Settings → Data Sources → Prometheus → http://prometheus:9090

# Prometheus erbaut sich die Metriken (dauert ~2 Min)
```

---

## 📚 Weitere Ressourcen

- **Wiki:** [KAI-OS Complete Documentation](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/archive/kai-os-legacy/docs/kai-os-wiki.md)
- **API Docs:** [API Reference](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/archive/kai-os-legacy/docs/api-reference.md)
- **Contract Guide:** [ATCLang Smart Contracts](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/archive/kai-os-legacy/docs/atclang-guide.md)
- **Issues:** [GitHub Issues](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues)

---

**Erstellt:** 2026-07-02  
**Autor:** Michael Wroblewski + Aurora AI  
**Lizenz:** Apache 2.0
