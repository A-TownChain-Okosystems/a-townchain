#!/usr/bin/env bash
# atc-engineering — CI-Workflow manuell aktivieren
# Grund: Agent-OAuth-Token ohne 'workflow'-Scope (GitHub verweigert
# Workflow-Pushes). Owner-Push erforderlich (bekanntes Org-Muster,
# vgl. atc-standards #13 / Issue #79).
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p .github/workflows
cp ci-fix/workflows/ci.yml .github/workflows/ci.yml
git add .github/workflows/ci.yml
git commit -m "ci: Struktur-Gate aktivieren (ci-fix/workflows/ci.yml, Owner-Push 14.09.2026)"
git push
echo "✅ CI-Workflow aktiv: .github/workflows/ci.yml"
