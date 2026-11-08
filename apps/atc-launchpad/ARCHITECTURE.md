---
document_id: ATC-DOC-LPD-ARCH-001
title: Architecture — ATC Launchpad
status: active
standard: ATC-STD-MD-001
owner: A-TownChain-Okosystems
version: 1.0.0
created: 2026-09-08
updated: 2026-09-08
---

# Architecture — ATC Launchpad

## Components

- **Sales Module:** Token-Presales, Fair-Launches und Direct Token Distribution.
- **Vesting & Whitelist Module:** Zeitgestaffelte Token-Freischaltungen und Inhaber-Verifizierung.
- **Integration Layer:** Schnittstellen zu `atc-contracts` und `atc-wallet`.

## Data Flow

```text
[User / Developer] -> [Launchpad UI / SDK] -> [Sales & Whitelist Module]
                                                     |
                                                     v
                                           [atc-contracts / ATVM]
                                                     |
                                                     v
                                          [atc-node / Blockchain]
```

## Dependencies

| Component | Purpose | Required |
|---|---|---|
| atc-contracts | Smart-Contract-Standards & Token-Vorlagen | Ja |
| atc-wallet | Wallet-Verbindung & Transaktionssignierung | Ja |
| atc-node | Blockchain-Interaktion & RPC-Node | Ja |
