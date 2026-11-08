# A-TownChain OS / KAI-OS — Evidence-driven roadmap

> **Refresh:** 2026-10-09  
> **Scope:** ecosystem status, architecture, standards, CI and exact-SHA evidence.  
> **Authority:** this is a system-level coordination roadmap, not a normative protocol specification or production-release approval.

## How to read this roadmap

Older versions carried dated sprint percentages, repository/module/test counts and a 2027 Mainnet phase. Those values are historical and are not current completion evidence. The ecosystem does not have an approved Mainnet launch date. A phase is not complete because a table says 100%; it is complete only when its acceptance criteria and evidence are recorded against the relevant source SHA.

Current inventory and ownership are in [ECOSYSTEM_CURRENT_STATUS.md](ECOSYSTEM_CURRENT_STATUS.md) and [ARCHITECTURE_CURRENT.md](ARCHITECTURE_CURRENT.md). Normative standards are owned by [`atc-standards`](https://github.com/A-TownChain-Okosystems/atc-standards).

## Priority 1 — Correct stale status and close existing blockers

These are the CI outcomes observed on documentation PR heads during the 2026-10-09 refresh. They are specific to those exact heads and do not automatically describe default-branch status.

| Area | Observed blocker | Next action / exit criterion |
|---|---|---|
| A-TownChain core | PR #65: Determinism and Code Quality failed; CodeQL was in progress at the last check. | Inspect failing run logs; remediate the underlying findings; rerun all required gates on the new SHA. |
| Algorithm / consensus support docs | PR #13: Test Suite and Determinism failed on the earlier head; the newest documentation head had no PR-triggered run returned in the query. | Re-run on the current head; do not treat the hybrid PoH/PoS/PoW diagram as final protocol. |
| ZKP | PR #7: Determinism failed while several other gates passed. | Resolve determinism variance and rerun; retain security and trusted-setup gates. |
| Node runtime | PR #13: Determinism failed; CodeQL was in progress at last check. | Inspect exact run logs, fix findings, and rerun all required gates. |
| Contracts | PR #12: Test Suite failed while Determinism and other listed gates passed. | Fix tests or implementation; do not change the gate to obtain a pass. |
| GlobusOS / ShivaCore | PR #46: System CI, ATC Test Suite and Rust CI failed; PR #24 in the supporting ShivaCore repo has a failed Determinism Gate. | Fix current failures and continue TCB/VMM/hardware evidence work; supporting-repo green checks do not substitute for canonical GlobusOS validation. |
| System orchestrator | PR #130: Governance and KAI-OS PR Validation failed; Release Pipeline, Integration Gate and Dependency Review passed. | Inspect governance/PR-validation logs and rerun on the corrected head; do not infer release readiness from the Windows pipeline pass. |
| ATC IDE | PR #2 corrects the documentation claim: Markdown Lint exists, while the README still declares no production build or application test suite. | Keep the product experimental until application build/test/security gates are implemented and evidenced. |
| Central documentation | PR #40: the earlier Docs Gate failed because the historical `REALITY_STATUS.md` length guard was violated; the historical content has been restored with a deprecation banner. Current-head checks must confirm the correction. | Preserve historical evidence and satisfy the existing gate; never weaken the length guard. |
| Standards registry | Main SHA `65d43b6b3ac2e82723bfd9bc2a10d001e3cc0fc9`: generated-view workflow failed because GitHub Actions could not create a PR, after generation succeeded. | Review generated-view PR [atc-standards #102](https://github.com/A-TownChain-Okosystems/atc-standards/pull/102); do not edit generated views manually. |
| Engineering fleet audit | Main SHA `470e6a9f500fdade0ea33065d30dd76b58d6492b`: fleet audit failed while Markdown lint, verification and self-test passed. | Inspect fleet-audit findings and remediate/re-audit; a green self-test does not close fleet findings. |
| System orchestrator | Main SHA `8f5df13dba2918ea3f53f6edb24d00ca9748013a`: audit failed; quality and integration checks passed. PR #130 also has Governance and PR Validation failures. | Inspect both current main audit and PR validation logs; keep release status blocked until evidence supports closure. |

Full SHAs, run IDs and observed results are in the [current status snapshot](ECOSYSTEM_CURRENT_STATUS.md). Refresh that table after any head change; a previous SHA's run is not evidence for a later SHA.

## Priority 2 — Consolidate architecture and source-of-truth ownership

- Keep normative standards and generated views in `atc-standards`; do not manually edit generated files or duplicate normative counts in this documentation hub.
- Keep the canonical ATC-VM at `a-townchain/components/vm`.
- Keep the canonical algorithm/consensus implementation at `a-townchain/components/algorithm`; protocol choice remains subject to approved specification and evidence.
- Keep the canonical ShivaCore kernel at `globus-os/modules/atc-shivacore/kernel/`; keep complex networking, storage, blockchain and AI runtimes in service space.
- Keep Aurora AI in the AI/service layer. Intent → policy → capability check → approval when required → bounded tool/service call → authoritative runtime.
- Keep `a-townchain-ecosystem` as integration/compliance/evidence control plane and `a-townchain-os` as system orchestrator; neither replaces component SSOT.
- Reconcile imported subtree snapshots against their canonical source SHAs and document interface/version compatibility explicitly.

## Priority 3 — Complete documentation, standards, CI and evidence refresh

For every non-archived repository, the refresh workflow must cover:

1. README, STATUS, ARCHITECTURE, ROADMAP and evidence registry.
2. Registry ownership, lifecycle/archive state and dependencies.
3. Current source SHA and relevant Actions runs, jobs, steps, exit statuses and logs.
4. Distinction between SPECIFIED, IMPLEMENTED, TESTED, VERIFIED, AUDITED, CONFORMANT and PRODUCTION_READY.
5. Dependency and interface compatibility across ATCLang → canonical ATC-VM → chain/node runtime → consumer services.
6. Roadmap acceptance criteria and residual blockers.
7. Link and Markdown checks on the documentation PR head.

The 2026-10-09 inventory/README pass covered 26 non-archived repositories, but exact-SHA CI evidence was inspected only for the PRs listed in the snapshot. This is a **partial fleet audit**, not a claim that all 26 repositories are verified.

## Release and security gates

- No release or Mainnet readiness until all applicable implementation, determinism, conformance, security, reproducibility, operations, recovery and owner-approval gates pass.
- A green governance check does not imply a green build or test suite.
- A green test suite does not imply a security audit or production readiness.
- An audit score does not replace current code review and exact-SHA evidence.
- Failed or unavailable checks remain FAILED / BLOCKED / UNKNOWN as appropriate; they are not silently converted to PASS.
- Do not weaken branch protection, required checks, security policy or standards to make the roadmap look complete.

## No scheduled Mainnet launch

There is no approved Mainnet launch date in this roadmap. Mainnet planning is gated by verified implementation and release evidence, not by calendar targets or historical sprint percentages.

## Evidence lifecycle

```text
REGISTERED → RUNNING → ANALYZED → VERIFIED
                    ↘ FAILED
                    ↘ BLOCKED
VERIFIED → RESIDUAL  (when new contradictory evidence appears)
```

After remediation, rerun the affected gate chain on the new SHA and update the status record. Preserve previous runs as immutable history.
