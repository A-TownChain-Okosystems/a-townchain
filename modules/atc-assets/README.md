# atc-assets

Asset-Management für das A-TownChain-Ökosystem.

## Features (geplant)
- Fungible Tokens (ATC-Standard, Mint, Burn, Transfer)
- Non-Fungible Tokens (NFTs, Metadata, Royalties)
- Semi-Fungible Tokens (Multi-Token, Batches)
- Asset-Registry (On-Chain, Off-Chain Metadata)
- Royalty-Distribution (Primary, Secondary Sales)
- Asset-Collections (Series, Editions)
- Fractional-Ownership (Tokenized Assets)

## Architektur
```
atc-assets/
├── src/
│   ├── lib.rs
│   ├── fungible.rs       # Fungible Tokens
│   ├── nft.rs            # Non-Fungible Tokens
│   ├── registry.rs       # Asset-Registry
│   └── royalty.rs        # Royalty-Distribution
├── Cargo.toml            # x86_64-unknown-none (no_std)
└── tests/
```


## Abhängigkeiten
- [`A-TownChain-Okosystems/atc-shivacore`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-shivacore)

## Copyright
Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.
