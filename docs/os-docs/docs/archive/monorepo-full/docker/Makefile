# A-TownChain OS — Docker Testnet Makefile
# Usage: make testnet-up | testnet-down | testnet-logs | testnet-status

.PHONY: testnet-up testnet-down testnet-logs testnet-status testnet-clean test lint

COMPOSE = docker compose -f docker/docker-compose.testnet.yml

testnet-up:
	@echo "🚀 Starting A-TownChain Testnet (3 nodes)..."
	$(COMPOSE) up -d
	@echo "✅ Testnet running — Gateway: http://localhost:4000"
	@echo "   Node 0 RPC: http://localhost:9100"
	@echo "   Node 1 RPC: http://localhost:9101"
	@echo "   Node 2 RPC: http://localhost:9102"
	@echo "   Grafana:    http://localhost:3000"
	@echo "   Prometheus: http://localhost:9090"

testnet-down:
	@echo "🛑 Stopping testnet..."
	$(COMPOSE) down -v
	@echo "✅ Testnet stopped"

testnet-logs:
	$(COMPOSE) logs -f --tail=100

testnet-status:
	$(COMPOSE) ps

testnet-clean:
	$(COMPOSE) down -v --remove-orphans
	docker system prune -f

test:
	python -m pytest tests/ -v --tb=short --timeout=30 -k "not testnet"

lint:
	python -m flake8 modules/ blockchain/ gateway/ monitoring/ atclang/ --max-line-length=120 || true

health:
	@curl -sf http://localhost:4000/health && echo "✅ Gateway OK" || echo "❌ Gateway DOWN"
	@curl -sf http://localhost:9100/health && echo "✅ Node-0 RPC OK" || echo "❌ Node-0 DOWN"
	@curl -sf http://localhost:9101/health && echo "✅ Node-1 RPC OK" || echo "❌ Node-1 DOWN"
	@curl -sf http://localhost:9102/health && echo "✅ Node-2 RPC OK" || echo "❌ Node-2 DOWN"
