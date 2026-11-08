# ATC Implementation Slots and Queue

**Registry ID:** `ATC-IMPLEMENTATION-SLOT-QUEUE-001`  
**Status:** `PROPOSED_ACTIVE_QUEUE`  
**Updated:** 2026-10-16  
**Scope:** Ecosystem implementation sequencing and evidence gates. This document is a planning/control-plane artifact; it does not replace any implementation repository or prove implementation/verification.

## Queue rules

1. One active implementation slot at a time for work sharing the same acceptance gate. Parallelize only independent work that does not edit the same SSOT or invalidate the same evidence.
2. Each slot must name its canonical owner, exact input revision, dependencies, deliverables, acceptance criteria, and terminal evidence.
3. A slot is not complete because code was committed or CI was triggered. Completion requires the relevant workflow to reach a terminal conclusion on the exact commit, with Run → Job → Step → exit code and log/artifact references.
4. Keep unresolved mappings `UNRESOLVED`, incomplete work `RESIDUAL`, and unsafe/unsupported release claims `BLOCKED`. Never infer `VERIFIED` from source mapping, structural validation, or unrelated green workflows.
5. No duplicate SSOTs, stable IDs, wallet/key derivation, VM, consensus, protocol, or standards implementations. `demo-repository` remains excluded from live drift gates.
6. Do not merge the current Draft PR or promote release readiness while required checks/evidence are missing.

## Ordered queue

| Priority | Slot | Work item | Canonical owner | Dependency | Exit condition |
|---|---|---|---|---|---|
| P0 | SLOT-00 | Validate PR #192 exact head: run taxonomy validator and regression suite; inspect every relevant Actions run/job/step; fix only concrete failures and rerun | `a-townchain-ecosystem` control plane; PR branch `catalog/category-family-class-taxonomy-001` | Current PR head `638f170b38e7c0912b78ce3ec411b164180c6da2`; new workflow runs must be checked at their actual tested SHA | Validator + tests have terminal pass evidence at exact head; failures documented with logs; no blanket inference from unrelated green workflows |
| P1 | SLOT-01 | Bind category/family/class fields to existing component/function/interface records without changing stable IDs or guessing source classifications | Canonical catalog in `a-townchain-ecosystem`; implementation facts from each canonical owner | SLOT-00 validator contract stable | All records classified `MAPPED`, `EXCLUDED`, or `UNRESOLVED`; all dangling/duplicate references rejected; unknown mappings stay unresolved |
| P1 | SLOT-02 | Reconcile source inventory and implementation SSOT ownership, starting with unmapped/high-risk runtime entries | Each canonical implementation repository; catalog stores references only | SLOT-01 | Each mapped record has repo, exact 40-char commit SHA, path, symbol/kind, source-file SHA-256 and canonical owner; exclusions have explicit reason |
| P1 | SLOT-03 | Reconcile function contracts and verification evidence | Canonical owner for each function; catalog is index/evidence ledger | SLOT-02 | Contract fields, tests, dependencies, security boundaries and resource limits evidenced; workflow Run/Job/Step/exit code/artifact captured; statuses remain conservative |
| P2 | SLOT-04 | Resolve cross-chain registry blockers and reconcile source symbols against normative interfaces | `a-townchain` and `atc-standards`; ecosystem registry only tracks evidence | SLOT-02; normative spec available | Source and standard agree for source/destination chain IDs, nonce, message ID, proof/finality; unresolved consensus/finality remains BLOCKED |
| P2 | SLOT-05 | Reconcile identity/login/registration/wallet integration against canonical `atc-wallet` interfaces | GlobusOS identity implementation + canonical wallet owner | SLOT-02 and interface evidence | No parallel private/public key derivation; login/registration flows consume canonical wallet interfaces; security tests and exact-SHA evidence recorded |
| P2 | SLOT-06 | Complete security subsystem inventory and contracts (capabilities, permissions, policy, audit, secure boot/identity boundaries) | Canonical GlobusOS/ShivaCore/security owners | SLOT-01/02 | Components and functions cataloged with explicit authority boundaries, failure behavior, tests and evidence; no ambient authority implied |
| P3 | SLOT-07 | System Settings / Control Panel component and function catalog (Windows-like settings taxonomy, settings APIs, policy/permission surfaces) | Canonical `globus-os` desktop/system services; ecosystem catalog indexes only | SLOT-01; reuse Issue #78 and existing context-menu work rather than opening duplicate issues | Each settings surface maps to existing/new stable ID, owner, interface, permission boundary and acceptance tests |
| P3 | SLOT-08 | Desktop shell and context-menu contracts (launcher/taskbar, window operations, menus, accessibility, IPC) | Canonical `globus-os` desktop shell | SLOT-07 taxonomy alignment; existing Issue #78 | Move/resize/minimize/maximize/restore/close and context actions have contracts, policy checks and reproducible tests |
| P3 | SLOT-09 | GateToHell browser inventory and architecture reconciliation | Canonical browser repository/issue identified by existing GateToHell census | SLOT-01/02; use existing Issue #112 | Browser components/functions are source-mapped; process isolation, permission prompts, navigation/network, storage and update boundaries are evidenced; no speculative implementation before census |
| P4 | SLOT-10 | Install/platform matrix: native disk-installed GlobusOS versus application mode on Windows/Linux | Canonical GlobusOS installer/runtime owners; catalog is cross-repo index | SLOT-02 and kernel/boot evidence | Boot/install/update/recovery/platform functions mapped to real owners; unsupported hardware/platform combinations explicitly residual/blocked |
| P4 | SLOT-11 | Full release readiness and drift gate reconciliation | Ecosystem control plane only | All required earlier slots and required CI terminal evidence | Release gate checks exact manifest SHAs and evidence; archived repos handled by policy; `demo-repository` excluded; no unresolved critical blocker silently passes |

## Current active slot

**SLOT-00 — PR #192 CI and regression evidence.** At the last recorded check, workflow runs existed for PR head `638f170b38e7c0912b78ce3ec411b164180c6da2`; several were queued/pending/in progress, and the combined commit-status endpoint returned no status entries. This is a snapshot, not a current terminal result. Re-fetch the exact run and job metadata before concluding.

PR: https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/pull/192  
Tracking issue: https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/issues/190

## Slot handoff template

- Slot ID and owner:
- Input commit SHA:
- Scope and non-goals:
- Dependencies:
- Files/functions changed:
- Tests run and exact command:
- Actions Run ID → Job → Step → exit code:
- Log/artifact URL and SHA-256:
- Result: `IMPLEMENTED` / `RESIDUAL` / `BLOCKED`
- Verification: `NOT_VERIFIED` / `VERIFIED` (only with evidence)
- Next slot and unresolved blockers:
