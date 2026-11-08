# 🔀 API Gateway — Technische Dokumentation
**Stand:** 09.06.2026 | **Version:** v2.1.0 | **Dateien:** `gateway/`

---

## Überblick

Das API-Gateway ist der **einzige Einstiegspunkt** für alle Frontend-Anfragen. Das Frontend kommuniziert **ausschließlich** mit Port 4000 — niemals direkt mit dem Backend.

```
BROWSER / EXTERNAL CLIENT
        │
        ▼
┌───────────────────────────────────────────┐
│         API GATEWAY  :4000               │
│                                           │
│  ┌─────────────────────────────────────┐  │
│  │ Middleware Pipeline                 │  │
│  │  1. log_request (Logger)            │  │
│  │  2. require_api_key (Auth)          │  │
│  │  3. rate_limiter (Rate Limit)       │  │
│  │  4. signature_verify (TX-Signatur)  │  │
│  └─────────────────────────────────────┘  │
│                    │                      │
│          GatewayRouter                    │
└──────────────────┬────────────────────────┘
                   │ Proxy zu :5000
                   ▼
┌──────────────────────────────────────────┐
│          BACKEND API SERVER  :5000       │
│  /api/blockchain  /api/wallet            │
│  /api/ai          /api/game              │
│  /api/governance  /api/marketplace       │
│  /api/nodes       /api/orchestrator      │
└──────────────────────────────────────────┘
```

---

## Middleware

**Verzeichnis:** `gateway/middleware/`

### 1. Logger (`middleware/logger.py`)
```python
@app.before_request
def log_request():
    logger.info(f"{request.method} {request.path} — {request.remote_addr}")
```

### 2. Auth (`middleware/auth.py`)
```python
def require_api_key(f):
    """API-Key Authentifizierung für sensitive Endpunkte."""
    @wraps(f)
    def decorated(*args, **kwargs):
        key = request.headers.get("X-API-Key")
        if not key or not validate_key(key):
            return jsonify({"error": "Unauthorized"}), 401
        return f(*args, **kwargs)
    return decorated
```

**Öffentliche Endpunkte** (kein API-Key):
- `GET /api/blockchain/status`
- `GET /api/blockchain/height`
- `GET /api/nodes/peers`

**Geschützte Endpunkte** (API-Key erforderlich):
- `POST /api/wallet/send`
- `POST /api/wallet/sign`
- `POST /api/ai/chat`
- Alle Governance-Endpunkte

### 3. Rate Limiter (`middleware/rate_limit.py`)
```python
LIMITS = {
    "/api/ai/":         "10/minute",
    "/api/wallet/send": "5/minute",
    "default":          "100/minute",
}
```

### 4. Signature Verify (`middleware/signature_verify.py`)
```python
# TX-Signaturen für /api/wallet/send werden vor dem Routing verifiziert
def verify_tx_signature(tx: dict) -> bool:
    signer = ECDSASigner()
    msg    = f"{tx['from']}{tx['to']}{tx['amount']}{tx['nonce']}"
    return signer.verify(msg, tx['signature'], tx['public_key'])
```

---

## Routing-Tabelle

**Datei:** `gateway/router.py`

| Präfix | Backend-Endpunkt | Beschreibung |
|--------|-----------------|-------------|
| `/api/blockchain` | `:5000/api/blockchain` | Chain-Daten |
| `/api/wallet` | `:5000/api/wallet` | Wallet-Operationen |
| `/api/ai` | `:5000/api/ai` | Gemini AI |
| `/api/game` | `:5000/api/game` | Shivamon Spiellogik |
| `/api/governance` | `:5000/api/governance` | DAO-Abstimmungen |
| `/api/marketplace` | `:5000/api/marketplace` | NFT-Marketplace |
| `/api/nodes` | `:5000/api/nodes` | P2P-Nodes |
| `/api/orchestrator` | `:5000/api/orchestrator` | KI-Orchestrierung |

---

## Vollständige API-Endpunkte

### Blockchain
```
GET  /api/blockchain/status          → Chain-Status
GET  /api/blockchain/height          → Aktuelle Block-Höhe
GET  /api/blockchain/block/:hash     → Block by Hash
GET  /api/blockchain/block/:height   → Block by Höhe
GET  /api/blockchain/tx/:hash        → Transaktion
GET  /api/blockchain/mempool         → Ausstehende TXs
POST /api/blockchain/submit-tx       → TX einreichen
```

### Wallet
```
POST /api/wallet/new                 → Neue Wallet
POST /api/wallet/restore             → Aus Mnemonic
GET  /api/wallet/balance/:addr       → Balance
GET  /api/wallet/history/:addr       → TX-Geschichte
POST /api/wallet/sign                → TX signieren
POST /api/wallet/send                → TX senden
```

### AI (Gemini)
```
POST /api/ai/chat                    → Chat-Nachricht
POST /api/ai/analyze-tx              → TX analysieren
POST /api/ai/explain-contract        → Contract erklären
POST /api/ai/generate-atclang        → ATCLang Code generieren
GET  /api/ai/status                  → AI-Status
```

### Governance
```
GET  /api/governance/proposals       → Aktive Vorschläge
POST /api/governance/propose         → Neuer Vorschlag
POST /api/governance/vote            → Abstimmen
GET  /api/governance/results/:id     → Abstimmungsergebnis
```

---

## Konfiguration

```python
# gateway/main.py
CORS(app, origins=["http://localhost:3000", "http://localhost:8080"])
app.config["BACKEND_BASE"] = os.environ.get("BACKEND_URL", "http://localhost:5000")
```

**Environment Variables:**
| Variable | Default | Beschreibung |
|----------|---------|-------------|
| `GATEWAY_PORT` | `4000` | Gateway-Port |
| `BACKEND_URL` | `http://localhost:5000` | Backend-Adresse |
| `ATC_API_KEY` | — | API-Key für geschützte Endpunkte |
| `RATE_LIMIT_ENABLED` | `true` | Rate-Limiting an/aus |
