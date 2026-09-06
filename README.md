# A-TownChain — Blockchain

> **Produkt-Repo des A-TownChain-Ökosystems** · [Monorepo](https://github.com/A-TownChain-Okosystems/a-townchain-os) · [Docs-Hub](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs) · Mainnet: **15.09.2026**

Layer-1-Blockchain des Ökosystems: hybrides ShivaConsensus (PoW SHA3-ATC + reputations-gewichtetes PoS + PoH-VDF), Chain-ID 658467, ATC-8300 Token-Standard, ATC-9000 NFTs, DEX, Cross-Chain-Bridge, Governance. Mainnet-Launch: 15.09.2026.

## Module (aus Monorepo `src/modules/` überführt)

| Modul | Dateien | Zeilen |
|---|---|---|
| `atc-blockchain` | 80 | 7,642 |
| `atcnet` | 32 | 3,893 |
| `atc-wallet` | 29 | 724 |
| `atc-contracts` | 39 | 4,504 |
| `atc-bridge` | 26 | 750 |
| `atc-dex` | 24 | 579 |
| `atc-explorer` | 24 | 596 |
| `atc-assets` | 22 | 528 |
| `atc-zkp` | 27 | 602 |
| `atc-governance` | 22 | 693 |
| `atc-dns` | 20 | 583 |
| `atc-testnet` | 24 | 510 |
| **Total** | **369** | **21,604** |

## Richtlinien

- Architektur-Vorgaben: AD-012/AD-013 (ShivaCore Microkernel, Gate v1.1) — siehe Docs-Hub `docs/architecture/`
- Neue Produkt-Entwicklung läuft hier; Integration & Deployment über das Monorepo
- Standards: ATC-01…35 · ATS-1000…1007 · Lizenz: All Rights Reserved (Michael Wroblewski / ShivaCore / A-TownChain-Okosystems)

*Eingerichtet am 06.09.2026 durch Agent Aurora (Base44) im Auftrag des Owners.*
