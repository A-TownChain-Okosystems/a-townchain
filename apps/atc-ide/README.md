# ATC IDE

**[L6] Developer-Tools des A-TownChain-Ökosystems** — adoptiert aus dem privaten `Lumino-IDE`-Prototyp (ShivaCoreDev, Stand 02.09.2026; Org-Adoption 13.09.2026).

## Komponenten (Adoptionsstand)

| Panel | Zweck |
|---|---|
| AtcVmSimulatorPanel | ATC-VM-Simulation (Bytecode, Steps, State) |
| AtcDocCompliancePanel | Standards-Compliance-Prüfung |
| AtcRepoDocumentationPanel | Repo-Dokumentation |
| GenesisConfiguratorPanel | Genesis-Engine-Konfiguration |
| GlobusFileFormatsPanel / GlobusV2/V3ArchitecturePanel | GlobusOS-Architektur & Formate |
| RustWorkspaceGeneratorPanel | Rust-Workspace-Scaffolding |
| CiCdGeneratorModal | CI/CD-Generierung |
| CustomLinterPanel / LanguageStrategyPanel | Linting & Sprachstrategie |

Vollständiges Inventar: `src/components/` (39 Panels).

## Status: EXPERIMENTAL

- Lauffähiger Prototyp (React 18 + Vite + Monaco-Editor, Gemini-Integration via `@google/genai`)
- **Kein** Production-Build und **keine** Application-Test-Suite laut aktuellem README. Ein Markdown-Lint-Workflow existiert; er ist kein Ersatz für Build-, Test-, Security- oder Release-Gates. Aktueller CI-Status muss am exakten SHA geprüft werden.
- Noch im Google-AI-Studio-Applet-Skelett (`package.json` name: `react-example`)

## Roadmap (Adoption)

1. AI-Studio-Skelett entfernen (Rebranding zu ATC IDE, `server.ts` Review)
2. CI-Governance-Wiring (Repository Governance, ATC-STD-201/202/203)
3. Application-Test-Setup + Evidence-Statusleiter (SPECIFIED → … → RELEASED); vorhandenes Markdown-Lint bleibt bestehen
4. Integration: ATC-VM-Simulator ↔ atc-vm, Doc-Compliance ↔ atc-standards-Registry

## Provenanz

- Quelle: privates Repo `ShivaCoreDev/Lumino-IDE` (Commit `b434e7c`, 02.09.2026)
- Secret-Check vor Adoption bestanden (nur Placeholders in `.env.example`)
- Registry-Antrag: `atc-standards` PR (registry/repositories.yaml + profiles/atc-ide.yaml)

## Lizenz & Governance

A-TownChain-Ökosystem · ATC-STD-201/202 · Chain-ID 658467
