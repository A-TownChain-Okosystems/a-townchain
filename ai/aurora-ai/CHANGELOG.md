# Changelog — aurora-ai

Alle wesentlichen Änderungen an diesem Projekt werden in dieser Datei dokumentiert.

## [Unreleased] — 2026-10-18

### Added
- Validated Rust agent registry with unique caller-assigned IDs, capability metadata and typed errors.
- Bounded task scheduler with explicit lifecycle states, dependency gating, timeout handling and cancellation.
- Result collector with expected-agent attribution, duplicate/empty-result rejection and deterministic ordering.
- Unit tests for registry, scheduling, dependency, timeout/cancellation and result aggregation invariants.

### Verification
- Implementation commit evidence is recorded in the pull request.
- Build/test and GitHub Actions results remain pending until the exact PR head SHA has completed its checks.
- The existing .atc files are legacy/planned stubs; this implementation is in the canonical Rust atc-aurora-core crate and does not claim the ATCLang stubs are implemented.

## [0.1.0] - 2026-09-09

### Added
- Standards-Konformität hergestellt gemäß ATC-STD-README-001 und ATC-STD-MD-001.
- Erstellung von STATUS.md, ROADMAP.md, ARCHITECTURE.md, CONTRIBUTING.md, GOVERNANCE.md, CODE_OF_CONDUCT.md.
- Aktualisierung der AI Agent Instructions in AGENTS.md.

## [0.0.1] - 2026-09-07

### Added
- Governance-Überarbeitung nach ATC-STD-201/202/203.
- Rust Core und Python AI Services Grundstruktur.
