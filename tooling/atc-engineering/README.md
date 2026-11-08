# ATC Engineering

> **A-TownChain Engineering & Governance Platform**

`atc-engineering` is the central engineering and governance control plane for the A-TownChain repository fleet.

## Mission

**No Evidence, No Trust.**

Repository state, compliance state and release readiness are derived from authoritative inputs and verifiable evidence rather than declarations.

## Responsibilities

- Repository discovery and intelligence
- Target software requirements discovery
- Standards enforcement
- Policy evaluation
- Governance gates
- Repository auditing
- Iterative remediation and re-audit
- Security validation
- Build and test orchestration
- Evidence generation
- Release engineering
- Repository bootstrapping
- GitHub integration
- AI-agent engineering governance

## Target Software Requirements Discovery

The `atc-requirements` crate analyzes the software being built and derives what that target needs before it can be considered complete. It separates **observable facts** from **required architecture and content**.

The planner covers:

- runtime and architectural components
- configuration and schemas
- dependency/integration contracts
- specifications and component inventories
- unit, integration and negative tests
- architecture and operations documentation
- ownership and change control
- security and supply-chain controls
- reproducible build and release evidence
- technology-specific CI requirements

A detected file or technology is only a signal; it does not prove implementation completeness.

```bash
cargo run --release -p atc-audit-cli -- /path/to/target --requirements
```

The command exits non-zero while mandatory requirements remain missing. See `docs/architecture/REQUIREMENTS-DISCOVERY-001.md`.

## Architectural Boundary

ATC Engineering is not the normative source of standards.

```text
atc-standards
      │ normative standards
      ▼
atc-engineering
      │ requirements, enforcement and automation
      ▼
A-TownChain repository fleet
```

`atc-standards` defines **what** must be true. `atc-engineering` determines the target requirements and validates/enforces those requirements.

## Implemented Audit Runtime

The `atc-maintenance` crate provides a dependency-free, fail-closed repository scanner covering:

- required governance files
- high-confidence credential patterns
- unsafe download/command pipelines
- GitHub Actions permission policy checks
- Rust workspace lock evidence
- broken Rust path dependencies
- unresolved source TODO/FIXME markers
- duplicate source/config content
- README/repository identity consistency

The `atc-audit-cli` crate exposes the scanner as an executable and is used by the organization-wide fleet workflow.

```bash
cargo run --release -p atc-audit-cli -- /path/to/repository
```

The CLI exits non-zero when findings are present, preserving fail-closed behavior.

## Iterative Remediation

ATC Engineering can execute a bounded closed-loop remediation run. It audits, applies only deterministic safe fixes, updates the affected artifact, and immediately re-audits. The loop stops only when the repository is clean, the finding set is stable, or the hard iteration limit is reached.

```bash
cargo run --release -p atc-audit-cli -- /path/to/repository --repair
cargo run --release -p atc-audit-cli -- /path/to/repository --repair --max-repair-iterations 12
```

Safe remediation currently includes creation of a missing engineering-audit document and correction of missing repository identity in `README.md`. Semantic source changes, security fixes, dependency repairs and CI policy changes remain explicit engineering work and are reported as blocked rather than guessed.

See `docs/architecture/ITERATIVE-REMEDIATION-001.md`.

## Release Gate and Machine-Readable Evidence

The `atc-gates` crate provides the deterministic enforcement boundary between audit evidence and release state. `UNKNOWN`, `MISSING` and `FAIL` evidence block the gate. Production additionally requires valid separation of duties and explicit human approval.

Run the gate-integrated audit with:

```bash
cargo run --release -p atc-audit-cli -- /path/to/repository --gate
```

The command emits one deterministic `GATE_REPORT` JSON record using schema `atc.gate-report/v1`.

## CI Enforcement

`.github/workflows/engineering-ci.yml` enforces formatting, workspace tests, Clippy with `-D warnings`, release build, the audit gate, gate-report schema validation and artifact publication.

```text
Target Requirements → Standards/Controls → Evidence → Audit → Remediation → Re-Audit → GateReport → CI Artifact → Release Gate
```

## Fleet Audit

`.github/workflows/fleet-audit.yml` validates the engineering platform itself and audits every discoverable **public** repository in `A-TownChain-Okosystems` on push, pull request, weekly schedule and manual dispatch.

Private repositories are not silently treated as clean; they require a separate authenticated audit path with credentials authorized for those repositories.

## CLI

```text
atc-audit-cli /path/to/repository
atc-audit-cli /path/to/repository --requirements
atc-audit-cli /path/to/repository --repair
atc-audit-cli /path/to/repository --gate
```

Planned higher-level commands remain:

```text
atc-engine repo audit atc-node
atc-engine org audit A-TownChain-Okosystems
atc-engine standards validate atc-node
atc-engine policy evaluate --repo atc-node --operation release
atc-engine evidence collect --repo atc-node
atc-engine release validate atc-node
```

## Governance

ATC Engineering uses separation of duties: Coder, Validator, Auditor, Release Authority and Human Approver.

AI agents may operate only within explicitly authorized scopes. Production release requires valid evidence and human governance approval.

## Failure Policy

ATC Engineering is fail-closed. Missing, invalid or unverifiable evidence blocks the affected operation.

## Standards

The canonical standards source is `A-TownChain-Okosystems/atc-standards`.

The `atc-standards` crate reads the canonical registry and validates lifecycle, identity, normative status and release status fail-closed. Standards remain owned by `atc-standards`; this repository is an enforcement consumer.

## Security

Security is a release requirement. The platform is designed to support dependency auditing, SBOM generation, SAST, secret detection, license validation, artifact integrity, provenance and supply-chain security.

See `SECURITY.md`.

## Development

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --release
```

## Status

See `STATUS.md` for the authoritative implementation/evidence state and `ROADMAP.md` for the planned progression.

Current status: **IMPLEMENTED (Phase 0, partial; Phase 1 partial)**. The repository is not a production platform yet.

## License

See `LICENSE`.
