# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# A-TownChain OS — Root Makefile (K5 Konsolidierung)
# Usage: make [target]
# Agent: Aurora #2 (6a275618)

.PHONY: build start stop test clean docker health testnet-up testnet-down testnet-logs lint help

PYTHON      = python3
PIP         = pip
COMPOSE     = docker compose -f docker/docker-compose.yml
TESTNET     = docker compose -f docker/docker-compose.testnet.yml
TESTS       = tests/
COVERAGE    = --cov=src --cov=modules --cov-report=term-missing --cov-fail-under=80

help:
	@echo "A-TownChain OS — Build System (K5)"
	@echo ""
	@echo "  make build       — Install deps + build frontend + Docker images"
	@echo "  make start       — Start all services (Docker mode)"
	@echo "  make start-local — Start all services (local mode)"
	@echo "  make stop        — Stop all services"
	@echo "  make health      — Check all service health endpoints"
	@echo "  make test        — Run all tests with coverage"
	@echo "  make test-fast   — Run tests (no coverage, fast)"
	@echo "  make lint        — Run flake8 on all modules"
	@echo "  make clean       — Remove build artifacts and caches"
	@echo "  make docker      — Build and start Docker services"
	@echo "  make testnet-up  — Start 3-node testnet"
	@echo "  make testnet-down— Stop testnet"
	@echo ""

build:
	@bash scripts/build.sh

start:
	@bash scripts/start.sh --docker

start-local:
	@bash scripts/start.sh --local

stop:
	@bash scripts/stop.sh

health:
	@bash scripts/health.sh

test:
	@$(PYTHON) -m pytest $(TESTS) $(COVERAGE) --timeout=30 -v

test-fast:
	@$(PYTHON) -m pytest $(TESTS) --timeout=30 -q -k "not testnet"

lint:
	@$(PYTHON) -m flake8 src/ modules/ blockchain/ atclang/ --max-line-length=120 || true

clean:
	@find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name ".pytest_cache" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name "*.egg-info" -exec rm -rf {} + 2>/dev/null || true
	@rm -rf frontend/dist frontend/node_modules 2>/dev/null || true
	@rm -rf .coverage htmlcov 2>/dev/null || true
	@echo "✅ Cleaned"

docker: build start

# ── Testnet (3 nodes) ───────────────────────
testnet-up:
	@echo "🚀 Starting A-TownChain Testnet..."
	@$(TESTNET) up -d
	@echo "✅ Testnet running — Gateway: http://localhost:4000"

testnet-down:
	@echo "🛑 Stopping testnet..."
	@$(TESTNET) down -v
	@echo "✅ Testnet stopped"

testnet-logs:
	@$(TESTNET) logs -f --tail=100
