#!/usr/bin/env bash
set -u

# Offline repository audit. Run from a checked-out organization workspace:
#   ./scripts/fleet_static_audit.sh /path/to/repos
# The script intentionally performs no network access.
#
# Exit code is non-zero when a repository has a blocking finding or a
# validation command fails. Informational findings (offene Marker/Duplikate) are
# recorded without failing the repository by themselves.

ROOT=${1:-.}
FAILED=0
TOTAL=0
PASS=0
OUT="${ROOT}/FLEET_STATIC_AUDIT.txt"
: > "$OUT"

log() { printf '%s\n' "$*" | tee -a "$OUT"; }

for repo in "$ROOT"/*; do
  [ -d "$repo/.git" ] || continue
  name=$(basename "$repo")
  TOTAL=$((TOTAL + 1))
  repo_failed=0
  log "=== $name ==="

  required=(README.md LICENSE SECURITY.md CHANGELOG.md CODEOWNERS STATUS.md ARCHITECTURE.md)
  for f in "${required[@]}"; do
    if [ ! -f "$repo/$f" ] && [ ! -f "$repo/.github/$f" ]; then
      log "MISSING: $f"
      repo_failed=1
    fi
  done

  if find "$repo/.github/workflows" -maxdepth 1 -type f \( -name '*.yml' -o -name '*.yaml' \) 2>/dev/null | grep -q .; then
    log "CI: workflow present"
  else
    log "CI: workflow missing"
    repo_failed=1
  fi

  if grep -RInE --exclude-dir=.git --exclude-dir=target --exclude-dir=node_modules --exclude-dir=archive \
      '(BEGIN (RSA|EC|OPENSSH|PRIVATE) KEY|AKIA[0-9A-Z]{16}|ghp_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|password[[:space:]]*=[[:space:]]*["'"'][^<][^"'"']+["'"'])' \
      "$repo" >/tmp/atc-secret-scan.$$ 2>/dev/null; then
    log "SECURITY: possible credential pattern"
    cat /tmp/atc-secret-scan.$$ >> "$OUT"
    repo_failed=1
  else
    log "SECURITY: credential pattern scan clean"
  fi
  rm -f /tmp/atc-secret-scan.$$

  if grep -RInE --exclude-dir=.git --exclude-dir=target --exclude-dir=node_modules --exclude-dir=archive \
      '(curl|wget)[^\n]*(\||-o[[:space:]]*/tmp/)[^\n]*(sh|bash)|\|[[:space:]]*(sh|bash)([[:space:]]|$)' \
      "$repo" >/tmp/atc-download-scan.$$ 2>/dev/null; then
    log "SECURITY: unsafe remote execution pattern"
    cat /tmp/atc-download-scan.$$ >> "$OUT"
    repo_failed=1
  else
    log "SECURITY: remote execution scan clean"
  fi
  rm -f /tmp/atc-download-scan.$$

  if find "$repo/.github/workflows" -maxdepth 1 -type f \( -name '*.yml' -o -name '*.yaml' \) 2>/dev/null | grep -q .; then
    if grep -RInE --include='*.yml' --include='*.yaml' 'pull_request_target:' "$repo/.github/workflows" >/tmp/atc-prtarget-scan.$$ 2>/dev/null; then
      log "SECURITY: pull_request_target workflow requires manual review"
      cat /tmp/atc-prtarget-scan.$$ >> "$OUT"
    fi
    if grep -RInE --include='*.yml' --include='*.yaml' 'uses:[[:space:]]*[^@[:space:]]+@(main|master|latest)$' "$repo/.github/workflows" >/tmp/atc-unpinned-scan.$$ 2>/dev/null; then
      log "SECURITY: mutable workflow action reference"
      cat /tmp/atc-unpinned-scan.$$ >> "$OUT"
    fi
    rm -f /tmp/atc-prtarget-scan.$$ /tmp/atc-unpinned-scan.$$
  fi

  if grep -RInE --exclude-dir=.git --exclude-dir=target --exclude-dir=node_modules --exclude-dir=archive \
      '(TODO|FIXME|HACK|XXX)[ :]' "$repo" >/tmp/atc-todo-scan.$$ 2>/dev/null; then
    log "QUALITY: TODO/FIXME markers present (informational)"
    head -50 /tmp/atc-todo-scan.$$ >> "$OUT"
  else
    log "QUALITY: no TODO/FIXME markers found"
  fi
  rm -f /tmp/atc-todo-scan.$$

  if find "$repo" -name Cargo.toml -not -path '*/target/*' -not -path '*/archive/*' | grep -q .; then
    if command -v cargo >/dev/null 2>&1; then
      while IFS= read -r manifest; do
        log "RUST: check $manifest"
        if ! cargo check --manifest-path "$manifest" --all-targets >>"$OUT" 2>&1; then repo_failed=1; fi
        if ! cargo test --manifest-path "$manifest" >>"$OUT" 2>&1; then repo_failed=1; fi
      done < <(find "$repo" -name Cargo.toml -not -path '*/target/*' -not -path '*/archive/*' | sort -u)
    else
      log "RUST: cargo unavailable; deferred"
      repo_failed=1
    fi
  fi

  if find "$repo" -name '*.py' -not -path '*/.git/*' -not -path '*/archive/*' | grep -q .; then
    if command -v python3 >/dev/null 2>&1; then
      if ! python3 -m compileall -q "$repo" -x '(^|/)(\.git|\.venv|node_modules|target|archive)(/|$)' >>"$OUT" 2>&1; then repo_failed=1; fi
    else
      log "PYTHON: python3 unavailable; deferred"
      repo_failed=1
    fi
  fi

  if find "$repo" -name '*.sh' -not -path '*/.git/*' -not -path '*/archive/*' | grep -q .; then
    while IFS= read -r script; do
      if ! bash -n "$script" >>"$OUT" 2>&1; then repo_failed=1; fi
    done < <(find "$repo" -name '*.sh' -not -path '*/.git/*' -not -path '*/archive/*' | sort -u)
  fi

  if find "$repo" -name '*.js' -not -path '*/node_modules/*' -not -path '*/archive/*' | grep -q .; then
    if command -v node >/dev/null 2>&1; then
      while IFS= read -r js; do
        case "$js" in *.test.js|*.spec.js) continue ;; esac
        if ! node --check "$js" >>"$OUT" 2>&1; then repo_failed=1; fi
      done < <(find "$repo" -name '*.js' -not -path '*/node_modules/*' -not -path '*/archive/*' | sort -u)
    else
      log "NODE: unavailable; JS syntax validation deferred"
    fi
  fi

  if find "$repo" -maxdepth 3 -name package.json -not -path '*/node_modules/*' -not -path '*/archive/*' | grep -q .; then
    while IFS= read -r pkg; do
      if [ ! -f "$(dirname "$pkg")/package-lock.json" ] && [ ! -f "$(dirname "$pkg")/pnpm-lock.yaml" ] && [ ! -f "$(dirname "$pkg")/yarn.lock" ]; then
        log "DEPENDENCY: lockfile missing near $pkg (informational)"
      fi
    done < <(find "$repo" -maxdepth 3 -name package.json -not -path '*/node_modules/*' -not -path '*/archive/*' | sort -u)
  fi

  if command -v sha256sum >/dev/null 2>&1; then
    dup=$(find "$repo" -type f \
      -not -path '*/.git/*' -not -path '*/target/*' -not -path '*/node_modules/*' -not -path '*/archive/*' \
      -print0 | xargs -0 sha256sum 2>/dev/null | sort | awk 'seen[$1]++ {print $1}' | sort -u | wc -l)
    if [ "$dup" -gt 0 ]; then log "QUALITY: $dup duplicate-content hash group(s) (informational)"; fi
  fi

  if [ "$repo_failed" -eq 0 ]; then
    PASS=$((PASS + 1)); log "RESULT: PASS"
  else
    FAILED=$((FAILED + 1)); log "RESULT: FINDINGS"
  fi
done

log "=== SUMMARY ==="
log "Repositories scanned: $TOTAL"
log "Pass: $PASS"
log "Findings: $FAILED"
[ "$FAILED" -eq 0 ]
