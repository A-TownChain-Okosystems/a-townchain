# Milestone 1 — ECS-Kern + 2D-Renderer (ERSTER ECHTER CODE)

> ✅ **Status: LAUFFAEHIG.** Im Unterschied zu allen anderen Dokumenten in
> diesem Repo (reine Vision) ist dies der erste tatsaechliche Code-Stand.

## Was funktioniert

- **`core/ecs.py`** — minimales Entity-Component-System: `World`,
  `System`-Basisklasse, generische `query()` ueber beliebige
  Component-Kombinationen. Komponenten: `Position`, `Velocity`, `Sprite`.
  System: `MovementSystem`.
- **`render/renderer2d.py`** — 2D-Renderer (pygame), zeichnet alle
  Entities mit `Position`+`Sprite` als farbige Rechtecke. Headless-Modus
  fuer Tests/CI ohne Display.
- **`main.py`** — Demo: 5 bewegte, farbige Entities, 60 Frames.
- **`tests/test_ecs.py`** — 4 Unit-Tests (Entity-Lifecycle, Component-Query,
  Movement-System, Query-Filterung). **Alle 4 bestehen.**

## Verifiziert (07.07.2026)

```
$ python3 tests/test_ecs.py
Alle 4 Tests bestanden - OK

$ python3 main.py --headless
[headless] Frame 59: 5 Entities gerendert
  Entity @ (70.0, 60.0) size=32x32 color=(255, 80, 80)
  ... (4 weitere)
```

## Bewusst NICHT enthalten (kein Scope-Creep)

Kein Asset-Pipeline, keine Shader, kein Networking, keine KI-Integration,
keine Physik-Engine, kein Editor. Das ist Absicht — Milestone 1 beweist nur:
ECS-Architektur funktioniert und laesst sich rendern. Alles Weitere sind
separate, spaeter zu planende Milestones.

## Naechste moegliche Schritte (nicht automatisch starten — erst nach Freigabe)

- Milestone 2: Input-Handling (Tastatur/Maus -> Velocity-Aenderung)
- Milestone 3: einfache Kollisionserkennung (AABB)
- Milestone 4: Asset-Loading (Sprites aus Bilddateien statt Farbrechtecke)

*Erstellt von Agent `aurora-base44-superagent-69c1e0c577ccf6c45a27a480`.*
