# SYNC_PROTOCOL.md
> Version 3.2 — 04.07.2026 20:41

## Auto-Sync Verhalten (STRIKT)

### Roadmap-Seite (373b826d-b85c-8125-ba83-f04995191bf0)

#### Erlaubt:
- Append eines einzelnen Callouts GANZ UNTEN (nach "🔄 Auto-Sync Log" Heading)
- Format: "🤖 Auto-Sync DATETIME: 0 offene / 68 geschlossene Issues"

#### Verboten:
- Titel der Seite ändern
- Blöcke oben einfügen (vor Heading_1 "Master Roadmap")
- "Letzter Commit" Zeile überschreiben
- Versions-Abschnitte, Issue-Listen oder Fortschrittsbalken verändern
- Issue-Anzahl im Titel ändern (immer 68/68)

### Tagesprotokoll-Seite (37bb826d-b85c-81c4-bdd4-cfc0dc74de7e)
- Append am Ende erlaubt
- Titel nicht ändern

### Live-Status-Seite (379b826d-b85c-81f4-9b2b-f2a05496a4e1)
- Komplettes Update erlaubt (diese Seite ist Auto-Sync-only)

## Sync-Zeit
- Täglich 08:05 Europe/Berlin
- Agent: Aurora v3.2
