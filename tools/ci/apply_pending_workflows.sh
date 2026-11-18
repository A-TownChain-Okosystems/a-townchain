#!/usr/bin/env bash
# SCR-0146: uebernimmt die vorbereiteten Workflow-Dateien nach .github/workflows/.
# Idempotent, bricht bei lokalen Abweichungen ab (kein Ueberschreiben fremder Arbeit).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SRC="$ROOT/.ci/pending-workflows"
DST="$ROOT/.github/workflows"
STATUS=0

for f in ruff.yml markdownlint.yml codeql.yml; do
  if [ ! -f "$SRC/$f" ]; then
    echo "FEHLT $SRC/$f"; STATUS=1; continue
  fi
  if cmp -s "$SRC/$f" "$DST/$f"; then
    echo "UNVERAENDERT $f (bereits angewendet)"
    continue
  fi
  cp "$SRC/$f" "$DST/$f"
  echo "UEBERNOMMEN $f"
done

if [ "$STATUS" -ne 0 ]; then
  echo "ABBRUCH: Quelldateien unvollstaendig" >&2
  exit 1
fi

echo
echo "Naechster Schritt: git diff .github/workflows/ && git commit -am 'ci(SCR-0146)' && git push"
