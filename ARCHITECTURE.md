---
document_id: ATC-DOC-ARC-ATCH-001
title: Repository Architecture Specification
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-13
updated: 2026-09-13
standard: ATC-STD-MD-001
---

# Architecture Specification — a-townchain

## Übersicht

`a-townchain` ist die Chain-Protokoll-Bibliothek (Layer L2) der A-TownChain: Chain-ID 658467, Hybrid-Konsens (PoW+PoS+PoH), ZKP-Verifikation, On-Chain-Governance und Chain-DNS. Nach AD-012 ist die Chain ein Kernel-System-Service des KAI-OS.

## Subsysteme

1. **Chain Protocol Core:** Block-/Transaktionsmodell, Chain-ID 658467, Validierungsregeln (`modules/`).
2. **Hybrid-Konsens-Bindung:** PoW+PoS+PoH — Konsens-Implementierung liegt kanonisch in `atc-algorithm`.
3. **ZKP-Integration:** Proof-Verifikation — kryptografische Primitive kanonisch in `atc-zkp`.
4. **On-Chain-Governance:** Abstimmungs- und Parameter-Änderungsmechanismen.
5. **Chain-DNS:** Namensauflösung auf der Chain.
6. **Kernel-Service (AD-012):** Dienste-Integration in das KAI-OS.

## Verantwortungsgrenzen

- `atc-node` betreibt das Protokoll (Full-Node-Binary/Runtime) — hier wird es definiert.
- `atc-algorithm` implementiert den Konsens — dieses Repo bindet ihn.
- `components/vm` ist die kanonische L3-ATC-VM und wird vom L2-Protokoll über einen deterministischen State-Transition-Vertrag aufgerufen.

## Registry-Einordnung

| Property | Value |
|---|---|
| Layer | L2 |
| Criticality | C1 |
| Security-Klasse | S4 |
| Maturity | R-Level laut `.atc/repository.yaml` · Statusleiter in `.atc/evidence/evidence.yaml` (SCR-0080) |
| Canonical | a-townchain (Chain-Protokoll, AD-012 Kernel-Service) |
| Domäne | domaene laut registry/repositories.yaml |

> Ehrlichkeitsregel: CLAIMED ≠ PASS · IMPLEMENTED ≠ VERIFIED — der verbindliche Implementierungsstand
> liegt ausschließlich in `.atc/evidence/evidence.yaml`, nicht in dieser Spezifikation.


## Normative L1 Determinism and Authority Contract

### State transition invariant

`F(State, Block) = State'` is the canonical L2 state-machine invariant. For the same canonical State and canonical Block, every conforming implementation MUST produce the same State'.

### Canonical processing chain

```text
Canonical Encoding
→ Authentication / Authorization
→ Transaction Validation
→ Canonical Ordering
→ Consensus
→ Deterministic Execution
→ State Transition
→ State Commitment
→ Finality
```

A later stage MUST NOT be inferred merely from success at an earlier stage.

### Authority boundaries

- L2 owns canonical blockchain state, block/transaction validation, ordering, consensus, finality and state transition.
- L3 owns deterministic contract/program execution and resource accounting.
- L4 transport and synchronization MUST NOT create authority.
- L5 economic/security rules MUST be deterministic and MUST NOT create an alternative finality mechanism.
- L7 clients MUST NOT directly mutate canonical state.
- X supplies identity, capability, policy, governance, audit and evidence controls; X MUST NOT become a second state machine.

### WHO / WHAT / WHERE

Every privileged state-affecting operation MUST identify:

```text
WHO  = principal / identity
WHAT = authorized action / state effect
WHERE = validation + enforcement boundary
```

Missing or ambiguous authority MUST fail closed.

### Exact-SHA evidence

Implementation status MUST be established independently from architecture claims. Verification requires:

```text
Source SHA → Workflow Run → Job → Step → Log/Artifact → Result
```

Existence, documentation, a historical PR, or a non-exact-SHA successful run MUST NOT establish VERIFIED.
