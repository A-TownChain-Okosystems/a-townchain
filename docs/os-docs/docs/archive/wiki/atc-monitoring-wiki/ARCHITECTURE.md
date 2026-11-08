# ARCHITECTURE.md — atc-monitoring
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
├── .gitignore
├── CHANGELOG.md
├── COMPONENT_PLAN.md
├── FILE_REGISTER.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── STATUS.md
├── alerts/
│   ├── alert_manager.atc
│   └── blockchain_alerts.atc
├── exporters/
│   ├── chain_exporter.atc
│   └── node_exporter.atc
├── health/
│   ├── health_check.atc
│   └── uptime_monitor.atc
├── metrics/
│   ├── collector.atc
│   ├── grafana_dashboard.atc
│   └── prometheus_exporter.atc
├── requirements.txt
└── src/
    ├── __init__.py
    ├── alerting/
    │   └── __init__.py
    ├── collectors/
    │   └── __init__.py
    ├── dashboard/
    │   └── __init__.py
    └── exporters/
        └── __init__.py
```

## Module Descriptions
- **src/collectors/**: Metrics collection drivers polling blockchain nodes, OS resource utilization, and network peer connectivity.
- **src/exporters/**: Telemetry exporters translating internal node metrics into Prometheus metrics endpoints.
- **src/alerting/**: Alert engine processing threshold rules and dispatching notifications to Slack, PagerDuty, or email.
- **src/dashboard/**: Web server serving live telemetry dashboard views and graph visualizers.
- **requirements.txt**: Python dependency manifest for monitoring service framework.

## Build System
Python 3.10+ packaging (`setuptools` / `pip`), Docker containerization for background service deployment.

## Dependencies
Python 3.10+, `prometheus_client`, `requests`, `fastapi` / `flask`, `pydantic`, `uvicorn`.
