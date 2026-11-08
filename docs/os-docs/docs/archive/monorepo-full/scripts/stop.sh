#!/bin/bash
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# A-TownChain OS — Unified Stop Script (K5 Konsolidierung)
# Usage: ./scripts/stop.sh
# Agent: Aurora #2 (6a275618)

set -euo pipefail

echo ""
echo "╔══════════════════════════════════════════════╗"
echo "║   A-TownChain OS — Stopping (K5)            ║"
echo "╚══════════════════════════════════════════════╝"
echo ""

# Stop Docker services
if command -v docker &>/dev/null; then
  echo "▸ Stopping Docker containers..."
  docker compose -f docker/docker-compose.yml down 2>/dev/null || true
  echo "  ✅ Docker services stopped"
fi

# Stop local processes (if PIDs file exists)
if [ -f /tmp/atc-pids ]; then
  echo "▸ Stopping local processes..."
  while read -r pid; do
    if kill -0 "$pid" 2>/dev/null; then
      kill "$pid" 2>/dev/null || true
      echo "  → Killed PID $pid"
    fi
  done < /tmp/atc-pids
  rm -f /tmp/atc-pids
  echo "  ✅ Local processes stopped"
else
  echo "  ℹ  No local processes to stop"
fi

# Fallback: kill by port
for port in 3000 4000 5000 8000 8001 8002 8003; do
  pid=$(lsof -ti :"$port" 2>/dev/null || true)
  if [ -n "$pid" ]; then
    kill "$pid" 2>/dev/null || true
    echo "  → Port $port freed (PID: $pid)"
  fi
done

echo ""
echo "╔══════════════════════════════════════════════╗"
echo "║   ✅ All services stopped                    ║"
echo "╚══════════════════════════════════════════════╝"
echo ""
