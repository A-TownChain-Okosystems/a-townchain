# 🔌 API Reference — atc-gateway

> **Repo:** [atc-gateway](https://github.com/A-TownChain-Okosystems/atc-gateway)
> **Stand:** 2026-08-06

---

## Öffentliche Funktionen

| # | Funktion | Rückgabe | Datei | Sprache |
|---|----------|----------|------|---------|
| 1 | `init()` | — | `service_discovery.atc` | ATCLang |
| 2 | `register_defaults()` | — | `service_discovery.atc` | ATCLang |
| 3 | `register()` | bool | `service_discovery.atc` | ATCLang |
| 4 | `deregister()` | bool | `service_discovery.atc` | ATCLang |
| 5 | `check_health()` | bool | `service_discovery.atc` | ATCLang |
| 6 | `run_health_cycle()` | — | `service_discovery.atc` | ATCLang |
| 7 | `get_healthy()` | List | `service_discovery.atc` | ATCLang |
| 8 | `get_service()` | ServiceEndpoint | `service_discovery.atc` | ATCLang |
| 9 | `get_url()` | String | `service_discovery.atc` | ATCLang |
| 10 | `update_version()` | bool | `service_discovery.atc` | ATCLang |
| 11 | `stats()` | — | `service_discovery.atc` | ATCLang |
| 12 | `init()` | — | `main.atc` | ATCLang |
| 13 | `start()` | — | `main.atc` | ATCLang |
| 14 | `before_request()` | — | `main.atc` | ATCLang |
| 15 | `health()` | — | `main.atc` | ATCLang |
| 16 | `forward()` | — | `main.atc` | ATCLang |
| 17 | `route_public()` | — | `main.atc` | ATCLang |
| 18 | `route_authenticated()` | — | `main.atc` | ATCLang |
| 19 | `route_admin()` | — | `main.atc` | ATCLang |
| 20 | `route_signed()` | — | `main.atc` | ATCLang |
| 21 | `stats()` | — | `main.atc` | ATCLang |
| 22 | `stop()` | — | `main.atc` | ATCLang |
| 23 | `start()` | — | `gateway.atc` | ATCLang |
| 24 | `stop()` | — | `gateway.atc` | ATCLang |
| 25 | `handle()` | Response | `gateway.atc` | ATCLang |
| 26 | `_check_rate()` | bool | `gateway.atc` | ATCLang |
| 27 | `_requires_auth()` | bool | `gateway.atc` | ATCLang |
| 28 | `_validate_key()` | bool | `gateway.atc` | ATCLang |
| 29 | `_verify_tx_signature()` | bool | `gateway.atc` | ATCLang |
| 30 | `_proxy()` | Response | `gateway.atc` | ATCLang |
| 31 | `stats()` | Map | `gateway.atc` | ATCLang |
| 32 | `init()` | — | `atclang/router.atc` | ATCLang |
| 33 | `check_circuit()` | bool | `atclang/router.atc` | ATCLang |
| 34 | `forward()` | — | `atclang/router.atc` | ATCLang |
| 35 | `check_backend()` | — | `atclang/router.atc` | ATCLang |
| 36 | `stats()` | — | `atclang/router.atc` | ATCLang |
| 37 | `init()` | — | `atclang/main.atc` | ATCLang |
| 38 | `register_default_routes()` | — | `atclang/main.atc` | ATCLang |
| 39 | `register_route()` | bool | `atclang/main.atc` | ATCLang |
| 40 | `route_request()` | — | `atclang/main.atc` | ATCLang |
| 41 | `check_rate_limit()` | bool | `atclang/main.atc` | ATCLang |
| 42 | `is_circuit_open()` | bool | `atclang/main.atc` | ATCLang |
| 43 | `record_failure()` | bool | `atclang/main.atc` | ATCLang |
| 44 | `record_success()` | — | `atclang/main.atc` | ATCLang |
| 45 | `get_stats()` | — | `atclang/main.atc` | ATCLang |
| 46 | `get_middleware()` | List | `atclang/main.atc` | ATCLang |
| 47 | `init()` | — | `atclang/middleware/rate_limit.atc` | ATCLang |
| 48 | `is_allowed()` | bool | `atclang/middleware/rate_limit.atc` | ATCLang |
| 49 | `get_blocked_count()` | u64 | `atclang/middleware/rate_limit.atc` | ATCLang |
| 50 | `init()` | — | `atclang/middleware/logger.atc` | ATCLang |
| 51 | `log_request()` | bool | `atclang/middleware/logger.atc` | ATCLang |
| 52 | `get_recent_logs()` | u64 | `atclang/middleware/logger.atc` | ATCLang |
| 53 | `get_stats()` | — | `atclang/middleware/logger.atc` | ATCLang |
| 54 | `clear_logs()` | bool | `atclang/middleware/logger.atc` | ATCLang |
| 55 | `init()` | — | `atclang/middleware/signature_verify.atc` | ATCLang |
| 56 | `verify_request()` | bool | `atclang/middleware/signature_verify.atc` | ATCLang |
| 57 | `verify_and_forward()` | — | `atclang/middleware/signature_verify.atc` | ATCLang |
| 58 | `stats()` | — | `atclang/middleware/signature_verify.atc` | ATCLang |
| 59 | `init()` | — | `atclang/middleware/auth.atc` | ATCLang |
| 60 | `require_api_key()` | bool | `atclang/middleware/auth.atc` | ATCLang |

*+54 weitere*

**Total: 114 Funktionen**

---

*Auto-generiert 2026-08-06 · Aurora*
