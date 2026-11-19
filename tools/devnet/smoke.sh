#!/usr/bin/env bash
# Devnet-Smoke-Test (SCR-0147): prueft, dass der laufende Node antwortet und
# der Konsens-Loop tatsaechlich Bloecke finalisiert.
# Exit 0 = alles belegt. Exit 1 = Befund.
set -uo pipefail
PORT="${1:-39471}"

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

fail=0
check() { # check <methode> <regex> <label>
  out="$(val "$1")"
  if printf '%s' "$out" | grep -Eq "$2"; then
    printf '  ✓ %-10s %s\n' "$1" "$out"
  else
    printf '  ✗ %-10s %s\n' "$1" "$out"; fail=1
  fi
}

echo "Devnet-Smoke auf 127.0.0.1:$PORT"
check ping      '^pong$'        "Node antwortet"
check chain_id  '^atc$'         "Chain-Identitaet"
check boot_hash '^[0-9a-f]{8,}$' "Genesis-Bindung"
check peers     '^[0-9]+$'      "Peer-Tabelle"
check did_count '^[0-9]+$'      "DID-Registry"
check height    '^[0-9]+$'      "Konsens-Hoehe"

# Konsens braucht Runden: warte bis mindestens ein Block finalisiert ist.
h="$(val height)"
for _ in $(seq 1 20); do
  [ "${h:-0}" -ge 1 ] && break
  sleep 0.5
  h="$(val height)"
done
if [ "${h:-0}" -ge 1 ]; then
  printf '  ✓ %-10s %s\n' "finalisiert" "Hoehe $h"
else
  printf '  ✗ %-10s %s\n' "finalisiert" "Hoehe bleibt 0"; fail=1
fi

c="$(val consensus)"
if printf '%s' "$c" | grep -Eq 'rounds=[1-9]'; then
  printf '  ✓ %-10s %s\n' "consensus" "$c"
else
  printf '  ✗ %-10s %s\n' "consensus" "$c"; fail=1
fi

if [ "$fail" -eq 0 ]; then echo "PASS — Node antwortet, Konsens finalisiert."; else echo "FAIL — siehe oben."; fi
exit "$fail"
