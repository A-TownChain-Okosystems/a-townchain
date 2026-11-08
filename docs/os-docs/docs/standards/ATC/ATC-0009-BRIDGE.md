# ATC-0009 — Cross-Chain Bridge Standard

## Status: DRAFT | Version: 0.1.0 | Datum: 2026-06-09

## Abstract
ATC-0009 definiert den Standard für Cross-Chain Asset-Transfers
im A-TownChain Ökosystem. Implementiert via Lock-Mint / Burn-Release.

## Motivation
ATC-Assets sollen auf EVM-kompatiblen Chains (ETH, BSC, Polygon)
nutzbar sein, ohne die Sicherheit der ATC-Hauptkette zu gefährden.

## Spezifikation

### 1. Unterstützte Chains
- `atc-mainnet-1` — A-TownChain Hauptkette
- `eth-mainnet` — Ethereum
- `bsc-mainnet` — Binance Smart Chain
- `polygon-mainnet` — Polygon

### 2. Protokoll: Lock-Mint

```
User → Lock(amount, ATC) → BridgeContract
BridgeContract → Event(BridgeLocked)
RelayNode → observe Event
RelayNode → Mint(amount, wATC) auf Ziel-Chain
```

### 3. Protokoll: Burn-Release

```
User → Burn(amount, wATC) auf Ziel-Chain
RelayNode → observe Burn
RelayNode → Release(amount, ATC) auf ATC-Chain
```

### 4. Gebühren
- Bridge-Gebühr: 0.1% des Betrags
- Mindestbetrag: 10 ATC
- Maximalbetrag: 1.000.000 ATC

### 5. Sicherheit
- L0-S3 konform (ATC-Chain-Isolierung)
- 3-of-5 Multisig für Relay-Nodes
- 24h Timelock für große Transfers (> 10.000 ATC)
- Pausierbar durch Owner (Notfall)

## Referenz-Implementierung
`atc-contracts/bridge/bridge_contract.py`

## Status
- [ ] Audit durch L0-Security-Team
- [ ] Testnet-Deployment
- [ ] Mainnet-Aktivierung nach MK3-Gate
