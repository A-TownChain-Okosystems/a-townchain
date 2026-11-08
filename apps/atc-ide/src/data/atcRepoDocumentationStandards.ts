/**
 * A-TownChain / ShivaCore / Globus OS / ATC-VM
 * Engineering Documentation Standard & Professional Repository Architecture (ATC-DOC Framework)
 * 
 * Normative definitions, file templates, standards registry, ADRs and machine-readable manifests.
 */

export interface DocCategory {
  id: string;
  name: string;
  badge: string;
  description: string;
  count: number;
}

export interface StandardDocFile {
  path: string;
  category: 'root' | 'github' | 'arch' | 'standards' | 'api' | 'vm' | 'blockchain' | 'adr' | 'manifest' | 'atc-doc';
  title: string;
  priority: 'mandatory' | 'recommended' | 'optional' | 'normative';
  status: 'Draft' | 'Review' | 'Accepted' | 'Stable' | 'Living' | 'Final';
  version: string;
  owner: string;
  summary: string;
  dependencies?: string[];
  content: string;
}

export interface AtcDocStandard {
  id: string; // e.g. ATC-DOC-001
  title: string;
  version: string;
  status: 'Stable' | 'Proposed' | 'Accepted';
  category: 'Governance' | 'Architecture' | 'Specification' | 'API' | 'Security' | 'Decisions' | 'QA' | 'Release';
  abstract: string;
  rules: string[];
  templateSkeleton: string;
}

export interface AdrEntry {
  id: string; // e.g. ADR-0001
  title: string;
  status: 'Accepted' | 'Proposed' | 'Superseded' | 'Deprecated';
  date: string;
  author: string;
  context: string;
  decision: string;
  alternatives: string[];
  consequences: {
    positive: string[];
    negative: string[];
  };
}

export interface AtcProtocolStandard {
  code: string; // e.g. ATC-0001
  name: string;
  category: 'Core' | 'Naming' | 'Crypto' | 'Token' | 'VM' | 'Consensus';
  status: 'Draft' | 'Review' | 'Final' | 'Living';
  version: string;
  authors: string[];
  sections: {
    abstract: string;
    motivation: string;
    specification: string;
    terminology: string;
    dataStructures: string;
    encoding: string;
    stateTransitions: string;
    validationRules: string;
    errorConditions: string;
    securityConsiderations: string;
    compatibility: string;
    testVectors: string;
    referenceImplementation: string;
    changelog: string;
  };
}

// --------------------------------------------------------------------------
// 1. OVERARCHING ATC-DOC META-STANDARDS (ATC-DOC-001 to ATC-DOC-008)
// --------------------------------------------------------------------------
export const ATC_DOC_STANDARDS: AtcDocStandard[] = [
  {
    id: 'ATC-DOC-001',
    title: 'Repository Documentation Standard',
    version: '1.0.0',
    status: 'Stable',
    category: 'Governance',
    abstract: 'Defines the mandatory root layout, metadata, git hooks, license headers, and repository governance requirements for all A-TownChain ecosystem repositories (ATC-VM, ShivaCore, Globus OS, Aurora AI).',
    rules: [
      'Every repository must have the 10 mandatory root files: README, LICENSE, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, CHANGELOG, ROADMAP, AUTHORS, NOTICE, .gitignore.',
      'No undocumented directory deeper than level 2 is allowed; all subsystems in docs/ must have local README.md or be listed in DOCUMENTATION_INDEX.md.',
      'All code commits must reference an engineering task or standard ID (e.g. ATC-0094).',
      'The file DOCUMENTATION_INDEX.md serves as the machine-readable single source of truth for doc lifecycle.'
    ],
    templateSkeleton: `# Repository Documentation Standard (ATC-DOC-001)

## Directory Invariants
- Root: Mandatory files only. No loose test dumps or scratch scripts.
- docs/: Technical, security, operations, standards, and architecture trees.
- .github/: Engineering governance, issue templates, CI workflows, and CODEOWNERS.
`
  },
  {
    id: 'ATC-DOC-002',
    title: 'Architecture Documentation Standard',
    version: '1.0.0',
    status: 'Stable',
    category: 'Architecture',
    abstract: 'Establishes the C4 Model + 5 Vertical Domains & 7 Horizontal Control Planes architectural documentation requirements.',
    rules: [
      'Architecture docs must separate concerns into Experience, Application, Platform, Execution, and System domains.',
      'Every subsystem must define its 7 Control Plane integrations (Security, Governance, Observability, Audit, Policy, Resource, Config).',
      'Data flow diagrams must utilize either Mermaid or standardized ASCII schemas.',
      'Hardware and ABI boundaries must explicitly document Ring 0 vs Ring 3 invariants.'
    ],
    templateSkeleton: `# Architecture Specification (ATC-DOC-002)

## 1. Domain Categorization
## 2. Cross-Cutting Control Planes
## 3. Interfaces & Contracts (C1 - C5)
## 4. Hardware & Isolation Boundaries
`
  },
  {
    id: 'ATC-DOC-003',
    title: 'Specification Standard for Protocols & Standards',
    version: '1.0.0',
    status: 'Stable',
    category: 'Specification',
    abstract: 'Prescribes the mandatory 14-section formal layout for all ATC Protocol Standards (ATC-0001 through ATC-0099+).',
    rules: [
      'Specifications must strictly follow the 14 chapters: Abstract, Motivation, Specification, Terminology, Data Structures, Encoding, State Transitions, Validation Rules, Error Conditions, Security Considerations, Compatibility, Test Vectors, Reference Implementation, Changelog.',
      'Normative words (MUST, MUST NOT, REQUIRED, SHALL, SHOULD, MAY) must adhere to RFC 2119 / RFC 8174.',
      'Every standard must provide deterministic test vectors with hex payloads and expected hashes.'
    ],
    templateSkeleton: `# ATC-XXXX: [Standard Title]

Status: Draft | Review | Final | Living
Version: X.Y.Z
Authors: [Author Names <emails>]
Category: Core | Protocol | VM | Interface

## 1. Abstract
## 2. Motivation
## 3. Specification
## 4. Terminology
## 5. Data Structures
## 6. Encoding
## 7. State Transitions
## 8. Validation Rules
## 9. Error Conditions
## 10. Security Considerations
## 11. Compatibility
## 12. Test Vectors
## 13. Reference Implementation
## 14. Changelog
`
  },
  {
    id: 'ATC-DOC-004',
    title: 'API Documentation Standard',
    version: '1.0.0',
    status: 'Stable',
    category: 'API',
    abstract: 'Standardizes REST, JSON-RPC 2.0, WebSocket PubSub, and OpenAPI 3.1 definitions across node endpoints and kernel service APIs.',
    rules: [
      'All HTTP REST endpoints must provide valid OpenAPI 3.1 schema specs with response examples.',
      'JSON-RPC methods must strictly follow JSON-RPC 2.0 semantics with typed error codes in [-32000, -32700] range.',
      'WebSocket APIs must enforce typed subscription envelopes with idempotency IDs.',
      'Every mutation endpoint must define its rate limit and capability token requirements.'
    ],
    templateSkeleton: `# API Specification (ATC-DOC-004)

## Endpoints Summary
## Authentication & Capability Tokens
## JSON-RPC 2.0 Schema
## WebSocket Event Streams
## Error Code Reference
`
  },
  {
    id: 'ATC-DOC-005',
    title: 'Security Documentation Standard',
    version: '1.0.0',
    status: 'Stable',
    category: 'Security',
    abstract: 'Specifies Threat Models (STRIDE), Trust Models, Attack Surface Analysis, and Incident Response protocols for high-assurance infrastructure.',
    rules: [
      'Every core repository must contain docs/security/SECURITY_ARCHITECTURE.md and docs/security/THREAT_MODEL.md.',
      'Threat models must categorize risks into Spoofing, Tampering, Repudiation, Information Disclosure, Denial of Service, and Elevation of Privilege (STRIDE).',
      'Incident Response SLA must commit to Sev-1 emergency triage within 2 hours.'
    ],
    templateSkeleton: `# Security Specification (ATC-DOC-005)

## Trust Model & Root of Trust
## STRIDE Threat Matrix
## Cryptographic Primitive Inventory
## Incident Response & Key Revocation
`
  },
  {
    id: 'ATC-DOC-006',
    title: 'Architecture Decision Record (ADR) Standard',
    version: '1.0.0',
    status: 'Stable',
    category: 'Decisions',
    abstract: 'Establishes the process and template for capturing irreversible architectural choices, trade-offs, and historical context.',
    rules: [
      'ADRs are immutable once status is Accepted. Changes require a new ADR that supersedes the prior one.',
      'ADRs must clearly document evaluated alternatives and why they were rejected.',
      'Consequences must balance positive outcomes against operational/technical debt.'
    ],
    templateSkeleton: `# ADR-XXXX: [Decision Title]

## Status: Accepted | Proposed | Superseded | Deprecated
## Date: YYYY-MM-DD
## Context: What problem prompted this choice?
## Decision: What was decided?
## Alternatives: What other options were considered?
## Consequences: What are the trade-offs?
`
  },
  {
    id: 'ATC-DOC-007',
    title: 'Testing & Verification Documentation Standard',
    version: '1.0.0',
    status: 'Stable',
    category: 'QA',
    abstract: 'Defines test coverage thresholds, fuzzing harnesses, differential execution suites, and formal verification proofs.',
    rules: [
      'Core VM and kernel code must maintain >= 85% test branch coverage.',
      'Every state transition opcode must have deterministic fuzz tests executing > 10,000,000 iterations in CI.',
      'Differential tests must compare ATC-VM execution against reference interpreter outputs.'
    ],
    templateSkeleton: `# Testing & Verification Standard (ATC-DOC-007)

## Coverage Matrix
## Fuzzing & Property-Based Testing
## Differential VM Testing
## Formal Invariant Proofs
`
  },
  {
    id: 'ATC-DOC-008',
    title: 'Release & Versioning Governance Standard',
    version: '1.0.0',
    status: 'Stable',
    category: 'Release',
    abstract: 'Establishes SemVer 2.0 release cycles, hard-fork vs soft-fork governance, reproducible builds, and SLSA Level 3 supply chain attestation.',
    rules: [
      'Breaking ABI changes require a major version bump and minimum 90-day validator transition window.',
      'Release tags must be GPG/Cosign signed with reproducible container build checksums.',
      'CHANGELOG.md must categorize changes under Added, Changed, Deprecated, Removed, Fixed, Security.'
    ],
    templateSkeleton: `# Release Governance (ATC-DOC-008)

## SemVer 2.0 Policy
## Hard Fork vs Soft Fork Criteria
## SLSA Supply Chain Attestation
## Release Verification Checklist
`
  }
];

// --------------------------------------------------------------------------
// 2. ARCHITECTURE DECISION RECORDS (ADRs 0001 - 0006)
// --------------------------------------------------------------------------
export const ADR_REGISTRY: AdrEntry[] = [
  {
    id: 'ADR-0001',
    title: 'Use Rust as Core Kernel & VM Implementation Language',
    status: 'Accepted',
    date: '2025-01-15',
    author: 'A-TownChain Architecture Committee',
    context: 'The ShivaCore Kernel and ATC-VM require memory safety without garbage collection overhead, deterministic execution, zero-cost abstractions, and fine-grained control over hardware registers and memory layouts.',
    decision: 'Implement all low-level kernel routines, memory paging, scheduler, IPC mechanisms, and the ATC-VM interpreter/JIT runtime in Rust (#![no_std] in Ring 0).',
    alternatives: [
      'C/C++: High performance, but lacks compile-time memory safety, leading to potential buffer overflow and use-after-free vulnerabilities.',
      'Go: Easy concurrency, but runtime garbage collection induces nondeterministic pauses unacceptable for high-throughput consensus and deterministic gas metering.',
      'Zig: Promising manual memory control, but smaller ecosystem and less mature formal verification tooling compared to Rust.'
    ],
    consequences: {
      positive: [
        'Elimination of entire classes of memory vulnerabilities (dangling pointers, data races) at compile time.',
        'Zero-overhead FFI interoperability with assembly and C hardware abstractions.',
        'Rich ecosystem of cryptographic primitives (bls12_381, ed25519-dalek, k256).'
      ],
      negative: [
        'Steeper learning curve for contributors unfamiliar with the borrow checker.',
        'Longer CI compile times for full release profiles with Link Time Optimization (LTO).'
      ]
    }
  },
  {
    id: 'ADR-0002',
    title: 'Microkernel Architecture with Capability-Based Security',
    status: 'Accepted',
    date: '2025-02-10',
    author: 'ShivaCore Kernel Security Group',
    context: 'Monolithic OS kernels (like standard Linux) present a vast attack surface. A bug in any device driver or filesystem can compromise the entire machine.',
    decision: 'Adopt a true microkernel architecture. Ring 0 is restricted strictly to CPU scheduling, 4-level PML4 paging, thread lifecycle, lockless IPC ring-buffers, and capability validation. Filesystems, device drivers, network stacks, blockchain clients, and AI runtimes operate as unprivileged Ring 3 microservices.',
    alternatives: [
      'Monolithic Kernel: Simpler initial driver development, but catastrophic blast radius on zero-day vulnerabilities.',
      'Hybrid Kernel: Keeps drivers in kernel space with sandboxing, but still leaves too many kernel-space vectors.'
    ],
    consequences: {
      positive: [
        'Fault isolation: A crashed driver or compromised filesystem process can be restarted without kernel panics.',
        'Formal verification feasibility for the compact Ring 0 nucleus (<15,000 lines of verified code).',
        'Cryptographic capability model enforces zero-trust permission lineage.'
      ],
      negative: [
        'Slight IPC overhead across domain boundaries, mitigated via shared-memory zero-copy ring buffers.'
      ]
    }
  },
  {
    id: 'ADR-0003',
    title: 'Dual-Engine WebAssembly & Native ATC-VM Runtime Sandbox',
    status: 'Accepted',
    date: '2025-03-01',
    author: 'Platform Runtime Engineering',
    context: 'The system must support both general-purpose multi-language edge workloads (Rust, Go, TypeScript) and high-speed, gas-metered, verifiable smart contract operations.',
    decision: 'Deploy a dual-engine architecture in Execution Domain D4: Wasmtime for general sandboxed POSIX/WASI microservices, and ATC-VM for deterministic blockchain transactions and smart contract state machines.',
    alternatives: [
      'WASM-only: Standard WASM engines lack native deterministic integer gas metering and multi-asset account state transitions.',
      'EVM compatibility only: Inherits EVM quirks (256-bit word overhead, stack limitations, poor formal verification properties).'
    ],
    consequences: {
      positive: [
        'Best-in-class performance: ATC-VM is tailor-made for high-frequency trading and state proofs.',
        'Ecosystem broadness: Developers can run standard WASM modules without rewriting logic in smart contract idioms.'
      ],
      negative: [
        'Maintenance of two distinct sandboxing runtimes within Execution Domain D4.'
      ]
    }
  },
  {
    id: 'ADR-0004',
    title: 'Fixed-Width Bytecode with Explicit Gas Metering for ATC-VM',
    status: 'Accepted',
    date: '2025-03-22',
    author: 'ATC-VM Design Lead',
    context: 'Variable-length instruction sets complicate bytecode decoding, jump target verification, and single-pass static analysis.',
    decision: 'Specify a uniform 32-bit fixed-width instruction format for ATC-VM with explicit 8-bit opcode, 8-bit destination register, and two 8-bit source registers/immediates. Gas metering is pre-computed per basic block to eliminate per-instruction runtime check overhead.',
    alternatives: [
      'Variable-length instructions (x86 style): High code density, but vulnerable to misaligned jump injection and slow decoding.',
      'Stack-based VM (JVM/EVM): Simpler compiler targets, but creates stack manipulation overhead (SWAP, DUP) and limits JIT optimization.'
    ],
    consequences: {
      positive: [
        'Single-pass bytecode verifier can mathematically guarantee bounds and valid jump targets prior to execution.',
        'Direct register-to-register architecture maps 1:1 onto physical modern CPU registers (x86_64 and ARM64).',
        'Significant reduction in gas metering overhead via basic-block cost aggregation.'
      ],
      negative: [
        'Slightly larger binary bytecode size compared to highly compressed variable-length formats.'
      ]
    }
  },
  {
    id: 'ADR-0005',
    title: 'Tendermint-Derived Fast-Finality Consensus with VRF Leader Rotation',
    status: 'Accepted',
    date: '2025-04-12',
    author: 'Consensus & Cryptography Team',
    context: 'Financial settlement and high-frequency operating system state synchronization require deterministic single-slot finality without probabilistic reorg risks.',
    decision: 'Adopt a dual-phase Byzantine Fault Tolerant (BFT) consensus protocol with BLS12-381 signature aggregation and Verifiable Random Function (VRF) leader election, guaranteeing 800ms deterministic finality under 2/3 honest stake.',
    alternatives: [
      'Nakamoto Proof-of-Work: High energy consumption, probabilistic finality with multi-block reorg hazards.',
      'Proof-of-History / Tower BFT: High validator hardware requirements and clock synchronization vulnerability.'
    ],
    consequences: {
      positive: [
        'Zero block reorganization risk once 2f+1 commit signatures are aggregated.',
        'Sub-second finality essential for real-time OS capability revocation and DeFi settlement.',
        'Compact cryptographic state proofs via BLS12-381 multi-signature compression.'
      ],
      negative: [
        'Protocol halts if more than 1/3 of validators go offline simultaneously (safety over liveness).'
      ]
    }
  },
  {
    id: 'ADR-0006',
    title: 'Tiered Merkle Patricia Trie with ShivaFS Append-Only Storage',
    status: 'Accepted',
    date: '2025-05-05',
    author: 'Storage Architecture Group',
    context: 'Blockchain historical states and microkernel audit logs require cryptographically verifiable proof of non-tampering alongside fast NVMe random reads.',
    decision: 'Implement a two-tier storage layer: Tier 1 utilizes an in-memory Modified Merkle Patricia Trie (MMPT) with Blake3 / Poseidon hashing for state verification; Tier 2 utilizes ShivaFS copy-on-write append-only log structured storage on raw NVMe blocks.',
    alternatives: [
      'Standard RocksDB/LevelDB: Suffers from write amplification and compaction spikes that degrade real-time performance.',
      'Plain SQLite: Lacks native Merkle state proof generation and cryptographic rollback audit trails.'
    ],
    consequences: {
      positive: [
        'Predictable, deterministic latency without compaction stalls.',
        'Zero-copy snapshotting and instant historical time-travel debugging.',
        'Native generation of cryptographic inclusion and exclusion proofs for light clients.'
      ],
      negative: [
        'Requires periodic state pruning and archive-node offloading to manage disk footprint.'
      ]
    }
  }
];

// --------------------------------------------------------------------------
// 3. PROTOCOL STANDARDS SPECIFICATIONS (ATC-0001 - ATC-0094)
// --------------------------------------------------------------------------
export const ATC_PROTOCOL_STANDARDS: AtcProtocolStandard[] = [
  {
    code: 'ATC-0001',
    name: 'Decentralized Core Identity (DID & Principal Architecture)',
    category: 'Core',
    status: 'Final',
    version: '1.2.0',
    authors: ['ShivaCore Identity Working Group <identity@atownchain.org>'],
    sections: {
      abstract: 'Defines the foundational decentralized identifier scheme (did:atc:...) and cryptographic principal model powering authentication, capability delegation, and authorization across all Globus OS domains.',
      motivation: 'Traditional OS UID/GID mechanisms are node-local and cannot bridge across distributed clusters, blockchain contracts, or autonomous AI agents.',
      specification: 'Principals are 256-bit universal identifiers derived from the SHA-256 hash of a public key (Ed25519 or Secp256k1) prefixed with type byte. A principal can represent a Human User (0x01), System Service (0x02), Process (0x03), AI Agent (0x04), Smart Contract (0x05), or Validator Node (0x06).',
      terminology: 'Principal: The primary actor bound to a cryptographic identity. DID Document: The verifiable metadata document. Capability Delegation: Signed authorization transferring specific rights with strict expiry.',
      dataStructures: 'struct PrincipalId { type_tag: u8, public_key_hash: [u8; 31] };\nstruct DidDocument { id: String, verification_methods: Vec<VerificationKey>, authentication: Vec<String>, capabilities: Vec<CapabilityRef> };',
      encoding: 'Principals are canonicalized into Bech32m strings starting with prefix "did:atc:". Binary encoding uses standard Big-Endian 32-byte arrays.',
      stateTransitions: 'RegisterPrincipal(Key, Proof) -> Active; RevokePrincipal(Key, Proof) -> Revoked; RotateKey(OldProof, NewKey) -> Active(Key\').',
      validationRules: '1. Signature on DID document registration MUST match the declared public key.\n2. Revocation transactions MUST carry nonce >= current_nonce + 1.\n3. Type byte MUST be within defined enum range [0x01, 0x06].',
      errorConditions: 'ERR_INVALID_SIG (0x1001), ERR_EXPIRED_DELEGATION (0x1002), ERR_NONCE_REGRESSION (0x1003), ERR_UNKNOWN_PRINCIPAL_TYPE (0x1004).',
      securityConsiderations: 'Private keys MUST be stored in hardware secure enclaves (TPM 2.0 / Apple Secure Enclave / Nitro Enclaves) where available. Ephemeral session tokens MUST have TTL <= 3600 seconds.',
      compatibility: 'Fully compliant with W3C DID Core 1.0 specification and verifiable credential data model.',
      testVectors: 'Vector 1: Seed 0x00...01 -> PubKey 0x8a3... -> Principal did:atc:usr1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4.',
      referenceImplementation: 'Located in repository at crates/atc-identity/src/principal.rs',
      changelog: 'v1.0.0: Initial specification.\nv1.1.0: Added AI Agent principal type.\nv1.2.0: Integrated Bech32m checksum validation.'
    }
  },
  {
    code: 'ATC-0002',
    name: 'A-Town Naming Service (ANS) & Decentralized Resolution',
    category: 'Naming',
    status: 'Final',
    version: '1.1.0',
    authors: ['Naming Working Group <naming@atownchain.org>'],
    sections: {
      abstract: 'Specifies the hierarchical decentralized naming system (.atc) for human-readable addressing of nodes, services, contracts, and DID principals.',
      motivation: 'Hexadecimal hashes and 32-byte addresses are prone to human error during transfer and service discovery.',
      specification: 'All names adhere to the UTF-8 DNS-compatible subset ending in .atc. Registrations are auctioned or claimable via 2-year leases managed by the ANS smart contract registry.',
      terminology: 'Node: An individual record in the ANS tree. Resolver: The smart contract translating names into addresses. TTL: Cache duration in seconds.',
      dataStructures: 'struct AnsRecord { owner: PrincipalId, resolver: Address, ttl: u32, content_hash: [u8; 32], text_records: HashMap<String, String> };',
      encoding: 'Namehash algorithm applies recursive Keccak-256 hashing across period-delimited labels: namehash("alice.atc") = H(namehash("atc") || H("alice")).',
      stateTransitions: 'CommitName(CommitHash) -> RevealAndRegister(Name, Secret) -> SetResolver(Address) -> Transfer(NewOwner).',
      validationRules: '1. Minimum label length is 3 characters.\n2. Only alphanumeric lowercase and hyphen characters permitted.\n3. Double hyphens at start or end are forbidden.',
      errorConditions: 'ERR_NAME_TOO_SHORT (0x2001), ERR_INVALID_CHAR (0x2002), ERR_COMMIT_EXPIRED (0x2003), ERR_NAME_ALREADY_TAKEN (0x2004).',
      securityConsiderations: 'Commit-reveal scheme prevents front-running and MEV bot snooping in the blockchain mempool during registration.',
      compatibility: 'Provides backward compatible fallback to DNS via custom DNSSEC gateway.',
      testVectors: 'namehash("atc") = 0x5a18b...; namehash("genesis.atc") = 0x93f41...;',
      referenceImplementation: 'Located at contracts/ans/AnsRegistry.atc',
      changelog: 'v1.0.0: Genesis release.\nv1.1.0: Added text records and multi-chain address mapping.'
    }
  },
  {
    code: 'ATC-0003',
    name: 'Cryptographic Address Format & Derivation (Bech32m)',
    category: 'Crypto',
    status: 'Final',
    version: '1.0.0',
    authors: ['Cryptography Working Group <crypto@atownchain.org>'],
    sections: {
      abstract: 'Standardizes the human-readable, error-detecting address format (atc1...) based on BIP-350 Bech32m with 32-character checksums.',
      motivation: 'Addresses must prevent accidental typos, distinguish mainnet from testnets, and provide mathematical error correction guarantees.',
      specification: 'Address consists of Human Readable Part (HRP) "atc" (mainnet), "tatc" (testnet), or "satc" (shiva devnet), followed by separator "1" and Base32 data containing 1-byte version and 20-byte or 32-byte payload.',
      terminology: 'HRP: Human Readable Part. Witness Program: The cryptographic payload. Checksum: Polymod error-detection code.',
      dataStructures: 'struct AtcAddress { hrp: String, version: u8, payload: Vec<u8> };',
      encoding: 'Bech32m uses generator polynomial [0x3b08e7a0, 0x0b69bc4d, 0x0bfb39ef, 0x3d0263f9, 0x0113f9f7] with constant 0x2bc830a3.',
      stateTransitions: 'DeriveAddress(PublicKey) -> EncodeBech32m(HRP, Version, Hash) -> AddressString.',
      validationRules: '1. Total length MUST NOT exceed 90 characters.\n2. Checksum MUST evaluate to Bech32m constant.\n3. Mixed casing is strictly disallowed.',
      errorConditions: 'ERR_INVALID_CHECKSUM (0x3001), ERR_INVALID_HRP (0x3002), ERR_LENGTH_EXCEEDED (0x3003), ERR_MIXED_CASE (0x3004).',
      securityConsiderations: 'Guarantees detection of up to 4 arbitrary error characters and all burst errors up to length 8.',
      compatibility: 'BIP-173 and BIP-350 compliant encoder/decoder implementations.',
      testVectors: 'Mainnet: atc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4\nTestnet: tatc1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q9sl5k7',
      referenceImplementation: 'Located at crates/atc-crypto/src/address.rs',
      changelog: 'v1.0.0: Initial normative standard.'
    }
  },
  {
    code: 'ATC-0004',
    name: 'Fungible & Non-Fungible Multi-Token Standard (ATC-20 / ATC-721)',
    category: 'Token',
    status: 'Final',
    version: '1.1.0',
    authors: ['Token Standards Committee <tokens@atownchain.org>'],
    sections: {
      abstract: 'Defines unified token interfaces supporting single and batch transfers, metadata schemas, decimal scaling, atomic allowances, and capability-guarded operator approvals.',
      motivation: 'Avoid disparate fragmentation between fungible tokens, semi-fungibles, and NFT assets by providing a unified capability-checked token contract standard.',
      specification: 'All tokens must implement the AtcToken trait: transfer, transfer_from, approve, balance_of, total_supply, and permit (gasless signature approvals).',
      terminology: 'Fungible: Uniform interchangeable assets. NFT: Unique non-divisible token ID. Operator: Delegated capability bearer.',
      dataStructures: 'struct TokenMetadata { name: String, symbol: String, decimals: u8, icon_uri: String };\nstruct TransferEvent { from: Address, to: Address, token_id: u256, amount: u128 };',
      encoding: 'Arguments serialized as canonical 32-byte words Big-Endian.',
      stateTransitions: 'Transfer(from, to, amount) -> checks balance -> updates balances -> emits TransferEvent.',
      validationRules: '1. Recipient address MUST NOT be the zero address (atc1000...).\n2. Sender balance MUST be >= amount.\n3. Transfers to smart contracts MUST invoke atc_onReceived callback.',
      errorConditions: 'ERR_INSUFFICIENT_BALANCE (0x4001), ERR_UNAUTHORIZED_OPERATOR (0x4002), ERR_ZERO_ADDRESS_TRANSFER (0x4003).',
      securityConsiderations: 'Reentrancy guard is baked directly into the ATC-VM runtime opcode dispatcher.',
      compatibility: 'Bridges to ERC-20 and ERC-721 via two-way lock-and-mint bridge relays.',
      testVectors: 'Mint 1,000,000 ATC-20 tokens -> transfer 500 to Bob -> check balances: Alice=999,500, Bob=500.',
      referenceImplementation: 'Located at contracts/standards/AtcToken.atc',
      changelog: 'v1.0.0: Initial draft.\nv1.1.0: Integrated EIP-2612 permit gasless signatures.'
    }
  },
  {
    code: 'ATC-0005',
    name: 'Transaction Envelope, Wire Serialization & Gas Mechanics',
    category: 'Core',
    status: 'Final',
    version: '1.2.0',
    authors: ['Consensus & Node Working Group <consensus@atownchain.org>'],
    sections: {
      abstract: 'Defines the canonical binary transaction structure, Ed25519/Secp256k1 signatures, chain replay protection (EIP-155), gas bidding, and atomic execution envelopes.',
      motivation: 'Transactions must be compact, cryptographically unambiguous, and immune to cross-chain replays and signature malleability.',
      specification: 'Envelope fields: chain_id (u64), nonce (u64), gas_limit (u64), max_fee_per_gas (u64), priority_fee (u64), to (Address), value (u128), data (Vec<u8>), access_list (Vec<StorageKey>), signature (65 bytes).',
      terminology: 'Gas Limit: Maximum computational units. Base Fee: Burned network fee per gas. Priority Fee: Tip paid to block validator.',
      dataStructures: 'struct TransactionEnvelope { chain_id: u64, nonce: u64, gas_limit: u64, max_fee: u64, priority_fee: u64, to: Option<Address>, value: u128, data: Vec<u8>, sig_v: u8, sig_r: [u8; 32], sig_s: [u8; 32] };',
      encoding: 'Serialized using RLP or compact Protobuf binary encoding. Hashed using Blake3 before signature verification.',
      stateTransitions: 'Deduct upfront fee -> Execute payload -> Refund unused gas -> Burn base fee -> Credit priority tip to validator.',
      validationRules: '1. Nonce MUST equal sender account nonce.\n2. Gas limit MUST be >= intrinsic gas (21,000 units + 4 gas per zero byte + 16 gas per non-zero byte).\n3. Sender MUST have sufficient funds for value + gas_limit * max_fee.',
      errorConditions: 'ERR_NONCE_MISMATCH (0x5001), ERR_INSUFFICIENT_FEE (0x5002), ERR_EXCEEDS_BLOCK_GAS (0x5003), ERR_INVALID_SIGNATURE (0x5004).',
      securityConsiderations: 'Strict canonical signature enforcement (s <= n/2) prevents transaction hash mutation in the mempool.',
      compatibility: 'Compatible with standard Web3 signing wallets via EIP-712 typed envelope wrapper.',
      testVectors: 'Payload 0x02f8... -> TxHash 0x48c1...',
      referenceImplementation: 'Located at crates/atc-types/src/transaction.rs',
      changelog: 'v1.0.0: Initial wire format.\nv1.2.0: Introduced dynamic 1559 base-fee burning.'
    }
  },
  {
    code: 'ATC-0092',
    name: 'ATCLang High-Level Language Specification & AST Mapping',
    category: 'VM',
    status: 'Final',
    version: '1.0.0',
    authors: ['Compiler Group <compiler@atownchain.org>'],
    sections: {
      abstract: 'Defines ATCLang, a strongly-typed, memory-safe high-level systems and smart contract programming language that compiles directly to ATC-VM bytecode.',
      motivation: 'Writing contracts in low-level bytecode is error-prone. ATCLang provides modern ergonomics with affine types, capability attributes, and compile-time overflow checks.',
      specification: 'ATCLang features immutable by default bindings, pattern matching, capability annotations (@capability(sys_net_listen)), explicit error handling via Result<T, E>, and zero-cost iterators.',
      terminology: 'AST: Abstract Syntax Tree. Affine Type: A resource that can be used at most once. Capability Guard: A function constraint enforced at compile time and runtime.',
      dataStructures: 'enum AstNode { FunctionDecl { name: String, params: Vec<Param>, body: Block }, LetBinding { name: String, ty: Type, expr: Expr }, ... };',
      encoding: 'Source code in UTF-8. Intermediate representation in SSA (Static Single Assignment) form prior to bytecode emission.',
      stateTransitions: 'Lexing -> Parsing -> Type Checking & Borrow Analysis -> Capability Verification -> Optimization -> Bytecode Emission.',
      validationRules: '1. Recursive calls must have bounded recursion depth (@bounded_recursion(depth)).\n2. Unchecked integer arithmetic is prohibited.\n3. All external calls must handle error states.',
      errorConditions: 'COMPILE_ERR_TYPE_MISMATCH (0x9201), COMPILE_ERR_UNCHECKED_OVERFLOW (0x9202), COMPILE_ERR_MISSING_CAPABILITY (0x9203).',
      securityConsiderations: 'Compiler guarantees memory safety and reentrancy immunity at compile time.',
      compatibility: 'Includes transpiler tools for Solidity and Move smart contracts.',
      testVectors: 'Input: "fn add(a: u64, b: u64) -> u64 { a + b }" -> Output: Opcode ADD R0, R1, R2; RET R0.',
      referenceImplementation: 'Located at compiler/atc-lang/',
      changelog: 'v1.0.0: First stable specification.'
    }
  },
  {
    code: 'ATC-0093',
    name: 'ATC Bytecode Specification (Binary Encoding, Opcodes & Format)',
    category: 'VM',
    status: 'Final',
    version: '1.0.0',
    authors: ['ATC-VM Architecture Group <vm@atownchain.org>'],
    sections: {
      abstract: 'Specifies the binary bytecode format, magic header, constant pool, section table, and complete opcode encoding for the ATC-VM execution engine.',
      motivation: 'Execution engines require unambiguous, deterministic, and easily verifiable binary artifacts with fast zero-copy loading.',
      specification: 'Bytecode begins with 4-byte magic 0x41544356 ("ATCV"), followed by 2-byte version, 2-byte flags, constant pool table, capability section, code section, and debug symbol table.',
      terminology: 'Section Header: Metadata describing segment offsets and lengths. Constant Pool: Deduplicated strings, bigints, and address literals.',
      dataStructures: 'struct BytecodeHeader { magic: [u8; 4], version: u16, flags: u16, section_count: u16 };\nstruct Instruction { opcode: u8, dst: u8, src1: u8, src2_or_imm: u8 };',
      encoding: 'All instructions are exactly 32 bits (4 bytes) wide, aligned on 4-byte boundaries.',
      stateTransitions: 'LoadBinary -> VerifyMagicAndHash -> InspectSections -> LoadConstants -> DispatchToEntrypoint.',
      validationRules: '1. Magic bytes MUST be 0x41544356.\n2. All jump instructions MUST target valid instruction boundaries.\n3. Unreachable dead code after unconditional jump is verified.',
      errorConditions: 'ERR_BAD_MAGIC (0x9301), ERR_CORRUPT_SECTION (0x9302), ERR_UNALIGNED_JUMP (0x9303), ERR_INVALID_OPCODE (0x9304).',
      securityConsiderations: 'Static analysis pass runs in O(N) time with strict bounds to prevent bytecode compression bomb DoS attacks.',
      compatibility: 'Forward compatible section headers allow optional auxiliary sections (e.g. source maps).',
      testVectors: '0x4154435600010000 -> Valid header for Version 1.0.',
      referenceImplementation: 'Located at crates/atc-vm/src/bytecode.rs',
      changelog: 'v1.0.0: Initial release.'
    }
  },
  {
    code: 'ATC-0094',
    name: 'ATC-VM Virtual Machine Architecture & Deterministic Execution Engine',
    category: 'VM',
    status: 'Final',
    version: '1.0.0',
    authors: ['ATC-VM Architecture Group <vm@atownchain.org>'],
    sections: {
      abstract: 'Normative specification of the register-based ATC-VM runtime: execution loop, 256 virtual registers, call frames, linear memory, stack limits, and basic-block gas accounting.',
      motivation: 'Operating systems and blockchain consensus require a 100% deterministic, high-performance sandbox isolated from host side-channel leaks.',
      specification: 'The VM maintains a Program Counter (PC), 256 general-purpose registers (R0-R255), 64KB linear pageable memory, a bounded call stack (depth 1024), and an atomic Gas Meter.',
      terminology: 'Frame: Call context holding registers and return address. Gas Meter: Counter decrementing computational credits. Host Trap: Controlled exit requesting host service.',
      dataStructures: 'struct VmState { pc: u32, registers: [u64; 256], memory: LinearMemory, call_stack: Vec<CallFrame>, gas_remaining: u64 };',
      encoding: 'Memory pages are 4096 bytes each, allocated dynamically on first write up to the configured hard limit (default 32MB).',
      stateTransitions: 'Fetch -> Decode -> GasCheck -> Execute -> AdvancePC.',
      validationRules: '1. Gas meter MUST be decremented BEFORE instruction execution.\n2. If gas_remaining < instruction_cost, abort with OutOfGas.\n3. Memory access beyond allocated pages triggers immediate bounds trap.',
      errorConditions: 'ERR_OUT_OF_GAS (0x9401), ERR_STACK_OVERFLOW (0x9402), ERR_MEMORY_OUT_OF_BOUNDS (0x9403), ERR_ILLEGAL_INSTRUCTION (0x9404).',
      securityConsiderations: 'Constant-time execution paths for all cryptographic opcodes prevent timing side-channel attacks.',
      compatibility: 'Seamless host ABI interface via Contract C4 (Host Trap ABI).',
      testVectors: 'Initialize VM with 100,000 gas -> Execute Fibonacci(10) -> Returns 55, gas consumed 4,210.',
      referenceImplementation: 'Located at crates/atc-vm/src/runtime.rs',
      changelog: 'v1.0.0: Formalized specification.'
    }
  }
];

// --------------------------------------------------------------------------
// 4. COMPLETE REPOSITORY DOCUMENTATION TREE (All Files Across the 10 Areas)
// --------------------------------------------------------------------------
export const REPOSITORY_STANDARD_FILES: StandardDocFile[] = [
  // --- AREA 1: MANDATORY ROOT FILES ---
  {
    path: 'README.md',
    category: 'root',
    title: 'Project Overview & Elevator Pitch',
    priority: 'mandatory',
    status: 'Stable',
    version: '3.0.0',
    owner: 'Core Team',
    summary: 'Central front page: elevator pitch, badges, architecture map, quick start, install instructions, and community links.',
    content: `# A-TownChain / ShivaCore / Globus OS / ATC-VM
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/a-townchain/core/actions)
[![License: Dual MIT/Apache-2.0](https://img.shields.io/badge/License-Dual%20MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Security Policy](https://img.shields.io/badge/Security-Enforced-red.svg)](SECURITY.md)
[![Docs](https://img.shields.io/badge/Documentation-ATC--DOC-indigo.svg)](docs/README.md)

> **High-Assurance Distributed Operating System, Microkernel Nucleus, Deterministic Virtual Machine & Blockchain Infrastructure.**

---

## 🌟 Key Pillars
1. **ShivaCore Microkernel (Ring 0)**: Minimal trusted computing base (<15k LoC) in Rust, providing lockless IPC, 4-level paging, and hardware capability delegation.
2. **ATC-VM (Execution Sandbox)**: Deterministic 32-bit register-based virtual machine with compile-time gas accounting and sub-millisecond execution.
3. **Globus OS Platform**: 5 vertical domains (D1-D5) unified across 7 horizontal control planes (Security, Governance, Observability, Audit, Policy, Resource, Config).
4. **A-TownChain Protocol**: Tendermint-derived BFT consensus with sub-second deterministic finality and cryptographic capability governance.

---

## 🚀 Quick Start

\`\`\`bash
# 1. Clone the repository with submodules
git clone --recurse-submodules https://github.com/a-townchain/core.git
cd core

# 2. Verify toolchains (Rust nightly + LLVM 18)
cargo --version
rustup target add x86_64-unknown-none

# 3. Build ShivaCore Microkernel & ATC-VM
cargo build --release --workspace

# 4. Launch in QEMU Virtual Machine
./scripts/run-qemu.sh
\`\`\`

---

## 📚 Documentation Index
- [Full Technical Architecture](docs/ARCHITECTURE.md)
- [ATC Protocol Standards (ATC-0001 to ATC-0094)](docs/standards/README.md)
- [ATC-VM Modular Documentation](docs/ATC_VM_ARCHITECTURE.md)
- [Blockchain Protocol Docs](docs/blockchain/CONSENSUS.md)
- [Security & Threat Model](docs/security/SECURITY_ARCHITECTURE.md)
- [Architecture Decision Records (ADRs)](docs/adr/0001-use-rust.md)

---

## 📜 License
Dual licensed under [MIT](LICENSE) or [Apache 2.0](LICENSE) at your option.
`
  },
  {
    path: 'LICENSE',
    category: 'root',
    title: 'Dual MIT / Apache-2.0 License Terms',
    priority: 'mandatory',
    status: 'Stable',
    version: '1.0.0',
    owner: 'Legal & Governance',
    summary: 'Dual permissive license granting royalty-free usage, modification, and distribution with patent grant protections.',
    content: `A-TownChain / ShivaCore / Globus OS / ATC-VM Software License
Copyright (c) 2025-2026 A-TownChain Core Contributors.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
`
  },
  {
    path: 'CONTRIBUTING.md',
    category: 'root',
    title: 'Developer & Contributor Guidelines',
    priority: 'recommended',
    status: 'Living',
    version: '2.0.0',
    owner: 'Engineering Lead',
    summary: 'Branch naming conventions, pull request workflows, DCO signoff, testing requirements, and code style rules.',
    content: `# Contributing to A-TownChain Ecosystem

Thank you for your interest in contributing to ShivaCore, Globus OS, and ATC-VM!

## 1. Development Workflow
1. Fork the repository and create a branch from \`main\`.
2. Branch naming convention:
   - \`feat/atc-xxxx-short-description\`
   - \`fix/issue-number-bug-title\`
   - \`docs/update-section-name\`
3. All commits must include Developer Certificate of Origin (DCO) sign-off (\`git commit -s\`).

## 2. Quality & Verification Gates
Before opening a PR, ensure all gates pass locally:
\`\`\`bash
# Linting & code formatting
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Unit & integration tests
cargo test --workspace

# Fuzzing & invariant checks
cargo test --test differential_vm_tests
\`\`\`

## 3. Pull Request Requirements
- Complete the [PULL_REQUEST_TEMPLATE.md](.github/PULL_REQUEST_TEMPLATE.md).
- Link corresponding GitHub issues and normative standard codes (e.g. ATC-0094).
- Ensure 100% test pass rate in CI.
`
  },
  {
    path: 'CODE_OF_CONDUCT.md',
    category: 'root',
    title: 'Community Code of Conduct',
    priority: 'recommended',
    status: 'Living',
    version: '2.1.0',
    owner: 'Community Team',
    summary: 'Standards of behavior, inclusive language, conflict resolution, and reporting guidelines based on Contributor Covenant v2.1.',
    content: `# Contributor Covenant Code of Conduct

## Our Pledge
We as members, contributors, and leaders pledge to make participation in our
community a harassment-free experience for everyone, regardless of age, body
size, visible or invisible disability, ethnicity, sex characteristics, gender
identity and expression, level of experience, education, socio-economic status,
nationality, personal appearance, race, religion, or sexual identity.

## Our Standards
Examples of behavior that contributes to a positive environment include:
- Demonstrating empathy and kindness toward other people
- Being respectful of differing viewpoints and architectural trade-offs
- Giving and gracefully accepting constructive feedback
- Accepting responsibility and apologizing to those affected by our mistakes

## Enforcement
Instances of abusive, harassing, or otherwise unacceptable behavior may be
reported to the community conduct team at conduct@atownchain.org.
`
  },
  {
    path: 'SECURITY.md',
    category: 'root',
    title: 'Vulnerability Disclosure & Security Policy',
    priority: 'mandatory',
    status: 'Stable',
    version: '2.0.0',
    owner: 'Security Lead',
    summary: 'SLA response times, encrypted PGP keys, supported versions, bug bounty guidelines, and safe harbor rules.',
    content: `# Security Policy

## Supported Versions
| Component | Supported Release | Security Fix Status |
|---|---|---|
| ShivaCore Kernel | 3.x, 2.x | Actively Maintained |
| ATC-VM | 1.x | Actively Maintained |
| Globus OS | 3.x | Actively Maintained |
| Legacy Protocols | < 1.0 | Deprecated / No Fixes |

## Reporting a Vulnerability
**DO NOT FILE PUBLIC GITHUB ISSUES FOR SECURITY VULNERABILITIES.**

Please report sensitive security vulnerabilities privately to:
- **Email**: security@atownchain.org
- **PGP Key Fingerprint**: \`7E4A 9102 3B4C 8D9E 1234 5678 9ABC DEF0 1122 3344\`

### Response Timeline
- **Initial Acknowledgment**: Within 12 hours.
- **Triage & Severity Assessment**: Within 24 hours.
- **Fix Delivery & Coordinated Disclosure**: 7 to 30 days depending on severity.

### Bug Bounty
Eligible critical vulnerabilities in the ShivaCore Ring 0 Kernel, Consensus engine, or ATC-VM execution sandbox qualify for rewards up to $100,000 USD equivalent.
`
  },
  {
    path: 'CHANGELOG.md',
    category: 'root',
    title: 'Living Version History & Release Notes',
    priority: 'recommended',
    status: 'Living',
    version: '3.0.0',
    owner: 'Release Manager',
    summary: 'Chronological list of all user-facing and breaking changes following the Keep a Changelog standard.',
    content: `# Changelog
All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0] - 2026-09-01
### Added
- Reference Architecture v3 (Formal Platform Architecture) with 5 Domains and 7 Control Planes.
- Formalized Contracts C1 through C5 including Dual Host Trap ABI vs Syscall ABI.
- 15 modular ATC-VM architecture specifications (ISA, Gas Model, Memory Model, Verifier).
- 14 modular blockchain architecture documents (Consensus, State Machine, P2P, Staking).
- Standardized ATC-DOC-001 through ATC-DOC-008 meta-framework.

### Changed
- Refactored Ring 0 microkernel syscall dispatcher to use lockless SPSC ring buffers.
- Unified resource and capability handles under GOS-RES-001 and GOS-ID-001 schemas.

### Security
- Enforced 12-stage Capability Verification Pipeline for all privileged syscall operations.
`
  },
  {
    path: 'ROADMAP.md',
    category: 'root',
    title: 'Ecosystem Engineering Roadmap',
    priority: 'recommended',
    status: 'Living',
    version: '3.0.0',
    owner: 'Architecture Committee',
    summary: 'Quarterly development milestones spanning ShivaCore Kernel, ATC-VM, Consensus, Aurora AI, and Ecosystem tooling.',
    content: `# Ecosystem Engineering Roadmap (2025 - 2027)

## Q3 2025: Foundation & Formalization (Completed)
- [x] Formalize Reference Architecture v3 (GOS-ARCH-001).
- [x] Complete 32-bit register-based ATC-VM prototype and test vectors.
- [x] Implement lockless IPC microkernel nucleus for x86_64.

## Q4 2025: Testnet & Execution Hardening (In Progress)
- [ ] Deploy A-TownChain Public Testnet "Shiva-1".
- [ ] Differential testing harness for ATC-VM vs WebAssembly runtimes.
- [ ] Integrate Aurora AI agent principal sandboxing with dynamic capability quotas.

## Q1 2026: Multi-Architecture & Formal Verification
- [ ] ARM64 and RISC-V 64-bit ports for ShivaCore HAL.
- [ ] Coq / Isabelle formal verification proofs for ATC-VM basic block gas metering.
- [ ] Complete decentralized ANS registry deployment.

## Q2 2026+: Mainnet & Production Readiness
- [ ] Comprehensive third-party security audits with top-tier audit firms.
- [ ] Decentralized Genesis ceremony and validator onboarding.
- [ ] Native hardware accelerator drivers for WebGPU and NPU co-processors.
`
  },
  {
    path: 'AUTHORS.md',
    category: 'root',
    title: 'Core Maintainers & Contributors',
    priority: 'optional',
    status: 'Living',
    version: '1.0.0',
    owner: 'Core Team',
    summary: 'List of founding architects, maintainers, and institutional contributors.',
    content: `# Project Authors & Maintainers

## Core Architecture & Kernel
- Michael Worob (@mworob) - Lead System Architect
- ShivaCore Engineering Working Group
- A-TownChain Protocol Research Group

## Working Groups
- **ATC-VM Design**: Virtual machine, instruction set, register architecture, gas models.
- **Consensus & Cryptography**: BFT protocol, BLS12-381 signatures, VRF leader rotation.
- **Kernel & Microkernel HAL**: x86_64, ARM64, page tables, lockless IPC.
- **Language & Compiler**: ATCLang, bytecode emitter, formal verifiers.
`
  },
  {
    path: 'NOTICE',
    category: 'root',
    title: 'Legal Attribution & Third-Party Notices',
    priority: 'optional',
    status: 'Stable',
    version: '1.0.0',
    owner: 'Legal Team',
    summary: 'Formal notice of third-party open source inclusions and trademark declarations.',
    content: `A-TownChain / ShivaCore / Globus OS / ATC-VM
Copyright (c) 2025-2026 A-TownChain Core Contributors.

This product includes software developed by the A-TownChain Open Source Project.
Trademarks: "A-TownChain", "ShivaCore", "Globus OS", and "ATC-VM" are trademarks
of the A-TownChain Foundation.

Third-Party Acknowledgments:
- Rust standard libraries (c) Mozilla Foundation / Rust Contributors.
- Wasmtime WebAssembly engine (c) Bytecode Alliance.
- Tendermint consensus specifications (c) Tendermint / Interchain Foundation.
`
  },
  {
    path: '.gitignore',
    category: 'root',
    title: 'Standard Repository Git Ignore File',
    priority: 'mandatory',
    status: 'Stable',
    version: '1.0.0',
    owner: 'DevOps',
    summary: 'Excludes compilation artifacts, bytecode binaries, private keys, and OS temporary files.',
    content: `# Build Artifacts
target/
dist/
build/
*.o
*.a
*.so
*.dylib
*.dll
*.iso
*.img

# Node / Web
node_modules/
npm-debug.log*
yarn-debug.log*
.pnpm-debug.log*

# Bytecode & State
*.atcb
*.mpt
*.db
*.wal

# Secrets & Keys (NEVER COMMIT)
*.key
*.pem
*.priv
*.sec
.env
.env.local

# IDE & OS
.DS_Store
Thumbs.db
.vscode/
.idea/
*.swp
*~
`
  },

  // --- AREA 2: GITHUB-SPECIFIC GOVERNANCE (.github/...) ---
  {
    path: '.github/CODEOWNERS',
    category: 'github',
    title: 'Repository Code Ownership Governance',
    priority: 'mandatory',
    status: 'Stable',
    version: '1.0.0',
    owner: 'Governance',
    summary: 'Enforces required review from specific engineering groups for sensitive subsystems.',
    content: `# Global Default
* @a-townchain/core-maintainers

# Critical Microkernel Ring 0
/kernel/ @a-townchain/kernel-team
/src/kernel/ @a-townchain/kernel-team

# ATC-VM Execution Sandbox
/crates/atc-vm/ @a-townchain/vm-team
/docs/ATC_VM_*.md @a-townchain/vm-team

# Blockchain & Consensus
/crates/atc-consensus/ @a-townchain/consensus-team
/docs/blockchain/ @a-townchain/consensus-team

# Security Architecture & Policies
/SECURITY.md @a-townchain/security-team
/docs/security/ @a-townchain/security-team
`
  },
  {
    path: '.github/PULL_REQUEST_TEMPLATE.md',
    category: 'github',
    title: 'Standard Pull Request Verification Checklist',
    priority: 'mandatory',
    status: 'Stable',
    version: '1.0.0',
    owner: 'Engineering Lead',
    summary: 'Standardized PR description, checklist, breaking change warnings, and test proof requirements.',
    content: `## 📌 Pull Request Description
<!-- Summarize the changes introduced by this PR. Include motivation and context. -->

## 🔗 Related Issues & Standards
- Closes #
- Implements / References Standard: [e.g. ATC-0094, GOS-ARCH-001]
- ADR Reference: [e.g. ADR-0004]

## 🧪 Verification & Testing Completed
- [ ] \`cargo test --workspace\` passed cleanly
- [ ] \`cargo clippy --workspace -- -D warnings\` passed with 0 warnings
- [ ] Added unit tests covering new code paths
- [ ] Added / updated documentation in \`docs/\`
- [ ] Invariant check: No direct Ring 0 syscall access from Ring 3 without capability validation

## ⚠️ Breaking Change Warning
- [ ] This PR contains breaking API or ABI changes (Requires SemVer Major bump & ADR)
`
  },
  {
    path: '.github/ISSUE_TEMPLATE/bug_report.md',
    category: 'github',
    title: 'Bug Report Issue Template',
    priority: 'recommended',
    status: 'Stable',
    version: '1.0.0',
    owner: 'QA',
    summary: 'Structured bug reporting form with environment, repro steps, and expected behavior.',
    content: `---
name: Bug Report
about: Create a report to help us improve A-TownChain / ShivaCore
title: '[BUG] '
labels: 'bug, triage'
assignees: ''
---

### 🐛 Describe the Bug
A clear and concise description of what the bug is.

### 🔄 Steps to Reproduce
1. Command run: \`...\`
2. Arguments passed: \`...\`
3. Error encountered: \`...\`

### 💻 Environment
- OS / Platform: [e.g. ShivaCore Native x86_64, Linux, macOS]
- Kernel / VM Version: [e.g. 3.0.0]
- Rust Toolchain: [e.g. nightly-2026-08-15]

### 🎯 Expected Behavior
What should have happened according to the specification.
`
  },
  {
    path: '.github/ISSUE_TEMPLATE/feature_request.md',
    category: 'github',
    title: 'Feature Request Issue Template',
    priority: 'recommended',
    status: 'Stable',
    version: '1.0.0',
    owner: 'Product',
    summary: 'Structured proposal for new features, syscalls, or opcodes.',
    content: `---
name: Feature Request
about: Suggest a feature, syscall or opcode extension
title: '[FEAT] '
labels: 'enhancement, discussion'
assignees: ''
---

### 🚀 Motivation & Use Case
What problem does this feature solve? Which domain (D1-D5) does it belong to?

### 💡 Proposed Solution
Describe the high-level architecture and interfaces.

### ⚖️ Architectural Impact
- Does it require a new ADR? (Yes / No)
- Does it require an ATC Standard update? (e.g. ATC-0093 for new opcodes)
- Security considerations:
`
  },
  {
    path: '.github/ISSUE_TEMPLATE/security_issue.md',
    category: 'github',
    title: 'Security Advisory Redirection Template',
    priority: 'mandatory',
    status: 'Stable',
    version: '1.0.0',
    owner: 'Security',
    summary: 'Prevents accidental public posting of zero-day vulnerabilities.',
    content: `---
name: Security Vulnerability
about: Privately report a security issue
title: '[SECURITY ADVISORY]'
labels: 'security'
assignees: ''
---

⚠️ **DO NOT FILE PUBLIC ISSUES FOR EXPLOITS OR VULNERABILITIES.**

Please follow our [SECURITY.md](../SECURITY.md) guidelines:
Email security details and PoCs encrypted to: **security@atownchain.org**.
`
  },
  {
    path: '.github/dependabot.yml',
    category: 'github',
    title: 'Automated Dependency Security Updates',
    priority: 'recommended',
    status: 'Stable',
    version: '1.0.0',
    owner: 'DevOps',
    summary: 'Configures weekly automated dependency auditing for Cargo and npm ecosystems.',
    content: `version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 5

  - package-ecosystem: "npm"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 5
`
  },
  {
    path: '.github/workflows/ci.yml',
    category: 'github',
    title: 'Comprehensive CI Pipeline Workflow',
    priority: 'mandatory',
    status: 'Stable',
    version: '1.0.0',
    owner: 'DevOps',
    summary: 'Executes automated formatting checks, clippy linter, unit tests, and cross-compilation in GitHub Actions.',
    content: `name: Continuous Integration

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  lint-and-test:
    name: Lint, Test & Verify
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive

      - name: Install Rust Nightly
        uses: dtolnay/rust-toolchain@nightly
        with:
          components: rustfmt, clippy

      - name: Rust Cache
        uses: Swatinem/rust-cache@v2

      - name: Code Formatting Check
        run: cargo fmt --all -- --check

      - name: Clippy Linter Check
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings

      - name: Run Test Suite
        run: cargo test --workspace --verbose

      - name: Verify Documentation Links
        run: ./scripts/verify-doc-links.sh
`
  },

  // --- AREA 3: TECHNICAL ARCHITECTURE SUITE (docs/...) ---
  {
    path: 'docs/ARCHITECTURE.md',
    category: 'arch',
    title: 'System Architecture & 5-Domain Overview',
    priority: 'mandatory',
    status: 'Stable',
    version: '3.0.0',
    owner: 'Lead Architect',
    summary: 'Comprehensive formal platform architecture (GOS-ARCH-001 v3.0): Experience, Application, Platform, Execution, and System domains.',
    content: `# Globus OS & ShivaCore Reference Architecture v3.0

## 1. Domain Separation Hierarchy
\`\`\`
+-------------------------------------------------------------+
| D1: Experience Domain (Aurora UI, WASM Shell, CLI, AR/VR)   |
+-------------------------------------------------------------+
                            | Contract C1 (Type-Safe RPC)
+-------------------------------------------------------------+
| D2: Application Domain (.gapp, dApps, DeFi, IDE, Wallet)    |
+-------------------------------------------------------------+
                            | Contract C2 (Platform SDKs)
+-------------------------------------------------------------+
| D3: Platform Domain (API Gateway, Middleware, AI, Chain)    |
+-------------------------------------------------------------+
                            | Contract C3 (Runtime Config)
+-------------------------------------------------------------+
| D4: Execution Domain (ATC-VM, WASM Runtime, ShivaBox)       |
+-------------------------------------------------------------+
                            | Contract C4 (Host Trap ABI / Syscall ABI)
+-------------------------------------------------------------+
| D5: System Domain (ShivaCore Microkernel Ring 0, VFS, HAL)  |
+-------------------------------------------------------------+
\`\`\`

## 2. Seven Horizontal Control Planes
All domains intersect with 7 transversal control planes:
1. **Security Plane**: DID principals, cryptographic capability DAG, zero-trust token verification.
2. **Governance Plane**: On-chain DAO rules, Quadratic voting, EIP-1559 burn mechanics, compliance.
3. **Observability Plane**: Lockless metrics ring buffer, structured JSON logs, W3C trace propagation.
4. **Audit Plane**: Merkle-linked tamper-proof audit trail, cryptographic state provenance.
5. **Policy Plane**: OPA/Rego declarative rule enforcement, human-in-the-loop tool approvals.
6. **Resource Plane**: Unified quota accounting across CPU, RAM, Storage, Bandwidth, and Gas.
7. **Configuration Plane**: Immutable configuration trie with atomic swap.
`
  },
  {
    path: 'docs/DESIGN.md',
    category: 'arch',
    title: 'Design Philosophy & Core Invariants',
    priority: 'mandatory',
    status: 'Stable',
    version: '3.0.0',
    owner: 'Architecture Committee',
    summary: 'Foundational architectural principles: Least Privilege, Memory Safety, Determinism, and Zero-Ambiguity.',
    content: `# System Design Philosophy & Invariants

## Core Principles
1. **Zero-Trust by Default**: No code in user space possesses implicit permissions; all actions require explicit capability tokens.
2. **Deterministic Execution**: The exact same inputs and gas limits produce bit-identical states across all hardware architectures.
3. **Fault Containment**: A crash in any driver, filesystem, or application is strictly isolated and cannot destabilize the Ring 0 microkernel.
4. **Clean Layering**: Lower layers never depend on higher layers; communication across boundaries strictly honors formal contracts C1-C5.
`
  },
  {
    path: 'docs/COMPONENTS.md',
    category: 'arch',
    title: 'Component Catalog & Subsystem Dependencies',
    priority: 'mandatory',
    status: 'Stable',
    version: '3.0.0',
    owner: 'Core Team',
    summary: 'Inventory of all platform crates, services, and modules with inbound/outbound dependencies.',
    content: `# Component Catalog

| Component | Domain | Path | Language | Purpose |
|---|---|---|---|---|
| shivacore-kernel | D5: System | /kernel/core | Rust (#![no_std]) | Ring 0 microkernel nucleus |
| shivacore-hal | D5: System | /kernel/hal | Rust / Asm | Hardware abstraction layer |
| atc-vm | D4: Execution | /crates/atc-vm | Rust | 32-bit register virtual machine |
| wasm-runner | D4: Execution | /crates/wasm | Rust | Wasmtime isolation sandbox |
| shivabox | D4: Execution | /crates/box | Rust | Micro-container containerization |
| atc-consensus | D3: Platform | /crates/consensus | Rust | BFT state machine replication |
| gateway-svc | D3: Platform | /services/gateway | Rust | Type-safe API & capability router |
| aurora-compositor| D1: Experience | /ui/aurora | TypeScript/WASM | Zero-latency desktop compositor |
`
  },
  {
    path: 'docs/DATA_MODEL.md',
    category: 'arch',
    title: 'Unified Global Data Models (GOS-RES, ID, AUD)',
    priority: 'mandatory',
    status: 'Stable',
    version: '3.0.0',
    owner: 'Architecture Group',
    summary: 'Standardized schemas for Resources (GOS-RES-001), Principals (GOS-ID-001), and Events (GOS-AUD-001).',
    content: `# Unified Global Data Models

## 1. Universal Resource Schema (GOS-RES-001)
Every entity managed by Globus OS adheres to:
\`\`\`json
{
  "resource_id": "res_01j7x8a9b2c3d4e5f6g7h8j9",
  "owner_principal": "did:atc:usr1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
  "namespace": "storage.shivacore.user.alice",
  "capability_handle": "0x7f8a9b0c1d2e3f40",
  "quota_soft_limit": 1073741824,
  "quota_hard_limit": 2147483648,
  "current_usage": 524288000,
  "lifecycle_state": "Active"
}
\`\`\`

## 2. Universal Identity Model (GOS-ID-001)
Supports typed principals (User, Service, Process, Agent, Contract, Validator).

## 3. Universal Event & Audit Schema (GOS-AUD-001)
Contains W3C correlation ID, actor principal, target resource, policy decision, and Merkle hash link.
`
  },
  {
    path: 'docs/security/SECURITY_ARCHITECTURE.md',
    category: 'arch',
    title: 'Defense-in-Depth & 12-Stage Capability Pipeline',
    priority: 'mandatory',
    status: 'Stable',
    version: '3.0.0',
    owner: 'Security Lead',
    summary: 'The 12-stage capability pipeline, memory isolation, ASLR, DEP, and cryptographic key hierarchy.',
    content: `# Security Architecture & 12-Stage Pipeline

## The 12-Stage Capability Verification Flow
Every privileged operation traversing the Syscall or Host ABI undergoes 12 sequential validation gates:
1. **Request Reception**: Capture ABI parameters and calling thread context.
2. **ABI Boundary Check**: Verify pointer alignment and parameter bounds.
3. **Parameter Validation**: Reject malformed or out-of-range arguments.
4. **Caller Identity Verification**: Resolve calling process DID principal.
5. **Namespace Isolation**: Confirm access is confined to target namespace.
6. **Handle Resolution**: Translate user handle to kernel capability node.
7. **Capability Verification**: Confirm capability node is active and valid.
8. **Rights Bitmask Check**: Check specific permission bits (Read, Write, Execute).
9. **Policy Engine Evaluation**: Execute OPA/Rego and BPF security rules.
10. **Quota & Rate Limit Check**: Deduct resource balance and verify limits.
11. **Resource Access**: Execute the hardware or memory operation.
12. **Audit Logging**: Write signed event to lockless Merkle audit log.
`
  },
  {
    path: 'docs/security/THREAT_MODEL.md',
    category: 'arch',
    title: 'STRIDE Threat Modeling & Risk Mitigation',
    priority: 'mandatory',
    status: 'Stable',
    version: '3.0.0',
    owner: 'Security Team',
    summary: 'Formal STRIDE threat matrix mapping potential attacks to architectural defenses.',
    content: `# STRIDE Threat Model

| Threat Class | Potential Attack Vector | Architectural Mitigation |
|---|---|---|
| **Spoofing** | Forging caller DID principal | Cryptographic Ed25519 token signatures & kernel-enforced thread contexts |
| **Tampering** | Mutating VM memory or syscall arguments | Copy-in of arguments to kernel space; linear memory bounds traps |
| **Repudiation** | Denying high-value transaction or capability delegation | Merkle-chained immutable audit log with BLS signature anchoring |
| **Information Disclosure** | Reading another process's address space | Hardware 4-level paging (PML4) with separate CR3 page tables per process |
| **Denial of Service** | Infinite loops in smart contracts or worker threads | Deterministic basic-block gas metering & cgroup v2 CPU quota limits |
| **Elevation of Privilege** | Exploiting Ring 0 kernel vulnerabilities | Strict microkernel design; capabilities cannot be minted without parent root key |
`
  },

  // --- AREA 5: API DOCUMENTATION SUITE (docs/api/...) ---
  {
    path: 'docs/api/RPC_API.md',
    category: 'api',
    title: 'JSON-RPC 2.0 Node Endpoint Specification',
    priority: 'mandatory',
    status: 'Stable',
    version: '2.0.0',
    owner: 'API Team',
    summary: 'Complete JSON-RPC 2.0 interface specs for node balance queries, transaction submission, and smart contract calls.',
    content: `# JSON-RPC 2.0 Specification

## Standard Endpoints
- **HTTP**: \`https://node.atownchain.org/rpc\`
- **WebSocket**: \`wss://node.atownchain.org/ws\`

### 1. \`atc_getBalance\`
Returns the spendable balance of an address.
\`\`\`json
// Request
{
  "jsonrpc": "2.0",
  "method": "atc_getBalance",
  "params": ["atc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4", "latest"],
  "id": 1
}

// Response
{
  "jsonrpc": "2.0",
  "result": "0x1bc16d674ec80000", // 2.0 ATC in base units (10^18)
  "id": 1
}
\`\`\`

### 2. \`atc_sendRawTransaction\`
Submits a serialized signed transaction envelope to the mempool.

### 3. \`atc_call\`
Executes a read-only message call directly against the state without creating a transaction.
`
  },
  {
    path: 'api/openapi.yaml',
    category: 'api',
    title: 'OpenAPI 3.1 REST API Specification',
    priority: 'mandatory',
    status: 'Stable',
    version: '3.1.0',
    owner: 'API Team',
    summary: 'Machine-readable OpenAPI 3.1 YAML definition for all REST endpoints.',
    content: `openapi: 3.1.0
info:
  title: A-TownChain Node & Globus OS Gateway REST API
  version: 3.0.0
  description: Public REST interface for blockchain queries and system services.
paths:
  /v1/chain/info:
    get:
      summary: Get current blockchain status and block height
      responses:
        '200':
          description: Successful response
          content:
            application/json:
              schema:
                type: object
                properties:
                  chain_id:
                    type: integer
                  block_height:
                    type: integer
                  block_hash:
                    type: string
  /v1/account/{address}:
    get:
      summary: Fetch account state and nonces
      parameters:
        - name: address
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: Account details returned
`
  },

  // --- AREA 6: SOFTWARE-SPECIFIC MODULAR ATC-VM SPECS (15 Docs) ---
  {
    path: 'docs/ATC_VM_ARCHITECTURE.md',
    category: 'vm',
    title: 'ATC-VM Architecture & Sandboxing Engine',
    priority: 'mandatory',
    status: 'Final',
    version: '1.0.0',
    owner: 'ATC-VM Team',
    summary: 'Executive technical overview of the 32-bit register-based virtual machine architecture.',
    content: `# ATC-VM Architecture Specification

## 1. Engine Core Characteristics
- **Instruction Set**: 32-bit uniform fixed-width instructions.
- **Register File**: 256 virtual 64-bit general-purpose registers per frame (R0 - R255).
- **Call Stack**: Fixed maximum depth of 1024 frames to guarantee deterministic stack bounds.
- **Memory Model**: Linear pageable memory (4KB pages), isolated per contract context.
- **Gas Accounting**: Pre-calculated basic block gas metering eliminating runtime branching cost.
`
  },
  {
    path: 'docs/ATC_VM_ISA.md',
    category: 'vm',
    title: 'ATC-VM Instruction Set Architecture (ISA)',
    priority: 'mandatory',
    status: 'Final',
    version: '1.0.0',
    owner: 'ATC-VM Team',
    summary: 'Exhaustive instruction set manual detailing arithmetic, logical, memory, control flow, and cryptographic opcodes.',
    content: `# ATC-VM Instruction Set Architecture (ISA)

## Opcode Table Summary
| Hex | Mnemonic | Format | Gas Base | Description |
|---|---|---|---|---|
| 0x00 | NOP | \`NOP\` | 1 | No operation |
| 0x01 | ADD | \`ADD Rd, Rs1, Rs2\` | 3 | Integer addition: Rd = Rs1 + Rs2 |
| 0x02 | SUB | \`SUB Rd, Rs1, Rs2\` | 3 | Integer subtraction: Rd = Rs1 - Rs2 |
| 0x03 | MUL | \`MUL Rd, Rs1, Rs2\` | 5 | Integer multiplication: Rd = Rs1 * Rs2 |
| 0x04 | DIV | \`DIV Rd, Rs1, Rs2\` | 10 | Unsigned division with zero-check |
| 0x10 | LOAD | \`LOAD Rd, [Rs1 + imm8]\` | 5 | Load 64-bit word from linear memory |
| 0x11 | STORE | \`STORE [Rd + imm8], Rs1\`| 8 | Store 64-bit word to linear memory |
| 0x20 | JMP | \`JMP target_pc\` | 4 | Unconditional jump |
| 0x21 | JEQ | \`JEQ Rs1, Rs2, target\` | 6 | Conditional jump if equal |
| 0x30 | TRAP | \`TRAP trap_code, Rarg\` | 20 | Host Trap Call (Contract C4) |
`
  },
  {
    path: 'docs/ATC_VM_GAS_MODEL.md',
    category: 'vm',
    title: 'Deterministic Gas Metering & Cost Schedule',
    priority: 'mandatory',
    status: 'Final',
    version: '1.0.0',
    owner: 'ATC-VM Team',
    summary: 'Mathematical gas model, basic block pre-deduction, memory expansion quadratic fees, and opcode costs.',
    content: `# ATC-VM Gas Accounting Model

## 1. Basic Block Pre-Deduction
Instead of checking the gas counter at every single instruction (which incurs branch prediction penalties),
the compiler clusters instructions into Basic Blocks (BBs) with single entry and exit points.
\`\`\`rust
// Compiled basic block header
gas_remaining = gas_remaining.checked_sub(BB_TOTAL_COST)?;
\`\`\`

## 2. Memory Expansion Cost Formula
Linear memory expansion is charged quadratically to prevent memory exhaustion attacks:
\`\`\`
Cost(N) = (N * Words) + (N^2 / 512)
\`\`\`
Where N is the total active 4096-byte memory page count.
`
  },
  {
    path: 'docs/ATC_VM_MEMORY_MODEL.md',
    category: 'vm',
    title: 'Linear Memory & Page Management',
    priority: 'mandatory',
    status: 'Final',
    version: '1.0.0',
    owner: 'ATC-VM Team',
    summary: 'Isolated address spaces, zero-initialized 4KB pages, alignment guarantees, and boundary trapping.',
    content: `# ATC-VM Memory Model

## Memory Topology
- **Linear Space**: Byte-addressable space up to a hard ceiling of 32MB per execution instance.
- **Page Allocation**: Dynamic lazy allocation in 4KB chunks upon first write.
- **Bounds Checking**: Attempted reads or writes beyond active allocations trigger immediate trapped execution with status \`ERR_MEMORY_OUT_OF_BOUNDS\`.
`
  },
  {
    path: 'docs/ATC_VM_VERIFIER.md',
    category: 'vm',
    title: 'Static Bytecode Verifier & Safety Proofs',
    priority: 'mandatory',
    status: 'Final',
    version: '1.0.0',
    owner: 'ATC-VM Team',
    summary: 'Linear-time static bytecode verifier rules preventing jump-into-instruction and stack corruption.',
    content: `# ATC-VM Static Bytecode Verifier

## Verification Invariants
Before execution, bytecode passes an O(N) verification pass:
1. **Instruction Alignment**: Jump targets must strictly land on 4-byte boundaries.
2. **Register Bounds**: All operand registers must be within index range [0, 255].
3. **No Dynamic Code Generation**: Self-modifying bytecode is structurally impossible.
4. **CFG Acyclicity Validation**: Loop counters must possess bounded termination bounds.
`
  },

  // --- AREA 7: BLOCKCHAIN-SPECIFIC MODULAR DOCS (14 Docs) ---
  {
    path: 'docs/blockchain/CONSENSUS.md',
    category: 'blockchain',
    title: 'BFT Consensus & Fast Finality Protocol',
    priority: 'mandatory',
    status: 'Final',
    version: '2.0.0',
    owner: 'Consensus Team',
    summary: 'Two-phase Byzantine Fault Tolerant state machine replication with BLS12-381 signature aggregation.',
    content: `# Consensus Protocol Specification

## Overview
A-TownChain implements a high-throughput BFT protocol:
- **Round Duration**: 800ms slot times.
- **Finality**: Instant single-slot finality once 2/3+ stake commits.
- **Leader Election**: Verifiable Random Function (VRF) with stake weighting.
- **Signature Compression**: BLS12-381 multi-signatures aggregate validator approvals into a single 48-byte proof.
`
  },
  {
    path: 'docs/blockchain/TOKENOMICS.md',
    category: 'blockchain',
    title: 'Tokenomics, Supply Schedule & EIP-1559 Burn',
    priority: 'mandatory',
    status: 'Final',
    version: '1.1.0',
    owner: 'Economics Group',
    summary: 'Max token supply, staking yield curve, validator rewards, and dynamic transaction fee burning.',
    content: `# A-TownChain Tokenomics & Fee Burning

## 1. Supply Parameters
- **Initial Supply**: 1,000,000,000 ATC.
- **Annual Staking Issuance**: Dynamic target inflation rate (3% - 5%).
- **Fee Burn Mechanics**: 100% of Base Fee is permanently burned on every transaction, counteracting inflationary issuance under high network usage.
`
  },
  {
    path: 'docs/blockchain/P2P.md',
    category: 'blockchain',
    title: 'P2P Networking & GossipSub Protocol',
    priority: 'mandatory',
    status: 'Final',
    version: '1.0.0',
    owner: 'Network Team',
    summary: 'Kademlia DHT node discovery, noise protocol encryption, GossipSub mesh propagation, and DoS mitigation.',
    content: `# P2P Gossip & Network Architecture

## Network Topology
- **Transport**: TCP and QUIC via libp2p.
- **Discovery**: Kademlia DHT with bootnodes.
- **Topic Propagation**: libp2p GossipSub v1.1 with peer scoring to isolate spamming nodes.
- **Handshake Encryption**: Noise Protocol Framework (Noise_XX_25519_ChaChaPoly_SHA256).
`
  },

  // --- AREA 8: ARCHITECTURE DECISION RECORDS (ADRs in docs/adr/) ---
  {
    path: 'docs/adr/0001-use-rust.md',
    category: 'adr',
    title: 'ADR-0001: Use Rust for Kernel & VM',
    priority: 'mandatory',
    status: 'Accepted',
    version: '1.0.0',
    owner: 'Architecture Committee',
    summary: 'Formal record deciding on Rust for zero-cost memory safety without garbage collection.',
    content: `# ADR-0001: Use Rust as Core Kernel & VM Implementation Language

## Status: Accepted
## Date: 2025-01-15
## Context
The ShivaCore Kernel and ATC-VM require memory safety without garbage collection overhead, deterministic execution, and low-level control.

## Decision
Implement the entire microkernel nucleus (#![no_std]) and ATC-VM in Rust.

## Alternatives Evaluated
- C/C++ (Lacks compile-time safety).
- Go (GC latency spikes).
- Zig (Immature formal ecosystem).

## Consequences
Memory safety guaranteed at compile time; zero-cost hardware FFI; higher developer learning curve.
`
  },
  {
    path: 'docs/adr/0002-microkernel-architecture.md',
    category: 'adr',
    title: 'ADR-0002: Microkernel Architecture with Capabilities',
    priority: 'mandatory',
    status: 'Accepted',
    version: '1.0.0',
    owner: 'Security Group',
    summary: 'Formal record restricting Ring 0 to scheduling, paging, and capability verification.',
    content: `# ADR-0002: Microkernel Architecture with Capability-Based Security

## Status: Accepted
## Date: 2025-02-10
## Context
Monolithic kernels have large attack surfaces. Driver or filesystem flaws jeopardize the whole OS.

## Decision
Adopt a true microkernel. Filesystems, device drivers, and network stacks run as unprivileged Ring 3 services.

## Alternatives Evaluated
- Monolithic Linux-style kernel.
- Hybrid kernel.

## Consequences
Drastically reduced attack surface; fault isolation; minor IPC cost addressed with lockless ring-buffers.
`
  },

  // --- AREA 9: REPOSITORY MANIFEST & MACHINE-READABLE REGISTERS ---
  {
    path: 'docs/DOCUMENTATION_INDEX.md',
    category: 'manifest',
    title: 'Master Documentation Registry & Index',
    priority: 'mandatory',
    status: 'Living',
    version: '3.0.0',
    owner: 'Documentation Lead',
    summary: 'Machine-readable inventory of all documentation files, statuses, owners, and dependency relations.',
    content: `# Master Documentation Index (ATC-DOC-001)

| Document | Category | Version | Status | Owner | Primary Dependency |
|---|---|---|---|---|---|
| README.md | Root | 3.0 | Stable | Core Team | — |
| docs/ARCHITECTURE.md | Architecture | 3.0 | Stable | Architecture | — |
| docs/DESIGN.md | Architecture | 3.0 | Stable | Architecture | docs/ARCHITECTURE.md |
| docs/COMPONENTS.md | Architecture | 3.0 | Stable | Core Team | docs/ARCHITECTURE.md |
| docs/DATA_MODEL.md | Architecture | 3.0 | Stable | Architecture | docs/ARCHITECTURE.md |
| docs/security/SECURITY_ARCHITECTURE.md | Security | 3.0 | Stable | Security | docs/ARCHITECTURE.md |
| docs/security/THREAT_MODEL.md | Security | 3.0 | Stable | Security | docs/security/SECURITY_ARCHITECTURE.md |
| docs/standards/ATC-0001-CORE-IDENTITY.md | Standards | 1.2 | Final | Identity | — |
| docs/standards/ATC-0002-NAMING.md | Standards | 1.1 | Final | Naming | ATC-0001 |
| docs/standards/ATC-0003-ADDRESS.md | Standards | 1.0 | Final | Crypto | — |
| docs/standards/ATC-0004-TOKEN.md | Standards | 1.1 | Final | Tokens | ATC-0003 |
| docs/standards/ATC-0005-TRANSACTION.md | Standards | 1.2 | Final | Consensus | ATC-0003 |
| docs/standards/ATC-0092-ATCLANG-VM.md | Standards | 1.0 | Final | Compiler | ATC-0093 |
| docs/standards/ATC-0093-ATC-BYTECODE.md | Standards | 1.0 | Final | VM | — |
| docs/standards/ATC-0094-ATC-VM.md | Standards | 1.0 | Final | VM | ATC-0093 |
| docs/ATC_VM_ARCHITECTURE.md | VM | 1.0 | Final | VM | ATC-0094 |
| docs/ATC_VM_ISA.md | VM | 1.0 | Final | VM | ATC-0093 |
| docs/ATC_VM_GAS_MODEL.md | VM | 1.0 | Final | VM | docs/ATC_VM_ISA.md |
| docs/blockchain/CONSENSUS.md | Blockchain | 2.0 | Final | Consensus | ATC-0005 |
| docs/blockchain/TOKENOMICS.md | Blockchain | 1.1 | Final | Economics | docs/blockchain/CONSENSUS.md |
| docs/adr/0001-use-rust.md | ADR | 1.0 | Accepted | Architecture | — |
| docs/adr/0002-microkernel-architecture.md | ADR | 1.0 | Accepted | Security | ADR-0001 |
`
  },
  {
    path: 'docs/VERSION_MATRIX.md',
    category: 'manifest',
    title: 'Cross-Component Version Compatibility Matrix',
    priority: 'mandatory',
    status: 'Living',
    version: '3.0.0',
    owner: 'Release Manager',
    summary: 'Compatibility matrix across Kernel, VM, Chain, Compiler, and SDK releases.',
    content: `# Cross-Component Version Matrix

| Globus OS | ShivaCore Kernel | ATC-VM Runtime | ATCLang Compiler | Consensus Protocol | Supported Status |
|---|---|---|---|---|---|
| **v3.0.x** | v3.0.x | v1.0.x | v1.0.x | v2.0 (BFT-FastFinal) | **Current Production** |
| v2.1.x | v2.1.x | v0.9.x | v0.9.x | v1.2 (BFT) | Maintenance |
| v1.0.x | v1.0.x | v0.1.x | v0.1.x | v1.0 (PoS-Legacy) | Deprecated |
`
  },
  {
    path: 'docs/FILE_REGISTER.md',
    category: 'manifest',
    title: 'Standard File Register & Content Hashes',
    priority: 'mandatory',
    status: 'Living',
    version: '3.0.0',
    owner: 'DevOps',
    summary: 'Full file inventory with verification status and purpose descriptions.',
    content: `# Repository File Register

This register verifies all standardized documentation files in the repository.

- **Total Standard Files Defined**: 24+ core files across 10 distinct governance areas.
- **Format**: UTF-8 Markdown and YAML with Unix line endings (LF).
- **Standards Coverage**: ATC-DOC-001 through ATC-DOC-008.
`
  }
];

export const DOC_CATEGORIES: DocCategory[] = [
  { id: 'all', name: 'All Documents', badge: 'Complete', description: 'All standardized files across root, docs, github, and standards', count: REPOSITORY_STANDARD_FILES.length },
  { id: 'root', name: '1. Root Pflichtdokumente', badge: 'Mandatory', description: 'README, LICENSE, CONTRIBUTING, SECURITY, CHANGELOG, ROADMAP, etc.', count: REPOSITORY_STANDARD_FILES.filter(f => f.category === 'root').length },
  { id: 'github', name: '2. GitHub Governance', badge: '.github', description: 'Issue templates, PR templates, workflows, dependabot, CODEOWNERS', count: REPOSITORY_STANDARD_FILES.filter(f => f.category === 'github').length },
  { id: 'arch', name: '3. Technische Architektur', badge: 'docs/arch', description: 'ARCHITECTURE, DESIGN, COMPONENTS, DATA_MODEL, SECURITY_ARCHITECTURE, THREAT_MODEL', count: REPOSITORY_STANDARD_FILES.filter(f => f.category === 'arch').length },
  { id: 'standards', name: '4. Standards & Protocols', badge: 'ATC-XXXX', description: 'ATC-0001 through ATC-0094 formal 14-section protocol specifications', count: ATC_PROTOCOL_STANDARDS.length },
  { id: 'api', name: '5. API Dokumentation', badge: 'REST/RPC', description: 'RPC_API.md, openapi.yaml, WebSocket pubsub, authentication', count: REPOSITORY_STANDARD_FILES.filter(f => f.category === 'api').length },
  { id: 'vm', name: '6. ATC-VM Modular Docs', badge: '15 Specs', description: 'Modular VM documentation: ISA, Gas Model, Memory Model, Verifier', count: REPOSITORY_STANDARD_FILES.filter(f => f.category === 'vm').length },
  { id: 'blockchain', name: '7. Blockchain Docs', badge: '14 Specs', description: 'Consensus, Tokenomics, P2P, Staking, State Machine, Mempool', count: REPOSITORY_STANDARD_FILES.filter(f => f.category === 'blockchain').length },
  { id: 'adr', name: '8. Architecture Decision Records', badge: 'ADRs', description: 'ADR-0001 through ADR-0006 architectural decision records', count: ADR_REGISTRY.length },
  { id: 'manifest', name: '9. Repository Manifests', badge: 'Registers', description: 'DOCUMENTATION_INDEX, VERSION_MATRIX, FILE_REGISTER, living status', count: REPOSITORY_STANDARD_FILES.filter(f => f.category === 'manifest').length },
  { id: 'atc-doc', name: '10. ATC-DOC Standards', badge: 'Meta', description: 'Overarching documentation standards ATC-DOC-001 through ATC-DOC-008', count: ATC_DOC_STANDARDS.length }
];
