#!/usr/bin/env bash
# ATC-OS Devnet starten (SCR-0147 / G1).
#
#   ./start.sh               1 Node  (RPC 39471, Gossip 39472)
#   ./start.sh --nodes 3     3 Nodes (RPC 39471/39481/39491)
#   ./start.sh stop          alles beenden
#   ./start.sh status        laufende Nodes und RPC-Antwort
#
# Logs: .devnet/logs/node-<n>.log · PIDs: .devnet/pids/
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"
RUN="$ROOT/.devnet"
BIN="$ROOT/target/release/atc-node"
NODES=1
BASE_RPC=39471
BASE_GOSSIP=39472

grn() { printf '\033[32m%s\033[0m\n' "$*"; }
red() { printf '\033[31m%s\033[0m\n' "$*"; }
ylw() { printf '\033[33m%s\033[0m\n' "$*"; }

rpc() { # rpc <port> <method>
  python3 - "$1" "$2" <<'PY'
import socket, sys, json
port, method = int(sys.argv[1]), sys.argv[2]
try:
    with socket.create_connection(("127.0.0.1", port), timeout=2) as s:
        s.sendall((json.dumps({"jsonrpc":"2.0","method":method,"id":1}, separators=(",",":"))+"\n").encode())
        data = s.makefile().readline().strip()
    print(data)
except Exception as e:
    print("FEHLER:", e); sys.exit(1)
PY
}

stop_all() {
  local n=0
  if [ -d "$RUN/pids" ]; then
    for p in "$RUN"/pids/*.pid; do
      [ -f "$p" ] || continue
      pid="$(cat "$p")"
      if kill -0 "$pid" 2>/dev/null; then kill "$pid" && n=$((n+1)); fi
      rm -f "$p"
    done
  fi
  grn "Gestoppt: $n Node(s)"
}

status() {
  local found=0
  for f in "$RUN"/pids/*.pid; do
    [ -f "$f" ] || continue
    found=1
    pid="$(cat "$f")"; name="$(basename "$f" .pid)"
    if kill -0 "$pid" 2>/dev/null; then
      idx="${name##*-}"; port=$((BASE_RPC + (idx-1)*10))
      grn "  ✓ $name (pid $pid) RPC $port → $(rpc "$port" chain_id)"
    else
      red "  ✗ $name (pid $pid) laeuft nicht mehr"
    fi
  done
  [ "$found" -eq 0 ] && ylw "Kein Node gestartet."
  return 0
}

case "${1:-start}" in
  stop) stop_all; exit 0 ;;
  status) status; exit 0 ;;
  --nodes) NODES="${2:?Anzahl fehlt}"; shift 2 ;;
  start) ;;
  *) red "Unbekanntes Kommando: $1 (start|stop|status|--nodes N)"; exit 2 ;;
esac

if [ ! -x "$BIN" ]; then
  ylw "Binary fehlt — starte ./build.sh"
  ./build.sh
fi

mkdir -p "$RUN/logs" "$RUN/pids"
stop_all >/dev/null 2>&1 || true

for i in $(seq 1 "$NODES"); do
  RPC_PORT=$((BASE_RPC + (i-1)*10))
  GOSSIP_PORT=$((BASE_GOSSIP + (i-1)*10))
  LOG="$RUN/logs/node-$i.log"
  nohup "$BIN" "127.0.0.1:$RPC_PORT" "127.0.0.1:$GOSSIP_PORT" >"$LOG" 2>&1 &
  echo $! > "$RUN/pids/node-$i.pid"
  printf "  Node %d gestartet (pid %s) RPC %d\n" "$i" "$!" "$RPC_PORT"
done

printf "\nWarte auf RPC-Bereitschaft"
ready=0
for _ in $(seq 1 40); do
  if rpc "$BASE_RPC" ping >/dev/null 2>&1; then ready=1; break; fi
  printf "."; sleep 0.25
done
printf "\n"

if [ "$ready" -ne 1 ]; then
  red "RPC antwortet nicht. Log: $RUN/logs/node-1.log"
  tail -20 "$RUN/logs/node-1.log" >&2 || true
  exit 1
fi

grn "Devnet laeuft."
echo
echo "  Chain-ID : $(rpc "$BASE_RPC" chain_id)"
echo "  Boot-Hash: $(rpc "$BASE_RPC" boot_hash)"
echo "  Peers    : $(rpc "$BASE_RPC" peers)"
echo
echo "  RPC-Beispiel:  ./tools/devnet/smoke.sh"
echo "  Stoppen     :  ./start.sh stop"
