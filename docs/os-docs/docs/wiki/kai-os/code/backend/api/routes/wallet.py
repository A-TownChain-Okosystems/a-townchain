# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/api/routes/wallet.py
# Wallet API Routes (via Gateway :5002)

from flask import Blueprint, jsonify, request
from backend.wallet.wallet import ATCWallet
from blockchain.atcoin.atcoin import ATCoin

wallet_bp = Blueprint("wallet", __name__)
_atcoin   = ATCoin()
_wallet   = ATCWallet(_atcoin)

@wallet_bp.route("/health", methods=["GET"])
def health():
    return jsonify({"service": "wallet", "status": "online"})

@wallet_bp.route("/create", methods=["POST"])
def create():
    d = request.json or {}
    return jsonify(_wallet.create_wallet(
        label      = d.get("label", "Main Wallet"),
        passphrase = d.get("passphrase", "A-TownChain")
    ))

@wallet_bp.route("/restore", methods=["POST"])
def restore():
    d      = request.json or {}
    phrase = d.get("seed_phrase", [])
    if isinstance(phrase, str):
        phrase = phrase.strip().split()
    return jsonify(_wallet.restore_wallet(phrase, d.get("passphrase", "A-TownChain")))

@wallet_bp.route("/balance/<address>", methods=["GET"])
def balance(address):
    return jsonify(_wallet.get_balance(address))

@wallet_bp.route("/send", methods=["POST"])
def send():
    d = request.json or {}
    return jsonify(_wallet.send(d.get("from"), d.get("to"), float(d.get("amount", 0))))

@wallet_bp.route("/validate/<address>", methods=["GET"])
def validate(address):
    return jsonify(_wallet.validate_address(address))

@wallet_bp.route("/accounts", methods=["GET"])
def accounts():
    return jsonify({"accounts": _wallet.get_accounts()})

@wallet_bp.route("/nft/mint", methods=["POST"])
def mint_nft():
    d = request.json or {}
    return jsonify(_wallet.mint_nft(d.get("owner"), d.get("nft_id"), d.get("metadata", {})))

@wallet_bp.route("/nfts/<address>", methods=["GET"])
def nfts(address):
    return jsonify({"address": address, "nfts": _wallet.get_nfts(address)})
