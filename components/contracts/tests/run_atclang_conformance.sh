#!/usr/bin/env bash
# Copyright (c) 2026 Michael Wroblewski - Apache-2.0
# ATCLang native contract conformance gate.
# No Python contract implementation is loaded or executed here.
# The ATCLang toolchain performs SecurityGate + semantic compilation;
# EXEC-CHAIN tests separately exercise the native Rust ATVM runner.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ATCLANG_ROOT="${ATCLANG_ROOT:-}"
ATC_CLI="${ATC_CLI:-$ATCLANG_ROOT/target/release/atc}"

if [[ -z "$ATCLANG_ROOT" ]]; then
  echo "ERROR: ATCLANG_ROOT must point to a checkout of A-TownChain-Okosystems/atclang" >&2
  exit 2
fi


count=0
failed=0
while IFS= read -r -d '' file; do
  count=$((count + 1))
  if "$ATC_CLI" check "$file" >/dev/null; then
    echo "PASS  ${file#"$ROOT/"}"
  else
    echo "FAIL  ${file#"$ROOT/"}" >&2
    failed=$((failed + 1))
  fi
done < <(find "$ROOT" -type f -name '*.atc' \
  -not -path "$ROOT/atclang/*" \
  -not -path '*/node_modules/*' \
  -not -path '*/archive/*' -print0 | sort -z)

echo "ATCLang conformance: $((count - failed))/$count PASS"
[[ "$count" -gt 0 && "$failed" -eq 0 ]]
