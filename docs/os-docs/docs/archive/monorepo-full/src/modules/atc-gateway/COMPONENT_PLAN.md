# 📋 Komponenten-Plan — atc-gateway

> **Erstellt:** 2026-08-06 | **Agent:** Aurora (MasterBrain · Base44)

## Übersicht

**Repo:** `atc-gateway`
**Name:** ATC Gateway — API-Gateway
**Beschreibung:** API-Gateway mit Middleware-Kette. Circuit-Breaker, Rate-Limiter, Signature-Verify, Auth, Logger, Service-Discovery, Router.
**Layer:** L4 — Gateway
**Sprint:** 3.0
**ATC-Standards:** ATC-24
**Komponenten:** 20

---

## Komponenten-Liste

| # | Datei | Zeilen | Typ | Beschreibung |
|---|-------|--------|-----|-------------|
| 1 | `atclang/middleware/auth.atc` | 82 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 2 | `atclang/middleware/logger.atc` | 70 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 3 | `atclang/middleware/rate_limit.atc` | 50 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 4 | `atclang/middleware/signature_verify.atc` | 43 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 5 | `atclang/router.atc` | 96 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 6 | `gateway.atc` | 138 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 7 | `main.atc` | 127 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 8 | `main.py` | 47 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 9 | `middleware/auth.py` | 19 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 10 | `middleware/logger.py` | 9 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 11 | `middleware/rate_limit.py` | 26 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 12 | `middleware/signature_verify.py` | 57 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 13 | `python/main.atc` | 127 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 14 | `python/main.py` | 47 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 15 | `python/middleware/auth.py` | 19 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 16 | `python/middleware/logger.py` | 9 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 17 | `python/middleware/rate_limit.py` | 26 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 18 | `python/middleware/signature_verify.py` | 57 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 19 | `python/router.py` | 50 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 20 | `router.py` | 50 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |

---

## Detaillierte Komponenten

### 1. `atclang/middleware/auth.atc`

**Datei:** `atclang/middleware/auth.atc`
**Zeilen:** 82
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct AuthConfig, init, require_api_key, add_valid_key, remove_valid_key, set_enabled, get_stats

**Status:** 🟢 IMPLEMENTIERT

---

### 2. `atclang/middleware/logger.atc`

**Datei:** `atclang/middleware/logger.atc`
**Zeilen:** 70
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct LogEntry, init, log_request, get_recent_logs, get_stats, clear_logs

**Status:** 🟢 IMPLEMENTIERT

---

### 3. `atclang/middleware/rate_limit.atc`

**Datei:** `atclang/middleware/rate_limit.atc`
**Zeilen:** 50
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** init, is_allowed, get_blocked_count

**Status:** 🔄 STUB

---

### 4. `atclang/middleware/signature_verify.atc`

**Datei:** `atclang/middleware/signature_verify.atc`
**Zeilen:** 43
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** init, verify_request, verify_and_forward, stats

**Status:** 🔄 STUB

---

### 5. `atclang/router.atc`

**Datei:** `atclang/router.atc`
**Zeilen:** 96
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** init, check_circuit, forward, check_backend, stats

**Status:** 🟢 IMPLEMENTIERT

---

### 6. `gateway.atc`

**Datei:** `gateway.atc`
**Zeilen:** 138
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct Request, struct Response, struct RateBucket, start, stop, handle, _check_rate, _requires_auth (+4 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 7. `main.atc`

**Datei:** `main.atc`
**Zeilen:** 127
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct RouteHandler, init, start, before_request, health, forward, route_public, route_authenticated (+4 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 8. `main.py`

**Datei:** `main.py`
**Zeilen:** 47
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** before, health, proxy

**Status:** 🔄 STUB

---

### 9. `middleware/auth.py`

**Datei:** `middleware/auth.py`
**Zeilen:** 19
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** require_api_key, decorated

**Status:** 🔄 STUB

---

### 10. `middleware/logger.py`

**Datei:** `middleware/logger.py`
**Zeilen:** 9
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** log_request

**Status:** 🔄 STUB

---

### 11. `middleware/rate_limit.py`

**Datei:** `middleware/rate_limit.py`
**Zeilen:** 26
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, is_allowed

**Status:** 🔄 STUB

---

### 12. `middleware/signature_verify.py`

**Datei:** `middleware/signature_verify.py`
**Zeilen:** 57
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** require_signature, send, decorated, verify_request

**Status:** 🟢 IMPLEMENTIERT

---

### 13. `python/main.atc`

**Datei:** `python/main.atc`
**Zeilen:** 127
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct RouteHandler, init, start, before_request, health, forward, route_public, route_authenticated (+4 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 14. `python/main.py`

**Datei:** `python/main.py`
**Zeilen:** 47
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** before, health, proxy

**Status:** 🔄 STUB

---

### 15. `python/middleware/auth.py`

**Datei:** `python/middleware/auth.py`
**Zeilen:** 19
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** require_api_key, decorated

**Status:** 🔄 STUB

---

### 16. `python/middleware/logger.py`

**Datei:** `python/middleware/logger.py`
**Zeilen:** 9
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** log_request

**Status:** 🔄 STUB

---

### 17. `python/middleware/rate_limit.py`

**Datei:** `python/middleware/rate_limit.py`
**Zeilen:** 26
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, is_allowed

**Status:** 🔄 STUB

---

### 18. `python/middleware/signature_verify.py`

**Datei:** `python/middleware/signature_verify.py`
**Zeilen:** 57
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** require_signature, send, decorated, verify_request

**Status:** 🟢 IMPLEMENTIERT

---

### 19. `python/router.py`

**Datei:** `python/router.py`
**Zeilen:** 50
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** get_service_status, forward

**Status:** 🔄 STUB

---

### 20. `router.py`

**Datei:** `router.py`
**Zeilen:** 50
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** get_service_status, forward

**Status:** 🔄 STUB

---

## Test-Strategie

1. Parse-Test: Jede .atc Datei muss mit ATCLang v0.3 Parser parsen
2. Unit-Tests: Mindestens 3 Tests pro Komponente
3. Integration-Test: Komponenten interagieren korrekt
4. Coverage-Ziel: >80%

## Dokumentations-Requirements

- ARCHITECTURE.md: Architektur-Baum + Komponenten-Übersicht ✅
- COMPONENT_PLAN.md: Dieser Plan ✅
- FILE_REGISTER.md: Datei-Liste ✅
- STATUS.md: Aktueller Status ✅
- ROADMAP.md: Sprint-Zuordnung ✅
- CHANGELOG.md: Änderungs-Historie ✅

---
*Auto-generiert 2026-08-06 · Aurora (MasterBrain · Base44)*
