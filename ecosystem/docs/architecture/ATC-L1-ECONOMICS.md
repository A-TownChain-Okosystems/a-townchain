# A-TownChain L1 Economics

## Canonical native coin

- Symbol: **ATC**
- Layer: **A-TownChain L1**
- Maximum lifetime supply: **360,000,000 ATC**
- Post-genesis protocol minting: **disabled**
- Supply accounting: balances + staked balances
- Supply is committed into the deterministic L1 state root.

## Consensus invariant

The 360,000,000 ATC ceiling is implemented in the canonical Rust L1 state machine, not only in documentation.

Genesis allocations must:

1. be created through the genesis allocation API;
2. keep total supply at or below 360,000,000 ATC;
3. be sealed when genesis is created;
4. be included in the state-root commitment.

After genesis, the state machine has no protocol mint operation. Transfers and staking only move existing ATC between balance/stake positions. Transaction fees reduce circulating state balance under the current execution model.

## L2 relationship

ATC is the native L1 settlement asset. The ATC-L2 architecture may use ATC as its settlement/gas asset, but L2 token issuance remains governed by the L2 protocol and does not create additional L1 ATC.

## Allocation

This change defines the **maximum supply**, not an allocation schedule. Genesis allocations must be specified explicitly in the genesis configuration before mainnet. No unrequested treasury, validator, founder, community, or reward allocation is assumed by this implementation.
