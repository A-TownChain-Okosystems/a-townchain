#!/bin/bash
# A-TownChain OS — Workflow Fix Script
# Run this locally to fix CI/CD workflow files
# Usage: ./scripts/fix-workflows.sh

set -e

echo "🔧 Fixing workflow files..."

# Fix codeql.yml (replace German keys with English)
cat > .github/workflows/codeql.yml << 'CODEQL'
name: "CodeQL Advanced"

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]
  schedule:
    - cron: '0 0 * * 0'

jobs:
  analyze:
    name: Analyze (${{ matrix.language }})
    runs-on: ${{ (matrix.language == 'swift' && 'macos-latest') || 'ubuntu-latest' }}
    permissions:
      security-events: write
      packages: read
      actions: read
      contents: read

    strategy:
      fail-fast: false
      matrix:
        include:
        - language: javascript-typescript
          build-mode: none
        - language: python
          build-mode: none

    steps:
    - name: Checkout repository
      uses: actions/checkout@v4

    - name: Initialize CodeQL
      uses: github/codeql-action/init@v4
      with:
        languages: ${{ matrix.language }}
        build-mode: ${{ matrix.build-mode }}

    - name: Perform CodeQL Analysis
      uses: github/codeql-action/analyze@v4
      with:
        category: "/language:${{ matrix.language }}"
CODEQL

echo "  ✅ codeql.yml fixed (German→English)"

# Copy release.yml from templates
cp docs/ci-templates/release.yml .github/workflows/release.yml
echo "  ✅ release.yml deployed"

# Commit and push
git add .github/workflows/codeql.yml .github/workflows/release.yml
git commit -m "fix: codeql.yml German→English + deploy release.yml (CI/CD repair) [agent: aurora-base44-superagent-6a2756186106d6f0fbb105b5]"
git push origin main

echo "🎉 Done! CI/CD workflows fixed."
