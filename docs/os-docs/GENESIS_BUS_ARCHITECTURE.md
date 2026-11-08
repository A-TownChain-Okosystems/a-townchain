# Genesis Communication Layer (GCL) v2.0

> **Standards:** AD-00 bis AD-14  
> **Datum:** 05.07.2026  
> **Autor:** Michael Wroblewski / Aurora  
> **Busse:** 13 + 1 Core Coordinator  

---

## Architektur-Übersicht

```
┌────────────────────────────────────────────────┐
│  Applications                                   │
│  ┌──────────────────────────────────────┐      │
│  │ Editor │ Runtime │ Launcher │ SDK    │      │
│  └──────────────────────────────────────┘      │
│                    │                            │
│             Genesis IPCBus                     │
│                    │                            │
═══════════════════════════════════════════════════
│          Genesis Core Bus (GCL Core)           │
═══════════════════════════════════════════════════
│                                                │
│  EventBus  CommandBus  QueryBus  MessageBus   │
│  AssetBus  RenderBus   PhysicsBus  AudioBus   │
│  InputBus  NetworkBus  PluginBus  AIBus       │
│  TelemetryBus                                   │
│                                                │
═══════════════════════════════════════════════════
│  Subsysteme                                     │
│  ECS │ Renderer │ Physics │ Audio │ Animation  │
│  UI │ Networking │ Asset Pipeline │ AI │ Script│
└────────────────────────────────────────────────┘
```

---

## Pipeline (Per Frame)

```
1. Input        → InputBus erfasst alle Eingaben
2. AI           → AIBus updatet NPCs, Pathfinding, Behavior Trees
3. Physics      → PhysicsBus simuliert Kollisionen, Constraints
4. GameLogic    → EventBus verarbeitet Gameplay-Events
5. Animation    → EventBus triggert Animation-Updates
6. Audio        → AudioBus updatet 3D-Positionen, Doppler
7. Render       → RenderBus enqueued Draw Calls, LOD, Shader
8. Telemetry    → TelemetryBus sampelt FPS, CPU, GPU, RAM
9. Present      → RenderBus präsentiert den Frame
```

---

## 13 Busse

| # | Bus | Standard | Zweck | Datei |
|---|-----|----------|-------|-------|
| 1 | EventBus | AD-01 | In-Process Events (Gameplay, UI, ECS) | ipc_bus_atc.ad.atc |
| 2 | CommandBus | AD-02 | Editor-Befehle, Undo/Redo, Macros | command_bus_ad02.atc |
| 3 | QueryBus | AD-07 | Read-Only Datenabfrage + Cache | query_bus_ad07.atc |
| 4 | MessageBus | AD-03 | Async Priority Queue + TTL + Dead Letter | message_bus_ad03.atc |
| 5 | AssetBus | AD-08 | Asset-Pipeline, Streaming, Hot Reload, Dedup | asset_bus_ad08.atc |
| 6 | RenderBus | AD-09 | Draw Calls, LOD, Framegraph, GPU Resources | render_bus_ad09.atc |
| 7 | PhysicsBus | AD-10 | Bodies, Collisions, Raycast, Constraints | physics_bus_ad10.atc |
| 8 | AudioBus | AD-11 | 3D Audio, Doppler, Reverb, Voice Chat | audio_bus_ad11.atc |
| 9 | InputBus | AD-12 | Multi-Device, Bindings, Dead Zone, Axes | input_bus_ad12.atc |
| 10 | NetworkBus | AD-05 | Multiplayer, State Sync, Delta, RPC, Snapshot | network_bus_ad05.atc |
| 11 | PluginBus | AD-06 | Sandbox, Hooks, API Registry, Hot Reload | plugin_bus_ad06.atc |
| 12 | AIBus | AD-13 | NPCs, Pathfinding, Behavior Trees, Dialog | ai_bus_ad13.atc |
| 13 | TelemetryBus | AD-14 | FPS, CPU/GPU, Crash Reports, Traces | telemetry_bus_ad14.atc |
| Core | GCL Core | AD-00 | 13-Bus Coordinator + Pipeline | gcl_core_ad00.atc |

---

## Vergleich

| Bus | Zweck | Speed | Speicher | Netzwerk | Sync |
|-----|-------|-------|----------|----------|------|
| EventBus | In-Process Events | <0.1ms | Shared | Nein | Sync |
| CommandBus | Editor Befehle | <0.1ms | Shared | Nein | Sync |
| QueryBus | Read-Only Abfrage | <0.1ms | Shared | Nein | Sync |
| MessageBus | Async Messages | Hoch | Shared | Nein | Async |
| AssetBus | Asset Pipeline | Hoch | Shared | Nein | Both |
| RenderBus | Renderer | Hoch | Shared | Nein | Sync |
| PhysicsBus | Physik | Hoch | Shared | Nein | Sync |
| AudioBus | Audio | Hoch | Shared | Nein | Sync |
| InputBus | Eingaben | <0.1ms | Shared | Nein | Sync |
| IPCBus | Cross-Process | Mittel | Getrennt | Optional | Async |
| NetworkBus | Multiplayer | Netzwerk | Getrennt | Ja | Async |
| PluginBus | Plugins | Hoch | Isoliert | Nein | Sync |
| AIBus | KI-Systeme | Hoch | Shared | Nein | Both |
| TelemetryBus | Diagnose | Sehr hoch | Shared | Optional | Async |

---

## Implementierung

Alle 13 Busse + GCL Core sind in ATCLang implementiert:

| Datei | Standard | Bytes |
|-------|----------|-------|
| gcl_core_ad00.atc | AD-00 | 7.862 |
| ipc_bus_atc.ad.atc | AD-01 | 7.718 |
| command_bus_ad02.atc | AD-02 | 4.542 |
| message_bus_ad03.atc | AD-03 | 6.633 |
| network_bus_ad05.atc | AD-05 | 8.948 |
| plugin_bus_ad06.atc | AD-06 | 8.393 |
| query_bus_ad07.atc | AD-07 | 3.411 |
| asset_bus_ad08.atc | AD-08 | 5.444 |
| render_bus_ad09.atc | AD-09 | 5.374 |
| physics_bus_ad10.atc | AD-10 | 7.637 |
| audio_bus_ad11.atc | AD-11 | 5.654 |
| input_bus_ad12.atc | AD-12 | 5.662 |
| ai_bus_ad13.atc | AD-13 | 9.160 |
| telemetry_bus_ad14.atc | AD-14 | 7.460 |
| **Total** | | **~93.898** |

---

*Genesis Communication Layer v2.0 — AD-00 bis AD-14 — 05.07.2026*
