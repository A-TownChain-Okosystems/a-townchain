# ATC Duplicate Implementation Audit 001

- Audit date: 2026-10-11
- Status: `PARTIAL / RESIDUAL`
- Purpose: compare competing implementations by code behavior, standards alignment, canonical ownership, test evidence, and migration risk.
- Change policy: this report does not authorize deletion, repository archival, or merge.

## Decision summary

| Domain | Canonical implementation | Preliminary decision | Evidence status |
|---|---|---|---|
| Wallet | `a-townchain/components/wallet` | KEEP as implementation SSOT; standalone `atc-wallet` remains read-only candidate until migration/history review | Code differences observed; exact-SHA CI evidence not established |
| Algorithm / consensus | `a-townchain/components/algorithm` | KEEP as implementation SSOT; standalone `atc-algorithm` should be spec/support only after migration review | Code differences observed; consensus safety and exact-SHA CI not verified |
| ATC-VM | `a-townchain/components/vm` | KEEP as implementation SSOT; standalone `atc-vm` should be spec/support only after delta review | Several files identical; material deltas remain to be tested |
| ShivaCore kernel | `globus-os/modules/atc-shivacore/kernel/` | KEEP as kernel SSOT; `atc-shivacore` remains specifications/governance/support | Architecture ownership is established; code quality and kernel release gates are not proven by this audit |

## Exact observed source refs

These refs were recorded from the organization census on 2026-10-11. Recheck the current branch HEAD before using this audit for a release or archive decision.

| Repository | Observed `main` HEAD |
|---|---|
| `a-townchain` | `0419b444dec4749cec7b146950d1304f5a406c25` |
| `atc-vm` | `21132a49355eb5fd6ea4b8a7b4e4deb5d9be60f0` |
| `atc-algorithm` | `5a5628044ef4daacca943062eac3fd27a2eaec93` |
| `atc-wallet` | `812a31d292364304c72cae8be6f448c130d7428e` |
| `globus-os` | `fef3af85ff9ecab90c25bc179e2a3284fa17f633` |
| `atc-shivacore` | `21c99769654ff5915c421c3446e62c2a29c22b40` |

## Findings

### 1. Wallet

The source comparison found the standalone wallet uses an Ed25519 transaction signer and `u64` balances, while the canonical component uses secp256k1 for transactions, `u128` amounts, and has more developed transaction/balance code. This aligns the canonical component more closely with the project's stated TX-v2 and amount-width requirements.

Required follow-up:
- Verify TX-v2 domain separation, RFC6979/SHA-256, low-S, compressed SEC1 public key, compact signature encoding, and rejection of legacy signing domains.
- Run wallet unit, property/fuzz, serialization-vector, and integration tests at the exact target SHA.
- Compare all standalone files and commit history against the canonical subtree before deciding archive status.

### 2. Algorithm / consensus

The source comparison found common files including `economics.rs`, `hash.rs`, and `lib.rs`, with implementation differences in `poh.rs` and `selection.rs`. The canonical selection path uses the intended hash path and `u128` stake values; the standalone path retains an older PoH/FNV route and `u64` stake values.

Required follow-up:
- Reconcile behavior against the normative consensus specification; do not infer consensus correctness from file presence or type widths.
- Add/verify deterministic cross-platform vectors, adversarial validator-selection tests, overflow handling, and replay/reorg edge cases.
- Do not label consensus production-ready without a frozen spec and exact-SHA evidence.

### 3. ATC-VM

The source comparison found `vm.rs`, `lib.rs`, and `main.rs` identical, with differences in `assembler.rs`, `context.rs`, and `ops.rs`. The canonical assembler has additional tests and explicit overflow checks. The standalone context retains the legacy `ATC-TX-DOMAIN` string, which conflicts with the specified `ATC-TX-DOMAIN-V2` transaction domain if that path is used for transaction signing.

Required follow-up:
- Review every changed hunk in `assembler.rs`, `context.rs`, and `ops.rs`.
- Verify malformed-bytecode rejection, ATCB magic/version/count encoding, operation/stack/storage limits, gas behavior, and deterministic execution.
- Check whether ATCB vs ATC1 format drift remains; resolve only against the normative spec.
- Run VM unit, negative-input, fuzz, and cross-component contract tests at the exact target SHA.

### 4. ShivaCore

The architecture ownership rule places the active kernel under `globus-os/modules/atc-shivacore/kernel/`; `atc-shivacore` is specification/governance/support, not a second kernel implementation.

Required follow-up:
- Verify the kernel tree against the no_std capability-microkernel boundary.
- Require exact-SHA Rust checks/tests, boot evidence, memory/VMM lifecycle tests, IPC/capability isolation tests, and the outstanding secure-boot/ACPI evidence before any production claim.
- Search for executable kernel code duplicated in the support repository and record exact path/blob evidence before removal.

## CI evidence and limitations

A query for PR-triggered workflow runs on the recorded commit SHAs returned no runs. The connector query is scoped to pull-request-triggered runs, so this result is **not evidence that no CI exists** and is **not evidence of success**. This audit therefore records exact-SHA CI as `NOT_VERIFIED`.

A matching or differing tree SHA alone cannot prove migration completeness. No archive decision is authorized until file/path/blob comparisons, history review, dependency checks, and required exact-SHA tests are complete.

## Required decision gates

1. Re-fetch current `main` HEAD and tree SHAs for every source and target.
2. Produce path-by-path inventories and blob-SHA comparisons for the standalone root versus the canonical subtree.
3. Compare commit history/provenance where possible; record missing, additional, and divergent files.
4. Run required tests on the exact target SHA and capture Run → Job → Step → exit status → logs/artifacts.
5. Record one explicit decision per repository: `KEEP`, `SYNC`, `ARCHIVE`, or `REACTIVATE`.
6. Keep all unresolved items `RESIDUAL`; do not delete or archive repositories automatically.
