# ARCHITECTURE.md — atc-gateway

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-gateway/
├── requirements.txt — Python service dependency specifications
├── README.md — Gateway architecture and configuration guide
├── docs/ — OpenAPI specs and route reference documentation
├── atclang/ — Custom domain-specific language for dynamic route policy rules
└── python/
    ├── routes/ — Gateway reverse proxy routes and load balancing logic
    ├── middleware/ — Rate limiting, signature verification, and DDoS protection
    └── service_discovery/ — Dynamic backend service registration and health probes
```

## Module Descriptions
- python/routes/ — External API ingress routes and intelligent request proxying.
- python/middleware/ — Request sanitization, client signature validation, and rate limiting.
- python/service_discovery/ — Automatic registration and active health probing of backend nodes.
- atclang/ — Policy engine for defining routing, rate limits, and access rules.

## Build System
- Python 3.11 API Gateway application configured via `requirements.txt`.

## Dependencies
- fastapi — High-speed API framework for gateway routes.
- httpx — Asynchronous HTTP client for proxying requests to internal microservices.
- redis — Distributed rate limiting cache and session state store.
