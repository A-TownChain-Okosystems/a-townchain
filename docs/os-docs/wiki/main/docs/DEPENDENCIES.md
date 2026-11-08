# Abhängigkeiten & Schnittstellen

## Python-Abhängigkeiten

### Produktions-Dependencies
```
cryptography>=41.0     # ECDSA secp256k1, SHA3-256 (FIPS-konform)
flask>=3.0             # API Gateway, REST Endpoints
flask-cors             # CORS Middleware
requests>=2.31         # HTTP-Client (Bridge Relayer)
prometheus-client      # Node-Monitoring /metrics
websockets             # WebSocket P2P + RPC
```

### Dev/Test-Dependencies
```
pytest>=7.0            # Test-Framework (23+ Tests)
pytest-asyncio         # Async Tests
hardhat (npm)          # Solidity Contract-Tests
ethers.js              # ETH Bridge Testing
```

### Bewusst KEINE externen Abhängigkeiten für:
- Blockchain-Kern (alles proprietär)
- ATCLang Compiler/VM (pure Python)
- ShivaOS Kernel (pure Python)
- Consensus-Algorithmus (pure Python + cryptography)
- BIP39 Wordlist (eingebettet)

## Interne Modul-Abhängigkeiten

```
atclang/vm/        ← atclang/compiler/
atclang/compiler/  ← atclang/parser/
atclang/parser/    ← atclang/lexer/

core/kernel.py     ← core/event_bus.py
                   ← blockchain/consensus/

backend/api/       ← blockchain/*
                   ← core/*
                   ← atclang/*

gateway/           ← backend/api/
                   ← blockchain/wallet/
```

## Externe Dienste (optional)

| Dienst | Zweck | Pflicht? |
|--------|-------|---------|
| Gemini API | KI-Features (BYOK) | Nein |
| Ethereum RPC | Bridge Relayer | Nur für Bridge |
| Solana RPC | Bridge Relayer | v3.0 geplant |
| Prometheus | Node-Monitoring | Optional |
| Docker | Containerisierung | Optional |

## Portbelegung

| Port | Dienst | Protokoll |
|------|--------|----------|
| 3000 | Frontend Dashboard | HTTP |
| 4000 | API Gateway | HTTP/REST |
| 4001 | P2P Node | TCP |
| 5005 | Bootstrap Node | TCP |
| 8080 | Monitoring Dashboard | HTTP |
| 9090 | Prometheus Metrics | HTTP |
| 9944 | WebSocket RPC | WS |

## Systemvoraussetzungen

| Komponente | Minimum | Empfohlen |
|-----------|---------|-----------|
| Python | 3.10+ | 3.12 |
| RAM | 512 MB | 2 GB |
| Storage | 1 GB | 10 GB (Archive) |
| CPU | 1 Core | 4 Cores |
| OS | Linux/macOS/Win | Ubuntu 22.04 LTS |
| Docker | 24+ | 25+ |
