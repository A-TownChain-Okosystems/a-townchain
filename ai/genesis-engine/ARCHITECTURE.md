---
document_id: ATC-DOC-ENGINE-ARCH-001
title: Architecture — genesis-engine
version: 1.1.0
status: active
standard: ATC-STD-MD-001
date: 2026-09-26
---

# Architecture — genesis-engine

> **Technische Architektur der ATC Genesis Engine (Layer L6)**

## Overview

Genesis Engine ist eine modulare, KI-native Game-Engine für das A-TownChain-Ökosystem. Sie ist auf Layer L6 der Bauhierarchie angesiedelt und bietet performante Abstraktionen für ECS, Kreaturen-Verhalten, Umwelt-Simulation, Gameplay, AI und Produktions-Workflows.

## Modul-Architektur

Das Repository gliedert sich unter `modules/` in vier Kerndomänen:

```
modules/
├── atc-genesis-engine/       # Core Engine Loop, Event Bus, System Scheduler
├── atc-genesis-ecs/          # High-Performance ECS Architecture
├── atc-genesis-creatures/    # Behavioral AI Trees & Creature Mechanics
└── atc-genesis-world/        # Procedural World Generation & Environment Simulation
```

Quest AI ist eine übergreifende Gameplay-/AI-Funktion und nutzt diese Runtime-Domänen, ohne den deterministischen Engine-Kern zu umgehen.

## Quest AI

Quest AI ist die kanonische Quest-Intelligence-Schicht der Genesis-Plattform. Sie wird durch die Master Architecture des `a-townchain-ecosystem` definiert und hier auf die Genesis-Engine-Runtime abgebildet.

### Verantwortungsbereiche

- Quest Generation
- Quest Planning
- Quest Director
- Quest Validation
- Quest Runtime
- Objective System
- Dynamic Difficulty
- Player-specific Quest Recommendation
- NPC-/Faction-Quest Integration
- Reward Validation
- Quest Memory / State Integration
- Anti-Exploit und Quest-Integrity

### Runtime Pipeline

```
World / Player Event
        ↓
World State
        ↓
Aurora AI Context
        ↓
Quest AI
  ├─ Generate
  ├─ Plan
  ├─ Validate
  └─ Direct
        ↓
Validated Quest Command
        ↓
Genesis Engine Runtime
        ↓
Deterministic Simulation
        ↓
World / Player State
        ↓
Rewards / Reputation / Consequences
```

### Quest State Machine

```
CREATED → AVAILABLE → ACCEPTED → ACTIVE
                         ↓
                OBJECTIVE_PROGRESS
                         ↓
                OBJECTIVE_COMPLETED
                         ↓
                    TURN_IN
                         ↓
                  REWARD_PENDING
                         ↓
                    COMPLETED

ACTIVE → FAILED
ACTIVE → ABANDONED
AVAILABLE → EXPIRED
```

### Quest Contract

Eine Quest-Definition enthält mindestens:

```
id
version
type
category
title
description
prerequisites
objectives[]
npc_refs[]
faction_refs[]
location_refs[]
enemy_refs[]
difficulty
level_range
time_limit
rewards
reputation
consequences
branches[]
dependencies[]
state
provenance
security_metadata
```

Die Runtime muss generierte Questdaten vor Aktivierung gegen Schema, World State, Lore-/Canon-Regeln, Economy-Regeln und Security Policies validieren.

## AI Boundary

Externe oder asynchrone AI darf den deterministischen Simulationszustand nicht direkt verändern:

```
AI Inference
    ↓
Quest Proposal / Validated Command
    ↓
Deterministic Simulation
    ↓
State Change
```

Quest AI darf insbesondere keine ECS-State-Mutation, Reward-Ausgabe oder Chain-/VM-State-Änderung ausschließlich aufgrund einer Modellantwort durchführen.

## Schichtenmodell & Integration

1. **Base Layer (L0-L5):** Nutzung von ATCLang, ShivaCore, Aurora AI und A-TownChain.
2. **Engine Layer (L6):** In-Memory Tick Pipeline, ECS und deterministische State Computations.
3. **Game Intelligence:** Quest AI, Creature AI, World Simulation, NPC-/Faction-Systeme und LiveOps.
4. **Application & GameFi (L6-L7):** Anbindung an GameFi-Titel wie `genesis-chronicles` und Monorepo-Integration `a-townchain-os`.

## Security & Trust Boundaries

- Die Engine läuft außerhalb des verifizierten Konsens-Kernels (S1 Security Class).
- Konsens- und State-Relevanz werden über Verifizierer an die ATVM (`atc-vm`) übergeben.
- Quest Rewards müssen vor der Ausgabe durch die Runtime validiert werden.
- On-chain Assets dürfen ausschließlich über die vorgesehenen Chain-/VM-Schnittstellen ausgegeben werden.
- AI-generierte Inhalte werden als untrusted input behandelt, bis Schema-, Policy-, Security- und Runtime-Validierung erfolgreich abgeschlossen sind.
