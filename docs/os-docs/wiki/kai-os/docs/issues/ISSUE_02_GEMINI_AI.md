# 📄 Issue #2 — Gemini AI Integration

> **Labels:** enhancement · ai · priority:high
> **Priorität:** 🔴 High · **Milestone:** v2.1.0
> **Referenz:** [GitHub Issue #2](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/2)

---

## Ziel

Anbindung der **Google Gemini 2.0 API** als KI-Gehirn des A-TownChain OS. Der AI Orchestrator im Dashboard wird zu einem vollwertigen Live-Chat-Interface mit A-TownChain-Kontext.

---

## Architektur

```
Frontend (AI Panel)
  └─→ POST /api/ai/query  (via Gateway :4000)
              │
     backend/api/routes/ai.py
              │
     AIOrchestrator.query()
              │
     Google Gemini 2.0 API
              │
     Response (Streaming / SSE)
              │
     Frontend Live-Typing-Effekt
```

---

## Technische Spezifikation

### Backend — `backend/api/routes/ai.py`

```python
import google.generativeai as genai
import os

SYSTEM_PROMPT = """
Du bist der A-TownChain AI Orchestrator — das KI-Gehirn von ShivaOS.
Du kennst das gesamte A-TownChain Ökosystem:
  - ATC-8300 Fungible Token (A-Town Coin)
  - ATC-9000 Shivamon NFTs
  - SHA-256 PoW + PoS + PoH Hybrid Consensus
  - API Gateway Architektur
  - Node-Netzwerk (FULL, LIGHT, VALIDATOR, MINER)
Antworte präzise, technisch korrekt und im Stil eines Blockchain-Experten.
"""

genai.configure(api_key=os.getenv("GEMINI_API_KEY"))
model = genai.GenerativeModel("gemini-2.0-flash")

@ai_bp.route("/query", methods=["POST"])
def query():
    prompt = request.json.get("prompt", "")
    chat   = model.start_chat(history=[])
    resp   = chat.send_message(SYSTEM_PROMPT + "\n\nUser: " + prompt)
    return jsonify({
        "response": resp.text,
        "model":    "gemini-2.0-flash",
        "tokens":   resp.usage_metadata.total_token_count
    })

@ai_bp.route("/stream", methods=["POST"])
def stream():
    """Server-Sent Events für Live-Typing-Effekt."""
    prompt = request.json.get("prompt", "")
    def generate():
        for chunk in model.generate_content(prompt, stream=True):
            yield f"data: {chunk.text}\n\n"
        yield "data: [DONE]\n\n"
    return Response(generate(), mimetype="text/event-stream")
```

### Umgebungsvariablen

```bash
# .env
GEMINI_API_KEY=your-gemini-api-key-here
GEMINI_MODEL=gemini-2.0-flash
GEMINI_MAX_TOKENS=2048
GEMINI_TEMPERATURE=0.7
```

### Frontend — AI Panel Update

```javascript
// Streaming-Antwort empfangen
async function queryAI(prompt) {
  const res = await fetch(`${GATEWAY}/ai/stream`, {
    method: "POST",
    headers: { ...headers },
    body: JSON.stringify({ prompt })
  });
  const reader = res.body.getReader();
  const decoder = new TextDecoder();
  let output = "";
  while (true) {
    const { done, value } = await reader.read();
    if (done) break;
    const chunk = decoder.decode(value);
    output += chunk.replace("data: ", "");
    document.getElementById("ai-output").textContent = output;
  }
}
```

---

## Aufgaben

- [ ] `backend/api/routes/ai.py` vollständig implementieren
- [ ] `google-generativeai` zu `backend/requirements.txt` hinzufügen
- [ ] `.env.example` mit `GEMINI_API_KEY` erweitern
- [ ] System-Prompt mit A-TownChain Kontext definieren
- [ ] Streaming-Endpoint (`/api/ai/stream`) für SSE
- [ ] Token-Zähler + verbrauchte Kosten loggen
- [ ] Chat-History im Frontend (letzte 10 Nachrichten)
- [ ] Gateway Route `/api/ai/` registrieren
- [ ] Frontend AI-Panel verdrahten (Live-Typing-Effekt)
- [ ] Fehlerbehandlung: API-Key fehlt / Rate Limit erreicht

---

## Akzeptanzkriterien

- [ ] AI antwortet auf A-TownChain-Fragen korrekt und kontextbewusst
- [ ] Streaming-Antwort mit Live-Typing im Dashboard
- [ ] Token-Verbrauch wird geloggt
- [ ] Graceful Degradation wenn API-Key fehlt

---

## Referenzen

- `backend/api/server.py` — Placeholder bereits vorhanden
- Gemini API Docs: https://ai.google.dev/gemini-api/docs
- `gateway/router.py` — Route `ai` bereits registriert
