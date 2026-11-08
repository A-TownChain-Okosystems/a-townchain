---
title: Genesis Engine Capability Audit
summary: Evidence-basierte Reifegrad-Messung der Genesis-Plattform gegen die Unity/Unreal-Komponentenmatrix; Status je P0/P1/P2-Subsystem mit Ableitung der Implementierungs-Roadmap und Repo-Promotion-Empfehlung.
category: REFERENZ
version: 0.1.0
status: DRAFT
date: 2026-09-15
source: Live-Inventar genesis-engine @ main (c8a1dcc) + Org-Repo-Survey 15.09.2026
---

# Genesis Engine Capability Audit (REFERENZ)

## 1. Zweck und Methode

Dieser Audit misst die tatsächliche Genesis-Plattform gegen die vollständige
Unity/Unreal-Komponentenmatrix (Owner-Zielbild 15.09.2026). Er folgt dem
Evidence-First-Grundsatz der Organisation: **kein Status ohne Code-Evidenz** —
README-Claims und CI-Evidence werden getrennt ausgewiesen.

Status-Skala:

| Status | Bedeutung |
|---|---|
| PRODUCTION | In Produktion einsetzbar, getestet, dokumentiert, evidenced |
| PARTIAL | Echte Teil-Implementierung mit Tests, aber ohne Produktionsreife |
| STUB | Interface/Contract vorhanden, keine funktionale Implementierung |
| MISSING | Nicht vorhanden |

Evidenzbasis: Live-Baum von `genesis-engine@main` (Commit c8a1dcc, 15.09.2026,
156 Dateien), inkl. der am 15.09. direkt gepushten Bootstrap-Module
(atc-genesis-platform/-renderer/-physics/-audio/-animation/-assets/-ui,
jeweils 18–87 Zeilen Rust) sowie der Alte-Module (engine, ecs, creatures, world).

## 2. P0-Matrix — zwingend für eine echte Game Engine

| # | Subsystem | Status | Evidenz | Gap |
|---|---|---|---|---|
| 1 | Genesis Runtime | PARTIAL | `engine/main.py` (49 Z.), Python-Loop | Produktionsreife, fixed timestep, frame budget |
| 2 | ECS / World Runtime | PARTIAL | `engine/core/ecs.py` (99 Z., 11 defs, `tests/test_ecs.py` grün) | Query-Systeme, Archetypes, Jobs/Scheduling |
| 3 | 3D Renderer | **STUB** | `modules/atc-genesis-renderer/src/lib.rs` (24 Z. Contract) | Vulkan/DX12/Metal-Backend, Render Graph, PBR, Shadows, GI — alles fehlt |
| 4 | 2D Renderer | PARTIAL | `engine/render/renderer2d.py` (46 Z., pygame-basiert) | GPU-Backend, Sprites-Batching, UI-Compositing |
| 5 | Asset Manager | **STUB** | `atc-genesis-assets/src/lib.rs` (27 Z., in-memory Registry) | Persistente UUIDs, Dependencies, Caching, Hot Reload |
| 6 | Asset Pipeline (Importer) | **MISSING** | — | glTF/FBX/OBJ/PNG/JPEG/WAV-Import, Konvertierung, Cook |
| 7 | Scene System | **MISSING** | — | Szenen, Prefabs, Instancing, Serialisierung |
| 8 | Physics (3D/2D) | **STUB** | `atc-genesis-physics/src/lib.rs` (18 Z., Null-Impl) | Rigid Bodies, Solver, Joints, Determinismus |
| 9 | Collision System | **MISSING** | — | Broad/Narrow Phase, Raycasts, Triggers |
| 10 | Animation | **STUB** | `atc-genesis-animation/src/lib.rs` (28 Z., Clip-Primitives) | Skeletal, Blend Trees, State Machines, Retargeting |
| 11 | Audio Engine | **STUB** | `atc-genesis-audio/src/lib.rs` (18 Z., NullAudioRuntime) | Spatial Audio, Mixer, Streaming, Formate |
| 12 | Input System | **MISSING** | — | Keyboard/Mouse/Gamepad/Touch, Mapping, Rebinds |
| 13 | Scripting/API | PARTIAL | Python-native Gameplay-API; ATCLang-Bindung als Vertrag im Org-Kontext | ATCLang-Integration in die Runtime, Sandbox |
| 14 | UI Framework | **STUB** | `atc-genesis-ui/src/lib.rs` (24 Z., Primitives) | Runtime-UI + Editor-UI, Layout, Text, Nine-Slice |
| 15 | Save/Load | **MISSING** | — | Versionierte Serialisierung, Migration |
| 16 | Resource-/Dependency-Graph | **MISSING** | — | Hot Reload, Asset-Dependencies, Streaming |

**P0-Zwischensumme: 0 PRODUCTION · 4 PARTIAL · 5 STUB · 7 MISSING**

## 3. P0-Block Genesis Editor (das entscheidende fehlende Produkt)

Von den 20 Editor-Subsystemen (Scene View, Game View, Hierarchy, Inspector,
Project/Asset Browser, Entity/Component Editor, Material/Shader/Animation/
Particle/Terrain/Prefab/UI/Audio/World Editor, Profiler, Debugger, Console,
Build/Packaging Manager) sind **20 MISSING** — Genesis kann Spiele ausführen
(vorerst nur simple 2D), aber nicht wie Unity/Unreal *entwickeln*.

Wichtig: **atc-ide (Lumino)** ist ein TypeScript-basiertes IDE-Framework mit
ATC-VM-Simulator-, Genesis-Configurator- und GlobusOS-Panels und damit der
natürliche Basis-Kandidat für Genesis Studio — nicht für die Runtime, sondern
als Editor-Shell. Der Sprung von IDE zu Editor ist erheblich (Viewport,
Asset-Browser, Inspectoren, Tooling-Protokoll zur Runtime), aber die
Null-Basis-Alternative wäre teurer.

## 4. P1/P2 — ohne separates Assessment bleibend ehrlich

| Cluster | Status |
|---|---|
| Renderer-Backends (Vulkan/DX12/Metal), Render Graph, PBR/HDR/GI/SSR/SSAO, Partikel, LOD | MISSING (Contract STUB) |
| World Building (Terrain, Foliage, Water, Weather, Streaming, World Partition, Prozedural) | STUB (atc-genesis-world Verträge) |
| Character (Skeleton, IK, Nav, Crowd) | STUB (animation-Verträge) |
| Dev-Tools (build, cli, profiler, debugger, test, benchmark, importer, packager) | MISSING |
| Multiplayer/Replication, AI-Framework, GAS, Sequencer, VFX, Modding, Marketplace, L10n, Accessibility, Telemetry | MISSING (P2) |

## 5. Top-3-Gaps (bestätigt das Owner-Zielbild)

1. **Genesis Editor** — 0/20 Subsysteme; ohne ihn keine Produktion auf Unity/Unreal-Niveau.
2. **Asset-/Content-Pipeline** — kein Importer, keine persistente Asset-Verwaltung; blockiert Editor, World und jeden Content-Workflow.
3. **Production-Renderer + World/Physics/Animation-Stack** — heute Null-Verträge; der größte Einzelanteil an Engineering-Kapazität.

## 6. Roadmap-Empfehlung (Wellen, je Welle evidenced)

- **W0 (jetzt):** CI grün halten. Determinism-Gate-Altfehler behoben
  (Selbst-Flagging des Checkers, dieser PR). Direkt-Pushes auf main stoppen —
  PR-Flow ist Pflicht; Branch-Protection bleibt P0-Owner-Aktion (27/30 Repos offen).
- **W1:** Subsystem-Contracts freezen (`atc-genesis-platform`, 87 Z.) + ECS/World
  auf Produktion heben (Archetypes/Queries), Scene-System + Save/Load (vernetzt),
  Asset-Registry persistente UUIDs + Dependency-Graph.
- **W2:** Genesis Studio MVP auf atc-ide-Basis (Scene View, Hierarchy, Inspector,
  Asset Browser) — vor dem 3D-Renderer, weil jeder spätere Renderer sofort einen
  Editor braucht, nie umgekehrt.
- **W3:** Renderer-Backend 1 (Vulkan) + Render Graph; 2D-GPU; Physics-Solver 2D→3D;
  Skeletal Animation; Audio-Backend (miniaudio/cpal-Kandidaten).
- **W4+:** P1-Toolchain (genesis-cli/build/…), dann P2.

## 7. Repo-Promotion (`docs/genesis-repositories.yaml`)

Die gepushte Promotion-Matrix (17 Ziel-Repos, contract-first) ist als
**Zielbild brauchbar**, aber: **16 leere Repos jetzt anzulegen wäre vorzeitig.**
Empfehlung: Promotion je Subsystem erst bei Implementierungsstart über einen SCR
(registry-pflichtig, ATC-REPO-*-IDs, repositories.yaml-Registrierung).
Sofort ausgenommen: **genesis-editor** (W2-Start) — dieses Repo früh anlegen,
sobald der SCR genehmigt ist. Monorepo bis dahin: ja — die Vertragsgrenzen sind
maschinenlesbar definiert.

## 8. Governance-Befunde des 15.09.-Pushes

- Direkt-Push-Serie (≥10 Commits) auf main **ohne PR und Review** — möglich, weil
  Branch-Protection auf genesis-engine AUS ist (P0-Owner-Aktion). Der
  Access-Standard ATC-AI-GOV-ACCESS-001 sieht Stufe-2-Zugriff als PR-Flow vor.
- Formale Qualität der Commits: konventionelle Nachrichten, FILE_REGISTER gepflegt,
  Promotion-Manifest maschinenlesbar — inhaltlich sauber.
- `determinism=red` auf main war ein **Altfehler** (Checker flaggte seine eigenen
  Pattern-Literale; SKIP_DIRS enthielt einen Dateipfad statt eines Verzeichnisses)
  — nicht vom 15.09.-Push verursacht, in diesem PR behoben.
- pytest-Race beim Evidence-Push (b66d892): transient, Test-Suite auf main-Spitze
  grün (4/4 lokal reproduziert).
- Grundsatz bestätigt: README/Chat-Claims ("echte Rust-Module gebootstrapped")
  sind gegen Code-Evidence zu prüfen — die Module sind 18–87 Zeilen Verträge.
  Status STUB, nicht Implementierung. So steht es ab jetzt auch hier schwarz auf weiß.

## 9. KPI-Schnappschuss

- P0-Subsysteme (16): **0 PRODUCTION / 4 PARTIAL / 5 STUB / 7 MISSING**
- Editor-Subsysteme (20): **0 / 0 / 0 / 20**
- Gesamtziel "Unity/Unreal-Klasse": aktuell **~12 % des P0-Raums** auf
  STUB-Level oder besser, 0 % produktionsreif. Der Weg dorthin führt über
  W1–W4, nicht über weitere Verträge.

*Nächste formale Schritte: SCR für W1 (ECS/Scene/Assets) + SCR für
genesis-editor-Anlage + Owner-Entscheidung Branch-Protection.*
