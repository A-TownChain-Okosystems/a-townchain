#!/bin/bash
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# APPLY_FIXES.sh -- Alle Bugfixes anwenden
# Ausfuehren im Stammverzeichnis von a-townchain-os:
#   bash <(curl -s https://raw.githubusercontent.com/ShivaCoreDev/kai-os-wiki/main/patches/APPLY_FIXES.sh)

WIKI=https://raw.githubusercontent.com/ShivaCoreDev/kai-os-wiki/main/patches
REPO_ROOT=$(pwd)

echo "[1/6] gateway/main.py -- create_app() Factory"
curl -s $WIKI/gateway_main.py > gateway/main.py

echo "[2/6] gateway/router.py -- forward() vollstaendig"
curl -s $WIKI/gateway_router.py > gateway/router.py

echo "[3/6] blockchain/consensus/poh.py -- tick_n() fix"
curl -s $WIKI/poh_fixed.py > blockchain/consensus/poh.py

echo "[4/6] requirements.txt -- google-generativeai"
pip install google-generativeai pynacl cryptography

echo "[5/6] docker-compose.yml -- 5-Node Testnet"
curl -s $WIKI/docker-compose.yml > docker-compose.yml

echo "[6/6] ATC-9900 Governance Contract"
mkdir -p blockchain/contracts/governance
touch blockchain/contracts/governance/__init__.py
curl -s $WIKI/atc9900_governance.py > blockchain/contracts/governance/atc9900_governance.py

echo ""
echo "Fixes angewendet. Tests ausfuehren:"
echo "  python -m pytest tests/ -v --tb=short"
