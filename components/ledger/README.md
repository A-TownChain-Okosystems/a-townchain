# ATC Ledger

Canonical deterministic L1 ledger-state transition crate.

## Ownership

`atc-ledger` owns:

- account balance and nonce state;
- deterministic transfer application;
- checked `u128` economic arithmetic;
- checked `u64` nonce and journal sequencing;
- atomic transaction application;
- deterministic journal ordering.

It does **not** own:

- transaction signing or wire encoding;
- consensus;
- VM execution;
- persistent storage;
- state-root hashing.

## Contract

Economic amounts use `u128`. Nonces and journal sequence numbers use `u64`.

A transfer debits `amount + fee` from the sender and credits `amount` to the recipient. Fees are represented explicitly in the journal; their final accounting destination is a separate protocol contract and is not invented by this crate.

Every fallible calculation completes before state mutation. A rejected transfer leaves the ledger unchanged.

## Specification status

The existing `ATC-STATE-001` document is currently a draft and explicitly states that its data models are pending spec freeze. Therefore this crate is an implementation baseline, not a claim that the state-transition specification has been frozen or that the ledger is consensus-verified.

State-root/Merkle semantics remain blocked until the canonical hashing/state-root contract is frozen.

## Verification

Run from the repository root:

`cargo test --manifest-path components/ledger/Cargo.toml`

CI evidence must be tied to the exact commit SHA before this module is considered VERIFIED.
