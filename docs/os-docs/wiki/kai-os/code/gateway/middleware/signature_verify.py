# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
from functools import wraps
from flask import request, jsonify
from blockchain.wallet.ecdsa import ECDSASigner


def require_signature(f):
    """
    Gateway Middleware — ECDSA Signatur-Verifikation.
    Issue #6 — Sichere TX-Autorisierung.

    Schutzt alle TX-Endpoints vor unauthentisierten Transfers.
    Usage:
        @app.route("/api/wallet/send", methods=["POST"])
        @require_signature
        def send():
            ...
    """
    @wraps(f)
    def decorated(*args, **kwargs):
        data      = request.json or {}
        tx        = data.get("tx", {})
        signature = data.get("signature")
        pub_key   = data.get("public_key")

        if not all([tx, signature, pub_key]):
            return jsonify({
                "error": "Missing signature fields",
                "required": ["tx", "signature", "public_key"]
            }), 400

        if not ECDSASigner.verify(tx, signature, pub_key):
            return jsonify({
                "error": "Invalid signature",
                "message": "TX rejected — signature verification failed"
            }), 401

        return f(*args, **kwargs)
    return decorated


def verify_request(data: dict) -> tuple:
    """
    Standalone-Verifikation (ohne Flask Decorator).
    Returns: (is_valid: bool, error_msg: str | None)
    """
    tx        = data.get("tx", {})
    signature = data.get("signature")
    pub_key   = data.get("public_key")

    if not all([tx, signature, pub_key]):
        return False, "Missing required fields: tx, signature, public_key"

    if not ECDSASigner.verify(tx, signature, pub_key):
        return False, "Signature verification failed"

    return True, None
