# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/api/routes/governance_routes.py
# Issue #9 — ATC-9900 Governance DAO Routes

from flask import Blueprint, jsonify, request
from blockchain.contracts.governance.governance_contract import GovernanceContract

governance_bp = Blueprint("governance", __name__)
_gov = GovernanceContract()


@governance_bp.route("/health", methods=["GET"])
def health():
    return jsonify({"service": "governance", "status": "online"})


@governance_bp.route("/proposals", methods=["GET"])
def proposals():
    status = request.args.get("status")
    return jsonify({"proposals": _gov.get_proposals(status)})


@governance_bp.route("/proposals/<proposal_id>", methods=["GET"])
def get_proposal(proposal_id):
    return jsonify(_gov.get_proposal(proposal_id))


@governance_bp.route("/proposals", methods=["POST"])
def create_proposal():
    d = request.json or {}
    return jsonify(_gov.create_proposal(
        creator      = d.get("creator", ""),
        title        = d.get("title", ""),
        description  = d.get("description", ""),
        options      = d.get("options", ["Ja", "Nein", "Enthaltung"]),
        duration_days= int(d.get("duration_days", 7))
    ))


@governance_bp.route("/vote", methods=["POST"])
def vote():
    d = request.json or {}
    return jsonify(_gov.vote(
        voter       = d.get("voter", ""),
        proposal_id = d.get("proposal_id", ""),
        option      = d.get("option", ""),
        voting_power= float(d.get("voting_power", 1.0))
    ))


@governance_bp.route("/execute/<proposal_id>", methods=["POST"])
def execute(proposal_id):
    return jsonify(_gov.execute_proposal(proposal_id))


@governance_bp.route("/voting-power/<address>", methods=["GET"])
def voting_power(address):
    return jsonify(_gov.get_voting_power(address))


@governance_bp.route("/stats", methods=["GET"])
def stats():
    return jsonify(_gov.get_stats())
