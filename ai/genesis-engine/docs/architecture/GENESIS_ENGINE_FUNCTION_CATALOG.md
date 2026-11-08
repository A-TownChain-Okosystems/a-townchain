# Genesis Engine – Funktionskatalog

## Zweck

Dieser Katalog definiert die funktionalen Vertragsanforderungen der allgemeinen Genesis Engine als maschinenlesbare Ausgangsbasis. Er ist **kein** Beleg, dass die aufgeführten API-Namen bereits als Quellcode existieren.

- Umfang: 205 Funktionsverträge in 23 Domänen.
- Status aller Verträge: `REQUIRED`; Implementierungsstatus: `NOT_ASSESSED`; Verifikation: `NOT_VERIFIED`.
- Kanonischer Quellbestand: `A-TownChain-Okosystems/genesis-engine`.
- Genesis Chronicles ist ausschließlich Konsument der Engine; Chronicles-spezifische Regeln gehören nicht in den Engine-Core.

## Verbindliche Reihenfolge

1. Katalog und Schema validieren.
2. Tatsächlichen Quellbaum am exakten HEAD inventarisieren.
3. Jedes Runtime-Symbol über `(repository, head_sha, source_path, symbol_kind, symbol_name)` zuordnen.
4. Jeden Vertrag zu Quellsymbol, Tests und API-/Architekturvertrag reconciliieren.
5. Fehlende Funktionen, Mehrfachzuständigkeiten und begründete Ausschlüsse dokumentieren.
6. Katalogvalidator und striktes Reconciliation-Gate erfolgreich am selben exakten SHA nachweisen.
7. Erst danach separate Implementierungs-PRs erstellen.

## Verifikationsregel

`IMPLEMENTED` erfordert exakte Quellzuordnung und Tests. `VERIFIED` erfordert zusätzlich exakten Source-SHA sowie Run → Job → Step → Exit-Code und Logs/Artefakte. Strukturvalidierung allein beweist keine Funktionalität. Ungeklärte oder ungebundene Runtime-Symbole bleiben blockierend.

## Domänen

- **Foundation** (`foundation`, 10 Verträge): runtime lifecycle, configuration, errors, logging, platform abstraction.
- **Memory & Resources** (`memory`, 8 Verträge): allocation policies, handles, lifetime and budgets.
- **Math & Geometry** (`math`, 8 Verträge): scalar/vector/matrix/quaternion math and bounds.
- **Platform & Devices** (`platform`, 8 Verträge): window, clocks, filesystem, input devices, threads.
- **Jobs & Scheduling** (`jobs`, 8 Verträge): task graph, worker pool, synchronization.
- **ECS & Reflection** (`ecs`, 12 Verträge): entities, components, queries, systems, metadata.
- **World, Scene & Streaming** (`world`, 10 Verträge): worlds, scenes, transforms, chunks, spatial queries.
- **Asset Pipeline** (`assets`, 10 Verträge): asset IDs, import, dependencies, cooking, caching.
- **Graphics & Rendering** (`render`, 12 Verträge): render extraction, GPU resources, passes, materials, diagnostics.
- **Physics & Simulation** (`simulation`, 11 Verträge): fixed-step clock, rigid bodies, collision, queries.
- **Animation & Character** (`animation`, 8 Verträge): skeletons, clips, blending, constraints, root motion.
- **Audio** (`audio`, 8 Verträge): mixing, spatialization, streaming, devices.
- **Input & Actions** (`input`, 8 Verträge): keyboard, pointer, gamepad, rebinding, action mapping.
- **Gameplay Framework** (`gameplay`, 9 Verträge): components, state machines, triggers, timers, generic rules.
- **Scripting & Extensions** (`scripting`, 8 Verträge): script modules, bindings, sandbox, hot reload.
- **Networking & Replication** (`network`, 10 Verträge): transport abstraction, sessions, replication, prediction.
- **Save, Load & Replay** (`persistence`, 8 Verträge): save slots, checkpoints, versioning, deterministic replay.
- **UI & Accessibility** (`ui`, 8 Verträge): widget tree, layout, text, localization, accessibility.
- **Editor & Authoring** (`editor`, 9 Verträge): project, scene editing, gizmos, undo/redo, play mode.
- **SDK & Public API** (`sdk`, 8 Verträge): stable API, plugins, versioning, compatibility.
- **Tools, Build & Diagnostics** (`tools`, 10 Verträge): CLI, build graph, profiling, tracing, crash reporting.
- **Security & Trust Boundaries** (`security`, 7 Verträge): capabilities, permissions, validation, safe content.
- **Testing & Conformance** (`testing`, 7 Verträge): unit/integration/property tests, fixtures, determinism.

## Dateien

- `genesis-function-catalog.json`: Funktionsverträge, Status, Abnahmekriterien und Evidenzfelder.
- `genesis-function-catalog.schema.json`: JSON Schema Draft 2020-12.

## Abgrenzung

Die API-Namen sind logische Vertragsnamen und dürfen nicht als vorhandene Rust-/Python-Symbole ausgegeben werden. Abweichende bestehende APIs werden auf ihre Semantik geprüft und entweder zugeordnet oder mit begründetem Delta erfasst. Keine Implementierung, kein Merge und kein Release wird durch diesen Katalog autorisiert.
