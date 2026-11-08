# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/api/routes/blockchain.py
# Blockchain API Routes (via Gateway :5001)

from flask import Blueprint, jsonify, request
from blockchain.consensus.hybrid_consensus import HybridConsensus
from blockchain.atcoin.atcoin import ATCoin

blockchain_bp = Blueprint("blockchain", __name__)
_consensus = HybridConsensus(difficulty=3)
_atcoin    = ATCoin()

@blockchain_bp.route("/health", methods=["GET"])
def health():
    return jsonify({"service": "blockchain", "status": "online"})

@blockchain_bp.route("/info", methods=["GET"])
def info():
    return jsonify(_consensus.get_chain_info())

@blockchain_bp.route("/blocks", methods=["GET"])
def blocks():
    return jsonify({"blocks": _consensus.blocks[-10:], "total": _consensus.height})

@blockchain_bp.route("/tx/<tx_id>", methods=["GET"])
def get_tx(tx_id):
    for block in _consensus.blocks:
        for tx in block.get("transactions", []):
            if tx.get("tx_id") == tx_id:
                return jsonify({"tx": tx, "block": block["height"], "confirmed": True})
    return jsonify({"error": "TX not found"}), 404

@blockchain_bp.route("/mine", methods=["POST"])
def mine():
    d     = request.json or {}
    miner = d.get("miner", "dev-miner")
    txs   = d.get("transactions", [])
    block = _consensus.create_block(txs, miner)
    reward = _atcoin.mint(miner, block["reward"])
    return jsonify({"block": block, "reward": reward})

@blockchain_bp.route("/validate", methods=["GET"])
def validate():
    valid = _consensus.validate_chain()
    return jsonify({"valid": valid, "height": _consensus.height})

@blockchain_bp.route("/coin", methods=["GET"])
def coin():
    return jsonify(_atcoin.to_dict())

@blockchain_bp.route("/pos/stake", methods=["POST"])
def stake():
    d = request.json or {}
    return jsonify(_consensus.pos.stake(d.get("address"), float(d.get("amount", 0))))

@blockchain_bp.route("/pos/stats", methods=["GET"])
def pos_stats():
    return jsonify(_consensus.pos.get_stats())

@blockchain_bp.route("/poh/state", methods=["GET"])
def poh_state():
    return jsonify(_consensus.poh.get_state())
