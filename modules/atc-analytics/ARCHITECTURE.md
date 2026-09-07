# ARCHITECTURE.md — atc-analytics
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
├── benchmarks/
│   └── benchmark_center.atc
├── chain/
│   └── chain_analytics.atc
├── github/
│   └── github_analytics.atc
├── metrics/
│   └── metrics_engine.atc
├── package.json
├── pipelines/
│   └── bigquery_pipeline.atc
├── reports/
│   └── report_generator.atc
├── src/
│   ├── api/
│   ├── dashboard/
│   ├── index.ts
│   ├── indexer/
│   └── storage/
└── tsconfig.json
```

## Module Descriptions
- **src/indexer/**: High-throughput blockchain event indexer processing blocks, logs, and transactions into structured data.
- **src/api/**: GraphQL and REST API server providing query endpoints for analytics data.
- **src/storage/**: Persistent storage integration layer supporting ClickHouse, TimescaleDB, and BigQuery.
- **src/dashboard/**: Interactive analytics dashboard rendering network metric charts and throughput statistics.
- **package.json** & **tsconfig.json**: Node project configuration and TypeScript settings.

## Build System
Node.js / npm environment using TypeScript compiler (`tsc`) and Vite / Express build pipeline.

## Dependencies
TypeScript, Fastify / Express, PostgreSQL / ClickHouse / BigQuery client drivers, Chart.js / D3.js.
