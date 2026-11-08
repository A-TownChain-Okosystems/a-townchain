# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/api/orchestrator/orchestrator.py
# A-TownChain — API Orchestrator
#
# Der Orchestrator koordiniert alle Backend-Services.
# Er ist der einzige, der direkt mit Core, Chain, Wallet,
# AI und Game kommuniziert. Das Gateway routet zum Orchestrator.
#
#   GATEWAY → ORCHESTRATOR → Services
#                          ↘ EventBus

from core.event_bus import EventBus
from core.module_loader import ModuleLoader
from backend.api.routes import blockchain, wallet
import time, threading

class APIOrchestrator:
    """Zentraler Koordinator aller API-Services."""

    def __init__(self):
        self.event_bus   = EventBus()
        self.loader      = ModuleLoader(self.event_bus)
        self.services    = {}
        self.start_time  = time.time()
        self._running    = False
        self._lock       = threading.Lock()

    def boot(self):
        """Startet alle Services in der richtigen Reihenfolge."""
        print("[ORCHESTRATOR] Booting A-TownChain services...")
        self._register_services()
        self._running = True
        self.event_bus.emit("orchestrator_ready", {"services": list(self.services.keys())})
        print(f"[ORCHESTRATOR] ✓ {len(self.services)} services online")

    def _register_services(self):
        self.services = {
            "core":       {"port": 5000, "status": "online", "version": "2.0"},
            "blockchain": {"port": 5001, "status": "online", "version": "2.0"},
            "wallet":     {"port": 5002, "status": "online", "version": "2.0"},
            "ai":         {"port": 5003, "status": "online", "version": "2.0"},
            "game":       {"port": 5004, "status": "online", "version": "2.0"},
            "nodes":      {"port": 5005, "status": "online", "version": "2.0"},
        }

    def get_status(self):
        uptime = int(time.time() - self.start_time)
        return {
            "orchestrator": "online",
            "uptime_seconds": uptime,
            "uptime": f"{uptime//3600}h {(uptime%3600)//60}m {uptime%60}s",
            "services": self.services,
            "chain": "A-TownChain",
            "version": "2.0"
        }

    def dispatch(self, service, action, payload=None):
        """Dispatcht eine Aktion an den zuständigen Service."""
        with self._lock:
            svc = self.services.get(service)
            if not svc:
                return {"error": f"Service '{service}' not found"}
            self.event_bus.emit(f"{service}:{action}", payload or {})
            return {"dispatched": True, "service": service, "action": action}

    def shutdown(self):
        self._running = False
        self.event_bus.emit("orchestrator_shutdown", {})
        print("[ORCHESTRATOR] Shutdown complete.")
