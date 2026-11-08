# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# gateway/main.py
# A-TownChain OS — API Gateway (Vermittler)
# 
# Architektur:
#   FRONTEND  →  API GATEWAY  →  BACKEND SERVICES
#
# Das Frontend spricht NUR mit dem Gateway.
# Das Gateway routet zu den Backend-Services.
# Kein direkter Kontakt zwischen Frontend und Backend.

from flask import Flask, jsonify, request
from flask_cors import CORS
from gateway.router import GatewayRouter
from gateway.middleware.auth import require_api_key
from gateway.middleware.rate_limit import rate_limiter
from gateway.middleware.logger import log_request
import os

app = Flask(__name__)
CORS(app, origins=["http://localhost:3000", "http://localhost:8080"])

router = GatewayRouter()

# ── Middleware ──────────────────────────────
@app.before_request
def before():
    log_request(request)

# ── Health ──────────────────────────────────
@app.route("/gateway/health", methods=["GET"])
def health():
    return jsonify({
        "gateway": "online",
        "version": "2.0",
        "services": router.get_service_status()
    })

# ── Proxy alle API-Calls ────────────────────
@app.route("/api/<path:endpoint>", methods=["GET","POST","PUT","DELETE"])
def proxy(endpoint):
    return router.forward(endpoint, request)

if __name__ == "__main__":
    port = int(os.getenv("GATEWAY_PORT", 4000))
    print(f"[GATEWAY] Running on http://localhost:{port}")
    app.run(host="0.0.0.0", port=port, debug=False)
