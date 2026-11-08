#!/bin/bash
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# CI/CD Fix Script — Manual push required (GitHub workflow scope)
# Run from repo root: bash apply-fix.sh

set -e

echo "=== Applying CI/CD fixes ==="

# Fix ci-cd.yml
cp ci-cd.yml .github/workflows/ci-cd.yml
echo "✅ .github/workflows/ci-cd.yml updated"

# Fix codeql.yml
cp codeql.yml .github/workflows/codeql.yml
echo "✅ .github/workflows/codeql.yml updated"

# Commit and push
git add .github/workflows/ci-cd.yml .github/workflows/codeql.yml
git commit -m "fix(ci): ATCLang native CI/CD + CodeQL workflow

- ci-cd.yml: ATCLang parse check, test suite, security scan, Docker build
- codeql.yml: Standard English CodeQL analysis (Python)
- Removed: Solidity tests, npm ci, broken German CodeQL"
git push origin main

echo "✅ CI/CD fixes pushed to main"
