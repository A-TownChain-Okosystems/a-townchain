# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# core/event_bus.py
# Event-Driven Communication Layer

class EventBus:
    def __init__(self):
        self.listeners = {}
    
    def subscribe(self, event, handler):
        if event not in self.listeners:
            self.listeners[event] = []
        self.listeners[event].append(handler)
    
    def emit(self, event, data=None):
        for handler in self.listeners.get(event, []):
            handler(data)
