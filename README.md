# atc-sdk

> Developer SDK, CLI and package tooling for the A-TownChain ecosystem.

**Project:** `atc-sdk`  
**Organization:** `A-TownChain-Okosystems`  
**Status:** `development`

## Purpose

`atc-sdk` provides developer-facing libraries and tooling for interacting with the A-TownChain stack. It is an SDK/tooling layer, not the chain protocol implementation and not the ATC-VM itself.

The repository currently contains or is intended to contain:

- `atc-sdk` — developer SDK libraries
- `atc-cli` — command-line tooling
- `atc-atcpkg` — ATC package-system tooling

## Architecture Boundary

```text
ATCLang
   ↓
ATC-VM
   ↓
A-TownChain protocol / node
   ↑
ATC SDK / CLI / package tooling
```

The SDK consumes stable protocol/VM interfaces. It must not silently redefine consensus, state-transition, or chain-identity semantics.

## Status

`development` means the repository is under active implementation/rebuild. Roadmap milestones or audit levels are not equivalent to `PRODUCTION_READY`.

Historical vault-restoration information is retained in repository history and documentation where relevant; it is not treated as evidence that the current implementation is complete.

## Repository Structure

```text
.
├── .atc/       # ATC repository metadata
├── docs/       # Documentation
├── modules/    # SDK / CLI / package modules, where present
├── tests/      # Tests, where present
├── AGENTS.md   # AI agent instructions, where present
├── CHANGELOG.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── SECURITY.md
└── STATUS.md
```

The exact module layout is authoritative in the current repository tree and Cargo workspace manifests.

## Requirements

- Rust/Cargo for Rust components
- Python only where repository tooling requires it
- Git

Use the versions declared by the repository's current toolchain/manifests rather than treating historical README values as permanent requirements.

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-sdk.git
cd atc-sdk
cargo build --workspace
```

If the current workspace contains additional language-specific modules, follow their local build instructions.

## Testing

```bash
cargo test --workspace
```

Use CI results for the authoritative status of the current commit. Planned milestone tests must not be reported as completed implementation evidence.

## Development

- Follow `ATC-STD-000` and the current ATC governance process.
- Use Conventional Commits where required by repository policy.
- Keep SDK abstractions aligned with canonical protocol and VM interfaces.
- Do not hard-code a chain identity in client code when the value belongs to the canonical network/chain configuration.
- Family-scoped standards use `ATC-STD-F{family}-{sequence}`. Legacy IDs remain historical references until explicitly migrated through governance.

## Security

Report vulnerabilities using [`SECURITY.md`](SECURITY.md). Do not disclose security-sensitive issues through public GitHub Issues.

## Documentation

Consult the repository's `docs/`, `STATUS.md`, `ROADMAP.md`, and current manifests before implementation work.

## Governance

`atc-sdk` is governed by the canonical ATC standards registry and `ATC-STD-000`. `APPROVED`, `AUDITED`, `IMPLEMENTED`, and `PRODUCTION_READY` are distinct states and must not be conflated.

## License

See [`LICENSE`](LICENSE) for the authoritative license text.

## AI Agent Instructions

Before making changes, inspect the current `AGENTS.md` (if present), `STATUS.md`, `ROADMAP.md`, Cargo manifests, and applicable ATC standards. Validate SDK behavior against the current protocol/VM interfaces and run the relevant test suite before claiming completion.
