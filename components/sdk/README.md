# atc-sdk

> Developer SDK, CLI and package tooling for the A-TownChain ecosystem.

**Project:** `atc-sdk`  
**Organization:** `A-TownChain-Okosystems`  
**Status:** `development`

## Purpose

`atc-sdk` provides developer-facing libraries and tooling for interacting with the A-TownChain stack. It is an SDK/tooling layer; it is not the consensus implementation and does not define ATC-VM semantics.

## Current repository layout

```text
.
├── modules/
│   ├── atc-sdk/      # SDK module and ATC-facing abstractions
│   ├── atc-cli/      # CLI and RPC client tooling
│   └── atc-atcpkg/   # ATC package tooling
├── .github/          # CI/security automation
├── docs/              # Repository documentation
├── AGENTS.md
├── ARCHITECTURE.md
├── CHANGELOG.md
├── FILE_REGISTER.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── SECURITY.md
└── STATUS.md
```

The authoritative source of module paths is the current Git tree. Documentation must not describe removed root-level Rust or TypeScript trees as active implementations.

## Architecture boundary

```text
ATCLang
   ↓
ATC-VM
   ↓
A-TownChain protocol / node
   ↑
ATC SDK / CLI / package tooling
```

The SDK consumes stable protocol and VM interfaces. It must not silently redefine consensus, state-transition, chain-identity, or execution semantics.

## Build and test

The repository contains multiple module-specific toolchains. Do not run `cargo build --workspace` from the repository root unless a root Cargo workspace is present.

Inspect the relevant module manifest first, then run its native build/test command. Examples:

```bash
# Rust module, when present
cargo build --manifest-path <module>/Cargo.toml
cargo test --manifest-path <module>/Cargo.toml

# Python CLI tooling, when present
python -m pip install -e <module>
python -m pytest <module>/tests
```

CI and module manifests are authoritative for the exact commands and supported toolchains.

## Integration rules

- Protocol and VM semantics remain canonical in their respective repositories.
- Client code must not hard-code network identity where the value belongs to canonical chain configuration.
- Cross-repository dependencies must be versioned or revision-pinned where determinism or reproducibility requires it.
- Devnet-only integration evidence must not be presented as production readiness.

## Security

Report vulnerabilities through [`SECURITY.md`](SECURITY.md). Do not disclose security-sensitive issues through public GitHub Issues.

## Documentation

Consult `AGENTS.md`, `STATUS.md`, `ROADMAP.md`, `ARCHITECTURE.md`, `FILE_REGISTER.md`, module manifests, and applicable ATC standards before implementation work.

## Governance

`atc-sdk` is governed by the canonical ATC standards registry and `ATC-STD-000`. `APPROVED`, `AUDITED`, `IMPLEMENTED`, and `PRODUCTION_READY` are distinct states and must not be conflated.

## License

See [`LICENSE`](LICENSE) for the authoritative license text.
