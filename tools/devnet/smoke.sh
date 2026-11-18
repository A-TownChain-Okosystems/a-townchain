#!/usr/bin/env bash
# Devnet-Smoke-Test (SCR-0147 / G2-Vorstufe): prueft, dass ein laufender Node
# die drei Kernfragen beantwortet und die Antworten plausibel sind.
# Exit 0 = Devnet antwortet korrekt. Exit 1 = Befund.
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

fail=0
check() { # check <methode> <erwartung-regex> <label>
  out="$(rpc "$2")"
  if printf '%s' "$out" | grep -Eq "$3"; then
    printf '  ✓ %-12s %s\n' "$1" "$out"
  else
    printf '  ✗ %-12s %s\n' "$1" "$out"; fail=1
  fi
}

echo "Devnet-Smoke auf 127.0.0.1:$PORT"
check ping      ping      '"result":"pong"'
check chain_id  chain_id  '"result":"atc"'
check boot_hash boot_hash '"result":"[0-9a-f]{8,}"'
check peers     peers     '"result":"[0-9]+"'

if [ "$fail" -eq 0 ]; then echo "PASS — Node antwortet korrekt."; else echo "FAIL — siehe oben."; fi
exit "$fail"
