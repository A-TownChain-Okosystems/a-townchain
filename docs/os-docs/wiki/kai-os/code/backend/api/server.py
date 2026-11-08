# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/api/server.py
# REST API — alle Blueprints registriert

from flask import Flask, jsonify, request
from flask_cors import CORS

from backend.api.routes.blockchain import blockchain_bp
from backend.api.routes.game_routes import game_bp
from backend.api.routes.wallet import wallet_bp
from backend.api.routes.nodes_routes import nodes_bp
from backend.api.routes.ai_routes import ai_bp
from backend.api.routes.governance_routes import governance_bp
from backend.api.routes.marketplace_routes import marketplace_bp
from backend.api.routes.orchestrator_routes import orchestrator_bp


def create_app(kernel=None):
    app = Flask(__name__)
    CORS(app)

    # ── Core ────────────────────────────────────────────
    @app.route("/api/status", methods=["GET"])
    def status():
        return jsonify({
            "status":  "online",
            "version": "2.1.0",
            "chain":   "A-TownChain",
            "kernel":  kernel.running if kernel else False
        })

    @app.route("/api/modules", methods=["GET"])
    def modules():
        mods = [m.__class__.__name__ for m in kernel.loader.modules] if kernel else []
        return jsonify({"modules": mods})

    # ── Blueprints ──────────────────────────────────────
    app.register_blueprint(blockchain_bp,  url_prefix="/api/blockchain")
    app.register_blueprint(game_bp,        url_prefix="/api/game")
    app.register_blueprint(wallet_bp,      url_prefix="/api/wallet")
    app.register_blueprint(nodes_bp,       url_prefix="/api/nodes")
    app.register_blueprint(ai_bp,          url_prefix="/api/ai")
    app.register_blueprint(governance_bp,  url_prefix="/api/governance")
    app.register_blueprint(marketplace_bp, url_prefix="/api/marketplace")
    app.register_blueprint(orchestrator_bp,url_prefix="/api/orchestrator")

    return app
