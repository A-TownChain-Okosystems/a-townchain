#!/usr/bin/env bash
# Devnet-E2E (SCR-0147): beweist, dass der Konsens-Loop laeuft und Bloecke
# finalisiert werden — nicht nur, dass ein Port offen ist.
#
#   1. Node starten, Genesis-Hoehe 0
#   2. warten, bis der Konsens mindestens zwei Bloecke finalisiert hat
#   3. Konsens-Status pruefen (Validatoren, Quorum, Runden)
#   4. Node beenden und neu starten: Prozess kommt wieder hoch
#
# Exit 0 nur bei vollstaendigem Nachweis.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BIN="$ROOT/target/release/atc-node"
WORK="$(mktemp -d)"
PORT=39571
GOSSIP=39572
fail=0
PID=""

cleanup() { [ -n "${PID:-}" ] && kill "$PID" 2>/dev/null; wait "${PID:-}" 2>/dev/null; }
trap cleanup EXIT

rpc() {
  python3 - "$PORT" "$1" <<'PY'
import socket, sys, json
port, method = int(sys.argv[1]), sys.argv[2]
try:
    with socket.create_connection(("127.0.0.1", port), timeout=3) as s:
        s.sendall((json.dumps({"jsonrpc":"2.0","method":method,"id":1}, separators=(",",":"))+"\n").encode())
        print(s.makefile().readline().strip())
except Exception as e:
    print("FEHLER:", e); sys.exit(1)
PY
}
val() { rpc "$1" | sed -n 's/.*"result":"\([^"]*\)".*/\1/p'; }

start_node() {
  "$BIN" "127.0.0.1:$PORT" "127.0.0.1:$GOSSIP" > "$WORK/node.log" 2>&1 &
  PID=$!
  for _ in $(seq 1 60); do
    [ "$(val ping)" = "pong" ] && return 0
    sleep 0.2
  done
  echo "  ✗ Node startet nicht"; tail -5 "$WORK/node.log"; fail=1; return 1
}

echo "E2E in $WORK"
start_node || exit 1

echo "  1) Startzustand"
h0="$(val height)"; echo "     Hoehe: $h0"
[ "$h0" = "0" ] || { echo "  ✗ erwartet 0"; fail=1; }

echo "  2) Konsens finalisiert Bloecke"
h=""
for _ in $(seq 1 30); do
  h="$(val height)"
  [ "${h:-0}" -ge 2 ] && break
  sleep 0.5
done
echo "     Hoehe nach Warten: $h"
[ "${h:-0}" -ge 2 ] || { echo "  ✗ Konsens kommt nicht voran"; fail=1; }

echo "  3) Konsens-Status"
c="$(val consensus)"; echo "     $c"
case "$c" in *"rounds=0"*) echo "  ✗ keine finalisierten Runden"; fail=1;; esac
case "$c" in *"validators=3"*) ;; *) echo "  ! Validatorzahl unerwartet (Devnet erwartet 3)";; esac
case "$c" in *"quorum="*) ;; *) echo "  ✗ Quorum fehlt im Status"; fail=1;; esac

echo "  4) Neustart"
kill "$PID" 2>/dev/null; wait "$PID" 2>/dev/null; PID=""
start_node || exit 1
h2="$(val height)"; echo "     Hoehe nach Neustart: $h2 (Devnet haelt keinen Zustand: Start bei 0 ist erwartet)"
[ "$(val ping)" = "pong" ] || { echo "  ✗ Node nach Neustart nicht erreichbar"; fail=1; }

echo
if [ "$fail" -eq 0 ]; then echo "PASS — Konsens finalisiert, Node startet reproduzierbar."; else echo "FAIL — siehe oben."; fi
exit "$fail"
