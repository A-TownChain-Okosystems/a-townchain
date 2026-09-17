---
document_id: ATC-DOC-INDEXER-006
title: "Technical Architecture"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-07
standard: ATC-STD-MD-001
---

# Technical Architecture — ATC Chain Indexer & Analytics

## Overview

ATC Indexer verarbeitet Blockchain-Events für Auswertungen und Explorer-Interfaces.

## Components

| Component | Purpose | Required |
|---|---|---|
| `atc-analytics` | Analytics Core Module | Yes |
| `pipelines` | ETL Ingestion Pipelines | Yes |
| `metrics` | Aggregations-Engine | Yes |

## Data Flow

```text
Node -> Event Ingest -> ETL -> Analytics DB -> Dashboards
```
