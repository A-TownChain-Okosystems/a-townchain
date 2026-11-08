# ⚡ API Gateway — Technische Dokumentation

> **Port:** 4000 · **Datei:** `gateway/main.py`

---

## Überblick

Das API Gateway ist der **einzige Kommunikationspunkt** zwischen Frontend und Backend. Das Frontend spricht ausschließlich mit dem Gateway — nie direkt mit einem Backend-Service.

```
Frontend (Browser)
    │  api.js → alle Calls gehen an :4000
    ▼
API Gateway (:4000)
    ├── Auth Middleware      ← X-API-Key prüfen
    ├── Rate Limiter         ← 100 req / 60s
    ├── Request Logger       ← alle Calls loggen
    └── Router               ← leitet weiter an:
         ├── Core    :5000   ← /api/status, /api/modules
         ├── Chain   :5001   ← /api/blockchain/*
         ├── Wallet  :5002   ← /api/wallet/*
         ├── AI      :5003   ← /api/ai/*
         ├── Game    :5004   ← /api/game/*
         └── Nodes   :5005   ← /api/nodes/*
```

---

## Middleware Stack

### 1. API Key Auth (`gateway/middleware/auth.py`)

```python
API_KEYS = {
    "atc-dev-key-2025":  "developer",
    "atc-admin-key":     "admin",
}

def auth_middleware(request):
    key = request.headers.get("X-API-Key")
    if key not in API_KEYS:
        return {"error": "Unauthorized"}, 401
    request.role = API_KEYS[key]
    return None  # weiter
```

### 2. Rate Limiter (`gateway/middleware/rate_limit.py`)

```python
# 100 Requests pro IP pro 60 Sekunden
RATE_LIMIT   = 100
WINDOW_SEC   = 60
request_log  = {}   # ip → [timestamps]

def rate_limit(ip):
    now    = time.time()
    window = [t for t in request_log.get(ip,[]) if now-t < WINDOW_SEC]
    if len(window) >= RATE_LIMIT:
        return {"error": "Rate limit exceeded"}, 429
    window.append(now)
    request_log[ip] = window
    return None
```

### 3. Request Logger (`gateway/middleware/logger.py`)

```
[2026-05-19 21:50:00] POST /api/wallet/create | 200 | 42ms | developer
[2026-05-19 21:50:01] GET  /api/blockchain/info | 200 | 8ms  | developer
```

---

## Service-Routing

| Prefix | Service | Port | Beispiel |
|--------|---------|------|---------|
| `/api/status` | Core | 5000 | `GET /api/status` |
| `/api/orchestrator` | Core | 5000 | `GET /api/orchestrator/status` |
| `/api/blockchain` | Chain | 5001 | `POST /api/blockchain/mine` |
| `/api/wallet` | Wallet | 5002 | `POST /api/wallet/create` |
| `/api/ai` | AI | 5003 | `POST /api/ai/query` |
| `/api/game` | Game | 5004 | `POST /api/game/shivamon/mint` |
| `/api/nodes` | Nodes | 5005 | `GET /api/nodes/` |

---

## Health Check

```bash
GET http://localhost:4000/gateway/health

{
  "gateway": "online",
  "version": "2.0",
  "services": {
    "core":       "online",
    "blockchain": "online",
    "wallet":     "online",
    "ai":         "offline",
    "game":       "online",
    "nodes":      "online"
  },
  "uptime": "2h 14m 33s"
}
```

---

> **Dokument:** `docs/architecture/GATEWAY.md`
> **Datum:** 2026-05-19 · **Autor:** A-TownChain-Okosystems × Aurora AI
