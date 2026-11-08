# ARCHITECTURE.md — atc-genesis-engine
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-genesis-engine/
├── README.md                 # Genesis engine overview & vision document
├── VISION_EVOLUTION_LOG.md   # Architectural log of vision changes
├── creature_spawner.atc      # Procedural creature spawning smart contract
├── engine.atc                # Genesis engine contract bindings
├── renderer.atc              # World renderer contract logic
├── terrain.atc               # Procedural terrain generation algorithm
└── engine/                   # Python ECS engine implementation
    ├── main.py               # Engine execution entry point
    ├── requirements.txt      # Python dependencies
    ├── core/
    │   └── ecs.py            # Entity Component System core implementation
    ├── render/
    │   └── renderer2d.py     # 2D viewport and world rendering pipeline
    └── tests/
        └── test_ecs.py       # Unit tests for ECS system
```

## Module Descriptions
- README.md — Architectural overview and vision statement for the Genesis Engine
- VISION_EVOLUTION_LOG.md — Evolutionary history of procedural generation specifications
- creature_spawner.atc — Smart contract controlling creature trait generation and spawning
- engine.atc — Smart contract interface for world state evolution
- renderer.atc — Visual rendering parameters contract
- terrain.atc — Heightmap and biome generation logic contract
- engine/main.py — Engine startup and game loop orchestration
- engine/core/ecs.py — High-performance Python Entity Component System implementation
- engine/render/renderer2d.py — 2D graphics rendering sub-system

## Build System
- Python setuptools / pip

## Dependencies
- Python 3.10+

## Status (Active/Migrated/Legacy)
Active (Python / Vision document)
