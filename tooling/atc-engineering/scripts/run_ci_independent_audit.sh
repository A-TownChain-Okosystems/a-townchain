#!/usr/bin/env bash
set -u

# CI-independent organization audit. No network access is performed.
# The caller must provide a workspace containing checked-out repositories.
# Runtime GitHub Actions evidence remains a separate verification layer.

ROOT=${1:-.}
OUT="${ROOT}/CI_INDEPENDENT_AUDIT.txt"
: > "$OUT"

log() { printf '%s\n' "$*" | tee -a "$OUT"; }

log "ATC CI-independent audit"
log "Organization: A-TownChain-Okosystems"
log "Mode: source/static only; GitHub Actions runtime evidence is not inferred"
log "Workspace: $ROOT"

STATUS=0

if ! python3 "$(dirname "$0")/standards_enforcement_audit.py" "$ROOT" --json "$ROOT/STANDARDS_ENFORCEMENT_AUDIT.json" >>"$OUT" 2>&1; then
  log "STANDARDS ENFORCEMENT: FINDINGS"
  STATUS=1
else
  log "STANDARDS ENFORCEMENT: PASS"
fi

if ! python3 "$(dirname "$0")/file_format_language_audit.py" "$ROOT" >>"$OUT" 2>&1; then
  log "FILE FORMAT / LANGUAGE POLICY: FINDINGS"
  STATUS=1
else
  log "FILE FORMAT / LANGUAGE POLICY: PASS"
fi

if ! "$(dirname "$0")/fleet_static_audit.sh" "$ROOT" >>"$OUT" 2>&1; then
  log "FLEET STATIC AUDIT: FINDINGS"
  STATUS=1
else
  log "FLEET STATIC AUDIT: PASS"
fi

log "RESULT: ${STATUS} (0=clean static audit, 1=findings)"
exit "$STATUS"
