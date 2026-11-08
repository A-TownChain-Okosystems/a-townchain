# Kapitel 73 — Sprint 2.8: Multi-Node Testnet Live

> **Stand:** 05.07.2026 | **Sprint:** 2.8

---

## Übersicht

Sprint 2.8 zielt auf ein live Multi-Node Testnet mit 5+ Validatoren, Public API, Explorer und Faucet.

## Aufgaben

| Task | Beschreibung | Status |
|------|--------------|--------|
| T-701 | Testnet Genesis Block generieren | 📋 |
| T-702 | 5+ Validator-Nodes deployen | 📋 |
| T-703 | Testnet Public API (Gateway Port 4000) | 📋 |
| T-704 | Testnet Explorer live | 📋 |
| T-705 | Faucet für Test-Token | 📋 |
| T-706 | Stress-Test (1000 TPS) | 📋 |
| T-707 | Testnet Documentation | 📋 |

## Voraussetzungen

- ✅ Sprint 2.2: P2P-Netzwerk implementiert
- ✅ Sprint 2.3: Konsens-Mechanismen (PoH/PoW/PoS) migriert
- ✅ Sprint 2.5: Smart Contracts lauffähig
- ⚠️ Sprint 2.7: CI/CD Pipeline muss repariert sein
- ⚠️ AD-002: EventBus vs IPCBus Entscheidung

## Testnet Architektur

```
[Validator 1] ←→ [Validator 2] ←→ [Validator 3]
      ↕                ↕                ↕
[Validator 4] ←→ [Validator 5] ←→ [Gateway:4000]
                                         ↕
                                  [Explorer UI]
                                  [Faucet API]
```

## Docker-Deployment

Vorhandene Docker-Konfiguration:
- `docker/Dockerfile.node` — Full Node
- `docker/Dockerfile.gateway` — API Gateway
- `docker/Dockerfile.bootstrap` — Bootstrap Node
- `docker/Dockerfile.backend` — Backend Service
- `docker/docker-compose.testnet.yml` — Multi-Node Setup

---

*Kapitel 73 · Sprint 2.8 · 05.07.2026 · Aurora*
