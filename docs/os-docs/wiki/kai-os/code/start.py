#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# start.py — A-TownChain OS startup script
# Gateway auf Port 5000 (Frontend + API Proxy)
# Backend-API auf Port 8080

import os
import threading

def run_backend():
    """Run the Flask backend on port 8080."""
    from backend.api.server import create_app
    from core.kernel import Kernel
    kernel = Kernel()
    kernel.start()
    app = create_app(kernel)
    print("[BACKEND] API Server running on http://localhost:8080")
    app.run(host="localhost", port=8080, debug=False, use_reloader=False)


def run_gateway():
    """Gateway auf Port 5000 — dient Frontend + proxied API."""
    import os
    from flask import Flask, jsonify, request, send_file, send_from_directory
    from flask_cors import CORS
    from gateway.router import GatewayRouter
    from gateway.middleware.logger import log_request

    app = Flask(__name__, static_folder=None)
    CORS(app, origins="*")
    router = GatewayRouter()

    @app.before_request
    def before():
        if not request.path.startswith("/api") and not request.path.startswith("/gateway"):
            return  # Kein Log fuer statische Dateien
        log_request(request)

    # ── Frontend statisch ────────────────────────────
    FRONTEND_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "frontend")

    @app.route("/")
    def index():
        return send_from_directory(FRONTEND_DIR, "index.html")

    @app.route("/<path:filename>")
    def static_files(filename):
        try:
            return send_from_directory(FRONTEND_DIR, filename)
        except Exception:
            return send_from_directory(FRONTEND_DIR, "index.html")

    # ── Gateway Health ───────────────────────────────
    @app.route("/gateway/health", methods=["GET"])
    def health():
        return jsonify({
            "gateway": "online",
            "version": "2.1.0",
            "services": router.get_service_status()
        })

    # ── API Proxy ────────────────────────────────────
    @app.route("/api/<path:endpoint>", methods=["GET", "POST", "PUT", "DELETE"])
    def proxy(endpoint):
        return router.forward(endpoint, request)

    print("[GATEWAY] Running on http://0.0.0.0:5000 (Frontend + API)")
    app.run(host="0.0.0.0", port=5000, debug=False, use_reloader=False)


if __name__ == "__main__":
    backend_thread = threading.Thread(target=run_backend, daemon=True)
    backend_thread.start()

    # Backend kurz warten lassen
    import time
    time.sleep(1)

    run_gateway()
