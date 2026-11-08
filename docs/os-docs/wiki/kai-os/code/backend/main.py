# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/main.py
# A-TownChain OS — Backend Entry Point
# Starte mit: python -m backend.main

from backend.api.server import create_app
from core.kernel import Kernel
import threading

def run():
    kernel = Kernel()
    kernel.start()

    app = create_app(kernel)
    print("[BACKEND] API Server running on http://localhost:5000")
    app.run(host="0.0.0.0", port=5000, debug=False)

if __name__ == "__main__":
    run()
