# A-TownChain — Enterprise Infrastructure

> Stand: 10. Juni 2026 | v3.0.0 Enterprise Release

## CI/CD Pipeline (GitHub Actions)

Automatische Qualitätssicherung bei jedem Commit:

| Stage | Tool | Checks |
|-------|------|--------|
| Lint | Ruff + Black + isort | PEP8, Format, Imports |
| Security | Bandit + Safety | SAST, Dependency CVEs |
| Test | pytest + matrix | Python 3.10/3.11/3.12 |
| Solidity | Hardhat | 22 Contract-Tests |
| Docker | BuildKit | Multi-stage, ghcr.io |
| Deploy | Docker Compose | Testnet auto-deploy |

## Docker Stack (9 Services)

| Service | Image | Port | Rolle |
|---------|-------|------|-------|
| atc-bootstrap | ghcr.io/atc-os | 5005/9944 | P2P Discovery |
| atc-validator-1 | ghcr.io/atc-os | 4000/4001 | Validator + API |
| atc-validator-2 | ghcr.io/atc-os | 4011 | Validator |
| atc-fullnode | ghcr.io/atc-os | 4021 | Full Node |
| atc-archive | ghcr.io/atc-os | — | Archive |
| atc-prometheus | prom/prometheus | 9090 | Metrics |
| atc-grafana | grafana/grafana | 3001 | Dashboard |
| atc-redis | redis:7-alpine | 6379 | Cache |
| atc-nginx | nginx:1.25 | 80/443 | TLS Proxy |

```bash
# Starten
docker compose up -d --wait
curl http://localhost:4000/health
```

## Monitoring (6 Alert Rules)

- **NodeDown** — CRITICAL — Node offline > 2min
- **LowTPS** — WARNING — TPS < 5 für 5min
- **BlockProductionStopped** — CRITICAL — Kein Block > 5min
- **BridgeInvariantViolation** — CRITICAL — Sofort
- **GatewayHighLatency** — WARNING — P99 > 2s für 5min
- **HighMemory** — WARNING — > 500MB für 10min

## Security Headers (Nginx)

```
X-Frame-Options: DENY
X-Content-Type-Options: nosniff
Strict-Transport-Security: max-age=31536000
Content-Security-Policy: default-src 'self'
```

## Bug Bounty

| Schwere | Bounty ATC |
|---------|-----------|
| Critical | 50.000 |
| High | 10.000 |
| Medium | 2.500 |
| Low | 500 |

Report: security@atownchain.io
