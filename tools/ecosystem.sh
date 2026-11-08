#!/usr/bin/env bash
# ATC-Ökosystem — ein Einstiegspunkt für Bau, Test und Laufzeit.
#
#   tools/ecosystem.sh build     alle Workspaces bauen
#   tools/ecosystem.sh test      alle Testsuiten ausführen
#   tools/ecosystem.sh smoke     laufende Kette prüfen (Node, RPC, Compiler, VM, Wallet)
#   tools/ecosystem.sh status    Übersicht: Workspaces, Crates, Programme
#
# Das Monorepo ist ein Root-Workspace plus sechs eigenständige Unter-Workspaces.
# GlobusOS enthält einen UEFI-Bootloader und baut deshalb auf Nightly; alle
# übrigen Workspaces bauen auf stable.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
BIN="$ROOT/target/debug"
WS_ROOT="."
WS_SUB=(
  "kernel/kai-os"
  "components/zkp"
  "tooling/atc-engineering"
  "apps/atc-marketplace"
  "ai/genesis-engine"
  "kernel/globus-os"
)
WS_ALL=("$WS_ROOT" "${WS_SUB[@]}")
CORE_BINS=(atc atc-node atc_wallet atc-vm-runner atc_cli)
FAIL=0
NODE_PID=""

cleanup() {
  if [ -n "$NODE_PID" ] && kill -0 "$NODE_PID" 2>/dev/null; then
    kill "$NODE_PID" 2>/dev/null
    wait "$NODE_PID" 2>/dev/null
  fi
}
trap cleanup EXIT

ok()   { printf '  \033[32m✓\033[0m %s\n' "$1"; }
bad()  { printf '  \033[31m✗\033[0m %s\n' "$1"; FAIL=1; }
info() { printf '  · %s\n' "$1"; }

ws_label() { [ "$1" = "." ] && echo "root" || echo "$1"; }

require_cargo() {
  command -v cargo >/dev/null || { echo "cargo fehlt im PATH"; exit 127; }
}

cmd_build() {
  require_cargo
  echo "Bau aller Workspaces"
  for ws in "${WS_ALL[@]}"; do
    local label; label=$(ws_label "$ws")
    printf '  %-26s ' "$label"
    if ( cd "$ws" && cargo build --workspace ) >/tmp/atc_build_${label//\//_}.log 2>&1; then
      echo "ok"
    else
      echo "FEHLER (Log: /tmp/atc_build_${label//\//_}.log)"
      FAIL=1
    fi
  done
  return $FAIL
}

cmd_test() {
  require_cargo
  echo "Testsuiten aller Workspaces"
  local total=0 failed=0
  for ws in "${WS_ALL[@]}"; do
    local label; label=$(ws_label "$ws"); local log="/tmp/atc_test_${label//\//_}.log"
    printf '  %-26s ' "$label"
    if ( cd "$ws" && cargo test --workspace ) >"$log" 2>&1; then
      local n; n=$(grep -E '^test result: ok\.' "$log" | awk '{s+=$4} END {print s+0}')
      total=$((total+n)); echo "ok ($n Tests)"
    else
      local n; n=$(grep -E '^test result: ok\.' "$log" | awk '{s+=$4} END {print s+0}')
      total=$((total+n)); failed=$((failed+1)); echo "FEHLER (Log: $log)"; FAIL=1
    fi
  done
  echo "  Gesamt: $total Tests bestanden, $failed Workspaces fehlerhaft"
  return $FAIL
}

cmd_smoke() {
  require_cargo
  echo "Laufzeit-Smoke — alle Schichten greifen ineinander"
  for b in "${CORE_BINS[@]}"; do
    [ -x "$BIN/$b" ] || ( cargo build -p atc-node -p atc-core -p atc_wallet -p atc-vm -p atc_cli >/tmp/atc_smoke_build.log 2>&1 )
  done
  for b in "${CORE_BINS[@]}"; do
    [ -x "$BIN/$b" ] && ok "Binary $b vorhanden" || bad "Binary $b fehlt"
  done

  # Freier Port für den Devnet-Node
  local port; port=$(python3 - <<'PY'
import socket
s=socket.socket(); s.bind(("127.0.0.1",0)); print(s.getsockname()[1]); s.close()
PY
)
  local addr="127.0.0.1:$port"
  info "Devnet-Node auf $addr"

  # Kette starten (Chain-Layer)
  "$BIN/atc-node" "$addr" >/tmp/atc_smoke_node.log 2>&1 &
  NODE_PID=$!
  local ready=0
  for _ in $(seq 1 50); do
    if "$BIN/atc_cli" --addr "$addr" ping >/dev/null 2>&1; then ready=1; break; fi
    sleep 0.2
  done
  [ "$ready" = 1 ] && ok "Node antwortet auf RPC" || { bad "Node nicht erreichbar (Log: /tmp/atc_smoke_node.log)"; return 1; }

  # RPC-Schicht über den SDK-Client
  local cid; cid=$("$BIN/atc_cli" --addr "$addr" chain-id 2>&1)
  [ "$cid" = "atc" ] && ok "Chain-ID über RPC: $cid" || bad "Chain-ID unerwartet: $cid"
  local bh; bh=$("$BIN/atc_cli" --addr "$addr" boot-hash 2>&1)
  [ -n "$bh" ] && [ "$bh" != "RPC error" ] && ok "Boot-Hash über RPC: ${bh:0:24}…" || bad "Boot-Hash nicht lesbar: $bh"
  local peers; peers=$("$BIN/atc_cli" --addr "$addr" peers 2>&1)
  [ "$peers" = "2" ] && ok "Peer-Tabelle über RPC: $peers" || bad "Peers unerwartet: $peers"

  # Sprach- und VM-Schicht
  local prog="ai/atclang/crates/atc-core/differential/corpus/arith_precedence.atc"
  if "$BIN/atc" check "$prog" >/tmp/atc_smoke_atc.log 2>&1; then
    ok "ATCLang-Quelle validiert: $(basename "$prog")"
  else
    bad "ATCLang-Validierung fehlgeschlagen"
  fi
  local out; out=$("$BIN/atc" run "$prog" 2>&1 | tail -1)
  [ -n "$out" ] && ok "ATCLang ausgeführt, Ergebnis: $out" || bad "ATCLang-Ausführung ohne Ergebnis"

  local ops="apps/atc-contracts/exec_chain/tests/ops/t10b_mint_positive_accept.ops"
  local vm; vm=$("$BIN/atc-vm-runner" --ops "$ops" 2>&1 | tail -1)
  case "$vm" in
    *"ATVM PASS"*) ok "VM-Gate: $vm" ;;
    *) bad "VM-Gate ohne PASS: $vm" ;;
  esac

  # Wallet-Schicht
  local wallet; wallet=$("$BIN/atc_wallet" 2>&1)
  case "$wallet" in
    *public_key=*) ok "Wallet leitet Schlüssel ab: $(echo "$wallet" | grep -o 'public_key=[0-9a-f]*' | cut -c1-24)…" ;;
    *) bad "Wallet lieferte keinen Schlüssel" ;;
  esac

  cleanup; NODE_PID=""
  echo "  Logs: /tmp/atc_smoke_node.log"
  return $FAIL
}

cmd_status() {
  echo "Workspace                  Crates  Programme"
  for ws in "${WS_ALL[@]}"; do
    local label; label=$(ws_label "$ws")
    local out; out=$( cd "$ws" && cargo metadata --no-deps --format-version 1 2>/dev/null )
    if [ -z "$out" ]; then printf '  %-24s %6s  %s\n' "$label" "?" "nicht auflösbar"; FAIL=1; continue; fi
    echo "$out" | python3 -c "
import json,sys
m=json.load(sys.stdin)
names=set(str(x) for x in m['workspace_members'])
bins=sorted(t[0] for p in m['packages'] if str(p['id']) in names for t in [(b['name'],) for b in p['targets'] if 'bin' in b['kind']])
print(f'{\"$label\":24s} {len(m[\"workspace_members\"]):6d}  {\", \".join(bins) if bins else \"-\"}')"
  done
}

case "${1:-}" in
  build)  cmd_build ;;
  test)   cmd_test ;;
  smoke)  cmd_smoke ;;
  status) cmd_status ;;
  *) sed -n '2,13p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 2 ;;
esac
