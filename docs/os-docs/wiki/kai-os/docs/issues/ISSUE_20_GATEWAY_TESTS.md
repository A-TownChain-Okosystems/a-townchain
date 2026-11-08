# 🧪 Issue #20 — API-Gateway Tests

**Erstellt:** 2026-06-03  
**Version:** v2.1.0  
**Status:** 🔄 In Progress  
**Priorität:** High  

## Übersicht

Vollständige Test-Suite für das A-TownChain API-Gateway (Port 4000).

## Zu testende Komponenten

| Modul | Test-Datei | Status |
|---|---|---|
| `gateway/main.py` | `tests/test_gateway.py` | 🔄 In Progress |
| `gateway/router.py` | `tests/test_gateway.py` | 🔄 In Progress |
| `gateway/middleware/auth.py` | `tests/test_gateway.py` | 🔄 In Progress |
| `gateway/middleware/rate_limit.py` | `tests/test_gateway.py` | 🔄 In Progress |
| `gateway/middleware/logger.py` | `tests/test_gateway.py` | 🔄 In Progress |
| `gateway/middleware/signature_verify.py` | `tests/test_gateway.py` | 🔄 In Progress |

## Test-Cases (15 gesamt)

### Health & Routing
- `test_health_endpoint_exists` — GET /health → 200
- `test_health_response_json` — JSON mit status:ok
- `test_unknown_route_returns_404` — 404 für unbekannte Routen

### Auth-Middleware
- `test_missing_token_rejected` — kein Token → abgelehnt
- `test_invalid_token_rejected` — falscher Token → False
- `test_valid_atc_address_format` — ATC-Adresse Format

### Rate-Limit
- `test_rate_limit_import` — Modul importierbar
- `test_rate_limit_counter_increments` — Counter hochzählen
- `test_rate_limit_blocks_excess` — Überschreitung → gesperrt

### Signature-Verify
- `test_signature_middleware_import` — Modul importierbar
- `test_empty_signature_rejected` — leere Signatur → ungültig

### Router
- `test_router_import` — Router importierbar
- `test_expected_routes_registered` — 4 API-Routen vorhanden
- `test_gateway_port_config` — Port 4000 in Config

### Logger
- `test_logger_import` — Modul importierbar
- `test_logger_captures_request` — Request-Logging

## Ausführen

```bash
python -m unittest tests.test_gateway -v
```

## Akzeptanzkriterien
- [ ] Alle 15 Tests grün
- [ ] Coverage ≥ 80%
- [ ] CI/CD Pipeline integriert
- [ ] Kein bestehender Test gebrochen
