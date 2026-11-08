# 🔌 API Reference — atc-backend

> **Version:** v1.0.0 · **Protokolle:** REST / JSON-RPC

---

## 1. Wallet Endpunkte

### `GET /api/v1/wallet/health`
Prüft den Status des Wallet-Service.

**Response:**
```json
{
  "service": "wallet",
  "status": "online"
}
```

### `POST /api/v1/wallet/create`
Erstellt ein neues Wallet Konto.

**Request:**
```json
{
  "label": "MyWallet",
  "passphrase": "secret-passphrase"
}
```

---

## 2. KI & LLM Endpunkte

### `POST /api/v1/ai/query`
Führt eine synchrone KI-Abfrage an das Aurora LLM Modell durch.

**Request:**
```json
{
  "prompt": "Generiere einen Smart Contract für ein Token Staking.",
  "model": "aurora-v1",
  "temperature": 10
}
```

**Response:**
```json
{
  "query_id": "0xabc123...",
  "response": "contract StakingContract { ... }",
  "tokens_used": 142
}
```

---

## 3. KAI Agenten Endpunkte

### `GET /api/v1/kai/agents`
Listet alle aktiven KAI-Agenten im Ökosystem auf.
