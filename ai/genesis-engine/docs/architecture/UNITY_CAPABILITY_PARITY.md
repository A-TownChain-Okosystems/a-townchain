# Unity Capability Parity — Genesis Engine

## Ziel

Genesis Engine soll eine eigenständige, vollständige Spieleentwicklung ermöglichen und ein breites, Unity-inspiriertes Funktionsspektrum abdecken. Dieses Dokument ist ein **Anforderungskatalog**, keine Behauptung, dass die Funktionen bereits implementiert oder mit Unity vollständig gleichwertig sind.

Die Umsetzung erfolgt als unabhängige Clean-Room-Implementierung. Es werden weder proprietärer Unity-Quellcode noch geschützte Assets, interne Implementierungsdetails oder Unity-eigene Text-/Codebestände kopiert. Plattformen mit proprietären SDKs benötigen autorisierte SDKs und Adapter.

## Umfang

Der maschinenlesbare Katalog `unity-capability-parity.json` enthält **313 Zielanforderungen in 25 Domänen**:

- **Editor, Projekt und Workflow** (`editor_project`): 12 Anforderungen
- **Runtime und Lebenszyklus** (`runtime_lifecycle`): 10 Anforderungen
- **Szenen und Weltverwaltung** (`scene_world`): 11 Anforderungen
- **Rendering und Grafikpipeline** (`render_pipeline`): 20 Anforderungen
- **Assets und Content Pipeline** (`assets_content`): 16 Anforderungen
- **Prefabs, Serialisierung und Daten** (`prefabs_data`): 12 Anforderungen
- **Scripting und Code-Integration** (`scripting`): 14 Anforderungen
- **Visual Scripting und Graphen** (`visual_scripting`): 12 Anforderungen
- **Physik, Kollision und Simulation** (`physics`): 14 Anforderungen
- **Animation und Character** (`animation`): 15 Anforderungen
- **Audio und Medien** (`audio`): 12 Anforderungen
- **UI, UX und Accessibility** (`ui`): 13 Anforderungen
- **Input und Geräte** (`input`): 12 Anforderungen
- **KI und Navigation** (`ai_navigation`): 12 Anforderungen
- **Netzwerk und Multiplayer** (`networking`): 13 Anforderungen
- **Terrain, Vegetation und Umgebung** (`terrain_environment`): 12 Anforderungen
- **Prozedurale Inhalte** (`procedural`): 10 Anforderungen
- **Cinematics und Sequencing** (`cinematics`): 12 Anforderungen
- **Build, Packaging und Plattformen** (`build_platforms`): 14 Anforderungen
- **Testing, Debugging und Profiling** (`testing_profiling`): 14 Anforderungen
- **Pakete, Plugins und Zusammenarbeit** (`packages_collaboration`): 12 Anforderungen
- **XR und räumliche Interaktion** (`xr`): 10 Anforderungen
- **Lokalisierung und Internationalisierung** (`localization`): 9 Anforderungen
- **Sicherheit und Zuverlässigkeit** (`security_reliability`): 10 Anforderungen
- **Erweiterbarkeit und Tooling** (`extensibility`): 12 Anforderungen

## Statusmodell

- `NOT_ASSESSED`: noch nicht gegen den Quellbaum geprüft
- `SPECIFIED`: Vertrag und Akzeptanzkriterien präzisiert
- `MAPPED`: konkrete Quelle, Symbol, Commit-SHA und Zuständigkeit zugeordnet
- `IMPLEMENTED`: Implementierung vorhanden; allein kein Verifikationsnachweis
- `VERIFIED`: Akzeptanztests und reproduzierbare Evidenz für exakten SHA bestanden
- `BLOCKED`: ungelöste Abhängigkeit oder Architekturentscheidung
- `EXCLUDED`: begründeter Ausschluss mit dokumentierter Begründung
- `EXTERNAL_DEPENDENCY`: externe, lizenzierte oder plattformspezifische Abhängigkeit

## Mindestdefinition „vollständig spielbar“

Ein vertikaler End-to-End-Nachweis muss mindestens ein Projekt erstellen/öffnen, eine Szene bearbeiten, Assets importieren und validieren, Gameplay und Eingabe konfigurieren, Grafik und Audio ausführen, Spielstände speichern/laden, automatisierte Tests starten sowie einen eigenständigen Build paketieren. Build-Provenienz, Testberichte und exakte Quell-SHA müssen als Artefakte vorliegen. Multiplayer, XR, Cinematics und zusätzliche Zielplattformen werden über eigene Akzeptanzprofile abgenommen.

## Verbindliche Reihenfolge

1. JSON-Syntax, Schema und Kataloginvarianten validieren.
2. Alle aktiven Genesis-Engine-Quellbäume und vorhandenen Schnittstellen inventarisieren.
3. Jede Anforderung exakt einem Pfad/Symbol und Commit-SHA zuordnen oder mit Begründung ausschließen/blockieren.
4. Verträge, Abhängigkeiten, Akzeptanztests und Validatoren ergänzen.
5. Reconciliation- und CI-Gates erfolgreich am exakten HEAD nachweisen.
6. Erst dann Implementierung in priorisierten, kleinen PRs beginnen.

Keine Funktion gilt als `VERIFIED`, nur weil sie im Katalog steht oder Code vorhanden ist. PR bleibt Draft, bis die festgelegten Gates nachweislich bestanden sind.
