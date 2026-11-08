---
standard:
  id: ATC-STD-600
  title: "Chain Identity & Network Identification"
  version: "1.3.0"
  status: approved
  lifecycle: frozen
  role: "Root Specification"
  category: blockchain
  authority: A-TownChain-Okosystems
  owner: "Standards Governance"
  created: "2026-09-14"
  updated: "2026-09-21"
  normative: true
  classification: PUBLIC
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
  related_standards:
    - ATC-STD-601
    - ATC-STD-602
    - ATC-STD-603
    - ATC-STD-604
    - ATC-STD-605
    - ATC-STD-606
  applies_to: "A-TownChain Core"
  license: "Copyright (c) 2026 A-TownChain-Okosystems"
---

# ATC-STD-600 — Chain Identity & Network Identification

> **Version:** 1.3.0  
> **Status:** APPROVED  
> **Lifecycle:** APPROVED  
> **Role:** Root Specification  
> **Scope:** A-TownChain Core  
> **Implementation:** SPECIFICATION_ONLY

## Abstract

ATC-STD-600 defines the canonical identity model of an A-TownChain chain instance and the normative boundary between chain identity, transaction authentication, and runtime compatibility.

The standard defines schema, semantics, validation rules, canonical encoding requirements, fail-closed behavior, and conformance requirements. It does **not** hardcode deployment-specific values for devnet, testnet, or mainnet.

A valid cryptographic signature alone does not make a transaction valid. The transaction MUST belong to the correct chain and network context and MUST be compatible with the applicable protocol/runtime context.

Keywords **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are normative according to ATC-STD-000.

## 1. Scope

This standard applies to A-TownChain Core components that create, validate, transport, sign, execute, display, or otherwise consume Chain Identity information.

It is the root specification for:

- ATC-STD-601 — Genesis Specification & Genesis Identity
- ATC-STD-602 — Transaction Domain Separation & Replay Protection
- ATC-STD-603 — Network Environment Specification
- ATC-STD-604 — Protocol & VM Version Compatibility
- ATC-STD-605 — Chain Identity Validation
- ATC-STD-606 — Chain Identity Registry

Concrete deployment values belong to authoritative Genesis and network configuration, not to this root specification.

## 2. Canonical Chain Identity

Every chain instance MUST expose the following canonical identity structure:

```yaml
chain:
  chain_id: "atc"

  network:
    network_id: "mainnet"

  genesis:
    genesis_id: "<canonical-genesis-hash>"

  protocol:
    protocol_version: "1.0.0"

  vm:
    vm_version: "1.0.0"
```

### 2.1 Identity fields

| Field | Meaning | Normative role |
|---|---|---|
| `chain_id` | Logical blockchain identity | Identifies **which blockchain** |
| `network_id` | Deployment/operating environment | Identifies **which environment** |
| `genesis_id` | Concrete Genesis state identity | Identifies **which Genesis state** |
| `protocol_version` | Consensus/protocol version | Runtime/protocol compatibility |
| `vm_version` | ATC-VM execution version | Execution compatibility |

### 2.2 Identity invariant

The canonical Chain Identity is:

```text
CHAIN_IDENTITY
    =
    chain_id
  + network_id
  + genesis_id
```

The fields MUST be interpreted as structured fields. Implementations MUST NOT rely on ambiguous string concatenation for identity derivation or comparison.

## 3. Network Model

A-TownChain uses one stable logical `chain_id` with explicitly identified deployment environments:

```text
A-TownChain
  chain_id = "atc"
       │
  ┌────┼────┐
 devnet testnet mainnet
```

Each environment MUST have its own `network_id` and authoritative Genesis configuration.

`network_id` MUST NOT be derived implicitly from `chain_id`.

A node MUST NOT silently reinterpret a configured network identity based on peer claims or discovered metadata.

## 4. Separation of Security Domains

ATC-STD-600 deliberately separates three concepts.

### 4.1 Chain Identity

```text
chain_id
network_id
genesis_id
```

This establishes the identity of the chain instance.

### 4.2 Transaction Authentication

```text
chain_id
network_id
protocol_version
transaction_type
canonical_transaction
signature
```

This establishes the transaction's cryptographic authentication within the defined transaction domain.

### 4.3 Runtime Compatibility

```text
protocol_version
vm_version
execution_rules
```

This establishes whether the receiving runtime is permitted to execute the transaction/state transition.

These domains MUST NOT be collapsed into one implicit identity mechanism.

## 5. Transaction Domain

The canonical A-TownChain L1 transaction signing domain is:

    ATC-TX-DOMAIN-V2

The legacy `ATC-TX-DOMAIN` format is retired and MUST NOT be used for L1 transaction signing.

### 5.1 Canonical L1 signing bytes

The authenticated byte representation is exactly:

    ATC-TX-DOMAIN-V2
    + chain_id          (u64, big-endian)
    + tx_type           (u8)
    + sender_did        (u32 length + UTF-8 bytes)
    + recipient_did     (presence byte + optional u32 length + UTF-8 bytes)
    + amount            (u64, big-endian)
    + gas_price         (u64, big-endian)
    + gas_limit         (u64, big-endian)
    + nonce              (u64, big-endian)
    + timestamp          (u64, big-endian)
    + payload            (u32 length + bytes)
    + poh_hash           (32 bytes)

The canonical numeric chain identifier for the current L1 is:

    chain_id = 658467

Implementations MUST produce byte-identical signing input. Rust L1 kernel, Rust SDK/wallet, and TypeScript SDK implementations MUST NOT maintain independent signing encodings.

`network_id`, `protocol_version`, `vm_version`, and `genesis_id` remain part of runtime/chain identity validation, but MUST NOT be silently inserted into the V2 transaction signing bytes.

### 5.2 Signature algorithm

The canonical L1 signature is Ed25519 over the exact V2 signing bytes.

    signature = Ed25519_Sign(private_key, signing_bytes_v2)

Verification MUST operate on those same bytes. Hashing or field re-encoding before Ed25519 verification is not permitted unless explicitly defined by a future governed protocol revision.

### 5.3 Canonical transaction identity

Transaction identity is separate from the signing domain:

    ATC-TX-ID-V2 + canonical transaction fields

The signature is not part of transaction identity.

### 5.4 Compatibility rule

A node MUST reject signatures produced using the retired `ATC-TX-DOMAIN` representation. There is no dual-acceptance mode in the canonical L1 path.

## 6. Genesis Identity

A Genesis state MUST have a deterministically derived `genesis_id`.

The canonical conceptual derivation is:

```text
 genesis_id
     =
 HASH(
     CANONICAL_ENCODE(
         genesis_document
     )
 )
```

The configured Genesis identity MUST equal the deterministically computed Genesis identity:

```text
configured_genesis_id == computed_genesis_id

YES -> ACCEPT
NO  -> REJECT
```

Implementations MUST NOT automatically correct, replace, or overwrite a configured `genesis_id` after detecting a mismatch.

Genesis identity is part of Chain Identity validation and is not merely descriptive metadata.

## 7. Runtime Context

ATC-VM and other execution components MUST operate against an explicit validated runtime context.

A conceptual Rust representation is:

```rust
struct ChainContext {
    chain_id: ChainId,
    network_id: NetworkId,
    genesis_id: GenesisId,
    protocol_version: ProtocolVersion,
    vm_version: VmVersion,
}
```

The runtime compatibility model is:

```text
RUNTIME_COMPATIBILITY
    =
    protocol_version
  + vm_version
  + execution_rules
```

ATC-VM MUST NOT perform a state transition until the relevant chain, network, Genesis, protocol, and runtime context has passed the required validation gates.

## 8. Fail-Closed Execution Model

The normative execution sequence is:

```text
Incoming Transaction
        │
        ▼
Chain Identity Validation
        │
   VALID / INVALID
        │
        ├── INVALID -> REJECT
        ▼
Transaction Domain Validation
        │
   VALID / INVALID
        │
        ├── INVALID -> REJECT
        ▼
Signature Validation
        │
   VALID / INVALID
        │
        ├── INVALID -> REJECT
        ▼
Runtime Compatibility
        │
    PASS / FAIL
        │
        ├── FAIL -> REJECT
        ▼
ATC-VM
        │
        ▼
STATE TRANSITION
```

The following safety rules are mandatory:

```text
IDENTITY != VALID -> REJECT
SIGNATURE != VALID -> REJECT
RUNTIME != COMPATIBLE -> REJECT
CONFORMANCE != PASS -> NO EXECUTION
```

A valid signature MUST NOT bypass identity, domain, runtime, or conformance validation.

## 9. Normative Requirements

### 9.1 Chain identity

- **REQ-STD-600-001:** Every chain instance MUST have `chain_id`.
- **REQ-STD-600-002:** Every deployment environment MUST have `network_id`.
- **REQ-STD-600-003:** Every Genesis state MUST have a deterministically derived `genesis_id`.
- **REQ-STD-600-004:** Nodes MUST validate local Chain Identity before joining or participating in the network.
- **REQ-STD-600-005:** `chain_id`, `network_id`, and `genesis_id` MUST be treated as structured identity fields.

### 9.2 Transaction domain

- **REQ-STD-600-006:** Transactions MUST use defined canonical encoding.
- **REQ-STD-600-007:** The transaction validation context MUST include `chain_id`, `network_id`, `protocol_version`, and an explicit transaction domain separator. These context fields are validated before signing-byte construction; `network_id` and `protocol_version` MUST NOT be inserted into the canonical V2 signing bytes unless a governed protocol revision explicitly defines that change.
- **REQ-STD-600-008:** Transaction domain validation MUST occur before state execution.
- **REQ-STD-600-009:** Non-canonical encoding variants MUST NOT be accepted as equivalent authenticated transactions.

### 9.3 Runtime

- **REQ-STD-600-010:** ATC-VM MUST enforce a validated execution context.
- **REQ-STD-600-011:** Protocol/runtime incompatibility MUST cause rejection.
- **REQ-STD-600-012:** No state transition MAY occur when conformance has not passed.

### 9.4 Fail-closed behavior

- **REQ-STD-600-013:** Identity validation errors MUST be fail-closed.
- **REQ-STD-600-014:** Signature validation errors MUST be fail-closed.
- **REQ-STD-600-015:** Runtime compatibility errors MUST be fail-closed.
- **REQ-STD-600-016:** Implementations MUST NOT silently repair identity mismatches.

## 10. Prohibited Behavior

Implementations MUST NOT:

1. automatically correct `chain_id`;
2. derive `network_id` from `chain_id`;
3. manually override `genesis_id` after validation failure;
4. accept non-canonical transaction encoding variants;
5. execute a transaction after Chain Identity mismatch;
6. execute a state transition when runtime compatibility fails;
7. allow divergent Chain Identity definitions between node, VM, SDK, wallet, explorer, or interoperability components.

## 11. Conformance

An implementation conforms to ATC-STD-600 only when all mandatory identity, domain, runtime, and fail-closed requirements are machine-verifiably satisfied.

Minimum conformance gate:

```text
ATC-STD-600 Conformance
  - Schema valid
  - Chain ID valid
  - Network ID valid
  - Genesis ID verified
  - Protocol compatible
  - VM compatible
  - Canonical encoding valid
  - Signature domain valid
  - Replay protection verified
        ↓
      PASS
```

A failed conformance gate MUST prevent execution of the affected functionality.

## 12. Implementation Matrix

| Component | ATC-STD-600 requirement |
|---|---|
| `atc-node` | Chain Identity + Genesis + Network validation |
| `atc-vm` | Enforce validated execution context |
| `atc-sdk` | Generate the correct transaction domain separator |
| `atc-wallet` | Sign using the correct Chain/Network context |
| `atc-explorer` | Display Chain Identity correctly |
| `atc-interop` | Address Chain Identity unambiguously |
| Genesis Configuration | Provide authoritative concrete values |
| CI / Conformance | Machine-validate all applicable rules |

## 13. Governance and Change Control

ATC-STD-600 defines schema, semantics, and validation rules. It does not define concrete deployment values.

Therefore:

```text
ATC-STD-600
    defines
SCHEMA + SEMANTICS + VALIDATION RULES
    does NOT define
specific deployment values
    ↓
Genesis / Network Configuration
```

The meanings of `chain_id`, `network_id`, and `genesis_id` are normative. A semantic change to any of these fields, to transaction-domain semantics, replay protection, or runtime compatibility MUST proceed through the ATC-STD lifecycle and MUST NOT be introduced as an implementation-only change.

ATC-STD-600 v1.3.0 is the current canonical baseline for L1 transaction-domain semantics. Deployment-specific values MAY evolve through governed Genesis/network configuration without changing the semantics of this standard.

## 14. Reference Deployment Model

The root standard does not hardcode concrete production values. Each environment MUST provide its own authoritative configuration.

```yaml
# devnet
chain:
  chain_id: "atc"
  network:
    network_id: "devnet"
  genesis:
    genesis_id: "<devnet-genesis-hash>"
```

```yaml
# testnet
chain:
  chain_id: "atc"
  network:
    network_id: "testnet"
  genesis:
    genesis_id: "<testnet-genesis-hash>"
```

```yaml
# mainnet
chain:
  chain_id: "atc"
  network:
    network_id: "mainnet"
  genesis:
    genesis_id: "<mainnet-genesis-hash>"
```

The concrete values MUST be supplied by authoritative Genesis/network configuration and MUST NOT be inferred from this example.

## 15. Standard Family

```text
ATC-STD-600  Chain Identity & Network Identification
ATC-STD-601  Genesis Specification & Genesis Identity
ATC-STD-602  Transaction Domain Separation & Replay Protection
ATC-STD-603  Network Environment Specification
ATC-STD-604  Protocol & VM Version Compatibility
ATC-STD-605  Chain Identity Validation
ATC-STD-606  Chain Identity Registry
```

ATC-STD-600 is the root specification. Standards 601–606 depend on its identity model and MUST NOT redefine the semantics of the root identity fields inconsistently.

## 16. Implementation Dependency Order

```text
atc-standards
      ↓
atc-node
      ↓
atc-vm
      ↓
atc-sdk
      ↓
atc-wallet
      ↓
atc-explorer
      ↓
atc-interop
```

The dependency order is implementation guidance and MUST NOT be interpreted as permission to bypass conformance gates.

## 17. Freeze Record

```text
ATC-STD-600 v1.3.0
Status: STABLE / BASELINE
Lifecycle: FROZEN
Role: Root Specification
Scope: A-TownChain Core
Implementation: AUTHORIZED
```

No further semantic changes are authorized within v1.0.0. Corrections that alter semantics require a governed standard revision.

## 18. Changelog

- **1.3.0 — 2026-09-21:** Retires the legacy transaction domain and defines ATC-TX-DOMAIN-V2 canonical L1 signing bytes with numeric chain_id 658467.
- **1.0.0 — 2026-09-14:** Root specification frozen. Establishes Chain Identity, Network Identity, Genesis Identity, transaction-domain separation, runtime compatibility, canonical encoding requirements, fail-closed execution, conformance gates, and the ATC-STD-600 family model.

## References

- ATC-STD-000 — Standards Governance & Specification
- ATC-STD-601 — Genesis Specification & Genesis Identity
- ATC-STD-602 — Transaction Domain Separation & Replay Protection
- ATC-STD-603 — Network Environment Specification
- ATC-STD-604 — Protocol & VM Version Compatibility
- ATC-STD-605 — Chain Identity Validation
- ATC-STD-606 — Chain Identity Registry
