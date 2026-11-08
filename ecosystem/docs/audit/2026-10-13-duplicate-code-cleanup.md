# Duplicate-Code-Cleanup Audit — 2026-10-13

**Status:** TRIAGE ONLY — no source files deleted  
**Scope:** Organization inventory and targeted duplicate/SSOT probes across active repositories  
**Decision rule:** Standalone First, Ecosystem Second; canonical source ownership beats textual similarity. No deletion without exact source HEADs, reference/dependency analysis, build/test evidence, and a Draft PR in the owning repository.

## 1. Repository inventory

The GitHub repository inventory query returned 33 repositories: 26 non-archived and 7 archived. All listed repositories use `main` as their default branch. This is an inventory snapshot, not a claim that every file in every repository was exhaustively scanned.

Active repositories included in scope:

- `.github`
- `a-townchain`
- `a-townchain-ecosystem`
- `a-townchain-os`
- `a-townchain-os-docs`
- `atc-algorithm`
- `atc-compute`
- `atc-contracts`
- `atc-engineering`
- `atc-ide`
- `atc-launchpad`
- `atc-marketplace`
- `atc-node`
- `atc-sdk`
- `atc-shivacore`
- `atc-standards`
- `atc-toolchain`
- `atc-vm`
- `atc-zkp`
- `atclang`
- `aurora-ai`
- `demo-repository` (private)
- `genesis-chronicles`
- `genesis-engine`
- `genesis-franchise-factory`
- `globus-os`

Protected archived repositories — do not modify or delete as part of this cleanup without a separate explicit governance decision:

- `atc-explorer`
- `atc-indexer`
- `atc-interop`
- `atc-mining`
- `atc-oracle`
- `atc-storage`
- `atc-wallet`

## 2. Canonical ownership constraints

- Blockchain core and canonical VM: `a-townchain`, especially `components/vm`.
- Canonical algorithm/consensus implementation: `a-townchain/components/algorithm`; unresolved consensus choices remain BLOCKED.
- ShivaCore kernel: `globus-os/modules/atc-shivacore/kernel/`; `atc-shivacore` is specifications/governance/support, not a second kernel implementation.
- ATCLang normative bytecode specification: `atclang`; `atc-standards` owns the normative registry/pointers.
- `a-townchain-ecosystem` is an integration, compliance and evidence control plane. Imported subtree snapshots are not a second implementation SSOT.

## 3. Findings from targeted code-search probes

| ID | Candidate | Evidence observed | Decision | Reason |
|---|---|---|---|---|
| DUP-001 | `a-townchain-ecosystem/components/a-townchain/components/sdk/typescript/chain-identity.ts` vs `a-townchain/components/sdk/typescript/chain-identity.ts` | Both expose canonical transaction-signing / transaction-ID symbols. The ecosystem file is an imported subtree snapshot; its body and validation behavior must be diffed against an exact canonical SHA before any sync decision. | **KEEP SNAPSHOT / RECONCILE** | Deleting an individual file from an imported subtree may break snapshot provenance or subtree synchronization. No safe deletion proven. |
| DUP-002 | `a-townchain-ecosystem/components/a-townchain/components/storage/src/store.rs` vs `a-townchain/components/storage/src/store.rs` | Both contain `InMemoryStore`, `fnv1a` and `mvp_root_hash`; the implementation explicitly labels the hash development-only/non-cryptographic. | **KEEP SNAPSHOT / RECONCILE** | Same subtree-snapshot constraint; do not confuse duplication in an integration mirror with duplicate production ownership. Cryptographic state-root gap remains a separate blocker. |
| DUP-003 | `a-townchain-ecosystem/components/atc-vm` and `components/atc-algorithm` | Root `Cargo.toml` explicitly excludes both as “Legacy duplicate snapshots”; canonical nested VM/algorithm workspaces are also excluded from the umbrella root workspace. | **NO BLIND DELETE** | Exclusion from the root workspace is useful evidence, but does not prove no scripts, tests, historical sync workflow, documentation, or audit references require these paths. |
| DUP-004 | Standalone `atc-vm` / `atc-algorithm` repositories vs canonical subtrees in `a-townchain` | Search surfaced SCR-0131 stating the standalone repositories are frozen/non-canonical and migrated to `a-townchain/components/vm` and `a-townchain/components/algorithm`; the registry still has ownership-era ambiguity that SCR-0131 is resolving. | **GOVERNANCE BLOCKED** | Do not delete repository contents or archive/decommission the repos until the registry migration decision is authoritative and references/build consumers are reconciled. |
| DUP-005 | `a-townchain-ecosystem/components/globus-os/modules/atc-globus-os/...` vs `globus-os/modules/atc-globus-os/...` | Search found matching `world_engine.atc` and `world_registry.atc` names and excerpts in the ecosystem snapshot and canonical GlobusOS repository. | **KEEP SNAPSHOT / RECONCILE** | Imported subtree copy, not yet proven dead code. Preserve evidence and compare snapshot SHA to canonical source before sync/removal. |
| DUP-006 | `a-townchain-ecosystem/components/atc-sdk/typescript/chain-identity.ts` vs canonical SDK paths | The legacy SDK snapshot has a separate transaction-signing implementation; the excerpt differs in field widths/features from the canonical V2 implementation (including u128 amount / transaction-ID behavior). | **DO NOT DELETE; FLAG DIVERGENCE** | This is not safe to classify as a byte-identical duplicate. It may be stale/incompatible implementation code and requires dependency and test analysis. |
| DUP-007 | ATCB specification copies in ecosystem / docs / ATCLang | Search found copies and pointers, while `atc-standards` states ATCB v1's normative carrier is `atclang/specs/bytecode/SPEC.md` and the standards repo should hold a pointer, not a competing copy. | **CLASSIFY MIRROR VS NORMATIVE CLAIM** | Remove only proven competing normative claims; preserve archived/mirror evidence and cross-references. No deletion from search excerpts alone. |

## 4. Deletion gate — required before a file is removed

For each candidate, the owning-repository Draft PR must record:

1. Exact 40-character source HEAD SHA and blob SHA for every compared file.
2. Byte-identical hash result or a reviewed semantic diff; matching names alone do not qualify.
3. Repository-wide imports, call sites, workspace members, build scripts, CI workflows, generated manifests, release packaging, and subtree/submodule references.
4. Existing build/test/release evidence on the exact base SHA and the proposed deletion SHA.
5. Explicit classification: `DELETE_SAFE`, `KEEP_CANONICAL`, `KEEP_MIRROR`, `ARCHIVE_ONLY`, or `BLOCKED`, with rationale and reviewer evidence.
6. No removal of canonical code, normative standards, tests needed by consumers, security evidence, provenance, or archived history.
7. A Draft PR per owning repository; no direct changes to `main`, no force-push, no weakened CI, and no merge before exact-head checks pass.

## 5. Result of this audit pass

- Confirmed: multiple duplicate-looking paths exist, including imported subtree mirrors and legacy migrated implementations.
- Confirmed: some candidates are divergent, not identical, and must not be deleted under a “duplicate code” assumption.
- Safe source deletions proven in this pass: **0**.
- Files deleted: **0**.
- Repositories exhaustively file-hashed and reference-scanned: **0 of 26**. The available repository inventory and indexed code-search probes are a triage pass, not a complete filesystem census.
- Release / production status: unchanged. This audit makes no new implementation or functional-verification claim.

## 6. Next execution sequence

1. Build a reproducible exact-HEAD source inventory for every non-archived repository; fail closed when a repository tree, source SHA, or index is unavailable.
2. Hash and diff candidate files, separating true byte-identical copies from semantic divergences, generated files, vendored dependencies, examples, tests, and mirrors.
3. Generate reference/dependency reports per owning repository and validate them against workspace/build/CI/release configuration.
4. Open Draft PRs only for individually proven safe removals; otherwise commit a BLOCKED finding with the missing evidence.
5. Run relevant validators/tests on the proposed deletion SHA and retain full Run → Job → Step → exit-code/log evidence.
6. Keep all archived repositories and historical/audit records untouched unless separately authorized.


## 7. Exact-blob comparison follow-up (2026-10-13)

The following comparisons were fetched from the explicit source commit refs and compared by Git blob SHA and returned file content. This is stronger than a filename/search-excerpt match, but it does not replace the repository-wide consumer/build/release scan.

| Pair | Source HEAD / ref | Path | Blob SHA | Result |
|---|---|---|---|---|
| Canonical SDK ↔ ecosystem subtree snapshot | `a-townchain@c631249ed6ff450bf45338e78cb14e2bf5c65c74` ↔ `a-townchain-ecosystem@73ad6c10a6b8dc10b777733561636aee69cdbb8d` | `components/sdk/typescript/chain-identity.ts` ↔ `components/a-townchain/components/sdk/typescript/chain-identity.ts` | `38b5bb99c2e427e86d237f6952b45be65125242b` on both sides | **BYTE-IDENTICAL**; both fetched bodies are 4,336 characters. Preserve snapshot; do not delete one side as “duplicate” without a subtree/provenance decision. |
| Canonical storage ↔ ecosystem subtree snapshot | `a-townchain@c631249ed6ff450bf45338e78cb14e2bf5c65c74` ↔ `a-townchain-ecosystem@73ad6c10a6b8dc10b777733561636aee69cdbb8d` | `components/storage/src/store.rs` ↔ `components/a-townchain/components/storage/src/store.rs` | `3755e0ce0f41e645ad40d00838403a624244a730` on both sides | **BYTE-IDENTICAL**; both fetched bodies are 2,546 characters. Snapshot is not evidence of a second production implementation. The file's non-cryptographic MVP root-hash warning remains relevant; no security claim is implied. |
| Standalone SDK ↔ canonical SDK | `atc-sdk@34eef345907935cdaf279e0546f230c03c86bb0c` ↔ `a-townchain@c631249ed6ff450bf45338e78cb14e2bf5c65c74` | `typescript/chain-identity.ts` ↔ `components/sdk/typescript/chain-identity.ts` | `25d6144ed787bad1f135f6638b74753411c2a903` ↔ `38b5bb99c2e427e86d237f6952b45be65125242b` | **DIVERGENT**; fetched bodies are 3,428 vs 4,336 characters. Not byte-identical. Do not delete until API/consumer, transaction-format compatibility, tests, and migration/registry evidence are checked. |
| Legacy VM manifest ↔ canonical VM manifest in ecosystem snapshot | `a-townchain-ecosystem@73ad6c10a6b8dc10b777733561636aee69cdbb8d` | `components/atc-vm/Cargo.toml` ↔ `components/a-townchain/components/vm/Cargo.toml` | `3bc54580a25d6295922d453d6426cdace0742fe8` ↔ `a84523b051190889b8fdfc7b3f7b784440a7fe34` | **NOT BYTE-IDENTICAL**; canonical manifest declares its own `[workspace]` and differs in content. The legacy manifest being excluded from the root workspace does not alone prove its source tree has no consumers. |

### Interpretation and limits

- Two comparisons now prove exact byte identity between canonical source files and their ecosystem subtree copies. That supports the snapshot/provenance classification, not a deletion.
- The standalone SDK comparison proves divergence, not redundancy. Semantic compatibility and live consumers remain unverified.
- The VM manifest comparison confirms distinct manifest content, not that the legacy implementation is actively used or safe to remove.
- No repository-wide import/call-site/build/release scan has yet been completed; no test or build was run for a proposed deletion commit.
- Safe deletions proven remain **0**; source files deleted remain **0**; archived repositories remain untouched.


## 8. Consumer and ownership-reference checks (targeted, 2026-10-13)

Additional targeted probes show why the legacy VM and algorithm paths must not be deleted based only on workspace exclusion.

### VM: active external references found

- The current `atc-contracts` determinism workflow at source ref `a7c8b417f482665ef30e94fcacae6469c28b1125` (workflow blob `558b88e95fb85c52869d9c9339d7e5ceaaef5eba`) explicitly checks out `A-TownChain-Okosystems/atc-vm` into `atc-vm`, builds `atc-vm-runner`, and executes determinism/conformance runs against that runner.
- The canonical `a-townchain` contract determinism workflow at `c631249ed6ff450bf45338e78cb14e2bf5c65c74` (workflow blob `e22e840d427948d7dbad4992ffdf9061065fbd05`) instead builds `components/vm`.
- The legacy snapshot's own `components/atc-vm/.github/workflows/contract-exec-gate.yml` (blob `5652f82ac1d6398f8f74ee64207f43295ea8acd9`) checks out `atc-contracts`, builds the local runner, executes the fixture, verifies a receipt, and uploads evidence.

**Decision: BLOCKED — DO NOT DELETE / DO NOT ARCHIVE.** A consumer repository and workflow still name the standalone `atc-vm` repository. This may be a stale consumer, but only a controlled migration to the canonical `a-townchain/components/vm`, followed by exact-head deterministic tests and release/evidence verification, can establish that it is safe to remove or decommission the standalone repository. The ecosystem snapshot's workspace exclusion alone is not sufficient.

### Algorithm: conflicting ownership claims found

- The ecosystem root `Cargo.toml` blob `d69730b8b7740afb8f55c1dbd798f6a1bf7442c8` excludes `components/atc-algorithm` as a legacy duplicate snapshot and states the canonical algorithm workspace is under `components/a-townchain/components/algorithm`.
- However, the legacy snapshot README at ecosystem ref `73ad6c10a6b8dc10b777733561636aee69cdbb8d`, blob `ba10d755a88c4492336dae08c5adaf479d45f5a8`, still says `atc-algorithm` is the canonical consensus repository and says `a-townchain` is only the orchestration/integration layer.
- The canonical nested algorithm manifest at that same ecosystem ref is blob `36097ac8943de8db884b6f04873f4e277c1e6177` and declares the canonical consensus implementation package under the `a-townchain` subtree.

**Decision: BLOCKED — GOVERNANCE / REGISTRY RECONCILIATION REQUIRED.** These sources make contradictory ownership claims. Do not delete the legacy tree or rewrite its history as a cleanup operation. First reconcile normative registry/ADR/SCR ownership, update stale README/architecture/status references in a separately reviewed Draft PR, and inspect consumers/build/release references in both standalone and canonical repositories. Consensus protocol rules remain BLOCKED wherever they are not authoritatively frozen.

### Evidence boundary

These are targeted reference hits, not a complete organization-wide import/call-site scan. No workflow was run by this audit. No build, test, release, or deletion has been verified on a proposed cleanup SHA. Safe source deletions remain **0**.
