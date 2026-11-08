# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# plugins/wallet.py
# ATC Wallet Plugin — ATC-8300 Standard

class Plugin:
    def __init__(self, event_bus):
        self.event_bus = event_bus
        self.balance = {}
    
    def register(self):
        self.event_bus.subscribe("transfer", self.on_transfer)
    
    def on_transfer(self, data):
        print(f"[WALLET] Transfer: {data}")
