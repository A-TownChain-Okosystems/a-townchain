# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/api/routes/orchestrator_routes.py
# REST Routes für den Orchestrator

from flask import Blueprint, jsonify, request

orchestrator_bp = Blueprint("orchestrator", __name__)
_orc = None  # wird beim App-Start injiziert

def init_orchestrator(orchestrator_instance):
    global _orc
    _orc = orchestrator_instance

@orchestrator_bp.route("/status", methods=["GET"])
def status():
    return jsonify(_orc.get_status() if _orc else {"status": "not initialized"})

@orchestrator_bp.route("/dispatch", methods=["POST"])
def dispatch():
    data    = request.json or {}
    service = data.get("service")
    action  = data.get("action")
    payload = data.get("payload", {})
    return jsonify(_orc.dispatch(service, action, payload))

@orchestrator_bp.route("/services", methods=["GET"])
def services():
    return jsonify({"services": _orc.services if _orc else {}})
