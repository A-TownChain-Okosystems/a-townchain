# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# gateway/router.py -- FIX(#20): forward() vollstaendig
import requests
from flask import jsonify, Response

SERVICES = {
    "status":       "http://localhost:8080/api/status",
    "modules":      "http://localhost:8080/api/modules",
    "orchestrator": "http://localhost:8080/api/orchestrator",
    "blockchain":   "http://localhost:8080/api/blockchain",
    "wallet":       "http://localhost:8080/api/wallet",
    "ai":           "http://localhost:8080/api/ai",
    "game":         "http://localhost:8080/api/game",
    "nodes":        "http://localhost:8080/api/nodes",
    "governance":   "http://localhost:8080/api/governance",
    "marketplace":  "http://localhost:8080/api/marketplace",
}

class GatewayRouter:
    def get_service_status(self):
        status = {}
        for name in SERVICES:
            try:
                r = requests.get(f"http://localhost:8080/api/{name}/health", timeout=0.5)
                status[name] = "online" if r.status_code == 200 else "degraded"
            except:
                status[name] = "offline"
        return status

    def forward(self, endpoint: str, req) -> Response:
        service_key = endpoint.split("/")[0]
        base_url    = SERVICES.get(service_key)
        if not base_url:
            return jsonify({"error": "Service not found", "available": list(SERVICES.keys())}), 404
        sub_path   = "/".join(endpoint.split("/")[1:])
        target_url = f"{base_url}/{sub_path}" if sub_path else base_url
        try:
            resp = requests.request(method=req.method, url=target_url,
                headers={k: v for k, v in req.headers if k.lower() != "host"},
                data=req.get_data(), timeout=10, allow_redirects=False)
            excluded = {"transfer-encoding", "content-encoding"}
            headers  = [(k, v) for k, v in resp.raw.headers.items() if k.lower() not in excluded]
            return Response(resp.content, resp.status_code, headers)
        except requests.exceptions.ConnectionError:
            return jsonify({"error": "Service offline", "service": service_key}), 503
        except requests.exceptions.Timeout:
            return jsonify({"error": "Service timeout", "service": service_key}), 504
        except Exception as exc:
            return jsonify({"error": str(exc)}), 500
