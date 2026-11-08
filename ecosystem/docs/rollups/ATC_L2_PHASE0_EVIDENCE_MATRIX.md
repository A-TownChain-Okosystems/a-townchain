# ATC-L2 Rollup — Phase 0 Evidence Matrix

Status: **BLOCKED — feasibility inventory in progress**

This file is an evidence register, not an implementation claim. The ecosystem repository owns architecture, catalog, integration, and evidence coordination; canonical runtime implementation belongs in the relevant source repository.

## Decision boundary

Target architecture: ATC-L2 as a rollup settling to A-TownChain L1.

**Normative proof-model constraint:** `ATC-STD-ZKP-008 v1.1.0` is marked approved/normative and explicitly requires cryptographic validity proofs for state-root transitions. Therefore the current standards baseline points to a **validity-proof (ZK) rollup**, not an unconstrained choice between optimistic and validity proofs. Any change to that direction requires a formal standards change. However, the implementation is not ready to proceed to production: the inspected ZKP verifier/SNARK source files are explicitly planned/not implemented.

Still unresolved:
- Batch commitment format and canonical state-root semantics
- Data-availability mechanism and retention guarantees
- Sequencer failure handling, forced inclusion, and censorship recovery
- Withdrawal finality and invalid-proof rejection rules
- L1 integration point and deterministic verification execution

No production bridge transfers or security guarantees may be enabled or claimed until these gates have evidence.

## Source-backed observations

| Area | Source path | Blob SHA | Observation | Status |
|---|---|---|---|---|
| ZKP crate dependencies | `a-townchain/modules/atc-zkp/Cargo.toml` | `d0ee2f0bfc1f922e4ee6b71821f89b49384e5297` | Cargo `[dependencies]` is empty in the inspected file. | BLOCKED |
| ZKP verifier | `a-townchain/modules/atc-zkp/src/verifier.rs` | `808882b1d0492690abea0652e79f9bc2363d3d49` | Source explicitly labels the module planned / not implemented. | BLOCKED |
| SNARK module | `a-townchain/modules/atc-zkp/src/snarks.rs` | `ebf43b8dbd13ae660d111aed0c6828b12a419094` | Source explicitly labels the module planned / not implemented. | BLOCKED |
| Bridge state proofs | `a-townchain/components/interop/modules/atc-bridge/proofs/state_proof.atc` | `b5ec1883e810a398f68f22fbae4bd4dc6578a622` | Inspected verifier functions contain comments only; no executable verification logic shown. | BLOCKED |
| Normative rollup standard | `atc-standards/standards/zkp/ATC-STD-ZKP-008.md` | `24e14052997b1945b79334f8f010a804e0c02301` | v1.1.0 is marked approved/normative; REQ-ZKP-804 requires validity proofs for every state-root transition; REQ-ZKP-803 forbids starting rollup construction before the base ZKP verification standards. | IMPLEMENTED (normative text only) |
| L1 block state root | `a-townchain/modules/atc-blockchain/kernel/src/blockchain.rs` | `5747452b10c46a862c251b4a0e68dc19757a0fbb` | `Block::new` initializes `state_root` to `[0u8; 32]`; this constructor alone does not demonstrate a computed/validated post-state root. | BLOCKED |
| Node chain boundary | `a-townchain/components/node/src/chain.rs` | `9876cda188a353195686ded9de533770d5c5fe94` | File explicitly says this module is not consensus/finality and calls its 64-bit TownHash a non-cryptographically-audited placeholder, not a Merkle tree. Do not treat it as rollup settlement. | BLOCKED |
| ZKP architecture | `a-townchain/components/zkp/docs/ZKP_ARCHITECTURE.md` | `631e3f110d84c8dea555621852218ac7d1d24319` | Architecture document describes a target layer; it is not evidence of executable verification. | RESIDUAL |
| L1 consensus candidate | `a-townchain/modules/atc-blockchain/consensus/consensus.atc` | `8ea2a39fa31073726e2dfde29111d4e231d7bd7c` | Block fields include `tx_count` and `merkle_root`; this alone does not establish canonical rollup state-root verification, batch commitment, or DA. | RESIDUAL |
| Ecosystem layer model | `ARCHITECTURE.md` | `ea3a93cdb6f66eb33caf8fd7715e184fd524b27e` | Current model defines L2 as blockchain core/consensus, L3 as deterministic execution, L4 as scaling/rollups. Protocol L1/L2 must be distinguished from internal responsibility layers. | RESIDUAL |

Source links:
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/main/modules/atc-zkp/Cargo.toml
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/main/modules/atc-zkp/src/verifier.rs
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/main/modules/atc-zkp/src/snarks.rs
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/main/components/interop/modules/atc-bridge/proofs/state_proof.atc
- https://github.com/A-TownChain-Okosystems/atc-standards/blob/68448117334b49729f423f1648f9a960f4411220/standards/zkp/ATC-STD-ZKP-008.md
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/a5e689671b1870699c7a1750f8a166e83590a41c/modules/atc-blockchain/kernel/src/blockchain.rs
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/a5e689671b1870699c7a1750f8a166e83590a41c/components/node/src/chain.rs
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/main/components/zkp/docs/ZKP_ARCHITECTURE.md
- https://github.com/A-TownChain-Okosystems/a-townchain/blob/main/modules/atc-blockchain/consensus/consensus.atc
- https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/blob/main/ARCHITECTURE.md

## Required evidence before runtime implementation

1. Pin exact repository commit SHAs for `a-townchain`, `atc-standards`, and `atc-sdk`; blob SHAs above are file-content identifiers, not repository commit SHAs.
2. Trace canonical L1 transaction validation, deterministic state transition, state-root construction, block finality, and upgrade rules; identify which implementation is actually active/canonical.
3. Read and satisfy the complete prerequisite chain ATC-STD-ZKP-001 through 008, especially the required base verifier and security requirements.
4. Identify existing tests and CI workflows for consensus determinism, state transition, cryptography, and bridge proofs; record run/job/step and artifact hashes.
5. Search all active source paths for existing batch, validity-proof, DA, sequencer, forced-inclusion, and withdrawal implementations; classify legacy/archive references separately.
6. Specify batch commitments, proof public inputs, state transition, DA commitments, sequencer recovery, and withdrawal/finality semantics in the canonical standards/source SSOT before coding the verifier integration.

## Acceptance criteria

- [ ] Repository commit SHAs pinned and recorded.
- [ ] Canonical L1 state-transition and finality paths identified.
- [ ] State-root and batch-commitment semantics specified.
- [ ] Base ZKP verifier exists and passes valid/invalid proof tests.
- [ ] L1 rejects invalid proofs and accepts only valid state-root transitions.
- [ ] Data availability and sequencer-failure recovery designed and tested.
- [ ] Deposit/withdrawal replay, double-spend, reorg/finality, and invalid-proof cases covered.
- [ ] Standards ownership and catalog layer mapping reconciled.
- [ ] CI evidence captured for the exact reviewed commit.

Until acceptance criteria are met, rollup runtime implementation and production settlement remain **BLOCKED**. Do not substitute stubs, comments, or architecture prose for working proof verification.
