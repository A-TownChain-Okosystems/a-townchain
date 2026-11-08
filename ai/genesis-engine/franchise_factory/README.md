# Genesis Franchise Factory

The Franchise Factory is now an integrated production subsystem of `genesis-engine`.

## Boundary

```text
Franchise Factory
  ├─ 17 AI production workflows
  ├─ Game Factory dependency graph
  ├─ Artifact + provenance contracts
  ├─ Franchise core pipeline
  ├─ DAO model
  └─ Lifecycle Manager
          │
          ▼
Genesis Engine Runtime / Build / Editor / SDK
          │
          ▼
Games and franchises
```

`genesis-chronicles` remains an independent consumer/flagship title and is not a dependency of this subsystem.

## Components

- `gff/core.py` — AD-20 franchise registry and pipeline orchestration
- `gff/dao.py` — ATC-9900 franchise DAO model
- `gff/lifecycle.py` — AD-43 lifecycle state machine
- `gff/spec_loader.py` — canonical `.atc` descriptor loader
- `gff/workflows.py` — 17 provider-neutral AI factory workflows
- `gff/artifacts.py` — typed artifact/provenance/evidence contracts
- `gff/game_factory.py` — deterministic Game Factory dependency graph

## Game Factory

The Game Factory models Concept, World, Lore, Character, Creature, Combat, Quest, Level, Item, Weapon, Animation, AI NPC, Audio, VFX, Economy, Multiplayer, Build, Testing and LiveOps as typed production stages.

The graph fails closed on missing producers and dependency cycles and exposes a deterministic topological order.

## AI boundary

The workflow engine does not call an AI provider directly. Model providers, credentials and external side effects are supplied through injected executors. This preserves deterministic orchestration and keeps provider concerns outside the core.

## Engine integration

The subsystem is located under `franchise_factory/` rather than being made a Rust workspace member. This preserves the existing Python reference implementation while making the Franchise Factory part of the Genesis Engine repository and CI/documentation surface.

Production adapters to engine runtime systems, GCL/ATC-VM and external providers remain explicit integration boundaries and must be backed by implementation evidence before being marked production-ready.
