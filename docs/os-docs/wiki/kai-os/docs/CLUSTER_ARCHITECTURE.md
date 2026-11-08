# Cluster-Architektur — A-TownChain OS

> **Status:** Aktiv | **Quelle:** `docker-compose.yml` (Produktion-nah) + `docker/docker-compose.testnet.yml` (lokales Dev-Testnet)
> **Zugehörig:** Critical Path #18 (Docker Testnet), #8 (Multi-Node Testnet)
> *Auto-generiert von Aurora · 2026-07-08*

Das System hat aktuell **zwei getrennte, unabhängige Cluster-Setups**, die nicht miteinander verwechselt werden dürfen. Beide starten unter dem Chain-ID `9000` (Testnet).

---

## 1. Produktions-nahes Cluster (`docker-compose.yml`, Root)

5-Node-Setup mit Rollentrennung + vollem Monitoring-/Gateway-Stack. Nutzt das gebaute Image aus dem Root-`Dockerfile` (Multi-Stage-Build, non-root User `atcnode`).

### 1.1 Blockchain-Node-Cluster

| Service | Container | Rolle | Ports (Host:Container) | Abhängigkeit |
|---|---|---|---|---|
| `bootstrap` | atc-bootstrap | Seed-/Discovery-Node | 5005:5005, 9944:9944 | — |
| `validator-1` | atc-validator-1 | Block-Validator | 4000:4000, 4001:4001 | wartet auf `bootstrap` (healthy) |
| `validator-2` | atc-validator-2 | Block-Validator | 4011:4001 | wartet auf `bootstrap` (healthy) |
| `fullnode` | atc-fullnode | Voll-synchronisierter Node (kein Staking) | — | wartet auf `validator-1` (healthy) |
| `archive` | atc-archive | Vollständige Historie/Archiv-Node | — | — |

Jeder Node teilt sich das Basis-Template (`x-node-base`): eigenes Docker-Volume, Healthcheck gegen `/health` auf Port 4000, Restart-Policy `unless-stopped`, JSON-Logging (max. 50 MB × 5 Dateien).

### 1.2 Monitoring-Cluster

| Service | Image | Zweck | Port |
|---|---|---|---|
| `prometheus` | prom/prometheus:v2.50.0 | Metriken-Sammlung (Block-Zeit, TPS, Node-Health) | 9090 |
| `grafana` | grafana/grafana:10.2.0 | Dashboards | 3001:3000 |

Config liegt in `./monitoring/prometheus.yml`. Grafana-Admin-Passwort wird aktuell **hartcodiert** im Compose-File gesetzt (`GF_SECURITY_ADMIN_PASSWORD`) — sollte vor Produktivbetrieb in ein Secret ausgelagert werden.

### 1.3 Gateway-/Cache-Cluster

| Service | Image | Zweck | Port |
|---|---|---|---|
| `redis` | redis:7-alpine | Caching (max. 256 MB) | 6379 |
| `nginx` | nginx:1.25-alpine | Reverse Proxy / Load Balancer vor den Validatoren | 80, 443 |

`nginx` hängt explizit von `validator-1` und `validator-2` ab und verteilt Traffic auf beide.

### 1.4 Netzwerk & Storage

- Netzwerk: `atc-net` (bridge), Subnet `172.20.0.0/16`
- Volumes: `bootstrap-data`, `val1-data`, `val2-data`, `full-data`, `archive-data`, `prom-data`, `grafana-data` (alle Docker-managed, kein Host-Bind-Mount)

---

## 2. Lokales Dev-Testnet-Cluster (`docker/docker-compose.testnet.yml`)

Leichtgewichtige 3-Node-Variante für schnelles lokales Testen — **kein Rollen-Unterschied** zwischen den Nodes (alle gleichwertig, nur `node0` ist Bootstrap-Seed). Nutzt kein eigenes Image, sondern `python:3.11-slim` direkt mit Live-Mount des Repos (`..:/app`).

| Service | Container | Rolle | P2P-Port | RPC-Port | Gateway-Port |
|---|---|---|---|---|---|
| `node0` | atc-node0 | Bootstrap-Seed | 9000 | 9100 | 4000 |
| `node1` | atc-node1 | Peer (verbindet zu node0) | 9001 | 9101 | — |
| `node2` | atc-node2 | Peer (verbindet zu node0 + node1) | 9002 | 9102 | — |

Plus eigenes `prometheus` (Port 9090, 7 Tage Retention) + `grafana` (Port **3000**, nicht 3001 wie im Produktions-Setup — Kollisionsgefahr, falls beide Stacks gleichzeitig laufen!).

- Netzwerk: `atcnet` (bridge), Subnet `172.20.0.0/24` (⚠️ überschneidet sich mit dem `/16`-Subnet des Produktions-Clusters — beide Stacks dürfen **nicht parallel** auf derselben Docker-Engine laufen)
- Volumes: `node0_data`, `node1_data`, `node2_data`, `atcfs_data`, `prometheus_data`, `grafana_data`

---

## 3. Unterstützende Dockerfiles (Rollen-spezifische Images)

Diese existieren zusätzlich zum Root-`Dockerfile`, werden aber **von keinem der beiden Compose-Files aktuell referenziert** (Root-Compose baut alle Services aus dem einen Root-`Dockerfile`) — potenzielles Aufräum-/Vereinheitlichungs-Thema:

| Dockerfile | Zweck | Exponierte Ports |
|---|---|---|
| `docker/Dockerfile.gateway` | Reiner Gateway-Service (Gunicorn, 2 Worker) | 4000 |
| `docker/Dockerfile.bootstrap` | Minimal-Image nur für Bootstrap-Node | 4001/udp |
| `docker/Dockerfile.node` | Generisches Node-Image (inkl. optionaler KAI-Requirements) | 5000, 4001-4004/udp, 4000 |
| `docker/Dockerfile.backend` | Backend-API (Gunicorn, 4 Worker, sync) | 5000 |

---

## 4. Abweichung von Issue #18

Issue #18 (*"Docker Compose — 5-Node lokales Netzwerk"*) spezifiziert ursprünglich:
**1 Bootstrap + 1 Validator + 1 Miner + 2 Full Nodes**

Tatsächlich implementiert im Produktions-Cluster (`docker-compose.yml`):
**1 Bootstrap + 2 Validatoren + 1 Full Node + 1 Archive Node** (kein dedizierter "Miner"-Service — passt, da A-TownChain PoS/PoH-basiert ist, nicht PoW; "Miner" in Issue #18 ist vermutlich Altlast aus einer früheren PoW-Planungsphase).

→ Diese Abweichung ist sachlich nachvollziehbar (PoS/PoH statt PoW), sollte aber im Decisions Register als **AD-011** festgehalten werden, damit Issue #18 nicht fälschlich als "nicht erfüllt" gilt.

---

## 5. Empfehlungen (offene Punkte)

1. **Konsolidierung**: Zwei parallele Compose-Setups mit überlappenden Subnetzen und unterschiedlichen Grafana-Ports (3000 vs 3001) sind eine Fehlerquelle. Empfehlung: `docker/docker-compose.testnet.yml` als "Dev/Lokal"-Variante klar kennzeichnen (Prefix `dev-` bei Volumes/Netzwerken) oder auf ein gemeinsames Overlay-Pattern (`docker-compose.yml` + `docker-compose.override.yml`) umstellen.
2. **Secrets**: Grafana-Admin-Passwörter sind hartcodiert im Klartext in beiden Compose-Files (`atcadmin2026!`, `atcchain`) — sollten vor Mainnet-Bezug per `.env`/Docker-Secret ersetzt werden.
3. **Dockerfile-Wildwuchs**: 4 rollen-spezifische Dockerfiles existieren, werden aber von keinem Compose-File genutzt (Root-Compose baut alles aus dem einen Root-`Dockerfile`). Entweder nutzen oder als Legacy markieren/entfernen.
4. **Bootstrap Node Production (Mainnet)**: Für den späteren Mainnet-Start (siehe Memory #7) braucht es zusätzlich einen **öffentlich erreichbaren Bootstrap-Node-Cluster** mit fester IP und 24/7-Uptime — aktuell in keinem der beiden Compose-Files abgebildet (beide sind Testnet-only, `ATC_CHAIN_ID=9000`).

---

*Diese Datei ist die kanonische Cluster-Doku. Bei Compose-Änderungen bitte hier mit-aktualisieren (Aurora Sync-Regel: keine Informationen dürfen verloren gehen).*
