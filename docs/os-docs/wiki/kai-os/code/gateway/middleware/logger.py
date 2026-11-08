# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# gateway/middleware/logger.py
# Request Logger

from datetime import datetime

def log_request(req):
    ts = datetime.now().strftime("%H:%M:%S")
    print(f"[GATEWAY] [{ts}] {req.method} /{req.path} — {req.remote_addr}")
