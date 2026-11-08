🧩 Komponenten-Plan — atc-globus-shell

«Version: 2.0
Datum: 2026-08-23
Projekt: ATC Globus Shell
Plattform: Globus OS
Sprache: ATCLang v0.3
Status: Architecture Planning»

---

0. Architekturziel

"atc-globus-shell" ist die zentrale Command-Line- und Steuerungsschicht von Globus OS.

Die Shell stellt eine standardisierte Runtime zwischen Benutzer, ATCLang Runtime, Prozessen, Services, IPC, Dateisystem, Security und Globus OS bereit.

User
 │
 ▼
Terminal / Input
 │
 ▼
Tokenizer
 │
 ▼
Parser
 │
 ▼
Command AST
 │
 ▼
Shell Runtime
 │
 ▼
Command Executor
 │
 ├── Builtin Commands
 ├── External Processes
 ├── Pipelines
 ├── Jobs
 ├── Services
 ├── Globus OS
 └── AI / Agents
 │
 ▼
I/O + IPC + Security
 │
 ▼
Globus OS

---

1. Core / Runtime

1.1 "src/shell.atc"

Beschreibung: Hauptmodul der interaktiven Shell.

Priorität: P0
Status: 📋 GEPLANT

Verantwortung:

- Shell initialisieren
- Input Loop starten
- Commands an Runtime übergeben
- Output verwalten
- Shell beenden

Abhängigkeiten:

- "shell_runtime.atc"
- "shell_session.atc"
- "cli_parser.atc"

Akzeptanzkriterien:

1. ATCLang-v0.3-kompatibel
2. Öffentliche Funktionen besitzen Type-Signatures
3. Shell kann gestartet und beendet werden
4. FILE_REGISTER.md aktualisiert

---

1.2 "src/shell_runtime.atc"

Beschreibung: Zentrale Shell Runtime.

Priorität: P0
Status: 📋 GEPLANT

Verantwortung:

- Runtime Lifecycle
- Command Dispatch
- Session State
- Fehlerbehandlung
- Execution Context

Abhängigkeiten:

- "shell_session.atc"
- "command_executor.atc"
- "environment.atc"

---

1.3 "src/shell_session.atc"

Beschreibung: Verwaltung einer aktiven Shell-Session.

Priorität: P0
Status: 📋 GEPLANT

Verantwortung:

- Session ID
- Benutzerkontext
- Working Directory
- Environment
- Shell State

---

1.4 "src/shell_state.atc"

Beschreibung: Persistenter und temporärer Zustand der Shell.

Priorität: P0
Status: 📋 GEPLANT

---

2. Input / CLI

2.1 "src/cli_commands.atc"

Beschreibung: Command Definition und Command Interface.

Priorität: P0
Status: 📋 GEPLANT

---

2.2 "src/cli_tokenizer.atc"

Beschreibung: Zerlegt Benutzereingaben in Tokens.

Priorität: P0
Status: 📋 GEPLANT

Unterstützt:

- Commands
- Arguments
- Quotes
- Escaping
- Pipes
- Redirects
- Variables
- Operators

---

2.3 "src/cli_parser.atc"

Beschreibung: Wandelt Tokens in eine Command-Struktur / AST um.

Priorität: P0
Status: 📋 GEPLANT

Unterstützt:

- Einzelbefehle
- Argumente
- Pipelines
- Redirects
- Command Chains
- Subcommands

---

2.4 "src/shell_history.atc"

Beschreibung: Command History.

Priorität: P1
Status: 📋 GEPLANT

Funktionen:

- Speichern
- Laden
- Suchen
- Löschen
- History Navigation

---

2.5 "src/shell_completion.atc"

Beschreibung: Auto-Completion.

Priorität: P1
Status: 📋 GEPLANT

Unterstützt:

- Commands
- Arguments
- Files
- Directories
- Options
- Plugins

---

3. Command Execution

3.1 "src/command_executor.atc"

Beschreibung: Führt geparste Commands aus.

Priorität: P0
Status: 📋 GEPLANT

Verantwortung:

- Command Dispatch
- Builtin-Aufruf
- Process-Aufruf
- Pipeline-Ausführung
- Exit Codes

---

3.2 "src/process_manager.atc"

Beschreibung: Verwaltung externer Prozesse.

Priorität: P1
Status: 📋 GEPLANT

Funktionen:

- Start
- Stop
- Kill
- Suspend
- Resume
- Process Listing

---

3.3 "src/job_control.atc"

Beschreibung: Verwaltung von Foreground- und Background-Jobs.

Priorität: P1
Status: 📋 GEPLANT

Unterstützt:

- Foreground
- Background
- Job IDs
- Job Status
- Job Switching

---

3.4 "src/exit_codes.atc"

Beschreibung: Standardisierte Exit-Code-Verwaltung.

Priorität: P1
Status: 📋 GEPLANT

---

3.5 "src/environment.atc"

Beschreibung: Shell Environment.

Priorität: P0
Status: 📋 GEPLANT

Unterstützt:

- Environment Variables
- Local Variables
- PATH
- Working Directory
- Session Variables

---

4. I/O Layer

4.1 "src/pipe_system.atc"

Beschreibung: Pipes und Command Chains.

Priorität: P0
Status: 📋 GEPLANT

Beispiel:

command1 | command2 | command3

---

4.2 "src/stream.atc"

Beschreibung: Abstraktion für Datenströme.

Priorität: P0
Status: 📋 GEPLANT

---

4.3 "src/stdin.atc"

Beschreibung: Standard Input.

Priorität: P0
Status: 📋 GEPLANT

---

4.4 "src/stdout.atc"

Beschreibung: Standard Output.

Priorität: P0
Status: 📋 GEPLANT

---

4.5 "src/stderr.atc"

Beschreibung: Standard Error.

Priorität: P0
Status: 📋 GEPLANT

---

4.6 "src/redirection.atc"

Beschreibung: Input-/Output-Redirects.

Priorität: P1
Status: 📋 GEPLANT

Beispiele:

command > file
command >> file
command < file

---

5. Built-in Commands

5.1 "src/builtin.atc"

Beschreibung: Registry und Runtime für Built-in Commands.

Priorität: P1
Status: 📋 GEPLANT

---

5.2 "src/filesystem_commands.atc"

Beschreibung: Dateisystembezogene Shell Commands.

Priorität: P1
Status: 📋 GEPLANT

Beispiele:

- "ls"
- "cd"
- "pwd"
- "mkdir"
- "rm"
- "cp"
- "mv"

---

5.3 "src/process_commands.atc"

Beschreibung: Process Commands.

Priorität: P1
Status: 📋 GEPLANT

---

5.4 "src/system_commands.atc"

Beschreibung: Globus-OS-Systembefehle.

Priorität: P1
Status: 📋 GEPLANT

---

5.5 "src/shell_commands.atc"

Beschreibung: Shell-interne Befehle.

Priorität: P1
Status: 📋 GEPLANT

Beispiele:

- "history"
- "alias"
- "set"
- "unset"
- "exit"
- "help"
- "clear"

---

6. Globus OS Integration

6.1 "src/globus_api.atc"

Beschreibung: Globus-OS-API-Abstraktion.

Priorität: P1
Status: 📋 GEPLANT

---

6.2 "src/ipc.atc"

Beschreibung: Inter-Process Communication.

Priorität: P1
Status: 📋 GEPLANT

Verantwortung:

- Process-to-Process Communication
- Service Communication
- Shell-to-System Communication

---

6.3 "src/service_manager.atc"

Beschreibung: Verwaltung von Globus-OS-Services.

Priorität: P2
Status: 📋 GEPLANT

Funktionen:

- Start
- Stop
- Restart
- Status
- Enable
- Disable

---

6.4 "src/device_manager.atc"

Beschreibung: Geräteverwaltung.

Priorität: P2
Status: 📋 GEPLANT

---

6.5 "src/session_manager.atc"

Beschreibung: System- und Benutzer-Session-Verwaltung.

Priorität: P2
Status: 📋 GEPLANT

---

7. Security

7.1 "src/permissions.atc"

Beschreibung: Berechtigungsprüfung.

Priorität: P1
Status: 📋 GEPLANT

---

7.2 "src/capability.atc"

Beschreibung: Capability-basierte Zugriffskontrolle.

Priorität: P1
Status: 📋 GEPLANT

---

7.3 "src/sandbox.atc"

Beschreibung: Isolierung nicht vertrauenswürdiger Commands und Plugins.

Priorität: P2
Status: 📋 GEPLANT

---

7.4 "src/authentication.atc"

Beschreibung: Benutzer- und Session-Authentifizierung.

Priorität: P2
Status: 📋 GEPLANT

---

7.5 "src/audit.atc"

Beschreibung: Security Audit und Command Audit Trail.

Priorität: P2
Status: 📋 GEPLANT

---

8. Configuration

8.1 "src/shell_config.atc"

Beschreibung: Zentrale Shell-Konfiguration.

Priorität: P1
Status: 📋 GEPLANT

---

8.2 "src/profiles.atc"

Beschreibung: Shell Profiles.

Priorität: P1
Status: 📋 GEPLANT

---

8.3 "src/aliases.atc"

Beschreibung: Command Aliases.

Priorität: P1
Status: 📋 GEPLANT

---

8.4 "src/variables.atc"

Beschreibung: Shell Variables.

Priorität: P1
Status: 📋 GEPLANT

---

8.5 "src/themes.atc"

Beschreibung: Prompt- und Terminal-Themes.

Priorität: P3
Status: 📋 GEPLANT

---

9. Plugin / Extension System

9.1 "src/plugin_loader.atc"

Beschreibung: Laden von Shell Plugins.

Priorität: P2
Status: 📋 GEPLANT

---

9.2 "src/plugin_registry.atc"

Beschreibung: Registrierung und Verwaltung von Plugins.

Priorität: P2
Status: 📋 GEPLANT

---

9.3 "src/command_registry.atc"

Beschreibung: Zentrale Command Registry.

Priorität: P2
Status: 📋 GEPLANT

---

9.4 "src/shell_hooks.atc"

Beschreibung: Lifecycle Hooks.

Priorität: P3
Status: 📋 GEPLANT

Beispiele:

- on_start
- on_command
- on_error
- on_exit

---

10. AI / Agent Integration

10.1 "src/agent_bridge.atc"

Beschreibung: Verbindung zwischen Shell und AI Agents.

Priorität: P3
Status: 📋 GEPLANT

---

10.2 "src/ai_command.atc"

Beschreibung: AI-bezogene Shell Commands.

Priorität: P3
Status: 📋 GEPLANT

Beispiele:

ai generate
ai analyze
ai explain
ai execute

---

10.3 "src/context.atc"

Beschreibung: Übergabe von Shell- und Systemkontext an Agents.

Priorität: P3
Status: 📋 GEPLANT

---

10.4 "src/tool_bridge.atc"

Beschreibung: Verbindung zwischen AI Agents und Globus-OS-Tools.

Priorität: P3
Status: 📋 GEPLANT

---

11. Diagnostics

11.1 "src/logging.atc"

Beschreibung: Zentrales Logging.

Priorität: P1
Status: 📋 GEPLANT

---

11.2 "src/errors.atc"

Beschreibung: Einheitliches Error-System.

Priorität: P0
Status: 📋 GEPLANT

---

11.3 "src/diagnostics.atc"

Beschreibung: Diagnose- und Debugging-System.

Priorität: P2
Status: 📋 GEPLANT

---

11.4 "src/telemetry.atc"

Beschreibung: Optionale System- und Performance-Telemetrie.

Priorität: P3
Status: 📋 GEPLANT

---

12. Testing

12.1 "tests/parser_tests.atc"

Parser Tests.

Priorität: P0
Status: 📋 GEPLANT

---

12.2 "tests/execution_tests.atc"

Command-Execution Tests.

Priorität: P0
Status: 📋 GEPLANT

---

12.3 "tests/io_tests.atc"

I/O-, Pipe- und Redirect-Tests.

Priorität: P1
Status: 📋 GEPLANT

---

12.4 "tests/security_tests.atc"

Security Tests.

Priorität: P1
Status: 📋 GEPLANT

---

12.5 "tests/integration_tests.atc"

Integration Tests für Shell + Globus OS.

Priorität: P1
Status: 📋 GEPLANT

---

13. Globale Akzeptanzkriterien

Jede Komponente muss:

1. mit ATCLang v0.3 kompatibel sein
2. erfolgreich durch den ATCLang Parser laufen
3. Type-Signatures für öffentliche Funktionen besitzen
4. definierte Ein- und Ausgabeschnittstellen besitzen
5. dokumentierte Abhängigkeiten besitzen
6. im "FILE_REGISTER.md" registriert sein
7. Fehlerzustände definiert behandeln
8. Security-Anforderungen berücksichtigen
9. testbar sein
10. keine unnötigen zyklischen Abhängigkeiten erzeugen

---

14. Dependency-Regeln

UI / Terminal
      ↓
CLI Input
      ↓
Tokenizer
      ↓
Parser
      ↓
AST
      ↓
Shell Runtime
      ↓
Command Executor
      ↓
┌───────────────┬───────────────┐
│ Builtins      │ Processes     │
└───────────────┴───────────────┘
      ↓
I/O Layer
      ↓
IPC / Globus API
      ↓
Globus OS

Security darf von keiner höheren Schicht umgangen werden:

Command
   ↓
Executor
   ↓
Permission / Capability Check
   ↓
Sandbox / Policy
   ↓
Execution

---

15. Entwicklungsphasen

Phase 1 — Shell Foundation

P0

- Shell Runtime
- Session
- Tokenizer
- Parser
- Executor
- Environment
- Error System
- stdin/stdout/stderr
- Streams

Ziel: Erste funktionierende Shell.

---

Phase 2 — Command System

P1

- Builtins
- Filesystem Commands
- Process Manager
- Job Control
- Exit Codes
- History
- Completion
- Pipes
- Redirects

Ziel: Vollwertige interaktive CLI.

---

Phase 3 — Globus OS

P1/P2

- Globus API
- IPC
- Service Manager
- Device Manager
- Session Manager

Ziel: Shell kann Globus OS direkt steuern.

---

Phase 4 — Security

P1/P2

- Permissions
- Capabilities
- Sandbox
- Authentication
- Audit

Ziel: Sichere System-Shell.

---

Phase 5 — Extension Platform

P2/P3

- Plugin Loader
- Plugin Registry
- Command Registry
- Hooks
- Themes

Ziel: Erweiterbare Shell-Plattform.

---

Phase 6 — AI / Agents

P3

- Agent Bridge
- AI Commands
- Context
- Tool Bridge

Ziel: AI-native Globus Shell.

---

Phase 7 — Production Hardening

P0–P3

- Unit Tests
- Integration Tests
- Security Tests
- Diagnostics
- Performance Testing
- Telemetry
- Documentation

Ziel: Production-Ready Release.

---

16. Definition of Done

Eine Komponente gilt erst als IMPLEMENTED, wenn:

[✓] Datei vorhanden
[✓] ATCLang v0.3 Parser erfolgreich
[✓] Type-Signatures vorhanden
[✓] API definiert
[✓] Dependencies definiert
[✓] Fehlerbehandlung implementiert
[✓] Security geprüft
[✓] Tests vorhanden
[✓] Tests erfolgreich
[✓] FILE_REGISTER.md aktualisiert
[✓] Dokumentation aktualisiert

---

17. Statusmodell

📋 GEPLANT
    ↓
🔨 IN ENTWICKLUNG
    ↓
🧪 TESTING
    ↓
✅ IMPLEMENTIERT
    ↓
🔒 STABIL

Fehlerzustand:

❌ BLOCKIERT

---

18. Architekturprinzipien

Separation of Concerns

Jede Komponente besitzt eine klar definierte Verantwortung.

Modularität

Komponenten müssen unabhängig testbar und austauschbar sein.

Security by Design

Berechtigungen und Capabilities werden vor der Ausführung geprüft.

API First

Öffentliche Schnittstellen werden vor der Implementierung definiert.

ATCLang Native

Die Shell wird vollständig auf ATCLang v0.3 ausgerichtet.

Globus Native

Globus-OS-Services werden über definierte APIs und IPC-Schnittstellen angesprochen.

AI Ready

Die Architektur erlaubt später native AI-Agent-Integration, ohne den Shell-Kern neu zu bauen.

Test First

Kritische Komponenten erhalten automatisierte Tests vor Produktionsfreigabe.

---

19. Gesamtstatus

Bereich| Priorität| Status
Core / Runtime| P0| 📋 GEPLANT
CLI / Parser| P0| 📋 GEPLANT
Execution| P0/P1| 📋 GEPLANT
I/O| P0| 📋 GEPLANT
Builtins| P1| 📋 GEPLANT
Globus OS| P1/P2| 📋 GEPLANT
Security| P1/P2| 📋 GEPLANT
Configuration| P1/P3| 📋 GEPLANT
Plugins| P2/P3| 📋 GEPLANT
AI / Agents| P3| 📋 GEPLANT
Diagnostics| P0–P3| 📋 GEPLANT
Testing| P0–P1| 📋 GEPLANT

---

Projektstatus: 🏗️ ARCHITECTURE EXPANSION

Ziel: Production-Ready ATC Globus Shell für Globus OS.

---

ATC Globus Shell · Component Architecture v2.0 · 2026-08-23