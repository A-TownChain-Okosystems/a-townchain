# 🤖 Gemini AI Integration — Technische Dokumentation
**Stand:** 09.06.2026 | **Version:** v2.1.0 | **Status:** ✅ Issue #2 DONE
**Datei:** `backend/api/orchestrator/orchestrator.py` (336 Zeilen)

---

## Überblick

A-TownChain integriert **Google Gemini 2.5 Flash** als primären KI-Agenten. Das System nutzt ein **BYOK-Modell** (Bring Your Own Key) — jeder Nutzer bringt seinen eigenen API-Key mit, der ausschließlich lokal im Browser gespeichert wird.

```
User-Browser
    │
    │ Gemini API-Key (localStorage)
    ▼
ATC-UI (atc-ui/index.html)
    │
    │ Direct API Call (HTTPS)
    ▼
Gemini API (generativelanguage.googleapis.com)
    │
    ▼
Antwort → Chat-Box

--- ODER ---

Backend-Client
    │
    ▼
POST /api/ai/chat  (Gateway :4000)
    │
    ▼
GeminiOrchestrator (backend/api/orchestrator/)
    │
    ├── PromptCache (TTL 300s)
    ├── RateLimiter (Token-Bucket)
    ├── RetryManager (exp. Backoff)
    └── Gemini 2.5 Flash API
```

---

## GeminiOrchestrator

**Datei:** `backend/api/orchestrator/orchestrator.py` (336 Zeilen)

```python
@dataclass
class AIRequest:
    prompt:      str
    model:       str   = "gemini-2.5-flash"
    temperature: float = 0.7
    max_tokens:  int   = 2048
    system_msg:  str   = ATC_SYSTEM_PROMPT
    context:     list  = field(default_factory=list)

@dataclass
class AIResponse:
    text:       str
    model:      str
    tokens_in:  int
    tokens_out: int
    latency_ms: int
    cached:     bool = False

class GeminiOrchestrator:
    """
    KI-Orchestrierung für A-TownChain OS.
    Features:
      - Prompt-Caching (TTL 300s)
      - Rate-Limiting (Token Bucket)
      - Automatisches Retry mit exponenziellem Backoff
      - Multi-Turn Konversation
      - ATC-spezifischer System-Prompt
    """

    ATC_SYSTEM_PROMPT = """
    Du bist der KI-Kern des A-TownChain OS — ein proprietäres dezentrales
    Blockchain-Betriebssystem. Kontext:
    - ShivaConsensus (PoH+PoS+PoW Hybrid)
    - ATCLang v0.2.0 (proprietäre Smart Contract Sprache)
    - ATCNet P2P (Kademlia DHT)
    - ATC-8300/9000 Token-Standards
    - Wallet-Adressen: ATC + 32 Zeichen
    Antworte präzise, technisch korrekt und auf Deutsch.
    """

    def generate(self, request: AIRequest) -> AIResponse:
        """Hauptmethode — generiert KI-Antwort."""
        ...

    def chat(self, messages: list, system_override: str = None) -> AIResponse:
        """Multi-Turn Konversation."""
        ...

    def analyze_transaction(self, tx: dict) -> str:
        """TX-Analyse: Risiko, Betrag, Empfänger."""
        ...

    def explain_contract(self, code: str) -> str:
        """ATCLang Contract in natürlicher Sprache erklären."""
        ...

    def generate_atclang(self, description: str) -> str:
        """ATCLang-Code aus Beschreibung generieren."""
        ...

    def stats(self) -> dict:
        """Cache-Hits, Requests, Tokens, Latenz."""
        ...
```

---

## System-Prompt

Der ATC-spezifische System-Prompt gibt dem Modell Kontext über das A-TownChain-Ökosystem:

```
Du bist der KI-Kern des A-TownChain OS.
Kontext:
- ShivaConsensus: 3-Phasen Hybrid PoH+PoS+PoW
- ATCLang v0.2.0: Proprietäre SC-Sprache mit eigener VM
- ATCNet: Kademlia-DHT P2P Netzwerk
- ATC-8300: Fungible Token (18 Decimals)
- ATC-9000: NFT Standard (Shivamon)
- ATC-9900: Governance DAO (in Development)
- Wallet: ATC + 32 Zeichen, ECDSA secp256k1
- Chain ID: 9000 | Testnet aktiv
Antworte immer technisch präzise und auf Deutsch.
```

---

## Frontend-Integration (BYOK)

**Datei:** `atc-ui/index.html`

```javascript
// Key wird nur in localStorage gespeichert — nie auf Server
const GEMINI_MODEL = "gemini-2.5-flash";

async function callGemini(prompt, key) {
    const url = `https://generativelanguage.googleapis.com/v1beta/models/${GEMINI_MODEL}:generateContent?key=${key}`;
    const body = {
        system_instruction: { parts: [{ text: ATC_SYSTEM_MSG }] },
        contents: [
            ...chatHistory.slice(-6),
            { role: "user", parts: [{ text: prompt }] }
        ],
        generationConfig: { temperature: 0.7, maxOutputTokens: 2048 }
    };
    const res  = await fetch(url, { method: "POST", body: JSON.stringify(body) });
    const data = await res.json();
    return data.candidates[0].content.parts[0].text;
}
```

**Features im Frontend:**
- 🔑 API-Key in Settings eingeben (per-User, localStorage)
- 👁 Sichtbarkeits-Toggle für Key-Feld
- 🔌 "Key testen" — Gemini-Ping-Test
- 🗑 "Key entfernen" — aus localStorage löschen
- ⚡ Quick Actions: ShivaConsensus erklären, ATCLang Contract, v2.1.0 Status, ATC-Standards
- 📊 Token-Counter, Cache-Hits, Request-Zähler

---

## Backend API-Endpunkte

```
POST /api/ai/chat
  Body: { "message": "...", "history": [...] }
  Auth: X-API-Key Header
  Rate: 10/Minute

POST /api/ai/analyze-tx
  Body: { "tx": {...transaction object...} }

POST /api/ai/explain-contract
  Body: { "code": "contract Counter { ... }" }

POST /api/ai/generate-atclang
  Body: { "description": "ERC-20 Token mit Staking" }

GET  /api/ai/status
  Response: { "model": "gemini-2.5-flash", "requests": 42, "tokens": 15000 }
```

---

## Rate-Limiting & Caching

| Parameter | Wert |
|-----------|------|
| Modell | gemini-2.5-flash |
| Cache TTL | 300 Sekunden |
| Rate Limit | 10 Requests/Minute |
| Max Tokens | 2048 (Standard) |
| Temperature | 0.7 (Standard) |
| Retry-Versuche | 3 (exp. Backoff: 1s, 2s, 4s) |
| Context-Fenster | 6 letzte Nachrichten |

---

## Geplante Erweiterungen (v2.2+)

| Feature | Version | Beschreibung |
|---------|---------|-------------|
| Gemini 2.0 Pro | v2.2 | Für komplexe Analysen |
| Embeddings | v2.2 | Semantische Suche in Blockchain-Daten |
| Function Calling | v2.3 | KI kann direkt Chain-Operationen ausführen |
| Fine-Tuning | v3.0 | ATC-spezifisches Fine-Tuned Model |
| Local LLM | v3.0 | Offline-Modus mit Llama/Mistral |
