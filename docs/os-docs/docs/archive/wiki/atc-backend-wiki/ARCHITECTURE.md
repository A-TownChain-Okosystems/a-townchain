# ARCHITECTURE.md — atc-backend

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-backend/
├── requirements.txt — Python package dependency specifications
├── README.md — Service overview and deployment documentation
├── tests/ — Automated integration and unit test suite
└── src/
    ├── __init__.py — Package initialization module
    ├── routes/ — API route handlers (auth, users, nodes, transactions, analytics)
    ├── models/ — Database ORM models and Pydantic validation schemas
    ├── services/ — Business logic services (node connection, indexer, notifications)
    └── middleware/ — Authentication middleware, CORS, rate limiting, and request logging
```

## Module Descriptions
- src/routes/ — REST endpoints for user authentication, node health monitoring, and transaction querying.
- src/models/ — Database schema declarations and input validation models using Pydantic.
- src/services/ — Core business workflows, blockchain node interaction, and indexing engines.
- src/middleware/ — Request processing pipeline including JWT token authentication and CORS policy enforcement.

## Build System
- Python 3.11 service configured via `requirements.txt`.

## Dependencies
- fastapi — Asynchronous Web framework for API endpoint handlers.
- uvicorn — High-performance ASGI server.
- sqlalchemy — Database Object-Relational Mapping (ORM) layer.
- pydantic — Data validation and settings management using Python type annotations.
