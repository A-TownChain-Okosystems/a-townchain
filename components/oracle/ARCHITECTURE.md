---
document_id: ATC-DOC-ORACLE-006
title: "Technical Architecture"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-07
standard: ATC-STD-MD-001
---

# Technical Architecture — ATC Oracle Services

## Overview

ATC Oracle Services verbindet externe Datenquellen und Preissignale mit ATVM Smart Contracts.

## Components

| Component | Purpose | Required |
|---|---|---|
| `price/` | Preisfutter Aggregation | Yes |
| `external-data/` | Blockchain- & API-Anbindung | Yes |
| `sc-016-binding` | SC-016 Standard Protocol Binding | Yes |

## Data Flow

```text
External Feeds -> Median Filter & Security Attestation -> SC-016 Contract -> ATVM State
```
