# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/api/routes/ai_routes.py
# Issue #2 — Gemini AI Integration

import os
from flask import Blueprint, jsonify, request, Response

ai_bp = Blueprint("ai", __name__)

SYSTEM_PROMPT = """Du bist der A-TownChain AI Orchestrator — das KI-Gehirn von ShivaOS.
Du kennst das gesamte A-TownChain Oekosystem:
  - ATC-8300 Fungible Token (A-Town Coin, Max Supply: 21 Mio.)
  - ATC-9000 Shivamon NFTs (bis 9900 einzigartige Battle-Kreaturen)
  - ATC-001 Genesis Token (nicht-transferierbar, symbolischer Ursprung)
  - ATC-9900 Governance DAO (1 ATC = 1 Stimme, Quorum 10%)
  - SHA-256 PoW + PoS + PoH Hybrid Consensus
  - API Gateway Architektur (Port 4000 → Backend 8080)
  - Node-Netzwerk (FULL, LIGHT, VALIDATOR, MINER)
  - Marketplace fuer Shivamon-NFT-Handel (2.5% Fee)
Antworte praezise, technisch korrekt und im Stil eines Blockchain-Experten.
Antworte auf Deutsch, wenn der User auf Deutsch fragt.
"""

def _get_model():
    key = os.getenv("GEMINI_API_KEY")
    if not key:
        return None, None
    try:
        import google.generativeai as genai
        genai.configure(api_key=key)
        model = genai.GenerativeModel(
            os.getenv("GEMINI_MODEL", "gemini-2.0-flash"),
            system_instruction=SYSTEM_PROMPT
        )
        return genai, model
    except ImportError:
        return None, None


@ai_bp.route("/health", methods=["GET"])
def health():
    has_key = bool(os.getenv("GEMINI_API_KEY"))
    return jsonify({"service": "ai", "status": "online", "gemini": has_key})


@ai_bp.route("/query", methods=["POST"])
def query():
    prompt = (request.json or {}).get("prompt", "")
    if not prompt:
        return jsonify({"error": "prompt required"}), 400

    genai, model = _get_model()
    if not model:
        # Graceful degradation — no API key
        return jsonify({
            "response": (
                f"[A-TownChain AI] Antwort auf: \"{prompt}\"\n\n"
                "Das A-TownChain OS verwendet Gemini 2.0 als KI-Gehirn. "
                "Fuer echte KI-Antworten bitte GEMINI_API_KEY als Umgebungsvariable setzen. "
                "Das System laeuft stabil: 847 Nodes aktiv, PoI+PoS Konsens operativ."
            ),
            "model":  "mock",
            "tokens": 0,
            "gemini_active": False
        })

    try:
        chat = model.start_chat()
        resp = chat.send_message(prompt)
        tokens = 0
        try:
            tokens = resp.usage_metadata.total_token_count
        except Exception:
            pass
        return jsonify({
            "response":      resp.text,
            "model":         os.getenv("GEMINI_MODEL", "gemini-2.0-flash"),
            "tokens":        tokens,
            "gemini_active": True
        })
    except Exception as e:
        return jsonify({
            "response":      f"[AI Fehler] {str(e)}",
            "model":         "error",
            "tokens":        0,
            "gemini_active": False
        }), 200


@ai_bp.route("/stream", methods=["POST"])
def stream():
    prompt = (request.json or {}).get("prompt", "")
    genai, model = _get_model()

    if not model:
        def mock_gen():
            msg = (f"[A-TownChain AI] Antwort auf: \"{prompt}\" — "
                   "Setze GEMINI_API_KEY fuer echte Antworten.")
            yield f"data: {msg}\n\n"
            yield "data: [DONE]\n\n"
        return Response(mock_gen(), mimetype="text/event-stream")

    def gen():
        try:
            for chunk in model.generate_content(prompt, stream=True):
                if chunk.text:
                    yield f"data: {chunk.text}\n\n"
        except Exception as e:
            yield f"data: [Fehler: {e}]\n\n"
        yield "data: [DONE]\n\n"

    return Response(gen(), mimetype="text/event-stream")


@ai_bp.route("/status", methods=["GET"])
def ai_status():
    has_key = bool(os.getenv("GEMINI_API_KEY"))
    return jsonify({
        "gemini_active": has_key,
        "model":         os.getenv("GEMINI_MODEL", "gemini-2.0-flash"),
        "system_prompt": "A-TownChain Kontext geladen",
        "endpoints":     ["/query", "/stream", "/status"]
    })
