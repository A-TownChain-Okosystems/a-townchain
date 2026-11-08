# Aurora Decentralized Memory Architecture

Status: specification / documented architecture baseline
Lifecycle: DEVELOPMENT / NOT_READY
Repository: A-TownChain-Okosystems/aurora-ai
Baseline: 87ed7bc87014708f3efad4c573871fe23f252b00
Date: 2026-09-27

## 1. Purpose

Aurora Memory is extended from a single-process memory/context layer toward a decentralized/federated memory system in which authorized PCs, smartphones, servers, GlobusOS nodes and other approved devices may act as memory nodes.

A node owns or stores memory locally. Federation is explicit and policy-controlled. A device does not become a source of globally readable memory merely by joining the network.

This document defines the target contract. It does not claim that the complete federation protocol, persistence, synchronization, mobile client, or production deployment already exists.

## 2. Authority model

```text
                         AURORA MEMORY FEDERATION
                                  |
        +-------------------------+-------------------------+
        |                         |                         |
   PC / Laptop               Smartphone                 Server/Node
        |                         |                         |
   Local Memory             Local Memory              Node Memory
        |                         |                         |
        +-------------------------+-------------------------+
                                  |
                         Memory Federation
                                  |
                 +----------------+----------------+
                 |                                 |
             Aurora AI                       Shared Knowledge
                 |                                 |
                 +----------------+----------------+
                                  |
                         Identity / Capability
                                  |
                           Policy / Consent
                                  |
                              GlobusOS
                                  |
                              ShivaCore
```

The authoritative security boundary remains outside Aurora:
- ShivaCore owns kernel/TCB primitives and capability enforcement.
- GlobusOS owns userspace identity, policy, services and privileged system interfaces.
- Aurora consumes authorized memory and context; it does not acquire implicit authority over memory.
- A-TownChain may provide identity, integrity/provenance or proof anchoring where explicitly specified, but memory payloads are not required to be stored on-chain.

## 3. Node model

A memory node is an authorized execution/storage endpoint.

Supported target node classes:
1. Device Node — PC, laptop, workstation, tablet or smartphone.
2. GlobusOS Node — a device running the GlobusOS userspace memory services.
3. Server Node — user-controlled or authorized infrastructure.
4. Federation Node — a service participating in shared-memory synchronization.
5. Read-only Knowledge Node — exposes approved knowledge without accepting writes.

Node identity must be explicit and cryptographically bound to its authorization context.

## 4. Memory domains

| Domain | Example | Default exposure |
|---|---|---|
| DEVICE | local device state | local |
| PERSONAL | user preferences and personal memory | owner only |
| CONVERSATION | Aurora conversation context | owner + authorized Aurora |
| KNOWLEDGE | documents/RAG knowledge | source policy |
| WORLD | Genesis world state | world policy |
| GAME | player/game progress | player/game policy |
| OS | operating-system state | system policy |
| BLOCKCHAIN | verified chain facts | public/chain policy |
| EVIDENCE | provenance/audit records | policy-controlled |
| SHARED | explicitly federated memory | explicit grant |
| MODEL | model/provider metadata | provider policy |

No domain is globally readable by default.

## 5. Memory record

The canonical target record contains:

```text
MEMORY-ID
OWNER
SOURCE-NODE
SCOPE
TYPE
CONTENT / CONTENT-REFERENCE
CREATED-AT
UPDATED-AT
VERSION
CONFIDENCE
PROVENANCE
ACCESS-POLICY
CAPABILITY-REQUIREMENT
RETENTION-POLICY
INTEGRITY-HASH
OPTIONAL-CHAIN-PROOF
```

Memory type is explicit and must distinguish at minimum:

FACT
USER_MEMORY
MODEL_INFERENCE
RAG_RESULT
WORLD_STATE
GAME_STATE
BLOCKCHAIN_FACT
UNVERIFIED_INFORMATION

An inference is not silently promoted to a fact.

## 6. Read path

```text
Aurora
  -> Memory Query
  -> Identity
  -> Capability Check
  -> Policy / Consent
  -> Memory Router
  -> Authorized Nodes
  -> Provenance / Integrity Validation
  -> Conflict Resolution
  -> Context Assembly
  -> Aurora
```

A failed authorization or integrity check must not be bypassed by querying another node.

## 7. Write path

```text
Source
  -> Memory Candidate
  -> Type Classification
  -> Provenance
  -> Owner / Scope
  -> Policy Validation
  -> Capability Authorization
  -> Persistence
  -> Version / Hash
  -> Optional Federation
  -> Optional Chain Proof
```

Aurora-generated content is initially a candidate/inference unless an explicit policy promotes it to another memory type.

## 8. Federation protocol

The target federation protocol is:

```text
Node A
  -> announce capability
  -> authenticate node
  -> negotiate scope
  -> request/offer memory
  -> validate policy
  -> exchange version/provenance
  -> resolve conflicts
  -> persist
  -> acknowledge
```

Federation must support:
- node identity;
- explicit scope grants;
- read/write capability separation;
- versioning;
- provenance preservation;
- integrity hashes;
- replay protection;
- revocation;
- retention/deletion policy;
- conflict resolution;
- audit records;
- offline operation and later synchronization.

## 9. Conflict resolution

Federation must not use silent last-write-wins for security-sensitive or semantically conflicting memory.

Target resolution order:
1. owner policy;
2. authoritative source;
3. version relationship;
4. explicit domain rules;
5. provenance/confidence;
6. deterministic conflict record.

Unresolved conflicts remain visible as conflicts; they are not silently collapsed into one asserted fact.

## 10. Privacy and security

The default posture is:

DENY BY DEFAULT
LOCAL FIRST
EXPLICIT FEDERATION
LEAST PRIVILEGE
AUDIT EVERY PRIVILEGED ACTION

Sensitive memory such as credentials, wallet/recovery material, private keys and private personal data must remain behind the applicable GlobusOS/ShivaCore security boundary.

Aurora must not receive raw private keys merely because it has access to a memory service.

## 11. A-TownChain relationship

A-TownChain is not the primary bulk-memory database.

Target use:
```text
Large Memory Payload
        |
        +--> local/federated storage
        |
        +--> content hash
        |
        +--> provenance record
        |
        +--> optional ATC proof/anchor
```

This separates scalable memory storage from deterministic chain verification.

## 12. Genesis integration

Genesis can use the same memory federation model:

```text
Player Device
     |
     v
Game Memory
     |
     v
Genesis Memory
     |
     +--> World Memory
     +--> Character Memory
     +--> Creature Memory
     +--> Quest Memory
     +--> Dialogue Memory
     |
     v
Aurora Context Assembly
```

World and player memory remain subject to their own ownership and game-world policies.

## 13. Current implementation evidence

The existing Aurora repository already contains a memory module with:
- KnowledgeBase;
- VectorStore;
- MemoryIndex;
- LearningPipeline;
- ContextWindow.

These are local Rust components. Their current APIs do not by themselves establish a distributed/federated memory protocol, persistent cross-device synchronization, mobile integration, authorization enforcement, conflict resolution, or production readiness.

Therefore this document records the decentralized-memory target as:

SPECIFIED -> DOCUMENTED

and not as:

IMPLEMENTED -> TESTED -> CI-VERIFIED -> E2E-VERIFIED -> AUDITED -> RELEASE-READY.

## 14. Required next implementation gates

1. Canonical memory record schema.
2. Canonical serialization and hashing.
3. Memory capability/policy contract.
4. Node identity and federation handshake.
5. Read/write federation protocol.
6. Persistent local store.
7. Versioning and conflict-resolution engine.
8. Revocation and deletion semantics.
9. Provenance/evidence records.
10. Exact-SHA unit/integration CI.
11. Cross-device E2E test.
12. Security audit before production claims.