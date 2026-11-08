#!/usr/bin/env bash
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# A-TownChain OS — Test Report Generator (K7.9)
# Agent: Aurora #2 (6a275618)

set -euo pipefail
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "╔══════════════════════════════════════════════════╗"
echo "║  A-TownChain OS — Test Report (K7.9)            ║"
echo "╚══════════════════════════════════════════════════╝"
echo "Generated: $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
echo ""

# Python test counts
echo "── Python Tests ──"
TEST_FILES=$(find tests/ -name "test_*.py" | wc -l)
TEST_FUNCS=$(grep -r "def test_" tests/ 2>/dev/null | wc -l)
echo "  Test files: $TEST_FILES"
echo "  Test functions: $TEST_FUNCS"
echo "  Test directories:"
for d in tests/unit/atclang tests/unit/core tests/unit/blockchain tests/unit/network tests/unit/contracts tests/integration tests/e2e; do
  COUNT=$(find "$d" -name "test_*.py" 2>/dev/null | wc -l)
  echo "    $d: $COUNT files"
done
echo ""

# Frontend test counts
echo "── Frontend Tests ──"
FE_TESTS=$(find . -path ./.git -prune -o -name "*.test.ts" -print -o -name "*.test.tsx" -print | wc -l)
echo "  Test files: $FE_TESTS"
echo ""

# Coverage threshold
echo "── Coverage Threshold ──"
echo "  Required: ≥80% (set in pytest.ini --cov-fail-under=80)"
echo ""

# Docker services
echo "── Docker Services ──"
if [ -f docker/docker-compose.yml ]; then
  SERVICES=$(python3 -c "
import yaml
with open('docker/docker-compose.yml') as f:
    d = yaml.safe_load(f)
for name in d.get('services', {}):
    hc = '✅' if 'healthcheck' in d['services'][name] else '❌'
    print(f'  {name}: {hc} healthcheck')
" 2>/dev/null || echo "  (yaml not available)")
  echo "$SERVICES"
fi
echo ""

echo "── CI/CD Pipeline ──"
echo "  ci.yml: 5 jobs (build-python, build-frontend, test, security-bandit, audit)"
echo "  codeql.yml: 1 job (weekly security scan)"
echo "  release.yml: 4 jobs (docker, binaries, changelog, pages)"
echo ""
echo "╔══════════════════════════════════════════════════╗"
echo "║  Report complete. Run ./scripts/test.sh all     ║"
echo "║  for full test suite execution.                 ║"
echo "╚══════════════════════════════════════════════════╝"
