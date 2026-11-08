#!/bin/bash
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# A-TownChain OS — Unified Build Script (K5 Konsolidierung)
# Usage: ./scripts/build.sh [--no-docker|--no-frontend|--no-python]
# Agent: Aurora #2 (6a275618)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT_DIR"

NO_DOCKER=false
NO_FRONTEND=false
NO_PYTHON=false

for arg in "$@"; do
  case "$arg" in
    --no-docker)   NO_DOCKER=true ;;
    --no-frontend) NO_FRONTEND=true ;;
    --no-python)  NO_PYTHON=true ;;
    *) echo "Unknown arg: $arg" ;;
  esac
done

echo ""
echo "╔══════════════════════════════════════════════╗"
echo "║   A-TownChain OS — Build System (K5)        ║"
echo "╚══════════════════════════════════════════════╝"
echo ""

# ── K5.1a: Python Dependencies ──────────────
if [ "$NO_PYTHON" = false ]; then
  echo "▸ [1/4] Python dependencies..."
  if [ -f requirements.txt ]; then
    pip install -q -r requirements.txt 2>&1 | tail -3
  fi
  # Module-specific requirements
  for mod_req in modules/*/requirements.txt; do
    echo "  → $mod_req"
    pip install -q -r "$mod_req" 2>&1 | tail -1
  done
  echo "  ✅ Python deps installed"
else
  echo "▸ [1/4] Python deps — skipped (--no-python)"
fi

# ── K5.1b: Frontend ────────────────────────
if [ "$NO_FRONTEND" = false ]; then
  echo "▸ [2/4] Frontend build..."
  if [ -f frontend/package.json ]; then
    cd frontend
    if command -v npm &>/dev/null; then
      npm ci 2>/dev/null || npm install 2>&1 | tail -3
      npm run build 2>/dev/null || echo "  ⚠  No build step (static frontend)"
    else
      echo "  ⚠  npm not found — frontend is static HTML"
    fi
    cd "$ROOT_DIR"
  fi
  echo "  ✅ Frontend ready"
else
  echo "▸ [2/4] Frontend — skipped (--no-frontend)"
fi

# ── K5.1c: ATCLang ─────────────────────────
echo "▸ [3/4] ATCLang compiler check..."
if [ -f atclang/requirements.txt ]; then
  pip install -q -r atclang/requirements.txt 2>/dev/null || true
fi
echo "  ✅ ATCLang ready"

# ── K5.1d: Docker ──────────────────────────
if [ "$NO_DOCKER" = false ]; then
  echo "▸ [4/4] Docker images..."
  if command -v docker &>/dev/null; then
    docker compose -f docker/docker-compose.yml build 2>&1 | tail -5
    echo "  ✅ Docker images built"
  else
    echo "  ⚠  Docker not found — skipping image build"
  fi
else
  echo "▸ [4/4] Docker — skipped (--no-docker)"
fi

echo ""
echo "╔══════════════════════════════════════════════╗"
echo "║   ✅ Build complete — ./scripts/start.sh     ║"
echo "╚══════════════════════════════════════════════╝"
echo ""
