# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# core/module_loader.py
# Dynamic Plugin/Module Loader

import importlib

class ModuleLoader:
    def __init__(self, event_bus):
        self.event_bus = event_bus
        self.modules = []
    
    def load(self, module_path):
        module = importlib.import_module(module_path)
        if hasattr(module, 'Plugin'):
            plugin = module.Plugin(self.event_bus)
            plugin.register()
            self.modules.append(plugin)
