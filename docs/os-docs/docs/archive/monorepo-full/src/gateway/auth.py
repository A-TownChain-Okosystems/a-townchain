# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# gateway/middleware/auth.py
# API Key Authentifizierung

from functools import wraps
from flask import request, jsonify
import os

VALID_KEYS = set(os.getenv("API_KEYS", "atc-dev-key-2025").split(","))

def require_api_key(f):
    """Decorator: schützt Endpoints mit API Key."""
    @wraps(f)
    def decorated(*args, **kwargs):
        key = request.headers.get("X-API-Key") or request.args.get("api_key")
        if key not in VALID_KEYS:
            return jsonify({"error": "Unauthorized", "code": 401}), 401
        return f(*args, **kwargs)
    return decorated
