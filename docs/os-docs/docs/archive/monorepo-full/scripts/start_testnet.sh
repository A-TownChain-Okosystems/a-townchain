#!/bin/bash
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# A-TownChain Testnet — lokales 5-Node-Netz starten
# Issue #8 · ATN-1000

set -e

echo "╔══════════════════════════════════════════════════════╗"
echo "║  A-TownChain OS — 5-Node Testnet                    ║"
echo "╠══════════════════════════════════════════════════════╣"
echo "║  Bootstrap:   172.28.0.10:4001  (UDP Discovery)     ║"
echo "║  Validator 1: 172.28.0.11:5011  (REST API)          ║"
echo "║  Validator 2: 172.28.0.12:5012  (REST API)          ║"
echo "║  FullNode:    172.28.0.13:5013  (REST API)          ║"
echo "║  Gateway:     172.28.0.14:4000  (Public API)        ║"
echo "║  Monitor:     localhost:3000    (Grafana)            ║"
echo "╚══════════════════════════════════════════════════════╝"

# Docker Compose starten
if command -v docker compose &> /dev/null; then
    docker compose up -d --build
elif command -v docker-compose &> /dev/null; then
    docker-compose up -d --build
else
    echo "❌ Docker Compose nicht gefunden!"
    exit 1
fi

echo ""
echo "⏳ Warte auf Bootstrap Node..."
sleep 5

# Nodes prüfen
echo ""
echo "🔍 Node-Status:"
for port in 5010 5011 5012 5013 5014; do
    status=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:$port/health 2>/dev/null || echo "000")
    if [ "$status" = "200" ]; then
        echo "  ✅ Port $port — OK"
    else
        echo "  ⚠️  Port $port — HTTP $status"
    fi
done

echo ""
echo "✅ Testnet läuft! Gateway: http://localhost:4000"
echo "📊 Monitoring:   http://localhost:3000 (admin/atcadmin)"
echo ""
echo "Stoppen: docker compose down"
