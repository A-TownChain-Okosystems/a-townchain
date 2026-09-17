# ARCHITECTURE.md — atc-sdk

## Scope

`atc-sdk` is the developer-facing integration layer for A-TownChain. It consumes canonical protocol and VM interfaces; it does not own consensus or redefine execution semantics.

## Current file/module model

```text
.
├── modules/
│   ├── atc-sdk/
│   │   ├── *.atc
│   │   ├── README.md
│   │   ├── STATUS.md
│   │   └── module-specific manifests/tooling
│   ├── atc-cli/
│   │   └── CLI, RPC and integration tooling
│   └── atc-atcpkg/
│       └── ATC package tooling
├── .github/
├── docs/
└── repository governance files
```

The previous documentation described `rust/` and `typescript/` directories at the repository root. Those paths are not present on the current `main` tree and must not be treated as active implementation paths.

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

### Rules

1. Consensus-critical semantics remain canonical in `a-townchain` / `atc-vm` and the applicable standards.
2. SDK clients may serialize, submit and decode protocol data but must not invent alternative state-transition rules.
3. Chain/network identity is obtained from canonical configuration or protocol responses rather than duplicated constants.
4. Cross-repository test dependencies must be revision-pinned when reproducibility or determinism depends on them.
5. Devnet-only behavior must be explicitly marked and must not be represented as production readiness.

## Build model

Build commands are module-specific. The repository root is not documented as a Cargo workspace unless a root `Cargo.toml` exists.

Use the manifest inside the target module and its CI workflow as the authoritative build/test contract.
