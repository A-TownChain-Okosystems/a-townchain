# ARCHITECTURE.md — atc-ci
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
├── .github/
│   └── workflows/
├── .gitignore
├── CHANGELOG.md
├── COMPONENT_PLAN.md
├── FILE_REGISTER.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── STATUS.md
├── deploy.atc
├── generate_validators.atc
├── health_checks_atc08.atc
├── lint.atc
├── monitor.atc
├── package.json
├── pipeline.atc
├── prometheus.yml
├── prometheus_metrics.atc
├── src/
│   └── components/
│       ├── CiCdPipelineView.tsx
│       └── DeploymentPipelineWidget.tsx
└── test_runner.atc
```

## Module Descriptions
- **.github/workflows/**: CI/CD workflow YAML pipeline files executing automated unit tests, linter checks, and deployment steps.
- **src/**: Rust / TypeScript CI helper binaries and scripts for network health checking, validator key generation, and test execution.
- **package.json**: Node package manifest for CI scripts, health check utilities, and monitoring wrappers.
- **prometheus.yml**: Prometheus configuration for CI runner performance and test execution telemetry.

## Build System
GitHub Actions workflow runner, Cargo for Rust helper tool compilation, npm for JS/TS pipeline scripts.

## Dependencies
GitHub Actions Runner, Rust toolchain, Node.js 18+, Prometheus server, Docker.
