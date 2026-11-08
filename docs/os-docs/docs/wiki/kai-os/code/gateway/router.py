# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# gateway/router.py
# Route-Tabelle: welcher Endpoint → welcher Service

import requests
from flask import jsonify

SERVICES = {
    "status":        "http://localhost:8080/api/status",
    "modules":       "http://localhost:8080/api/modules",
    "orchestrator":  "http://localhost:8080/api/orchestrator",
    "blockchain":    "http://localhost:8080/api/blockchain",
    "wallet":        "http://localhost:8080/api/wallet",
    "ai":            "http://localhost:8080/api/ai",
    "game":          "http://localhost:8080/api/game",
    "nodes":         "http://localhost:8080/api/nodes",
    "governance":    "http://localhost:8080/api/governance",
    "marketplace":   "http://localhost:8080/api/marketplace",
}

class GatewayRouter:

    def get_service_status(self):
        status = {}
        for name, url in SERVICES.items():
            try:
                r = requests.get(url.split("/api/")[0] + "/api/" + name.split("/")[0] + "/health", timeout=1)
                status[name] = "online" if r.status_code == 200 else "degraded"
            except:
                status[name] = "offline"
        return status

    def forward(self, endpoint, req):
        service_key = endpoint.split("/")[0]
        base_url    = SERVICES.get(service_key)
        if not base_url:
            return jsonify({"error": "Service not found", "available": list(SERVICES.keys())}), 404
        sub_path   = "/".join(endpoint.split("/")[1:])
        target_url = f"{base_url}/{sub_path}" if sub_path else base_url
        try:
            headers = {k: v for k, v in req.headers if k != "Host"}
            resp    = requests.request(
                method=req.method, url=target_url, headers=headers,
                json=req.get_json(silent=True), params=req.args, timeout=10
            )
            return (resp.content, resp.status_code, dict(resp.headers))
        except requests.exceptions.ConnectionError:
            return jsonify({"error": "Service unavailable", "service": service_key}), 503
        except Exception as e:
            return jsonify({"error": str(e)}), 500
