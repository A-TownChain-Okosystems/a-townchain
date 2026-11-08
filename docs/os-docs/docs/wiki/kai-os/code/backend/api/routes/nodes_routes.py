# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/api/routes/nodes_routes.py
# Node Network API Routes (via Gateway :5005)

from flask import Blueprint, jsonify, request
from blockchain.nodes.node import ATCNode, NodeType, NodeNetwork

nodes_bp    = Blueprint("nodes", __name__)
_network    = NodeNetwork()

# Demo-Nodes beim Start
for i in range(1, 6):
    n = ATCNode(f"node-{i:03d}", NodeType.FULL)
    n.stake = i * 1000
    _network.add_node(n)
# Validator + Miner
v = ATCNode("validator-001", NodeType.VALIDATOR); v.stake = 50000; _network.add_node(v)
m = ATCNode("miner-001",     NodeType.MINER);     _network.add_node(m)

@nodes_bp.route("/health", methods=["GET"])
def health():
    return jsonify({"service": "nodes", "status": "online"})

@nodes_bp.route("/", methods=["GET"])
def list_nodes():
    return jsonify({"nodes": _network.get_all_info(), "stats": _network.get_stats()})

@nodes_bp.route("/stats", methods=["GET"])
def stats():
    return jsonify(_network.get_stats())

@nodes_bp.route("/<node_id>", methods=["GET"])
def get_node(node_id):
    node = _network.get_node(node_id)
    if not node:
        return jsonify({"error": "Node not found"}), 404
    return jsonify(node.get_info())

@nodes_bp.route("/register", methods=["POST"])
def register():
    d = request.json or {}
    node_id   = d.get("node_id", f"node-{len(_network.nodes)+1:03d}")
    node_type = NodeType(d.get("type", "full"))
    node      = ATCNode(node_id, node_type)
    node.stake = int(d.get("stake", 0))
    _network.add_node(node)
    return jsonify({"registered": True, "node": node.get_info()})
