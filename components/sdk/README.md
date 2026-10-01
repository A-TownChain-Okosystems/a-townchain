# Canonical A-TownChain SDK

The SDK is the canonical client/developer interface at `a-townchain/components/sdk`.

The former standalone `atc-sdk` repository is a migration source only. New implementation MUST target this component.

## Boundary

The SDK consumes canonical protocol contracts. It MUST NOT redefine:

- consensus or finality;
- canonical state transitions;
- ATC-VM semantics;
- transaction wire encoding;
- transaction signature domain;
- chain identity.

For transaction construction/signing the SDK MUST conform to the normative protocol/crypto specifications and the canonical L2 validation contract.

## Canonical transaction constants

- chain ID: `658467`
- transaction domain: `ATC-TX-DOMAIN-V2`
- transaction economic amounts: `u128`
- counters such as nonce: `u64`
- transaction authorization: ECDSA secp256k1, RFC6979, low-S
- canonical byte encoding: fixed, deterministic, cross-language tested

## Verification boundary

SDK output is non-authoritative until accepted by the canonical L2/node validation path.

`SDK output != canonical state`.

CI evidence must bind the exact source SHA to the corresponding workflow/run/job/step/log/result.
