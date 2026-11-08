# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# gateway/middleware/rate_limit.py
# Rate Limiter — schützt Backend vor Überlastung

from collections import defaultdict
from time import time
from flask import request, jsonify

class RateLimiter:
    def __init__(self, max_requests=100, window=60):
        self.max_requests = max_requests
        self.window = window
        self.requests = defaultdict(list)

    def is_allowed(self, client_ip):
        now = time()
        history = self.requests[client_ip]
        # Alte Einträge entfernen
        self.requests[client_ip] = [t for t in history if now - t < self.window]
        if len(self.requests[client_ip]) >= self.max_requests:
            return False
        self.requests[client_ip].append(now)
        return True

rate_limiter = RateLimiter(max_requests=100, window=60)
