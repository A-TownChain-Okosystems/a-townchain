# 🏛️ Architektur — atc-gateway

> **Repo:** [atc-gateway](https://github.com/A-TownChain-Okosystems/atc-gateway)
> **Layer:** L7 | **Titel:** API Gateway
> **Stand:** 2026-08-06 | **Version:** v1.0.0

---

## Übersicht

API Gateway mit Circuit Breaker, Rate Limiting, Middleware-Kette, Service Discovery.

## Komponenten

### ATCLang Module (.atc)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `atclang/main.atc` | 180 | Main |
| `atclang/middleware/auth.atc` | 82 | Auth |
| `atclang/middleware/logger.atc` | 70 | Logger |
| `atclang/middleware/rate_limit.atc` | 50 | Rate Limit |
| `atclang/middleware/signature_verify.atc` | 43 | Signature Verify |
| `atclang/router.atc` | 96 | Router |
| `gateway.atc` | 138 | Gateway |
| `main.atc` | 127 | Main |
| `python/main.atc` | 127 | Main |
| `python/service_discovery.atc` | 168 | Service Discovery |
| `service_discovery.atc` | 168 | Service Discovery |

### Python Module (.py)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `main.py` | 47 | Main |
| `middleware/auth.py` | 19 | Auth |
| `middleware/logger.py` | 9 | Logger |
| `middleware/rate_limit.py` | 26 | Rate Limit |
| `middleware/signature_verify.py` | 57 | Signature Verify |
| `python/main.py` | 47 | Main |
| `python/middleware/auth.py` | 19 | Auth |
| `python/middleware/logger.py` | 9 | Logger |
| `python/middleware/rate_limit.py` | 26 | Rate Limit |
| `python/middleware/signature_verify.py` | 57 | Signature Verify |
| `python/router.py` | 50 | Router |
| `router.py` | 50 | Router |

## Statistik

| Metrik | Wert |
|--------|------|
| .atc | 11 |
| .py | 12 |
| .rs | 0 |
| .ts | 0 |
| Total Zeilen | 1,665 |

---

*Auto-generiert 2026-08-06 · Aurora*
