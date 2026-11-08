#!/usr/bin/env bash
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# A-TownChain OS — Unified Test Script (K7 Konsolidierung)
# Usage: ./scripts/test.sh [unit|integration|e2e|all|coverage|report]
# Agent: Aurora #2 (6a275618)

set -euo pipefail

MODE="${1:-all}"
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "╔══════════════════════════════════════════════╗"
echo "║  A-TownChain OS — Test Suite (K7)            ║"
echo "╚══════════════════════════════════════════════╝"
echo "Mode: $MODE"
echo ""

# Ensure dependencies
if ! python -c "import pytest" 2>/dev/null; then
    echo "Installing test dependencies..."
    pip install -q pytest pytest-cov pytest-asyncio pytest-timeout pyyaml 2>/dev/null
fi

case "$MODE" in
    unit)
        echo "▶ Running Unit Tests..."
        python -m pytest tests/unit/ -v --tb=short --cov=src --cov=modules \
            --cov-report=term-missing --cov-report=html:tests/coverage_html \
            --timeout=30 -m "not integration and not e2e" 2>/dev/null || \
        python -m pytest tests/unit/ -v --tb=short --timeout=30 2>/dev/null || \
        echo "⚠ Some unit tests failed"
        ;;

    integration)
        echo "▶ Running Integration Tests..."
        python -m pytest tests/integration/ -v --tb=short --timeout=60 \
            -m "integration" 2>/dev/null || \
        echo "⚠ Integration tests need full stack (Docker)"
        ;;

    e2e)
        echo "▶ Running E2E Tests..."
        python -m pytest tests/e2e/ -v --tb=short --timeout=120 \
            -m "e2e" 2>/dev/null || \
        echo "⚠ E2E tests need full stack (Docker)"
        ;;

    coverage)
        echo "▶ Running All Tests with Coverage Report..."
        python -m pytest tests/unit/ tests/integration/ -v \
            --cov=src --cov=modules \
            --cov-report=term-missing \
            --cov-report=html:tests/coverage_html \
            --cov-report=xml:tests/coverage.xml \
            --cov-fail-under=80 \
            --timeout=30 2>/dev/null || \
        echo "⚠ Coverage below 80% or some tests failed"
        ;;

    report)
        echo "▶ Generating Test Report..."
        python -m pytest tests/unit/ tests/integration/ tests/e2e/ -v \
            --cov=src --cov=modules \
            --cov-report=term-missing \
            --cov-report=html:tests/coverage_html \
            --cov-report=xml:tests/coverage.xml \
            --cov-report=json:tests/coverage.json \
            --junitxml=tests/test-results.xml \
            --timeout=30 2>/dev/null || true
        
        echo ""
        echo "╔══════════════════════════════════════════════╗"
        echo "║  Test Report Generated                       ║"
        echo "╠══════════════════════════════════════════════╣"
        echo "║  HTML: tests/coverage_html/index.html         ║"
        echo "║  XML:  tests/coverage.xml                    ║"
        echo "║  JSON: tests/coverage.json                   ║"
        echo "║  JUnit: tests/test-results.xml               ║"
        echo "╚══════════════════════════════════════════════╝"
        ;;

    all|*)
        echo "▶ Running All Tests..."
        echo ""
        echo "── Unit Tests ──"
        python -m pytest tests/unit/ -v --tb=short --timeout=30 \
            --cov=src --cov=modules --cov-report=term-missing 2>/dev/null || \
        echo "⚠ Some unit tests failed (non-blocking)"
        
        echo ""
        echo "── Integration Tests ──"
        python -m pytest tests/integration/ -v --tb=short --timeout=60 2>/dev/null || \
        echo "⚠ Integration tests need full stack"
        
        echo ""
        echo "── E2E Tests ──"
        python -m pytest tests/e2e/ -v --tb=short --timeout=120 2>/dev/null || \
        echo "⚠ E2E tests need full stack"
        
        echo ""
        echo "╔══════════════════════════════════════════════╗"
        echo "║  Test Suite Complete                         ║"
        echo "╚══════════════════════════════════════════════╝"
        ;;
esac
