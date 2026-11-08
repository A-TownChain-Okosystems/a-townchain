# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# core/kernel.py
# A-TownChain OS — Core Kernel v2.0
# Kein Agent darf ohne Review Core-Logik verändern

from core.event_bus import EventBus
from core.module_loader import ModuleLoader

class Kernel:
    def __init__(self):
        self.event_bus = EventBus()
        self.loader = ModuleLoader(self.event_bus)
        self.running = False
    
    def start(self):
        print("[KERNEL] A-TownChain OS starting...")
        self.running = True
        self.event_bus.emit("system_started", {"version": "2.0", "chain": "A-TownChain"})
    
    def stop(self):
        self.running = False
        print("[KERNEL] System shutdown.")
