#!/usr/bin/env bash
# ATC-OS Build — ein Befehl, reproduzierbar, ohne Ueberraschungen (SCR-0147 / G1).
#
#   ./build.sh            Node-Binary + Python-Umgebung (Standard, ~2-4 min)
#   ./build.sh --all      zusaetzlich kompletter Rust-Workspace und die Apps
#   ./build.sh --check    nur Voraussetzungen pruefen, nichts bauen
#
# Das Skript ist idempotent: was schon gebaut ist, wird nicht neu gebaut.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

MODE="default"
case "${1:-}" in
  --all) MODE="all" ;;
  --check) MODE="check" ;;
  --help|-h) sed -n '2,12p' "$0"; exit 0 ;;
  "") ;;
  *) echo "Unbekannte Option: $1 (--all, --check, --help)" >&2; exit 2 ;;
esac

red() { printf '\033[31m%s\033[0m\n' "$*"; }
grn() { printf '\033[32m%s\033[0m\n' "$*"; }
ylw() { printf '\033[33m%s\033[0m\n' "$*"; }
step() { printf '\n\033[1m▸ %s\033[0m\n' "$*"; }

MISSING=0
need() {
  if command -v "$1" >/dev/null 2>&1; then
    grn "  ✓ $1 $("$1" --version 2>/dev/null | head -1 | cut -c1-40)"
  else
    red "  ✗ $1 fehlt"; MISSING=1
  fi
}

step "Voraussetzungen"
need cargo
need python3
if [ "$MODE" = "all" ]; then need node; need npm; fi

if [ "$MODE" = "check" ]; then
  [ "$MISSING" -eq 0 ] && grn "\nAlles vorhanden." || red "\nFehlende Werkzeuge — siehe oben."
  exit "$MISSING"
fi

if [ "$MISSING" -ne 0 ]; then
  red "\nAbbruch: fehlende Werkzeuge. Rust: https://rustup.rs — Python 3.11+"
  exit 1
fi

step "Rust: Node-Binary (apps/atc-node)"
if [ -x target/release/atc-node ] && [ "$MODE" != "all" ]; then
  grn "  ✓ target/release/atc-node vorhanden (uebersprungen)"
else
  cargo build --release -p atc-node
  grn "  ✓ target/release/atc-node"
fi

if [ "$MODE" = "all" ]; then
  step "Rust: kompletter Workspace"
  cargo build --release --workspace
  grn "  ✓ Workspace gebaut"

  step "Apps: Abhaengigkeiten und Build"
  for app in apps/atc-explorer apps/atc-ide apps/atc-sdk; do
    [ -f "$app/package.json" ] || continue
    if [ -d "$app/node_modules" ]; then
      ylw "  • $app: node_modules vorhanden (npm ci uebersprungen)"
    else
      ( cd "$app" && npm ci --no-audit --no-fund )
    fi
    if grep -q '"build"' "$app/package.json"; then
      ( cd "$app" && npm run build --silent )
      grn "  ✓ $app gebaut"
    else
      grn "  ✓ $app (kein Build-Schritt)"
    fi
  done
fi

step "Python: virtuelle Umgebung"
if [ ! -d .venv ]; then
  python3 -m venv .venv
  grn "  ✓ .venv angelegt"
else
  grn "  ✓ .venv vorhanden"
fi
if [ -f requirements.txt ]; then
  ./.venv/bin/pip install -q -r requirements.txt && grn "  ✓ requirements installiert"
else
  ylw "  • keine requirements.txt im Root — Python-Abhaengigkeiten der Module bleiben unberuehrt"
fi

step "Fertig"
grn "Start: ./start.sh"
grn "Pruefung: ./tools/devnet/smoke.sh"
