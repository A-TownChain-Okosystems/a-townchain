# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# conftest.py — pytest Root-Konfiguration
# Stellt sicher dass 'tests/' im sys.path liegt (lokal + CI)
import sys
import os

# Repo-Root zum Path hinzufügen
sys.path.insert(0, os.path.dirname(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "tests"))
