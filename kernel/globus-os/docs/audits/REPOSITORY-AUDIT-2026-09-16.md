# Repository Audit — globus-os — 2026-09-16

## Audit contract

**Vision → Konzept → Komponenten → Code → Test → Fehler beheben → Re-Test → tatsächliche Komponenten dokumentieren**

Audit mode: CI-independent source audit plus GitHub workflow enforcement review. Findings are not closed by documentation alone.

## Repository role

`globus-os` is the canonical GlobusOS userspace/platform repository and the canonical owner of the integrated ShivaCore source under `modules/atc-shivacore/kernel/`. AI and blockchain integration remain outside the ShivaCore TCB.

## Classification

- Class: OS / platform
- Category: operating-system platform and kernel integration
- Families: kernel, userspace, security, storage, networking, device integration, identity, update/recovery
- Languages: Rust canonical for system implementation; YAML for CI/governance; Markdown for normative documentation
- Readiness: development / NOT_READY for production hardware claims

## Findings

### F-GLOBUS-001 — P1 — STUB — LKM dependency API
- Category: correctness / completeness
- Family: kernel / LKM / dependency-resolution
- Tags: `P1`, `stub`, `kernel`, `lkm`, `correctness`, `completeness`, `api`
- `DependencyGraph::dependencies()` is an `unimplemented!()` placeholder with an impossible `&[String]` API over `BTreeSet<String>`.
- Existing issue: #18.
- Closure requires replacement with a lifetime-safe deterministic API, caller/test updates, fmt, clippy, workspace tests and CI.

### F-GLOBUS-002 — P1 — LOGIC — dependency ordering
- Category: correctness / determinism
- Family: kernel / LKM / dependency-resolution
- Tags: `P1`, `lkm`, `topological-sort`, `determinism`
- `topological_sort()` currently increments the in-degree of dependency nodes for `module -> dependency` edges, while the required load contract is dependency-first. `load_order()` already expresses the correct direction.
- Tracked in issue #20.

### F-GLOBUS-003 — P1 — SECURITY — unresolved imports not fail-closed
- Category: security / symbol resolution
- Family: kernel / LKM / symbol-resolution
- Tags: `P1`, `security`, `fail-closed`, `imports`
- `load()` only enters unresolved-symbol rejection when optional dependencies are present. Required unresolved imports must be rejected independently of optional dependency configuration.
- Tracked in issue #20.

### F-GLOBUS-004 — P1 — CONSISTENCY — exports are also recorded as imports
- Category: correctness / accounting
- Family: kernel / LKM / symbol-table
- Tags: `P1`, `symbols`, `exports`, `imports`, `consistency`
- `ModuleDescriptor::with_export()` and `ModuleBuilder::export()` add exported symbols to `imports`, contaminating import accounting and symbol-direction semantics.
- Tracked in issue #20.

### F-GLOBUS-005 — P1 — CI ENFORCEMENT
- Category: governance / CI
- Family: CI / test enforcement
- Tags: `P1`, `CI`, `pull-request`, `enforcement`
- The main test suite was previously configured for manual dispatch and pushes to `main`, but not pull requests. This allowed the primary test suite not to be an enforced PR gate.
- Correction implemented on audit branch: `pull_request:` trigger added.
- Closure requires a fresh GitHub Actions run on the corrected branch.

## Security / malware evidence

The repository has RustSec dependency auditing and additional security workflow coverage. The test suite and governance workflow use read-only permissions except the evidence-writing job. This is evidence against specific classes of supply-chain and regression failures, not proof of universal malware immunity.

No claim of absolute protection against hacking or viruses is made. Stronger assurance requires current CI evidence, dependency vulnerability results, secret scanning, immutable/reproducible build evidence, signed provenance, SBOM validation, fuzz/property testing and hardware/QEMU execution where applicable.

## Architecture and integration

Canonical boundaries are:

`ATCLang → ATC-VM → A-TownChain`

`Aurora → GlobusOS IPC/API → ShivaCore`

`GlobusOS Identity → atc-wallet integration → A-TownChain identity`

Hardware-facing P1 work remains separate from production readiness. UEFI, PCIe/IOMMU, NVMe, Ethernet DMA, persistent filesystem and measured/cryptographic boot require execution evidence before being promoted to production status.

## File/language assessment

- Rust source: correct canonical implementation language for kernel/system components.
- YAML: appropriate for GitHub Actions and machine-readable governance metadata.
- Markdown: appropriate for human/normative audit and architecture documentation.
- No automatic file-format migration is justified without a concrete semantic benefit.

## Verification state

| Area | State |
|---|---|
| Architecture | REVIEWED |
| Syntax | SOURCE REVIEWED; CI RECHECK REQUIRED |
| Logic | P1 findings OPEN |
| Security | P1 finding OPEN |
| CI enforcement | CORRECTED ON AUDIT BRANCH; RUN REQUIRED |
| Stubs | P1 `unimplemented!()` OPEN |
| Completeness | NOT COMPLETE |
| Production readiness | NOT READY |

## Closure rule

A finding becomes CLOSED only after source change, source re-read, targeted regression test, applicable static/security checks, and current CI evidence. Historical evidence is never treated as current verification.
