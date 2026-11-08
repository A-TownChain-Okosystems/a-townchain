# Genesis Engine — Enterprise Engineering Baseline

**Status:** Development / Enterprise hardening in progress  
**Scope:** Workspace, runtime, networking, ECS integration, security, CI/CD and release engineering

## Integration audit

The runtime depends directly on `atc-genesis-ecs` through a local Cargo path dependency, and the root workspace now explicitly registers `modules/atc-genesis-ecs`. This keeps ECS in the same workspace graph as world, physics, gameplay, input, animation, audio, networking, renderer and runtime.

The runtime tick connects animation → gameplay → world/ECS bridge → world/physics bridge → physics → ECS → audio → renderer → network tick. Network receive connects packet validation → entity validation → existing ECS entity → transform application.

## ECS hierarchy hardening

The ECS hierarchy rejects missing parents, self-parenting and cycles before a parent link is committed. World-transform resolution also rejects unknown entities instead of returning a fabricated transform.

Loaded world chunks are mirrored into deterministic reserved ECS IDs and stale chunk entities are removed when chunks are no longer loaded. The bridge uses a set for active-chunk membership and sorts returned results by chunk ID, avoiding order-dependent synchronization behavior.

## Runtime network trust boundary

Transport → packet-size boundary → binary decoding → wire-format validation → session validation → expected-entity authorization → peer/sequence/tick validation → existing ECS entity validation → ECS state application.

`PacketGuard` enforces packet size, session identity, packet structure, entity/tick consistency, replay protection, monotonic tick ordering, and replicated quaternion bounds. The entity-scoped validation path now performs ownership validation before committing peer sequence/tick state. Therefore an unauthorized packet cannot consume a valid sequence number and create a replay-state side effect.

Unknown ECS entities are never implicitly created from replication traffic.

## Replication and security hardening

The former `rotation_millirad` field was inconsistent with the actual quaternion x/y/z micro-unit representation. It is now `rotation_xyz_microunits`, with a consistent `1e-6` interpretation. The packet byte layout remains 64 bytes.

Invalid quaternion vector magnitudes are rejected using deterministic integer-only validation before network acceptance or ECS mutation. Local publication rejects non-finite transforms and normalizes valid quaternions before quantization. Regression coverage exercises malformed rotations, interpolation rejection, secure-packet rejection, non-finite transforms and no-mutation-on-rejection behavior.

## Error register

| ID | Finding | Severity | State | Resolution |
|---|---|---|---|---|
| NET-001 | Replication rotation units were inconsistent | High | Resolved in source | Unified on `rotation_xyz_microunits` |
| SEC-003 | Invalid quaternion vector magnitude could cross the network trust boundary | High | Resolved in source | Integer norm validation and fail-closed rejection |
| SEC-004 | Entity authorization occurred after peer sequence state was committed | High | Resolved in source | Entity-scoped validation now authorizes before sequence/tick commit; regression added |
| CONS-001 | Stale `rotation_millirad` references remained after wire-format correction | High | Resolved in source | References removed and repository search performed |
| CONS-002 | `atc-genesis-ecs` was a runtime path dependency but absent from root workspace members | High | Resolved in source | ECS explicitly added to `[workspace].members` |
| ECS-001 | Hierarchy/world-transform path could be called for an unknown entity; cycle traversal guard was based on total transforms | Medium | Resolved in source | Unknown entities return `None`; cycle traversal bounded by hierarchy edges; regression added |
| ECS-002 | Chunk bridge used linear active-chunk membership checks | Low | Resolved in source | Active chunk IDs now use a set; deterministic output ordering retained |
| REL-001 | No committed `Cargo.lock` | High | Open | Generate, review and commit before reproducible release claims |
| SEC-001 | No verified cryptographic peer authentication/encryption | High | Open | Production transport security required |
| NET-002 | No verified UDP/QUIC production transport | High | Open | Production transport implementation and integration evidence required |
| SEC-002 | No verified rate limiting/bandwidth/DoS controls | High | Open | Resource governance and abuse tests required |
| REL-002 | No verified SBOM/signing/attestation | Medium | Open | Release supply-chain pipeline required |

## Verification status

Source-level verification for this audit confirms:

- ECS is explicitly registered in the workspace.
- Runtime has a direct ECS dependency.
- ECS hierarchy rejects self-parenting, missing parents and cycles.
- Unknown entities do not resolve to fabricated world transforms.
- World chunks are mirrored deterministically and stale chunk entities are removed.
- Entity-scoped network validation performs ownership authorization before peer sequence/tick state is committed.
- Unauthorized entity regression coverage verifies that sequence state remains unchanged.
- Network validation rejects malformed/invalid replication state before ECS mutation.
- Repository search previously found no stale `rotation_millirad` reference.

A successful local `cargo check`, `cargo test`, `cargo fmt`, `cargo clippy` or `cargo doc` run is **not** claimed. Full CI remains the authoritative verification layer.

## Remaining production security gaps

- cryptographic peer authentication
- encrypted transport
- production UDP/QUIC transport
- rate limiting and bandwidth budgets
- connection lifecycle management
- key rotation
- transport-level DoS protection
- committed release lockfile
- SBOM and artifact signing/attestation

> No Evidence, No Trust.
