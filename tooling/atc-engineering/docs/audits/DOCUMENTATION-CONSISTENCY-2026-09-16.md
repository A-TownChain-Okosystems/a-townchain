# Documentation consistency audit — 2026-09-16

## Confirmed contradiction

`a-townchain-os-docs/wiki/main/docs/ROADMAP.md` previously represented the repository program as `43/43` complete and pointed directly to Mainnet launch, while the current multi-repository architecture still has open P1 implementation findings and incomplete CI evidence.

The roadmap was corrected to an audit-driven state and now explicitly distinguishes historical milestones from current verification evidence.

## Historical-vs-current rule

Documents under explicit archive paths are historical reference material. Current readiness, implementation and release claims must be derived from current source plus current verification evidence.

## Remaining sweep

The documentation fleet still requires repository-by-repository synchronization of README, STATUS, SECURITY, CHANGELOG, roadmap, TODO, sprint and wiki material. Legacy completion claims must either be updated against current evidence or explicitly classified as historical.

## Classification

- Class: P2
- Category: consistency / documentation
- Family: documentation lifecycle / current-vs-historical state
- Tags: `P2`, `documentation`, `consistency`, `roadmap`, `wiki`, `legacy`, `audit`
- Status: IN PROGRESS
