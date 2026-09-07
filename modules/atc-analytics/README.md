# atc-analytics

Analytics-Plattform für das A-TownChain-Ökosystem.

## Features (geplant)
- On-Chain Analytics (Transaction-Volume, Active-Addresses, Gas-Usage)
- Block-Indexer (Real-Time, Historical)
- Data-Lake (S3-kompatibel, Parquet-Format)
- GraphQL-API (Flexible Queries)
- Dashboard (TVL, Volume, Metrics)
- Event-Streaming (WebSocket, Kafka)
- Custom-Metrics (User-Defined Queries)

## Architektur
```
atc-analytics/
├── src/
│   ├── indexer/           # Block-Indexer
│   ├── api/               # GraphQL-API
│   ├── storage/           # Data-Lake
│   └── dashboard/        # Web-Dashboard
├── package.json
├── tsconfig.json
└── tests/
```


## Abhängigkeiten
- [`A-TownChain-Okosystems/atc-blockchain`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-blockchain)

## Copyright
Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.
