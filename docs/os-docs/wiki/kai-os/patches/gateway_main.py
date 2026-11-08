# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# gateway/main.py -- FIX(#20): create_app() Factory
from flask import Flask, jsonify, request
from flask_cors import CORS
from gateway.router import GatewayRouter
from gateway.middleware.logger import log_request
import os

def create_app(testing=False):
    app = Flask(__name__)
    app.config["TESTING"] = testing
    CORS(app, origins=["http://localhost:3000", "http://localhost:8080"])
    router = GatewayRouter()

    @app.before_request
    def before():
        if not testing:
            log_request(request)

    @app.route("/health", methods=["GET"])
    @app.route("/gateway/health", methods=["GET"])
    def health():
        return jsonify({"status": "ok", "gateway": "online", "version": "2.0",
                        "services": router.get_service_status() if not testing else {}})

    @app.route("/api/<path:endpoint>", methods=["GET","POST","PUT","DELETE"])
    def proxy(endpoint):
        return router.forward(endpoint, request)

    @app.errorhandler(404)
    def not_found(e):
        return jsonify({"error": "Not Found", "path": request.path}), 404

    @app.errorhandler(500)
    def server_error(e):
        return jsonify({"error": "Internal Server Error"}), 500

    return app

app = create_app()

if __name__ == "__main__":
    port = int(os.getenv("GATEWAY_PORT", 4000))
    create_app().run(host="0.0.0.0", port=port, debug=False)
