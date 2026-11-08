# Genesis Engine Architecture

## Layer model

Genesis Engine is organized as a deterministic, backend-independent engine core with explicit integration boundaries.

```text
Game / Application
        |
      SDK
        |
 Gameplay / World / ECS
        |
 Platform Contracts
   /    |      |     \
Render Physics  Audio  Assets
  |      |       |       |
GPU    Physics Device  Import/Cook
Backend Backend Backend Pipeline
        |
   OS / Hardware
```

## Workspace components

- `atc-genesis-platform`: stable contracts and shared identifiers.
- `atc-genesis-renderer`: render graph and rendering abstractions.
- `atc-genesis-physics`: deterministic simulation core.
- `atc-genesis-audio`: audio runtime abstractions.
- `atc-genesis-animation`: skeleton, clips and animation playback.
- `atc-genesis-assets`: asset registry and deterministic asset metadata.
- `atc-genesis-ui`: UI interaction primitives.
- `atc-genesis-sdk`: application-facing engine API.
- `atc-genesis-ai`: deterministic gameplay AI primitives.
- `atc-genesis-network`: replication and transport abstractions.
- `atc-genesis-build`: deterministic build/package foundations.
- `atc-genesis-tools`: profiling/tooling primitives.
- `atc-genesis-cli`: project/build/test command-line interface.
- `atc-genesis-editor`: scene document/editor core.
- `atc-genesis-input`: input state/event model.
- `atc-genesis-world`: world/chunk streaming model.
- `atc-genesis-gameplay`: gameplay runtime foundation.

## Integration boundaries

The engine core must not silently depend on a specific GPU, operating-system audio API, socket implementation, or proprietary content format. Concrete platform integrations belong behind explicit backend traits or adapters.

### Rendering

The renderer core owns render-graph construction, deterministic ordering, visibility decisions and resource references. A concrete GPU backend is responsible for command submission, resource allocation and shader execution.

### Physics

The physics core owns deterministic simulation state and collision primitives. Platform integration must provide timing and, where required, hardware acceleration without changing deterministic simulation semantics.

### Audio

The audio runtime owns voices, mixing policy and spatial parameters. A platform backend owns device output and sample delivery.

### Networking

Replication state and serialization remain transport-independent. Socket, encryption and connection-management implementations belong to a concrete transport adapter.

### Assets

Asset IDs and dependency metadata must remain deterministic. Format-specific importers and platform/GPU uploads belong to the import/cook/backend layers.

## Determinism

Systems that participate in authoritative simulation must avoid dependence on wall-clock timing, unordered iteration, platform-specific floating-point behavior where deterministic alternatives are required, and nondeterministic external state.

Network snapshots, build manifests and gameplay decisions should use stable ordering and explicit identifiers.

## Verification levels

The project distinguishes:

1. **Contract** — API/type exists and invariants are documented.
2. **Core** — deterministic local implementation exists with unit tests.
3. **Integration** — subsystem is connected to another concrete subsystem.
4. **Backend** — real OS/hardware/runtime backend exists.
5. **Production** — reproducible build, integration tests, security review and release evidence exist.

Presence of a contract or unit test must not be interpreted as production readiness.
