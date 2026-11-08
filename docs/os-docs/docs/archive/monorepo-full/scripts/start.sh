#!/bin/bash
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# A-TownChain OS — Unified Start Script (K5 Konsolidierung)
# Usage: ./scripts/start.sh [--docker|--local]
# Agent: Aurora #2 (6a275618)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT_DIR"

# ── Modul-Registry: Laufzeit-Inventar aller 60 Module (Verschmelzung 04.09.2026) ──
# Importiert importierbare Python-Pakete und meldet den Workspace-Status
# aller Rust-Crates — EIN Systemstart berührt ALLE Module.
python3 src/modules/registry.py --summary || echo "  [registry] WARNUNG: Registry-Fehler (nicht fatal)"

MODE="${1:---docker}"

echo ""
echo "╔══════════════════════════════════════════════╗"
echo "║   A-TownChain OS — Starting (K5)            ║"
echo "╚══════════════════════════════════════════════╝"
echo ""

if [ "$MODE" = "--local" ] || [ "$MODE" = "-l" ]; then
  # ── Local Mode (no Docker) ────────────────
  echo "▸ Starting 7 services in local mode..."
  export PYTHONPATH="$ROOT_DIR:$PYTHONPATH"
  export ATC_CHAIN_ID=658467
  export ATC_ENV=development

  # 1. Gateway
  python -m src.gateway.main &
  echo "  [1/7] Gateway      → http://localhost:4000  (PID: $!)"
  sleep 1

  # 2. Blockchain Core
  python -m src.blockchain &
  echo "  [2/7] Blockchain   → http://localhost:5000  (PID: $!)"
  sleep 1

  # 3. Core Service
  python -m src.core &
  echo "  [3/7] Core         → http://localhost:8000  (PID: $!)"
  sleep 1

  # 4. Contracts
  python -m src.contracts &
  echo "  [4/7] Contracts    → http://localhost:8001  (PID: $!)"
  sleep 0.5

  # 5. Franchise
  python -m src.franchise &
  echo "  [5/7] Franchise    → http://localhost:8002  (PID: $!)"
  sleep 0.5

  # 6. Game/Shivamon
  python -m src.game &
  echo "  [6/7] Shivamon     → http://localhost:8003  (PID: $!)"
  sleep 0.5

  # 7. Frontend (static)
  cd frontend
  python -m http.server 3000 &
  echo "  [7/7] Frontend     → http://localhost:3000  (PID: $!)"
  cd "$ROOT_DIR"

  echo ""
  echo "  PIDs saved to /tmp/atc-pids"
  jobs -p > /tmp/atc-pids
  echo ""

else
  # ── Docker Mode ───────────────────────────
  echo "▸ Starting via docker compose..."
  docker compose -f docker/docker-compose.yml up -d 2>&1 | tail -10
  echo ""
fi

echo "╔══════════════════════════════════════════════╗"
echo "║   ✅ Services started                        ║"
echo "╠══════════════════════════════════════════════╣"
echo "║   Gateway:      http://localhost:4000        ║"
echo "║   Frontend:     http://localhost:3000        ║"
echo "║   Blockchain:   http://localhost:5000        ║"
echo "║   Core:         http://localhost:8000        ║"
echo "║   Contracts:    http://localhost:8001        ║"
echo "║   Franchise:    http://localhost:8002        ║"
echo "║   Shivamon:     http://localhost:8003        ║"
echo "╚══════════════════════════════════════════════╝"
echo ""
echo "  Health: ./scripts/health.sh"
echo "  Stop:   ./scripts/stop.sh"
echo ""
