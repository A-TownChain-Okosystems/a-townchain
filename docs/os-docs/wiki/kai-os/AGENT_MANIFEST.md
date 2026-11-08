# AGENT_MANIFEST.md
> Letzte Aktualisierung: 2026-07-07 06:05 UTC | Aurora Master Sync v3.0

## Repositories
- **Code:** https://github.com/A-TownChain-Okosystems/a-townchain-os
- **Docs:** https://github.com/A-TownChain-Okosystems/a-townchain-os-docs

## Integrationen (17 aktiv)
| Integration | Status | Zweck |
|-------------|--------|-------|
| GitHub | ✅ | Code + Docs Hosting |
| Notion | ✅ | Roadmap + Protokolle |
| Google Sheets | ✅ | Dashboard + Metriken |
| Google Docs | ✅ | Projekt-Reports |
| Google Slides | ✅ | Sprint-Präsentationen |
| Google Calendar | ✅ | Sprint-Deadlines |
| Google Drive | ✅ | Datei-Archiv |
| Google Analytics | ✅ | Web-Traffic |
| Google BigQuery | ✅ | Langzeit-Metriken |
| Google Search Console | ✅ | SEO + Keywords |
| Google Tasks | ✅ | Issue-Tracking |
| Google Meet | ✅ | Standup-Meetings |
| Google Classroom | ✅ | Entwickler-Kurse |
| Gmail | ✅ | Status-Reports |
| Microsoft Outlook | ✅ | Status-Reports |
| Microsoft Teams | ✅ | Team-Kommunikation |
| Microsoft OneDrive | ✅ | Cross-Cloud-Backup |
| Hugging Face | ✅ | KI-Modelle |

## Google Sheets Dashboard
ID: 1xR5c24NrtYC58OsGrLaUHkQUiL_O6eYVyx8KmFcvBD4
URL: https://docs.google.com/spreadsheets/d/1xR5c24NrtYC58OsGrLaUHkQUiL_O6eYVyx8KmFcvBD4

## Notion
- Roadmap: 373b826d-b85c-8125-ba83-f04995191bf0
- Tagesprotokoll: 37bb826d-b85c-81c4-bdd4-cfc0dc74de7e
- Live-Status: 379b826d-b85c-81f4-9b2b-f2a05496a4e1

## Kritischer Entwicklungspfad
#14 Bootstrap → #15 Propagation → #16 Sync → #17 Fork Resolution → #18 Docker → #8 Multi-Node LIVE

## ATCLang Implementierungs-Guide (ATC-01 bis ATC-99)
Vollstaendige Standard-zu-Code-Zuordnung + Implementierungs-Patterns fuer KI-Agenten:
siehe [docs/ATCLANG_AGENT_BUILD_GUIDE.md](docs/ATCLANG_AGENT_BUILD_GUIDE.md) (Status-Quelle: docs/standards/STANDARDS_REGISTRY.md)

## 🤖 Bekannte Base44-Superagent-Instanzen (5)
| # | App-ID | Git-Identitaet | Rolle | Signiert? |
|---|--------|----------------|-------|-----------|
| 1 | `69c1e0c577ccf6c45a27a480` | Michael Wroblewski (+ Tag) | Compliance (unverifiziert, kein Commit-Nachweis) | ✅ |
| 2 | `6a2756186106d6f0fbb105b5` | Michael Wroblewski (+ Tag) | Sync/Cleanup/Governance (dieser Agent) | ✅ |
| 3 | `6a27614c7219ab1e4f951842` | Aurora (MasterBrain) `<aurora@a-townchain.dev>` | ATCLang-Parser, Reality-Checks | ✅ (meist) |
| 4 | `6a0a3f408dced6c5ca7506ef` | Michael Wroblewski (+ Tag) | Reality-Check/Audit | ✅ |
| 5 | ⚠️ unbekannt | `Aurora-Bot <aurora@base44.ai>` | Taeglicher Wiki-Kapitel-Sync | ❌ unsigniert |

> Vollstaendiges Register mit Details: `docs/AGENT_COORDINATION.md`

## Sync-Konfiguration
- **Schedule:** täglich 08:05 Europe/Berlin
- **Agent:** Aurora (Base44 Superagent)
- **Script:** .agents/skills/kai_os_sync/scripts/master_sync.py
- **Version:** v3.0
