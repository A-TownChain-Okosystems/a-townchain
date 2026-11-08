# A-TownChain-Okosystems — Repository Audit Matrix

> Date: 2026-09-16
> Status: **IN PROGRESS**
> CI mode: **source/static audit while GitHub Actions evidence is unavailable or incomplete**
> Fleet inventory: **33 repositories discovered on the live GitHub organization inventory**
> Purpose: one auditable inventory for the organization-wide engineering audit.

## Audit contract

Each repository is processed through:

`DISCOVER → CLASSIFY → DOCUMENT → IMPLEMENT/FIX → TEST → RE-AUDIT → VERIFY → DOCUMENT STATE`

Required dimensions:

- code, syntax and logic
- functional behavior and tests
- security and dependency risk
- architecture and interfaces
- integration/connectivity
- completeness/stubs/TODO/FIXME/HACK
- duplicates/dead/legacy sources
- README/STATUS/SECURITY/CHANGELOG
- roadmap/TODO/sprints/wiki
- contradiction detection and resolution
- class/category/family/tag classification

A repository is **COMPLETE** only when all applicable dimensions have current evidence. `IN PROGRESS` is the default until verified.

## Fleet

| Repository | Audit state | Known current focus |
|---|---|---|
| `.github` | IN PROGRESS | org governance / shared automation |
| `atc-standards` | IN PROGRESS | standards registry/conformance consistency |
| `atclang` | IN PROGRESS | compiler/VM boundary and implementation completeness |
| `a-townchain` | IN PROGRESS | deterministic L1 / consensus integration |
| `a-townchain-os` | IN PROGRESS | system integration/deployment surface |
| `a-townchain-os-docs` | IN PROGRESS | stale roadmap/TODO/wiki claims |
| `atc-shivacore` | IN PROGRESS | historical source boundary; canonical source moved |
| `atc-contracts` | IN PROGRESS | contract/ATVM integration |
| `atc-node` | IN PROGRESS | node/runtime integration |
| `atc-compute` | IN PROGRESS | compute/runtime interfaces |
| `atc-oracle` | IN PROGRESS | oracle/security integration |
| `atc-sdk` | IN PROGRESS | SDK structure/docs consistency |
| `atc-marketplace` | IN PROGRESS | application/API integration |
| `atc-wallet` | IN PROGRESS | key/security/integration |
| `atc-vm` | IN PROGRESS | deterministic VM / ATCLang boundary |
| `atc-zkp` | IN PROGRESS | proof verification/integration |
| `aurora-ai` | IN PROGRESS | AI boundary / OS integration |
| `globus-os` | **P1 OPEN** | canonical ShivaCore LKM stub + graph ordering + symbol validation |
| `genesis-engine` | IN PROGRESS | engine subsystem completeness |
| `genesis-chronicles` | IN PROGRESS | game/application integration |
| `atc-mining` | IN PROGRESS | mining/runtime integration |
| `atc-indexer` | IN PROGRESS | chain/indexer consistency |
| `atc-interop` | IN PROGRESS | cross-chain/interface security |
| `atc-launchpad` | IN PROGRESS | deployment/application integration |
| `atc-explorer` | IN PROGRESS | indexer/API/UI integration |
| `atc-storage` | IN PROGRESS | persistence/runtime correctness |
| `atc-algorithm` | IN PROGRESS | deterministic algorithm/consensus correctness |
| `atc-ide` | IN PROGRESS | SDK/compiler/toolchain integration |
| `genesis-franchise-factory` | IN PROGRESS | game/economy integration |
| `atc-engineering` | ACTIVE | audit/control plane itself under audit |
| `a-townchain-ecosystem` | IN PROGRESS | integration / compliance / evidence / architecture control plane |
| `demo-repository` | IN PROGRESS | private-repository access/evidence path |
| `atc-toolchain` | IN PROGRESS | orchestration/tooling; must not become an authority layer |

## Fleet inventory reconciliation — 2026-10-06

The previous matrix contained 31 repositories. The live GitHub organization inventory contains 33 repositories.

The two repositories missing from the previous matrix were:

- `a-townchain-ecosystem`
- `atc-toolchain`

This update adds both repositories without changing historical audit snapshots.

The live inventory is the discovery source for fleet scope. Historical standards/audit snapshots that state 31 repositories remain historical evidence and must not be rewritten to imply that they were produced against the current 33-repository fleet.

## Confirmed findings currently tracked

### GlobusOS #18 — LKM dependency API stub

- Class: **P1**
- Category: **correctness / completeness**
- Family: **kernel / LKM / dependency-resolution**
- Tags: `P1`, `stub`, `kernel`, `lkm`, `dependency-graph`, `correctness`, `completeness`, `api`
- Status: **OPEN**

`DependencyGraph::dependencies()` is an `unimplemented!()` placeholder. The graph stores dependencies in `BTreeSet<String>`, so the declared borrowed-slice API cannot safely expose that storage. The existing owned `get_dependencies()` representation preserves deterministic ordering and is the appropriate canonical API shape.

### GlobusOS #19 — export/import semantic contradiction

- Class: **P1**
- Category: **logic / correctness / API semantics**
- Family: **kernel / LKM / symbol-resolution / reference-accounting**
- Tags: `P1`, `kernel`, `lkm`, `symbols`, `imports`, `exports`, `refcount`, `logic`
- Status: **OPEN**

`ModuleDescriptor::with_export()` currently adds the exported symbol to `imports` as well as `exports`. Export and import are distinct provider/consumer edges and must remain explicit.

### F-20260916-005 — topological-sort direction

- Repository: `globus-os`
- Path: `modules/atc-shivacore/kernel/src/lkm.rs`
- Class: **P1**
- Category: **correctness / logic / integration**
- Family: **kernel / LKM / dependency-resolution / graph-algorithm**
- Tags: `P1`, `kernel`, `lkm`, `dependency-graph`, `topological-sort`, `logic`
- Status: **OPEN**

The graph represents `module → dependency`. The current Kahn implementation increments the dependency node's indegree and therefore produces the reverse of the required dependency-first ordering. The load path separately implements dependency-first DFS, creating two competing ordering semantics. The graph API must have one canonical dependency-first meaning, with deterministic ordering and regression tests.

### F-20260916-006 — required symbol validation is not universally fail-closed

- Repository: `globus-os`
- Path: `modules/atc-shivacore/kernel/src/lkm.rs`
- Class: **P1**
- Category: **correctness / security / logic**
- Family: **kernel / LKM / symbol-resolution / load-validation**
- Tags: `P1`, `kernel`, `lkm`, `symbols`, `security`, `validation`, `logic`
- Status: **OPEN**

The load path computes unresolved imports but only enters the rejection branch when `optional_deps` is non-empty. Required unresolved imports therefore do not have a universal fail-closed gate. Required imports must always be rejected; optionality must be represented explicitly rather than inferred from the presence of the optional-dependency list.

## Static security checks

Organization-wide indexed source searches currently show no matches for:

- `pull_request_target`
- `actions/checkout@main`
- `actions/checkout@master`

Scanner definitions containing token/private-key regular expressions are not themselves credential exposure. Kernel `unsafe` and `panic!()` occurrences require contextual review and are not automatically vulnerabilities.

## CI mode

When GitHub Actions execution/logs are unavailable or incomplete, source-level audit continues. Runtime findings are not fabricated. A finding requiring CI proof remains open until verifiable CI evidence exists.

## Completion rule

Do not replace `IN PROGRESS` with `COMPLETE` based on historical wiki/README claims. Completion requires current source inspection plus verification evidence appropriate to the repository.
