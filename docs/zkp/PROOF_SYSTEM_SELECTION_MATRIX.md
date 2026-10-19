# ATC ZKP Proof-System Selection Matrix

**Status:** DECISION RECORD / candidate selection for a validation spike; not production approval  
**Scope:** `modules/atc-zkp`, with the initial target being validity proofs for state-root transitions  
**Standards baseline:** `ATC-STD-ZKP-001…010`, version 1.1.0, approved/normative in `A-TownChain-Okosystems/atc-standards`

## Decision summary

Select **Zcash `halo2_proofs` v0.4.0 (PLONK-based Halo 2, IPA/Pasta configuration) as the first candidate for a bounded integration spike**, not as an already approved production backend. Its upstream manifest describes a PLONK-based proof system without a trusted setup, which aligns with the transparent-alternative requirement in ZKP-010 and the pluggable interface requirement in ZKP-002.

This is a candidate decision only. Production integration remains **BLOCKED** until the exact source revision, license inheritance, dependency/MSRV compatibility, no_std strategy, canonical proof encoding/versioning, circuit soundness, resource limits, and independent security review are evidenced. Do not enable the backend by default or treat interface tests as cryptographic verification.

Upstream evidence inspected:
- Repository: https://github.com/zcash/halo2
- Manifest: `halo2_proofs/Cargo.toml`, package version `0.4.0`, blob SHA `9a571de8302c3574c3cbf1d7a05205cff94fc91f`
- Manifest description: “Fast PLONK-based zero-knowledge proving system with no trusted setup”
- Library source: `halo2_proofs/src/lib.rs`, blob SHA `8fc0e0be0ddff43211b5d7bfe05d3771eac943b7`
- These blob SHAs identify the inspected upstream files, not a pinned upstream commit. Pinning a commit is still required before integration.

## Candidate comparison

| Candidate family | Standards fit | Main trade-off for ATC | Current decision |
|---|---|---|---|
| Groth16 | Listed by ZKP-002; compact proofs and established verifier pattern | Requires a trusted setup for each circuit; ZKP-010 requires documented ceremony criteria, at least 10 independent participants, and toxic-waste elimination | Not first choice until ceremony and circuit lifecycle are justified |
| Generic PLONK | Listed by ZKP-002; supports pluggable proof-system architecture | Setup assumptions vary by implementation; exact transcript, SRS, encoding, and audit posture must be fixed | Too broad to integrate without naming a specific implementation |
| Halo 2 / `zcash/halo2` v0.4.0 | Rust; PLONK-based; upstream manifest says no trusted setup; aligns with versioned backend abstraction | Current upstream crate has no demonstrated `no_std` support in this review; exact integration and circuit compatibility remain unproven | **First candidate for a validation spike; production approval BLOCKED** |
| STARK | Listed by ZKP-002; transparent setup and a natural fit for execution-trace proofs in ZKP-009 | Need to select a concrete Rust implementation, validate no_std/resource characteristics, proof encoding, and L1 verification cost; no implementation evidence currently exists | Keep as fallback if Halo 2 cannot meet no_std/runtime constraints |

## Normative gates mapped to ATC-STD-ZKP-001…010

| Standard | Required gate for this candidate | Evidence / current status |
|---|---|---|
| ZKP-001 Architecture | Keep ZKP as an L1 verification layer; preserve the six-module architecture and Rust implementation | Interface exists; complete architecture/module compliance not yet audited |
| ZKP-002 Proof interface | Adapter must cover setup/prove/verify/serialize/deserialize/version; versioned header; strict timeouts; controlled parse failures; retain backend interchangeability | Existing `ProofBackend::verify` covers verification only. Full interface and timeouts are **BLOCKED** |
| ZKP-003 Circuits | Registry/versioning, deterministic circuit artifact, SHA-256 pinning, public/private signal classification, under-constrained tests and constraint limits | State-transition circuit not implemented; **BLOCKED** |
| ZKP-004 On-chain verification | Verify proof → check nullifier → update state; bound deterministic costs; public-input range checks; atomic batch failure | L1 integration and cost measurement not implemented; **BLOCKED** |
| ZKP-005 Commitments/nullifiers | ATC-approved hash primitive or explicit AD; fixed Merkle depth; zero-nullifier rejection; persistent replay protection | Not in scope of current interface; **BLOCKED** |
| ZKP-006 ZK identity | Privacy, issuer signature, revocation, nonce/freshness and registry controls when identity circuits are introduced | Not required for the initial state-root spike; mark **NOT IN SCOPE**, not verified |
| ZKP-007 Private transactions | Range/overflow safety, balance conservation, commitments and nullifiers when private-transfer circuits are introduced | Not required for the initial state-root spike; mark **NOT IN SCOPE**, not verified |
| ZKP-008 Rollups | Prove every state-root transition; data availability; no rollup launch before ZKP-001…007 base verification; escape hatches and forced L1 transactions | Rollup settlement remains **BLOCKED** |
| ZKP-009 ZKVM | If proving ATC execution traces, use canonical ATC bytecode, deterministic traces, bounded steps and memory-consistency proofs | Not implemented in this candidate spike; **BLOCKED** for ZKVM claims |
| ZKP-010 Security | Security review before approval/freeze; fuzzing; document setup assumptions; audit critical circuits | No independent review or continuous fuzzing evidence yet; **BLOCKED** |

## Acceptance gates before enabling the backend

1. Pin the exact upstream Git commit and dependency checksum; confirm license and transitive dependency licenses.
2. Demonstrate the chosen integration can coexist with the declared `#![no_std]` library configuration. If it cannot, propose an explicit feature/crate boundary and obtain architecture review; do not silently remove `no_std`.
3. Define canonical, versioned proof and verification-key encodings, strict length bounds, and compatibility rejection tests.
4. Implement a real state-transition circuit binding `old_state_root`, `new_state_root`, and all public inputs; constrain the state transition rather than merely proving an unconstrained witness.
5. Add known-good proof vectors and negative vectors: mutated proof, wrong public inputs, wrong key/circuit, malformed/truncated encoding, unsupported version, oversized inputs, and backend errors.
6. Demonstrate that an invalid proof cannot update the L1 state root; verify atomic failure and no state mutation on all negative cases.
7. Add deterministic resource/time limits, fuzzing, reproducible build evidence, and independent G18 security review required by ZKP-010.
8. Only then move from candidate to approved backend and change any production status from BLOCKED.

## Current implementation boundary

The existing `VerificationEngine` is a fail-closed interface, not a cryptographic verifier. The existing tests use a mock backend. The dedicated `atc-zkp` CI job has passed compilation and unit tests for the current no_std library, but that does not establish compatibility of Halo 2 or correctness of a state-transition circuit.

**Status:** Candidate chosen for spike = Halo 2; production cryptographic backend = **BLOCKED**; rollup settlement = **BLOCKED**.
