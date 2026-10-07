# ATC-MONETARY-POLICY-001 — A-TownChain Monetary Policy

Status: DRAFT / implementation baseline
Owner: atc-algorithm
Scope: L1 native ATC issuance
Chain ID: 658467

## Normative parameters

| Parameter | Value |
|---|---:|
| Maximum supply | 360,000,000 ATC |
| Base unit | 10^18 |
| Target block time | 360 seconds |
| Halving interval | 360,000 blocks |
| Halvings | 36 |
| First halving | block 360,000 |
| Final emission block | 12,959,999 |
| Subsidy from block 12,960,000 | 0 ATC |
| Initial subsidy | 500 ATC/block |

## Consensus rules

1. Halving is determined exclusively from block_height.
2. timestamp, wall-clock time, timezone and local clock state MUST NOT affect issuance.
3. Epoch is floor(block_height / 360,000).
4. Epochs 0..35 have non-zero subsidy; epoch 36+ has zero subsidy.
5. Raw subsidy is 500 ATC / 2^epoch, represented in the 10^18 base unit and rounded down.
6. The final emission block may receive the remaining integer-denominated supply, but never more than MAX_SUPPLY.
7. Total native issuance MUST satisfy issued <= 360,000,000 ATC.
8. Transaction fees are separate from issuance.


## Block-time semantics

360 seconds is the target block interval for network operation. It is not used as an issuance trigger.
One 360,000-block epoch represents 129,600,000 target seconds (1,500 days), about 4.11 years.
Thirty-six halving intervals span 12,960,000 blocks and about 147.95 target years.


## Integration contract

- atc-algorithm/src/economics.rs is the canonical implementation.
- a-townchain MUST consume these parameters and MUST NOT define a competing monetary policy.
- atc-mining MUST consume the canonical subsidy rule.
- Genesis configuration MUST NOT pre-mint supply outside the 360,000,000 ATC cap.
- Wallet, explorer, indexer and state layers MUST treat issued supply as consensus-derived state.


## Verification vectors

| Height | Epoch | Raw subsidy |
|---:|---:|---:|
| 0 | 0 | 500 ATC |
| 359,999 | 0 | 500 ATC |
| 360,000 | 1 | 250 ATC |
| 720,000 | 2 | 125 ATC |
| 1,080,000 | 3 | 62.5 ATC |
| 12,600,000 | 35 | ~0.00000001455 ATC |
| 12,960,000 | 36 | 0 ATC |

The final emission block is 12,959,999. The remainder rule exists only to close integer rounding dust exactly.


## Security properties

- deterministic across nodes
- independent of local wall-clock time
- checked arithmetic / bounded issuance
- explicit off-by-one boundaries
- no reward authority in AI or telemetry components

Implementation note: consensus code is validated by the repository test suite before merge.
