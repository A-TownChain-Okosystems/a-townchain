# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Franchise Factory API Routes v2.0.0
"""
from flask import Blueprint, jsonify, request
from franchise_factory.factory import FranchiseFactory, FranchiseStatus, RoyaltyTier

bp = Blueprint("franchise", __name__, url_prefix="/api/franchise")
factory = FranchiseFactory()

@bp.route("/", methods=["GET"])
def list_franchises():
    status_filter = request.args.get("status")
    sf = FranchiseStatus(status_filter) if status_filter else None
    franchises = [
        {
            "id": f.id, "name": f.name, "owner": f.owner,
            "status": f.status.value, "members": len(f.members),
            "vault_balance": f.vault.balance,
            "token_symbol": f.token_symbol,
        }
        for f in factory.list_all(sf)
    ]
    return jsonify({"franchises": franchises, "total": len(franchises)})

@bp.route("/create", methods=["POST"])
def create():
    d = request.get_json()
    if not all(k in d for k in ["name","owner","token_symbol"]):
        return jsonify({"error": "name, owner, token_symbol required"}), 400
    tier = RoyaltyTier[d.get("royalty_tier","BRONZE").upper()]
    f = factory.create(
        name=d["name"], owner=d["owner"],
        description=d.get("description",""),
        token_symbol=d["token_symbol"],
        token_supply=float(d.get("token_supply", 1_000_000)),
        royalty_tier=tier,
    )
    return jsonify({"id": f.id, "name": f.name, "status": f.status.value}), 201

@bp.route("/<fid>", methods=["GET"])
def get_franchise(fid):
    f = factory.get(fid)
    if not f: return jsonify({"error": "Not found"}), 404
    return jsonify({
        "id": f.id, "name": f.name, "owner": f.owner,
        "description": f.description,
        "token_symbol": f.token_symbol,
        "token_supply": f.token_supply,
        "royalty_tier": f.royalty_tier.name,
        "status": f.status.value,
        "members": len(f.members),
        "vault": {"balance": f.vault.balance, "total_in": f.vault.total_in},
        "created": f.created,
    })

@bp.route("/<fid>/join", methods=["POST"])
def join(fid):
    d = request.get_json()
    if not all(k in d for k in ["member","stake"]):
        return jsonify({"error": "member and stake required"}), 400
    ok = factory.join(fid, d["member"], float(d["stake"]))
    return jsonify({"success": ok})

@bp.route("/stats", methods=["GET"])
def stats():
    return jsonify(factory.stats())
