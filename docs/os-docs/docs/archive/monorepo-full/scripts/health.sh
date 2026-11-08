#!/bin/bash
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# A-TownChain OS — Health Check Script (K5 Konsolidierung)
# Usage: ./scripts/health.sh
# Agent: Aurora #2 (6a275618)

echo ""
echo "╔══════════════════════════════════════════════╗"
echo "║   A-TownChain OS — Health Checks (K5)       ║"
echo "╚══════════════════════════════════════════════╝"
echo ""

check() {
  local name="$1"
  local url="$2"
  local port="$3"

  if curl -sf "$url" --connect-timeout 2 >/dev/null 2>&1; then
    echo "  ✅ $name   → $url"
  elif lsof -i :"$port" >/dev/null 2>&1; then
    echo "  🟡 $name   → Port $port open (health endpoint not ready)"
  else
    echo "  ❌ $name   → Port $port (not running)"
  fi
}

check "Gateway    " "http://localhost:4000/gateway/health" 4000
check "Frontend   " "http://localhost:3000"                  3000
check "Blockchain " "http://localhost:5000/health"          5000
check "Core      " "http://localhost:8000/health"            8000
check "Contracts " "http://localhost:8001/health"            8001
check "Franchise " "http://localhost:8002/health"            8002
check "Shivamon  " "http://localhost:8003/health"            8003

echo ""
# Docker container status
if command -v docker &>/dev/null; then
  echo "  Docker containers:"
  docker ps --filter "name=atc-" --format "    {{.Names}}: {{.Status}}" 2>/dev/null || true
fi
echo ""
