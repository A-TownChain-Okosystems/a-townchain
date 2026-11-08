#!/usr/bin/env bash
# Copyright (c) 2026 Michael Wroblewski - Apache-2.0
# EXEC-CHAIN-Testrunner (ATC-95-Testmigration, atc-contracts#5):
#   .atc-Testvertrag + Vektor -> native Rust EXEC-GATE-Assembler -> .ops -> ATVM
# Native assembler implementation: atc-vm/src/assembler.rs (Rust, dependency-free).
# Evidenz: Exit 0 = alle Tests PASS, Exit 1 = mind. ein FAIL (fail-closed).
#
# Voraussetzung:
#   ATC_VM_RUNNER=/pfad/zum/atc-vm-runner ./exec_chain/tests/run_tests.sh
set -u
BASE="$(cd "$(dirname "$0")" && pwd)"
VM_RUNNER="${ATC_VM_RUNNER:-atc-vm-runner}"
BUILD_DIR="${BASE}/build"
mkdir -p "$BUILD_DIR"

if ! command -v "$VM_RUNNER" >/dev/null 2>&1; then
  echo "ERROR: atc-vm-runner nicht gefunden (ATC_VM_RUNNER setzen)" >&2
  exit 2
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "ERROR: jq wird fuer Testvektoren benoetigt" >&2
  exit 2
fi

total=0; failed=0
for atc in "$BASE"/t*.atc; do
  [ -f "$atc" ] || continue
  name="$(basename "$atc" .atc)"
  vec="$BASE/vectors/${name%%_*}.json"
  ops_file="$BUILD_DIR/$name.ops"
  total=$((total + 1))

  if ! "$VM_RUNNER" --contract "$atc" --vector "$vec" --out "$BUILD_DIR" >/dev/null 2>&1; then
    echo "FAIL  $name (native assembler)"
    failed=$((failed + 1))
    continue
  fi

  expect="$(jq -er '.expected' "$vec")" || {
    echo "FAIL  $name (invalid vector)"
    failed=$((failed + 1))
    continue
  }
  if "$VM_RUNNER" --ops "$ops_file" --expect "$expect" >/dev/null 2>&1; then
    echo "PASS  $name ($(grep -vc '^#' "$ops_file") Ops, expected=$expect)"
  else
    echo "FAIL  $name (ATVM, expected=$expect)"
    failed=$((failed + 1))
  fi
done

for ops_f in "$BASE"/ops/*.ops; do
  [ -f "$ops_f" ] || continue
  name="$(basename "$ops_f" .ops)"
  vec="$BASE/ops/$name.json"
  total=$((total + 1))

  args=(--ops "$ops_f" --expect "$(jq -er '.expected' "$vec")")
  while IFS= read -r item; do
    args+=(--set-slot "$item")
  done < <(jq -r '.set_slot // {} | to_entries[] | "\(.key):\(.value)"' "$vec")
  while IFS= read -r item; do
    args+=(--expect-slot "$item")
  done < <(jq -r '.expect_slot // {} | to_entries[] | "\(.key):\(.value)"' "$vec")
  if jq -e 'has("caller")' "$vec" >/dev/null 2>&1; then
    args+=(--caller "$(jq -er '.caller' "$vec")")
  fi

  if "$VM_RUNNER" "${args[@]}" >/dev/null 2>&1; then
    echo "PASS  $name (Bytecode, ATVM + Storage-Evidenz)"
  else
    echo "FAIL  $name (Bytecode)"
    failed=$((failed + 1))
  fi
done

echo "=============================="
echo "EXEC-CHAIN-Tests: $((total - failed))/$total PASS"
[ "$failed" -eq 0 ] || exit 1
