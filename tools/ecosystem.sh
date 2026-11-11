#!/usr/bin/env bash
# ATC-Ökosystem — ein Einstiegspunkt für Bau, Test und Laufzeit.
#
#   tools/ecosystem.sh build     alle Workspaces bauen
#   tools/ecosystem.sh test      alle Testsuiten ausführen
#   tools/ecosystem.sh smoke     laufende Kette prüfen (Node, RPC, Compiler, VM, Wallet)
#   tools/ecosystem.sh status    Übersicht: Workspaces, Crates, Programme
#   tools/ecosystem.sh boot      ShivaCore-Kernel unter QEMU booten (UEFI)
#
# Das Monorepo ist ein Root-Workspace plus sechs eigenständige Unter-Workspaces.
# GlobusOS enthält einen UEFI-Bootloader und baut deshalb auf Nightly; alle
# übrigen Workspaces bauen auf stable.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
# Doppelungsaufloesung 11.11.2026: Die Kernel-Ebene hat je Repository genau eine
# Quelle. Die frueheren Kopien kernel/globus-os und kernel/kai-os sind entfernt;
# gebaut und gebootet wird aus dem kanonischen Auschecken, dessen Pfad hier steht.
KERNEL_DIR="${GLOBUS_OS_DIR:-$ROOT/../globus-os}"
OS_DIR="${A_TOWNCHAIN_OS_DIR:-$ROOT/../a-townchain-os}"
BIN="$ROOT/target/debug"
WS_ROOT="."
# Unter-Workspaces im Monorepo. Die Kernel-Workspaces liegen nicht mehr hier:
# a-townchain-os (kai-os) und globus-os sind eigene Repositories mit eigenem
# Workspace und eigener CI.
WS_SUB=(
  "components/zkp"
  "tooling/atc-engineering"
  "apps/atc-marketplace"
  "ai/genesis-engine"
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

# Zeilen-RPC über bash-TCP (kein netcat nötig)
rpc_line() {
  local hostport="$1" cmd="$2"
  exec 9<>"/dev/tcp/${hostport%:*}/${hostport#*:}" 2>/dev/null || { echo ""; return 1; }
  printf '%s\n' "$cmd" >&9
  local antwort; IFS= read -r -t 5 antwort <&9
  exec 9<&-
  printf '%s' "$antwort"
}
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
  build_kernel
  return $FAIL
}

# ShivaCore-Kernel: no_std, eigenes Ziel, deshalb nicht Teil von
# `cargo build --workspace`. Braucht Nightly und build-std.
build_kernel() {
  printf '  %-26s ' "globus-os/shivacore"
  local log=/tmp/atc_build_kernel.log
  local bin=target/x86_64-unknown-none/release/shivacore
  if [ ! -d "$KERNEL_DIR" ]; then
    echo "uebersprungen (globus-os nicht ausgecheckt: $KERNEL_DIR, GLOBUS_OS_DIR setzen)"
    return 0
  fi
  if ( cd "$KERNEL_DIR" && cargo +nightly build --release -p shivacore \
        --features x86-boot --target x86_64-unknown-none \
        -Z build-std=core,alloc ) >"$log" 2>&1 && [ -f "$KERNEL_DIR/$bin" ]; then
    local sz; sz=$(du -h "$KERNEL_DIR/$bin" | cut -f1)
    echo "ok (bare metal, $sz)"
  else
    echo "FEHLER (Log: $log)"
    FAIL=1
  fi
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

cmd_boot() {
  require_cargo
  echo "ShivaCore-Kernel booten (QEMU, UEFI)"
  local elf="$KERNEL_DIR/target/x86_64-unknown-none/release/shivacore"
  if [ ! -d "$KERNEL_DIR" ]; then
    echo "  $KERNEL_DIR fehlt: globus-os auschecken oder GLOBUS_OS_DIR setzen"
    return 1
  fi
  local out=/tmp/atc_boot; local serial=/tmp/atc_boot_serial.log
  local code=223
  if [ ! -f "$elf" ]; then
    printf '  %-26s ' "Kernel-ELF"
    if ! ( cd "$KERNEL_DIR" && cargo +nightly build --release -p shivacore \
           --features x86-boot --target x86_64-unknown-none \
           -Z build-std=core,alloc ) >/tmp/atc_boot_kernel.log 2>&1; then
      echo "FEHLER (Log: /tmp/atc_boot_kernel.log)"; return 1
    fi
    echo "ok"
  fi

  printf '  %-26s ' "Boot-Image"
  if ( cd "$KERNEL_DIR" && cargo build -q -p boot && \
       ./target/debug/boot target/x86_64-unknown-none/release/shivacore "$out" ) \
       >/tmp/atc_boot_image.log 2>&1; then
    echo "ok ($out/shivacore-uefi.img)"
  else
    echo "FEHLER (Log: /tmp/atc_boot_image.log)"; return 1
  fi

  if ! command -v qemu-system-x86_64 >/dev/null; then
    printf '  %-26s\n' "QEMU" "fehlt — apt-get install qemu-system-x86 ovmf"
    return 1
  fi
  local ovmf=/usr/share/OVMF/OVMF_CODE.fd
  if [ ! -f "$ovmf" ]; then
    printf '  %-26s\n' "UEFI-Firmware" "fehlt — apt-get install ovmf"
    return 1
  fi

  local secs="${ATC_BOOT_SECONDS:-25}"
  cp /usr/share/OVMF/OVMF_VARS.fd /tmp/atc_boot_vars.fd 2>/dev/null
  printf '  %-26s ' "QEMU-Start"
  timeout $((secs + 15)) qemu-system-x86_64 -machine q35 -m 512 -display none \
    -serial "file:$serial" -no-reboot \
    -drive if=pflash,format=raw,unit=0,readonly=on,file="$ovmf" \
    -drive if=pflash,format=raw,unit=1,file=/tmp/atc_boot_vars.fd \
    -drive format=raw,file="$out/shivacore-uefi.img" >/tmp/atc_boot_qemu.log 2>&1
  code=$?
  # 124 = lief bis zum Timeout, also erwartet: ein Kernel bleibt stehen
  if [ "$code" = "124" ]; then echo "ok ($secs s)"; else echo "beendet mit $code"; fi

  local entry switches
  entry=$(grep -ac "Kernel-Einstiegspunkt erreicht" "$serial" 2>/dev/null || echo 0)
  switches=$(grep -ac "context switch to PID" "$serial" 2>/dev/null || echo 0)
  [ "$entry" -gt 0 ] && ok "Kernel-Einstiegspunkt erreicht" || bad "Kernel ist nicht gestartet"
  [ "$switches" -gt 0 ] && ok "Scheduler wechselt Kontexte ($switches Wechsel)" || info "kein Kontextwechsel gesehen"
  grep -aq "ACPI.*validated" "$serial" && ok "ACPI-Tabellen geprüft" || info "ACPI nicht bestätigt"
  info "Serielles Protokoll: $serial"
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

  # Konsens-Schicht (SCR-0117): Runden finalisieren und Höhe wächst
  local h0; h0=$(rpc_line "$addr" HEIGHT)
  sleep 5
  local h1; h1=$(rpc_line "$addr" HEIGHT)
  if [[ "$h1" =~ ^[0-9]+$ ]] && [ "$h1" -gt "${h0:-0}" ]; then
    ok "Konsens finalisiert Blöcke (Höhe ${h1:-?})"
  else
    bad "Kein Konsens-Fortschritt (Höhe ${h0:-?} -> ${h1:-?})"
  fi
  local cons; cons=$(rpc_line "$addr" CONSENSUS)
  [[ "$cons" == *"quorum_power="* ]] && ok "Quorum-Status über RPC: ${cons:0:46}…" || bad "Konsens-Status fehlt: $cons"
  local chash; chash=$(rpc_line "$addr" BEST_HASH)
  [ -n "$chash" ] && [ "$chash" != "-" ] && ok "Finaler Block-Hash: ${chash:0:24}…" || bad "Kein finaler Block-Hash"

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
  boot)   cmd_boot ;;
  status) cmd_status ;;
  *) sed -n '2,13p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 2 ;;
esac
