# Genesis Communication Layer (GCL) v2.0 — Technische Dokumentation

> **Dokument-Typ:** Technische Architektur-Dokumentation  
> **Standard:** AD-00 bis AD-14  
> **Version:** 2.0.0  
> **Datum:** 05.07.2026  
> **Autor:** Michael Wroblewski / Aurora  
> **Status:** STABLE  

---

## 1. Einleitung

Die Genesis Communication Layer (GCL) ist der zentrale Kommunikationskern der Genesis Engine. Sie vereinheitlicht alle Kommunikationssysteme — von In-Process-Events über Cross-Process-Messaging bis hin zu Multiplayer-Netzwerk — in einer modularen, skalierbaren Architektur.

### 1.1 Design-Prinzipien

- **Separation of Concerns:** Jeder Bus hat einen klar definierten Zweck
- **Loose Coupling:** Module kommunizieren über Busse, nicht direkt
- **Skalierbarkeit:** Von kleinen Tools bis zu AAA-Projekten
- **Performance:** Synchron wo möglich, asynchron wo nötig
- **Erweiterbarkeit:** Neue Busse können ohne Breaking Changes hinzugefügt werden

### 1.2 Abgrenzung

| Eigenschaft | EventBus | IPCBus |
|-------------|----------|--------|
| Zweck | Innerhalb eines Prozesses | Zwischen Prozessen |
| Geschwindigkeit | Sehr hoch (<0.1ms) | Niedriger |
| Speicher | Gemeinsam | Getrennt |
| Einsatz | Gameplay, UI, Engine-Systeme | Editor↔Engine, Tools |
| Netzwerk | Nein | Optional |

---

## 2. Architektur-Übersicht

```
┌──────────────────────────────────────────────────────┐
│  Applications                                         │
│  ┌────────────────────────────────────────────┐      │
│  │ Editor │ Runtime │ Launcher │ SDK          │      │
│  └────────────────────────────────────────────┘      │
│                       │                               │
│                Genesis IPCBus                        │
│                       │                               │
══════════════════════════════════════════════════════════
│           Genesis Core Bus (GCL Core)                │
══════════════════════════════════════════════════════════
│                                                      │
│  EventBus   CommandBus   QueryBus    MessageBus     │
│  AssetBus   RenderBus    PhysicsBus  AudioBus       │
│  InputBus   NetworkBus   PluginBus   AIBus          │
│  TelemetryBus                                         │
│                                                      │
══════════════════════════════════════════════════════════
│  Subsysteme                                           │
│  ECS │ Renderer │ Physics │ Audio │ Animation        │
│  UI │ Networking │ Asset Pipeline │ AI │ Scripting   │
└──────────────────────────────────────────────────────┘
```

---

## 3. GCL Core Coordinator (AD-00)

Der GCL Core ist der zentrale Koordinator, der alle 13 Busse verwaltet und die Frame-Pipeline steuert.

### 3.1 Pipeline (9 Stages pro Frame)

| Stage | Bus | Beschreibung | Target (ms) |
|-------|-----|-------------|-------------|
| 1. Input | InputBus | Alle Eingaben erfassen | 1.0 |
| 2. AI | AIBus | NPCs, Pathfinding, Behavior Trees | 5.0 |
| 3. Physics | PhysicsBus | Kollisionen, Constraints, Raycast | 8.0 |
| 4. GameLogic | EventBus | Gameplay-Events verarbeiten | 3.0 |
| 5. Animation | EventBus | Animation-Updates | 2.0 |
| 6. Audio | AudioBus | 3D-Positionen, Doppler, Reverb | 1.0 |
| 7. Render | RenderBus | Draw Calls, LOD, Shader, Framegraph | 10.0 |
| 8. Telemetry | TelemetryBus | FPS, CPU, GPU, RAM sampeln | 0.5 |
| 9. Present | RenderBus | Frame präsentieren | 1.0 |

### 3.2 API

```
init_gcl()           → GCLCore           // Initialisieren
start_gcl(core)      → Bool              // Start
gcl_frame(core, dt)  → Bool              // Ein Frame ausführen
stop_gcl(core)       → Bool              // Stop
shutdown_gcl(core)   → Bool              // Komplett herunterfahren
get_bus(core, name)  → T                 // Bus-Referenz abrufen
set_stage_enabled(core, name, enabled)   // Pipeline-Stage aktivieren/deaktivieren
pipeline_stats(core) → List<PipelineStat> // Performance-Statistiken
```

### 3.3 Datei
- `modules/kernel/gcl_core_ad00.atc` — 7.862 bytes

---

## 4. Bus-Referenz

### 4.1 EventBus (AD-01)

**Zweck:** Verteilt Ereignisse innerhalb desselben Prozesses.

**Einsatzbereiche:** Spieler springt, Inventar geändert, Asset geladen, NPC gestorben, Shader kompiliert.

**Features:**
- Subscribe/Publish/Unsubscribe
- Priority-Queue (Handler nach Priorität sortiert)
- Event-History mit konfigurierbarem Max-Size
- Once-Handler (auto-unsubscribe nach erstem Aufruf)

**Geschwindigkeit:** <0.1ms (synchron, Shared Memory)

**Datei:** `modules/kernel/ipc_bus_atc.ad.atc` (EventBus Teil)

---

### 4.2 CommandBus (AD-02)

**Zweck:** Verarbeitet Aktionen, die den Systemzustand verändern.

**Einsatzbereiche:** Asset importieren, Objekt erzeugen, Szene speichern, Spiel starten, Build ausführen.

**Features:**
- Execute/Undo/Redo-Stack
- Macro-Aufnahme (mehrere Commands als einer)
- Keyboard-Shortcuts mit Command-Registry
- History-Limit (konfigurierbar)
- Transaktionen und Berechtigungen

**Eigenschaften:** Synchron oder asynchron, Undo/Redo-fähig, mit Prioritäten

**Datei:** `modules/kernel/command_bus_ad02.atc` — 4.542 bytes

---

### 4.3 QueryBus (AD-07)

**Zweck:** Read-Only-Datenabfrage ohne Seiteneffekte.

**Einsatzbereiche:** Alle Entities abfragen, Asset-Metadaten lesen, GPU-Auslastung, Speicherverbrauch, Komponentenliste.

**Features:**
- Handler-Registry für Query-Typen
- Cache mit TTL und auto-Eviction
- Cache-Hit-Rate-Tracking
- Keine Seiteneffekte (pure reads)

**Vorteile:** Gute Cachebarkeit, einfache Optimierung, keine State-Mutation

**Datei:** `modules/kernel/query_bus_ad07.atc` — 3.411 bytes

---

### 4.4 MessageBus (AD-03)

**Zweck:** Verteilt asynchrone Nachrichten zwischen Modulen.

**Einsatzbereiche:** "Shader erfolgreich kompiliert", "Build abgeschlossen", "Cloud-Synchronisation beendet".

**Features:**
- Priority-Queue (Low → Normal → High → Critical)
- TTL (Time-to-Live) für Messages
- Retry mit max_retries
- Dead-Letter-Queue für fehlgeschlagene Messages
- Tag-Filtering für Subscriber
- Batch-Processing

**Eigenschaften:** Asynchron, verzögerte Verarbeitung, Broadcast oder gezielte Zustellung

**Datei:** `modules/kernel/message_bus_ad03.atc` — 6.633 bytes

---

### 4.5 AssetBus (AD-08)

**Zweck:** Speziell für die Asset-Pipeline.

**Funktionen:**
- Asset laden (synchron und asynchron)
- Streaming-Queue mit max_concurrent_loads
- Hot Reload (automatische Versionserhöhung)
- Asset-Versionierung
- Dependency-Auflösung (rekursiv)
- Deduplizierung (gleiche Assets werden nur einmal geladen)
- Memory Budget-Prüfung

**Asset-Typen:** Texture, Mesh, Shader, Audio, Font, Material, Animation, Scene, Script, Prefab, Config

**Datei:** `modules/kernel/asset_bus_ad08.atc` — 5.444 bytes

---

### 4.6 RenderBus (AD-09)

**Zweck:** Kommunikation zwischen Engine und Renderer.

**Einsatzbereiche:** Render-Queue aktualisieren, Shader wechseln, LOD aktualisieren, GPU-Ressourcen erzeugen, Framegraph aktualisieren.

**Render-Commands:**
- DrawCall, ShaderBind, TextureBind, MeshBind
- LODUpdate, ViewportSet, CameraUpdate, LightUpdate
- FramegraphRebuild, GPUResourceCreate/Destroy
- ClearTarget, Present, Blit

**LOD-Strategien:** DistanceBased, ScreenSizeBased, Fixed, Adaptive

**Datei:** `modules/kernel/render_bus_ad09.atc` — 5.374 bytes

---

### 4.7 PhysicsBus (AD-10)

**Zweck:** Steuert die Physiksysteme.

**Events:** Kollision, Trigger, Ragdoll, Raycast, Constraints, Zerstörung.

**Body-Typen:** Dynamic, Static, Kinematic

**Collider-Shapes:** Box, Sphere, Capsule, Mesh, ConvexHull, Plane

**Constraints:** Hinge, Spring, Distance, Point, Slider, ConeTwist, Ragdoll

**Broadphase:** AABB, SweepPrune, BVH, Grid

**Features:**
- Kraft/Impuls anwenden
- Raycast mit Ergebnis
- Simulation Step mit Substeps
- Trigger-Modus (keine physische Reaktion)
- Layer-basierte Kollision-Filterung

**Datei:** `modules/kernel/physics_bus_ad10.atc` — 7.637 bytes

---

### 4.8 AudioBus (AD-11)

**Zweck:** Verwaltet Audioereignisse.

**Einsatzbereiche:** Musik starten, 3D-Sound aktualisieren, Lautstärke ändern, Hall-Effekte, Voice-Chat.

**Source-Typen:** SFX, Music, Voice, Ambient, UI, VoiceChat

**Features:**
- 3D-Spatial Audio mit Position/Velocity
- Doppler-Effekt (geschwindigkeitsbasiert)
- Reverb-Presets (None, Room, Hall, Cavern, Outdoor, Studio)
- Channel-basierte Lautstärkeregulierung
- Master-Volume
- Voice-Chat-Integration
- Max-Voices-Limit mit auto-Eviction
- Listener-System (mehrere Zuhörer)

**Datei:** `modules/kernel/audio_bus_ad11.atc` — 5.654 bytes

---

### 4.9 InputBus (AD-12)

**Zweck:** Zentrale Verarbeitung aller Eingaben.

**Unterstützte Geräte:**
- Tastatur
- Maus
- Touch
- Controller
- VR-Controller
- Gesten
- Gyroskop

**Features:**
- Action-Bindings (mehrere Bindings pro Action)
- Modifiers (Ctrl, Shift, Alt)
- Axis-Unterstützung mit Dead Zone
- Sensitivity pro Binding
- State-Tracking: is_pressed, was_just_pressed, was_just_released
- Event-Konsumierung (für UI-Layer)
- Frame-basiertes Reset

**Datei:** `modules/kernel/input_bus_ad12.atc` — 5.662 bytes

---

### 4.10 NetworkBus (AD-05)

**Zweck:** Multiplayer und Online-Funktionen.

**Aufgaben:** Replikation, Serverautorität, RPCs, Synchronisation, Matchmaking, Voice-Daten, Latenzkompensation.

**Message-Typen:** StateSync, RPC, Event, Join, Leave, Snapshot, Delta, Heartbeat, Disconnect

**Features:**
- State-Sync (reliable, ordered)
- Delta-Compression (nur Änderungen)
- RPC (unreliable, fast)
- Periodische Snapshots
- Payload-Fragmentierung für große Daten
- Heartbeat mit ACK/Retry
- Interpolation-Buffer (Client-Side)
- Dead-Peer-Cleanup

**Datei:** `modules/kernel/network_bus_ad05.atc` — 8.948 bytes

---

### 4.11 PluginBus (AD-06)

**Zweck:** Ermöglicht die Erweiterung der Engine.

**Plugins können:** Neue Dateiformate registrieren, Renderer erweitern, Werkzeuge hinzufügen, Gameplay-Systeme integrieren, Editor-Fenster bereitstellen.

**Features:**
- Plugin-Lifecycle: Load → Activate → Deactivate → Unload
- Hook-System (on_load, on_unload, on_frame_end, custom hooks)
- Cross-Plugin API Registry
- Permission-System (FileSystem, Network, Rendering, Audio, Input, Scene, Memory, Compute, Editor, All)
- Sandbox-Modus
- Hot-Reload (für Development)

**Datei:** `modules/kernel/plugin_bus_ad06.atc` — 8.393 bytes

---

### 4.12 AIBus (AD-13)

**Zweck:** Kommunikation mit KI-Komponenten.

**Einsatzbereiche:** NPC-Verhalten, Pathfinding, Dialogsysteme, Agentenkoordination, KI-gestützte Asset-Erstellung, Gameplay-Unterstützung.

**Agent-Typen:** NPC, Enemy, Companion, Animal, Boss, Swarm, Crowd

**Features:**
- A* Pathfinding auf NavMesh
- Behavior Trees (Root, Sequence, Selector, Action, Condition, Decorator)
- Dialog-System mit Choices und Conditions
- LOD-AI (ferne Agents seltener updaten)
- Async Pathfinding-Queue
- Agent-Memory pro Agent
- Perception-Range

**Datei:** `modules/kernel/ai_bus_ad13.atc` — 9.160 bytes

---

### 4.13 TelemetryBus (AD-14)

**Zweck:** Sammelt Laufzeit- und Diagnosedaten.

**Erfasst:** FPS, CPU-/GPU-Auslastung, RAM-/VRAM-Nutzung, Asset-Ladezeiten, Netzwerkstatistiken, Absturzberichte, Performance-Traces.

**Features:**
- Sampling mit konfigurierbarem Intervall
- Threshold-Alerts (Info → Warning → Error → Critical)
- Crash-Reports mit Stack-Trace, OS-Info, GPU-Info
- Auto-Report-Funktion für Crashes
- Performance-Traces (begin_trace / end_trace)
- Durchschnittswerte-Berechnung
- Alert-Management (raise / resolve)

**Datei:** `modules/kernel/telemetry_bus_ad14.atc` — 7.460 bytes

---

## 5. Vergleichstabelle

| Bus | Zweck | Speed | Speicher | Netzwerk | Sync | Standard |
|-----|-------|-------|----------|----------|------|----------|
| EventBus | In-Process Events | <0.1ms | Shared | Nein | Sync | AD-01 |
| CommandBus | Editor Befehle | <0.1ms | Shared | Nein | Sync | AD-02 |
| QueryBus | Read-Only Abfrage | <0.1ms | Shared | Nein | Sync | AD-07 |
| MessageBus | Async Messages | Hoch | Shared | Nein | Async | AD-03 |
| AssetBus | Asset Pipeline | Hoch | Shared | Nein | Both | AD-08 |
| RenderBus | Renderer | Hoch | Shared | Nein | Sync | AD-09 |
| PhysicsBus | Physik | Hoch | Shared | Nein | Sync | AD-10 |
| AudioBus | Audio | Hoch | Shared | Nein | Sync | AD-11 |
| InputBus | Eingaben | <0.1ms | Shared | Nein | Sync | AD-12 |
| IPCBus | Cross-Process | Mittel | Getrennt | Optional | Async | AD-01 |
| NetworkBus | Multiplayer | Netzwerk | Getrennt | Ja | Async | AD-05 |
| PluginBus | Plugins | Hoch | Isoliert | Nein | Sync | AD-06 |
| AIBus | KI-Systeme | Hoch | Shared | Nein | Both | AD-13 |
| TelemetryBus | Diagnose | Sehr hoch | Shared | Optional | Async | AD-14 |

---

## 6. Datei-Übersicht

| Datei | Standard | Bytes |
|-------|----------|-------|
| `modules/kernel/gcl_core_ad00.atc` | AD-00 | 7.862 |
| `modules/kernel/ipc_bus_atc.ad.atc` | AD-01 | 7.718 |
| `modules/kernel/command_bus_ad02.atc` | AD-02 | 4.542 |
| `modules/kernel/message_bus_ad03.atc` | AD-03 | 6.633 |
| `modules/kernel/network_bus_ad05.atc` | AD-05 | 8.948 |
| `modules/kernel/plugin_bus_ad06.atc` | AD-06 | 8.393 |
| `modules/kernel/query_bus_ad07.atc` | AD-07 | 3.411 |
| `modules/kernel/asset_bus_ad08.atc` | AD-08 | 5.444 |
| `modules/kernel/render_bus_ad09.atc` | AD-09 | 5.374 |
| `modules/kernel/physics_bus_ad10.atc` | AD-10 | 7.637 |
| `modules/kernel/audio_bus_ad11.atc` | AD-11 | 5.654 |
| `modules/kernel/input_bus_ad12.atc` | AD-12 | 5.662 |
| `modules/kernel/ai_bus_ad13.atc` | AD-13 | 9.160 |
| `modules/kernel/telemetry_bus_ad14.atc` | AD-14 | 7.460 |
| **Total** | **15 Standards** | **~93.898** |

---

## 7. Typische Technologien (Referenz)

| Bus | Mögliche Technologien |
|-----|----------------------|
| EventBus | In-Process Function Calls |
| IPCBus | Shared Memory, Named Pipes, Unix Domain Sockets, TCP, gRPC, ZeroMQ |
| NetworkBus | UDP (State Sync), TCP (Reliable), WebSocket |
| PluginBus | Dynamic Loading (dlopen), WebAssembly, Lua |

---

## 8. Querverweise

- [ATCLang Spec v1.0](atclang/ATCLANG_SPEC.md) — ATC-92
- [ATC-93 Bytecode Spec](docs/standards/ATC-93-ATCLANG_VM_BYTECODE.md) — VM Op-Codes
- [Sprint 2.3+2.4+2.7 Report](reports/SPRINT_2.3_2.4_2.7_REPORT.md) — Implementation Report
- [STATUS.md](STATUS.md) — Projekt-Status

---

*GCL v2.0 Technische Dokumentation — AD-00 bis AD-14 — 05.07.2026 — Aurora v3.2*
