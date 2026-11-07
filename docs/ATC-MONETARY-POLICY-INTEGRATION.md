# A-TownChain Monetary Policy Integration

This repository is the orchestration layer. The migrated algorithm component is located at `components/algorithm`; final capability ownership and normative monetary-policy authority remain subject to SCR-0131 and owner/§9 approval.

## Bound parameters

- Maximum supply: 360,000,000 ATC
- Target block time: 360 seconds
- Halving interval: 360,000 blocks
- Halvings: 36
- First halving: block 360,000
- Final emission block: 12,959,999
- Subsidy at block 12,960,000 and later: 0
- Initial subsidy: 500 ATC/block
- Native genesis supply: 0 ATC

## Integration rule

`modules/atc-blockchain/consensus/monetary_policy_binding.atc` exposes the integration constants for the orchestration layer. It MUST NOT become an independent consensus authority.

`modules/atc-blockchain/mainnet/mainnet_config.atc` and `launch_manager.atc` now use the same block time, cap and halving parameters.

The Rust implementation currently resides in `components/algorithm`. Any future monetary-policy change MUST follow the approved normative specification and ownership decision first, then update the implementation, this binding and conformance vectors.

## Current validation status

- The configuration conflict with the old 10-second block target and 21M genesis supply has been corrected on this branch.
- Legacy consensus code still contains prototype logic; it must not be treated as canonical.
- End-to-end node integration remains dependent on the planned `atc-algorithm` dependency refactor.