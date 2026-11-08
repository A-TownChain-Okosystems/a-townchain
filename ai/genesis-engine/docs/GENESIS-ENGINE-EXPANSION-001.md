# GENESIS-ENGINE-EXPANSION-001

## Purpose

Define the repository topology required to evolve Genesis Engine from a runtime/simulation project into a general-purpose game development platform comparable in capability class to Unity/Unreal.

## Repository topology

| Repository | Responsibility | Priority |
|---|---|---|
| `genesis-engine` | Core runtime, ECS orchestration, lifecycle | P0 |
| `genesis-renderer` | Render graph, GPU abstraction, PBR, materials, shaders | P0 |
| `genesis-physics` | 2D/3D physics, collision, queries, constraints | P0 |
| `genesis-audio` | Mixer, spatial audio, streaming, DSP | P0 |
| `genesis-animation` | Skeletons, clips, state machines, blend trees, IK | P0 |
| `genesis-assets` | Import, UUIDs, dependency graph, cache, cooking | P0 |
| `genesis-world` | Scenes, terrain, foliage, streaming, world partition | P0 |
| `genesis-ui` | Runtime UI, layout, input, styling, accessibility | P0 |
| `genesis-tools` | Asset/scene/shader/mesh processing and diagnostics | P1 |
| `genesis-cli` | Project, build, test, run, package commands | P1 |
| `genesis-build` | Cross-platform build/cook/package pipeline | P1 |
| `genesis-sdk` | Stable game-facing APIs and plugin ABI | P1 |
| `genesis-ai` | Navigation, behavior trees, utility AI, gameplay AI | P1 |
| `genesis-network` | Replication, prediction, dedicated server runtime | P1 |
| `genesis-templates` | Official game/project templates | P1 |
| `genesis-samples` | Reference games and technical samples | P1 |
| `genesis-docs` | Engine documentation and tutorials | P1 |

## Existing repository mapping

`genesis-engine` already contains the core engine, ECS, creatures and world modules. `atc-ide` is the existing organization-level IDE foundation and can host shared editor technology while a dedicated Genesis editor product boundary is established.

## Architectural rules

1. Genesis remains usable without A-TownChain.
2. ATC integration is an optional integration layer, never a runtime prerequisite for ordinary games.
3. Game-specific code stays outside the engine core.
4. Renderer, physics, audio and asset systems expose stable interfaces and replaceable backends.
5. Editor tooling must consume the same public engine APIs available to games.
6. Runtime data must be serializable and versioned.
7. Import/cook/build operations are deterministic where practical and produce evidence artifacts.
8. Platform-specific code is isolated behind explicit backend interfaces.

## P0 completion gate

Genesis is not considered an engine-platform baseline until the P0 systems provide:

- ECS/world lifecycle
- 2D and 3D rendering abstraction
- asset import and dependency management
- scene serialization
- physics/collision interfaces
- animation runtime
- audio runtime
- input and runtime UI
- deterministic project loading
- automated smoke tests

## P1 completion gate

The platform becomes a practical game-production stack when the P1 systems additionally provide:

- editor integration
- CLI/project generation
- asset cooking
- cross-platform packaging
- profiling/debugging
- networking
- AI/navigation
- stable game SDK
- templates and samples

## Current execution strategy

Until organization-level repository creation is available through the connected GitHub tool, the new subsystem contracts are bootstrapped inside `genesis-engine/modules` and can later be promoted to dedicated repositories without changing the public interfaces.
