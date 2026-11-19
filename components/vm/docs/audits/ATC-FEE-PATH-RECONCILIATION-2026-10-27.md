# ATC Fee Path Reconciliation — 2026-10-27

Status: RESIDUAL / BLOCKED — source audit only; no consensus fee-settlement implementation is claimed.

## Source identity

- Repository: `A-TownChain-Okosystems/a-townchain`
- Audited base commit: `55b062b01acb35a970aa41c1934e3f3228857939`
- `components/vm/src/lib.rs`, blob `a5e0ff2c06a5d8dc167d0d4730868cf40d85347c`
- `components/vm/src/vm.rs`, blob `57a919d911ae7900f740bab739bb20fabf0d4313`
- `components/vm/src/context.rs`, blob `ef0a0cee2ff6a73adc19870a75fc1ed5e00ac7bc`
- `components/vm/Cargo.toml`, blob `3bc54580a25d6295922d453d6426cdace0742fe8`
- `components/vm/tests/exec_chain_e2e.rs`, blob `129122feb41a1af630a59a0f04c2cf679853bc27`
- `components/wallet/src/tx.rs`, blob `2693b7ba015456addcc8d129455b7e5a9e2f1860`
- Legacy candidate: `modules/atc-blockchain/consensus/gas_fee_atc86.atc`, base blob `31f17a490cd0f9899ff83543b462849a5a1b8a06`

## Findings

1. The canonical VM crate exports `vm`, `ops`, and `context`; it has no fee-settlement module in its current source tree.
2. `vm.rs` meters opcodes using a local `gas_cost` function and accepts a caller-supplied `gas_limit`. `OutOfGas` is an execution error. This is execution metering, not transaction fee charging.
3. The VM state-transition wrapper validates chain/network/genesis/protocol/VM identity and runs the interpreter. The VM does not debit sender balances, credit a fee recipient, settle unused gas reservations, split fee proceeds, or distribute holder rewards.
4. The wallet transaction currently serializes `gas_price: u64` and `gas_limit: u64`. The legacy ATC-86 proposal instead uses `max_fee`, `priority_fee`, and `gas_used` as `UInt256`. The relationship between these transaction models is not yet specified or proven byte-/behavior-compatible.
5. The legacy ATC-86 arithmetic uses multiplication and addition for fee calculation and block-gas aggregation without visible checked-arithmetic error paths. Refund correction alone does not establish overflow safety or transaction validity.
6. Existing VM E2E tests cover interpreter execution and simple storage transfers; they do not prove transaction fee debit/settlement, dynamic base-fee transition, holder reward accounting, reorg rollback, or cross-node replay equivalence.

## Required implementation boundary

Do not wire the legacy ATC-86 module directly into consensus as an assumed implementation. First approve one normative transaction-fee contract that specifies:

- fee fields and widths, including whether `gas_price` remains the signed V2 field or is replaced/extended by `max_fee_per_gas` and `priority_fee_per_gas`;
- checked arithmetic and explicit rejection for invalid fee caps, `gas_used > gas_limit`, invalid base-fee configuration, and overflow;
- reservation, actual charge, unused-gas refund, and fee-recipient accounting as one atomic state transition;
- deterministic base-fee update order and bounds;
- fee allocation policy (holder/validator/reserve/burn), eligible-balance snapshot, duplicate-payout prevention, and same-block finality/reorg semantics;
- identical Rust/TypeScript/wallet vectors and deterministic multi-node replay tests.

## Gate decision

- Source audit: TESTED by inspection only; no executable tests run as part of this audit.
- Canonical VM fee settlement: BLOCKED.
- Dynamic fee market: BLOCKED.
- Same-block holder rewards: BLOCKED.
- Reorg-safe reward/fee rollback: BLOCKED.
- Merge/release: NOT AUTHORIZED by this document.
