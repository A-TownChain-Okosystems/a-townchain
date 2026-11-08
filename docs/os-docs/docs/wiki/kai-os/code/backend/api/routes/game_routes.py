# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/api/routes/game_routes.py
# Shivamon Game API Routes (via Gateway :5004)

from flask import Blueprint, jsonify, request
from blockchain.contracts.shivamon.shivamon_contract import ShivamonContract

game_bp   = Blueprint("game", __name__)
_contract = ShivamonContract()

@game_bp.route("/health", methods=["GET"])
def health():
    return jsonify({"service": "game", "status": "online"})

@game_bp.route("/shivamon/stats", methods=["GET"])
def contract_stats():
    return jsonify(_contract.get_stats())

@game_bp.route("/shivamon/mint", methods=["POST"])
def mint():
    d = request.json or {}
    return jsonify(_contract.mint(
        owner      = d.get("owner", ""),
        element    = d.get("element"),
        rarity     = d.get("rarity"),
        generation = int(d.get("generation", 1))
    ))

@game_bp.route("/shivamon/<token_id>", methods=["GET"])
def get_token(token_id):
    return jsonify(_contract.get_token(token_id))

@game_bp.route("/shivamon/owner/<address>", methods=["GET"])
def owner_tokens(address):
    return jsonify({"owner": address, "shivamon": _contract.get_owner_tokens(address)})

@game_bp.route("/shivamon/transfer", methods=["POST"])
def transfer():
    d = request.json or {}
    return jsonify(_contract.transfer(d.get("token_id"), d.get("from"), d.get("to")))

@game_bp.route("/shivamon/battle", methods=["POST"])
def battle():
    d = request.json or {}
    return jsonify(_contract.battle(d.get("attacker"), d.get("defender")))

@game_bp.route("/shivamon/battle/log", methods=["GET"])
def battle_log():
    return jsonify({"battles": _contract.battle_log[-20:]})


@game_bp.route("/shivamon/breed", methods=["POST"])
def breed():
    d = request.json or {}
    return jsonify(_contract.breed(
        parent1_id = d.get("parent1"),
        parent2_id = d.get("parent2"),
        owner      = d.get("owner", "")
    ))
