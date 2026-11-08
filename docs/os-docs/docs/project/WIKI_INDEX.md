# A-TownChain — Wiki Index

> Generated: 2026-08-06
> 102 repositories | 50 code + 52 wiki

## Core Documentation

| Document | Repo | Status |
|----------|------|--------|
| Whitepaper v3.0.0 | `atc-whitepaper` | ✅ |
| Standards Registry | `a-townchain-os-docs` | ✅ |
| Decisions Register | `a-townchain-os-docs` | ✅ |
| Licensing Overview | `a-townchain-os-docs` | ✅ |
| Agent Policy | `a-townchain-os-docs` | ✅ |
| BaFin Compliance Report | `a-townchain-os-docs` | ✅ |
| Architecture Document | `a-townchain-os-docs` | ✅ |
| Roadmap Sync | `a-townchain-os-docs` | ✅ |

## Standards (ATC-01 – ATC-35)

| Standard | Title | Tier | Status |
|----------|-------|------|--------|
| ATC-01 | Core Node Protocol & P2P Mesh | T1 | Partial |
| ATC-02 | Liquid State Migration & Failover | T1 | Partial |
| ATC-03 | Decentralized Identity (DID) | T1 | Partial |
| ATC-04 | DAG Consensus & Propagation | T1 | Conceptual |
| ATC-05 | Smart Contract VM | T1 | Implemented |
| ATC-06 | Token Standards (ATC-8300/9000) | T1 | Implemented |
| ATC-07 | Cross-Chain Bridge Protocol | T1 | Implemented |
| ATC-08 | Zero-Knowledge Proof Layer | T1 | Conceptual |
| ATC-09 | Cross-Chain Interop Bridge | T1 | Implemented |
| ATC-10 | Global Time Sync & Oracles | T1 | Partial |
| ATC-11–ATC-16 | Tier 2 Standards | T2 | Documented |
| ATC-17–ATC-25 | Tier 3 Standards | T3 | Documented |
| ATC-24 | Autonomous Agent Scheduling | T4 | Documented |
| ATC-25 | Tensor Compute Orchestration | T4 | Documented |
| ATC-26–ATC-31 | Tier 3-4 Standards | T3-4 | Documented |
| ATC-32 | UX Abstraction | T5 | Documented |
| ATC-33 | Reward Reinforcement | T5 | Documented |
| ATC-34 | Cross-Layer Interoperability | T5 | Documented |
| ATC-35 | Data Privacy | T5 | Documented |

## Kernel Standards (ATS-1000 – ATS-1007)

| Standard | Title | Status |
|----------|-------|--------|
| ATS-1000 | Kernel Base Layer | ✅ Implemented |
| ATS-1001 | Process Management | ✅ Implemented |
| ATS-1002 | Memory Management | ✅ Implemented |
| ATS-1003 | IPC & Capability System | ✅ Implemented |
| ATS-1004 | File System (ATCFS) | ✅ Implemented |
| ATS-1005 | Network Stack (ATCNet) | ✅ Implemented |
| ATS-1006 | Security & Crypto | ✅ Implemented |
| ATS-1007 | Scheduler (DA-HEFT) | ✅ Implemented |

## Kernel Module Status

| Module | Sprint | Tests | Status |
|--------|--------|-------|--------|
| capability.rs | K3 | 8 | ✅ |
| process.rs | K3b | 10 | ✅ |
| scheduler.rs | K4 | 10 | ✅ |
| ipc.rs | K5 | 12 | ✅ |
| did.rs | K6/K6b | 15 | ✅ |
| remote_caps.rs | K6 | 6 | ✅ |
| knowledge_graph.rs | K7 | 18 | ✅ |
| memory_manager.rs | K8/K21 | 28 | ✅ |
| atcfs.rs | K8 | 22 | ✅ |
| kernel_init.rs | K22 | 11 | ✅ |
| cross_subsystem.rs | K23 | 15 | ✅ |
| atcnet.rs | K24 | 32 | ✅ |
| genesis.rs | K26 | 38 | ✅ |
| genesis_bridge.rs | K27 | 40 | ✅ |
| gossip_bridge.rs | K28 | 45 | ✅ |
| security_audit.rs | K29 | 34 | ✅ |

## Wiki Repositories (52)

Each code repo has a `-wiki` companion. Key wikis:

| Wiki | Repo | Content |
|------|------|---------|
| Main Wiki | `kai-os-wiki` | 1.5MB, Python docs |
| Docs Hub | `a-townchain-os-docs` | 4.0MB, TypeScript |
| Kernel Wiki | `atc-shivacore-wiki` | Kernel docs |
| Standards Wiki | `atc-standards-wiki` | Standards docs |
| Blockchain Wiki | `atc-blockchain-wiki` | Chain docs |
| Contracts Wiki | `atc-contracts-wiki` | Contract docs |
| Genesis Engine Wiki | `atc-genesis-engine-wiki` | Engine docs |
| Franchise Wiki | `franchise-factory-wiki` | Org hub docs |
