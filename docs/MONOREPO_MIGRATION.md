# A‑TownChain Blockchain Monorepo Migration

## Target

`A-TownChain-Okosystems/a-townchain` is the canonical repository for the A‑TownChain blockchain stack.

The existing repository remains the root/core component. Other blockchain repositories are integrated below `components/` to prevent path collisions.

## Components

| Source repository | Target path |
|---|---|
| `atc-node` | `components/node` |
| `atc-algorithm` | `components/algorithm` |
| `atc-vm` | `components/vm` |
| `atc-contracts` | `components/contracts` |
| `atc-sdk` | `components/sdk` |
| `atc-wallet` | `components/wallet` |
| `atc-explorer` | `components/explorer` |
| `atc-indexer` | `components/indexer` |
| `atc-mining` | `components/mining` |
| `atc-oracle` | `components/oracle` |
| `atc-storage` | `components/storage` |
| `atc-interop` | `components/interop` |
| `atc-zkp` | `components/zkp` |

## Migration requirements

1. Preserve source Git history with `git subtree` or an equivalent history-preserving import.
2. Import into the namespaced target path; do not flatten component roots.
3. Inspect and reconcile `Cargo.toml`, workspace membership, lockfiles, CI workflows, CODEOWNERS, README files, AGENTS files, licenses, and security policies.
4. Update internal repository URLs and cross-component paths.
5. Update the root architecture, status, roadmap, file register, and dependency documentation.
6. Run formatting, compilation, tests, linting, security checks, and repository governance checks after each migration batch.
7. Use GitHub Dependency Graph checks when available; skip only where the GitHub feature is unavailable.
8. Keep source repositories intact until the monorepo has passed validation.
9. Only after validation may source repositories be archived/decommissioned; deletion is a separate explicit operation.

## Current connector boundary

The connected GitHub API can create and update individual repository files but does not expose a bulk Git clone/push or Git tree-write operation. Therefore the complete history-preserving import must be executed by Git locally or by a GitHub Actions workflow with repository write permissions.

## Recommended history-preserving import

From a local clone of `a-townchain`:

```bash
git remote add atc-node https://github.com/A-TownChain-Okosystems/atc-node.git
git fetch atc-node main
git subtree add --prefix=components/node atc-node main --squash
```

Repeat the same operation for every source repository in `MIGRATION_MANIFEST.yaml`. Omit `--squash` if full source commit history is required in the resulting repository.

After all imports:

```bash
git status
git log --oneline --decorate -20
```

Then validate the complete workspace and governance gates before archiving any source repository.
