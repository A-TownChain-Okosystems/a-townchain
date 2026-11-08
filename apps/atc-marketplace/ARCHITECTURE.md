---
document_id: ATC-DOC-ARC-MRKT-001
title: Repository Architecture Specification
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-13
updated: 2026-09-13
standard: ATC-STD-MD-001
---

# Architecture Specification — atc-marketplace

## Übersicht

`atc-marketplace` (Layer L5) vereint DEX und NFT-/Asset-Registry (ATC-9000). Vault-restauriert (07.09.2026, AD-020/026/027); Module `atc-dex` und `atc-assets`. Meile: M6.

## Subsysteme

1. **atc-dex (`modules/`):** Dezentrale Börse — Orderbuch/AMM, Token-Swaps gegen die Chain.
2. **atc-assets:** NFT-/Asset-Registry (ATC-9000-Namensraum) — Minting, Ownership, Transfer.
3. **Wallet-Anbindung:** Integration mit `atc-wallet` (Key-Verwaltung kanonisch dort).

## Verantwortungsgrenzen

- `atc-wallet`: Key-/Wallet-Logik ist kanonisch dort, nicht hier.
- `atc-vm`: Contract-Ausführung der Marketplace-Contracts.
- Chain-Endgültigkeit: Konsens liegt in `atc-algorithm`/`a-townchain`.

## Registry-Einordnung

| Property | Value |
|---|---|
| Layer | L6 |
| Criticality | C2 |
| Security-Klasse | S3 |
| Maturity | R-Level laut `.atc/repository.yaml` · Statusleiter in `.atc/evidence/evidence.yaml` (SCR-0080) |
| Canonical | atc-marketplace (DEX + Asset-Registry) |
| Domäne | domaene laut registry/repositories.yaml |

> Ehrlichkeitsregel: CLAIMED ≠ PASS · IMPLEMENTED ≠ VERIFIED — der verbindliche Implementierungsstand
> liegt ausschließlich in `.atc/evidence/evidence.yaml`, nicht in dieser Spezifikation.
