# Ecosystem Audit Baseline — G1/G2

**Audit date:** 2026-09-25  
**Scope:** A-TownChain-Okosystems ecosystem  
**Audit mode:** evidence-first, fail-closed  
**Initial technical path:** `atc-algorithm → a-townchain → atc-node → atc-vm → atc-storage → atc-indexer`

## 1. Audit contract

A repository's existence is not evidence that a feature exists or is integrated.

Every claimed integration must progress through:

`ARCHITECTURE CLAIM → SOURCE-CODE EVIDENCE → TEST EVIDENCE → CI EVIDENCE → E2E EVIDENCE`

Missing evidence MUST NOT be promoted to a stronger readiness state.

## 2. Status vocabulary

| Status | Meaning |
|---|---|
| `IMPLEMENTED` | Relevant production source exists |
| `TESTED` | Behaviour is demonstrated by tests |
| `CI_VERIFIED` | Relevant CI execution is green |
| `E2E_VERIFIED` | Real components are proven together |
| `STUB` | Placeholder/mock implementation |
| `DUPLICATE` | Competing implementation exists |
| `DISCONNECTED` | Source exists but is not on the claimed runtime path |
| `BROKEN` | Integration exists but fails |
| `MISSING` | Required component/connection is absent |
| `UNKNOWN` | Evidence has not yet been collected |

## 3. G1 — Repository Reality

The audit MUST record, for every in-scope repository:

- repository name and canonical SSOT
- default branch
- audited HEAD
- actual source tree
- build system/manifests
- tests and test entry points
- CI workflows and latest relevant runs
- stubs, mocks, TODO/FIXME markers where materially relevant
- subtree/snapshot relationship to this umbrella repository

The umbrella repository is an integration/control plane, not a replacement for component SSOT.

## 4. G2 — Core Dependency Chain

### Required chain

`atc-algorithm → a-townchain → atc-node → atc-vm → atc-storage → atc-indexer`

For every edge the audit MUST answer:

1. Which protocol rule/API/type/constant crosses the boundary?
2. Where is its SSOT?
3. Which exact source symbol consumes it?
4. Are there duplicate definitions?
5. Are the same canonical types/constants used?
6. Is the state transition deterministic?
7. Is the observed path production code or a test/mock-only path?
8. Which test proves the connection?
9. Which CI run proves it?
10. Is there an E2E proof using the real components?

## 5. G3–G9 follow-on gates

These gates are registered now but MUST NOT be marked verified by documentation alone.

- **G3 — Execution Stack:** ATCLang → Bytecode → BytecodeVerifier → ATC-VM → State → Storage
- **G4 — Financial Stack:** Wallet → SDK → Node → VM → State → DEX/Marketplace/Launchpad/DAO/Treasury
- **G5 — OS Stack:** Hardware → ShivaCore → A-TownChain OS → GlobusOS → Aurora
- **G6 — AI Integration:** Aurora Model/Runtime/Agent/Tool/Skill/Memory/RAG/Policy/Approval → GlobusOS IPC/Policy
- **G7 — Genesis:** Aurora → GlobusOS → Genesis Engine → Genesis Chronicles
- **G8 — Security/Governance:** atc-standards → atc-engineering → CI/Evidence → Governance → Release Gates
- **G9 — Ecosystem E2E:** real cross-system execution after component-level evidence is established

## 6. Initial evidence boundary

The current umbrella repository already documents that production-readiness requires successful CI and that the canonical runtime path must be backed by evidence. This audit therefore treats claims without current source/test/CI/E2E evidence as unverified.

The repository currently maps:
- `components/atclang` → `atclang`
- `components/a-townchain` → `a-townchain`
- `components/aurora-ai` → `aurora-ai`
- `components/genesis-engine` → `genesis-engine`
- `components/atc-shivacore` → `atc-shivacore`

The G2 audit MUST additionally verify the externally canonical repositories for `atc-node`, `atc-vm`, `atc-storage`, and `atc-indexer` rather than assuming that a subtree or component directory proves integration.

## 7. Audit output

The authoritative machine-readable register is:

`docs/audits/ECOSYSTEM-READINESS-MATRIX.yaml`

The matrix records evidence references rather than subjective readiness scores.

## 8. Change control

This document defines the audit procedure only. It does not redefine protocol rules or component SSOTs.

Protocol and implementation changes MUST remain in their canonical source repositories and subsequently be synchronized into this umbrella repository according to the existing SSOT discipline.
