---
spec_id: GEN-PROD-001
title: "Genesis Engine Product & Architecture Strategy (Engine/Game-Trennung)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Owner-Freigabe (§9) ausstehend
repository: genesis-engine
layer: L6-Engine
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-13
scr: TBD — SCR nach Owner-Freigabe
depends: [GEN-ECS-001, GEN-DET-001, GEN-TICK-001, GEN-WORLD-001]
---

# Genesis Engine Product & Architecture Strategy (GEN-PROD-001)

## 1. Produktdefinition (verbindlich nach Spec-Freeze)

> **ATC Genesis Engine** ist die general-purpose Game-Development-Plattform des
> A-TownChain-Ökosystems — unabhängig von jedem konkreten Spiel.

> **Genesis Chronicles** ist das Premium-Flagship-/Reference-Game auf dieser Engine
> — nicht ihr Eigentümer und nicht ihre Architektur-Instanz.

Produktformel:

> **Genesis Engine — Create Anything.**
> **Genesis Chronicles — Experience What's Possible.**

Namenskonvention: Produktname „Genesis Engine"; technische Bezeichnung innerhalb
des Ökosystems „ATC Genesis Engine" (Repo: `genesis-engine`). Der Name wird NICHT
umbenannt (Registry-Referenzen, AD-Nummern, Bauhierarchie L0–L7 bleiben unangetastet).

## 2. Rollenverteilung

| Produkt | Rolle |
|---|---|
| Genesis Engine | universelle Game-Engine (Plattform) |
| Genesis Runtime | Ausführungsschicht der Spiele |
| Genesis Editor | Entwicklungsumgebung (geplant) |
| Genesis SDK | Entwickler-SDK / stabile APIs (geplant) |
| Genesis Asset Pipeline | Import/Processing/Packaging (geplant) |
| Genesis Build System | Cross-Platform-Builds (geplant) |
| Genesis Server | Dedicated-/Backend-Runtime (geplant) |
| Genesis Tools | Profiler, Debugger, CLI (geplant) |
| Genesis Marketplace | Assets/Plugins/Tools (Langfrist) |
| Genesis Chronicles | Premium-Flagship-Game (L6, eigenes Repo `genesis-chronicles`) |

Status „geplant" = Zielarchitektur dieser Spec, noch nicht existierend. Bestehend
sind die vier Kernmodule (§5). Kein README-Claim ohne CI-Evidence (Evidence-First).

## 3. Die vier Architekturregeln

1. **Engine Independence** — Genesis Engine MUSS vollständig ohne
   Genesis Chronicles kompilierbar, testbar und nutzbar sein. CI-Gate: Workspace-Build
   ohne jede Referenz auf `genesis-chronicles`.
2. **Game Independence** — Der Engine-Core darf KEIN Genesis-Chronicles-spezifisches
   Gameplay enthalten. Spielspezifische Logik (Contracts, Battle-System, Breeding,
   NFT-Mint) lebt ausschließlich in `genesis-chronicles`.
3. **Feature Promotion** — Ein Feature darf aus einem Spiel (z. B. Chronicles-Anforderung)
   stammen, wird aber erst dann Engine-Feature, wenn es generisch, dokumentiert, getestet
   und über eine stabile API zugänglich ist (Feature-Promotion-Gate, §4).
4. **Flagship Pressure** — Genesis Chronicles soll die Engine regelmäßig an ihre
   technischen Grenzen bringen (Rendering, Streaming, AI, Physik, Networking, Simulation).

## 4. Feature-Promotion-Gate

```
Game-Anforderung (z. B. Genesis Chronicles)
        │
        v
Is it generally useful?  --NO-->  bleibt in der Game-Layer (genesis-chronicles)
        | YES
        v
Engine-RFC + Architecture Review
        v
Generic API (Spec: GEN-*-001)
        v
Implementierung im Engine-Modul
        v
Unit-/Engine-Tests + Chronicles-Integrationstest
        v
Production Validation + Performance Gate
        v
Stable API -> Genesis SDK
```

Kein Feature-Skip: keine Engine-Commits mit Spiel-Logik (Review-Kriterium;
CI-Evidence via Engine-Independence-Build).

## 5. Ziel-Layermodell (abgestuft auf den Ist-Zustand)

| Layer | Zweck | Ist-Stand |
|---|---|---|
| Foundation | Memory, Threading, Jobs, Platform-Abstraktion | Teil von `atc-genesis-engine` |
| Core | ECS, Runtime, Resources, Tick-Contract | `atc-genesis-engine`, `atc-genesis-ecs` (GEN-ECS-001, GEN-TICK-001) |
| Simulation | Physics, Animation, AI, Creatures | `atc-genesis-creatures` (GEN-AI-001; generisch: Intent/Policy, keine Chronicles-Logik) |
| World | Scenes, Streaming, Prozedural, World Partition | `atc-genesis-world` (GEN-WORLD-001) |
| Graphics | Renderer, GPU, Materials, Shaders, VFX | geplant |
| Gameplay | Input, Events, Scripting-Framework | geplant (ATCLang-Anbindung, §6) |
| Network | Replication, Server, Multiplayer | geplant |
| Editor / SDK / Tools / Asset Pipeline | Entwicklungsumgebung, APIs, Toolchain | geplant |

Determinismus-Pflicht für alle Simulations-Layer: GEN-DET-001 (Intent-Policy-Trennung,
sortierte Iteration, kein `random/clock/races` gemäß REQ-ENG-002).

## 6. ATCLang als Gameplay-Scripting-Layer (Ziel)

```
Game Developer -> ATCLang -> ATC-VM -> Genesis Runtime -> Rust Core -> Hardware/OS
```

- Gameplay-/Scripting-Logik läuft kontrolliert in der ATC-VM (chain-fähig, deterministisch).
- Rust bleibt der native Engine-/System-Core (Sprachgrenze AD-008).
- Engine-internes Test-Beispiel: `battle_system.atc` / `breeding.atc` in
  `genesis-chronicles` — Game-Logik, NICHT Engine-Kern.

## 7. Verhältnis zu anderen Ökosystem-Repos

- `genesis-engine` (L6-Engine) liefert die Plattform; M7 „Spiel läuft" (AD-027) bleibt Meilenstein der Roadmap.
- `genesis-chronicles` (L6 GameFi, AD-025) ist das Reference Game und nutzt Engine-Releases über stabile APIs.
- Integration/Orchestrierung über Monorepo `a-townchain-os` (L7) bleibt unverändert.
- Engine entwickelt Technologie -> Chronicles zeigt sie (Technology Feedback Loop):
  Chronicles-Anforderung -> Engine-Feature -> Production-Testing -> Engine-Improvement.

## 8. Konformanz

Diese Spec ist SPEC-DRAFT: normativ erst nach Spec-Freeze und SCR. Die vier
Architekturregeln (§3) werden nach Freigabe als Review-Kriterien für alle
`genesis-engine`-PRs herangezogen (CODEOWNERS/AGENTS.md-Ergänzung folgt mit SCR).
