# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/api/routes/marketplace_routes.py
# Issue #13 — ATC Marketplace Routes

from flask import Blueprint, jsonify, request
from blockchain.contracts.marketplace.marketplace_contract import MarketplaceContract

marketplace_bp = Blueprint("marketplace", __name__)
_market = MarketplaceContract()


@marketplace_bp.route("/health", methods=["GET"])
def health():
    return jsonify({"service": "marketplace", "status": "online"})


@marketplace_bp.route("/listings", methods=["GET"])
def listings():
    return jsonify({
        "listings": _market.get_listings(
            element   = request.args.get("element"),
            rarity    = request.args.get("rarity"),
            min_price = float(request.args.get("min_price", 0) or 0),
            max_price = float(request.args.get("max_price", 0) or 0) or None
        )
    })


@marketplace_bp.route("/list", methods=["POST"])
def list_nft():
    d = request.json or {}
    return jsonify(_market.list_for_sale(
        token_id  = d.get("token_id", ""),
        seller    = d.get("seller", ""),
        price_atc = float(d.get("price_atc", 0))
    ))


@marketplace_bp.route("/buy", methods=["POST"])
def buy():
    d = request.json or {}
    return jsonify(_market.buy(
        listing_id = d.get("listing_id", ""),
        buyer      = d.get("buyer", "")
    ))


@marketplace_bp.route("/cancel", methods=["POST"])
def cancel():
    d = request.json or {}
    return jsonify(_market.cancel_listing(
        listing_id = d.get("listing_id", ""),
        seller     = d.get("seller", "")
    ))


@marketplace_bp.route("/offer", methods=["POST"])
def make_offer():
    d = request.json or {}
    return jsonify(_market.make_offer(
        token_id  = d.get("token_id", ""),
        buyer     = d.get("buyer", ""),
        offer_atc = float(d.get("offer_atc", 0))
    ))


@marketplace_bp.route("/stats", methods=["GET"])
def stats():
    return jsonify(_market.get_stats())
