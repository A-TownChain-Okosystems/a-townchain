Ich würde die ROADMAP.md jetzt auf denselben v2.0-Architekturstand vom 23.08.2026 bringen wie COMPONENT_PLAN.md, ARCHITECTURE.md, CHANGELOG.md und FILE_REGISTER.md.

Da GitHub den direkten Abruf der Datei aktuell blockiert, kann ich den bestehenden Inhalt nicht zuverlässig übernehmen. Für den neuen Zielstand würde ich folgende vollständige ROADMAP.md verwenden:

# 🗺️ ATC Globus Shell — Roadmap

> **Version:** 2.0
> **Datum:** 2026-08-23
> **Projekt:** ATC Globus Shell
> **Plattform:** Globus OS
> **Sprache:** ATCLang v0.3
> **Status:** Architecture Expansion

---

# 1. Roadmap Vision

Die ATC Globus Shell wird als zentrale Command-Line- und
System-Steuerungsschicht für Globus OS entwickelt.

Ziel ist eine modulare, sichere, erweiterbare und
AI-fähige Shell Runtime.

```text
User
 ↓
Terminal
 ↓
CLI Input
 ↓
Tokenizer
 ↓
Parser
 ↓
Command AST
 ↓
Shell Runtime
 ↓
Command Executor
 ↓
Security
 ↓
I/O / IPC
 ↓
Globus OS

Langfristiges Ziel:

ATC Globus Shell
        │
        ├── CLI
        ├── Runtime
        ├── Processes
        ├── Services
        ├── Filesystem
        ├── Security
        ├── Plugins
        ├── IPC
        └── AI Agents
                 │
                 ▼
             Globus OS


---

2. Development Strategy

Die Entwicklung erfolgt in sieben Hauptphasen:

Phase 1
Foundation
   ↓
Phase 2
Command System
   ↓
Phase 3
Globus OS Integration
   ↓
Phase 4
Security
   ↓
Phase 5
Extension Platform
   ↓
Phase 6
AI / Agents
   ↓
Phase 7
Production Hardening

Prioritätssystem:

P0 = Core / Blocker
P1 = Required
P2 = Advanced
P3 = Future / Optional


---

3. Phase 0 — Architecture Foundation

Status

🧪 ARCHITECTURE EXPANSION

Ziel

Die technische Grundlage des Projekts vollständig definieren, bevor größere Implementierungen beginnen.

Deliverables

ARCHITECTURE.md

COMPONENT_PLAN.md

FILE_REGISTER.md

CHANGELOG.md

ROADMAP.md

LICENSE

ATC License Definition


Architekturprinzipien

Separation of Concerns

API First

Security by Design

ATCLang Native

Globus Native

AI Ready

Test First

keine unnötigen zyklischen Dependencies


Exit Criteria

[✓] Architektur definiert
[✓] Komponenten definiert
[✓] Dependency-Regeln definiert
[✓] Statusmodell definiert
[✓] Definition of Done definiert
[✓] Lizenzmodell definiert


---

4. Phase 1 — Shell Foundation

Priorität

P0

Ziel

Eine minimale funktionierende ATCLang-Shell.

Komponenten

src/shell.atc
src/shell_runtime.atc
src/shell_session.atc
src/shell_state.atc
src/environment.atc

src/cli_commands.atc
src/cli_tokenizer.atc
src/cli_parser.atc

src/command_executor.atc

src/stream.atc
src/stdin.atc
src/stdout.atc
src/stderror.atc

src/errors.atc

Kernfunktionen

Shell Startup

Shell Shutdown

Input Loop

Tokenization

Parsing

AST

Command Dispatch

Execution Context

Environment

stdin

stdout

stderr

Error Handling


Exit Criteria

[ ] Shell startet
[ ] Shell akzeptiert Input
[ ] Tokenizer funktioniert
[ ] Parser erzeugt AST
[ ] Executor führt Commands aus
[ ] stdout funktioniert
[ ] stderr funktioniert
[ ] Exit Codes funktionieren
[ ] Fehler werden kontrolliert behandelt


---

5. Phase 2 — Command System

Priorität

P1

Ziel

Aus der Core-Shell wird eine vollwertige interaktive CLI.

Komponenten

src/builtin.atc
src/filesystem_commands.atc
src/process_commands.atc
src/system_commands.atc
src/shell_commands.atc

src/process_manager.atc
src/job_control.atc
src/exit_codes.atc

src/pipe_system.atc
src/redirection.atc

src/shell_history.atc
src/shell_completion.atc

Features

Filesystem

ls
cd
pwd
mkdir
rm
cp
mv

Shell

history
alias
set
unset
help
clear
exit

Process

process list
process start
process stop
process kill
process suspend
process resume

Pipelines

command1 | command2 | command3

Redirection

command > file
command >> file
command < file

Exit Criteria

[ ] Builtin Registry
[ ] Filesystem Commands
[ ] Process Commands
[ ] Job Control
[ ] Pipes
[ ] Redirects
[ ] History
[ ] Completion


---

6. Phase 3 — Globus OS Integration

Priorität

P1 / P2

Ziel

Die Shell wird zur nativen Steuerungsschicht von Globus OS.

Komponenten

src/globus_api.atc
src/ipc.atc
src/service_manager.atc
src/device_manager.atc
src/session_manager.atc

Features

Globus API

System Information
System Configuration
Service Access
Device Access
Session Access

IPC

Shell
 ↓
IPC
 ↓
Service

Unterstützte Kommunikation:

Process → Process

Process → Service

Shell → Service

Shell → System

Agent → Tool


Service Management

service start
service stop
service restart
service status
service enable
service disable

Exit Criteria

[ ] Globus API definiert
[ ] IPC Interface definiert
[ ] Services erreichbar
[ ] Service Lifecycle funktioniert
[ ] Device Management integriert
[ ] Session Management integriert


---

7. Phase 4 — Security Architecture

Priorität

P1 / P2

Ziel

Production-taugliche sichere System-Shell.

Komponenten

src/permissions.atc
src/capability.atc
src/sandbox.atc
src/authentication.atc
src/audit.atc

Security Pipeline

Command
 ↓
Authentication
 ↓
Permission Check
 ↓
Capability Check
 ↓
Policy Check
 ↓
Sandbox
 ↓
Execution

Security Controls

Authentication

Authorization

Permissions

Capabilities

Sandbox

Audit Trail

Command Audit

Policy Enforcement

Revocation


Grundregel

Keine Execution darf die Security-Schicht umgehen.

Dies gilt ebenfalls für:

Plugins

AI Agents

externe Prozesse

System Commands

Globus Services


Exit Criteria

[ ] Permission System
[ ] Capability System
[ ] Sandbox
[ ] Authentication
[ ] Audit
[ ] Security Tests


---

8. Phase 5 — Extension Platform

Priorität

P2 / P3

Ziel

Die Shell wird zu einer modularen Plattform.

Komponenten

src/plugin_loader.atc
src/plugin_registry.atc
src/command_registry.atc
src/shell_hooks.atc
src/themes.atc

Plugin Lifecycle

Plugin
 ↓
Loader
 ↓
Validation
 ↓
Permission Check
 ↓
Capability Assignment
 ↓
Sandbox
 ↓
Registry
 ↓
Activation

Features

Dynamic Plugins

Command Extensions

Lifecycle Hooks

Plugin Registry

Command Registry

Plugin Permissions

Plugin Capabilities

Plugin Isolation


Hooks

on_start
on_command
on_error
on_exit


---

9. Phase 6 — AI / Agent Platform

Priorität

P3

Ziel

ATC Globus Shell wird AI-native.

Komponenten

src/agent_bridge.atc
src/ai_command.atc
src/context.atc
src/tool_bridge.atc

AI Commands

ai generate
ai analyze
ai explain
ai execute

Agent Architecture

User
 ↓
Shell
 ↓
AI Command
 ↓
Agent Bridge
 ↓
Agent
 ↓
Context
 ↓
Tool Bridge
 ↓
Command / Tool
 ↓
Security
 ↓
Execution

Security Requirement

AI Agents erhalten niemals automatisch höhere Berechtigungen als normale Shell-Prozesse.

AI Actions müssen denselben Security Flow durchlaufen:

AI Action
 ↓
Parser
 ↓
Executor
 ↓
Permission
 ↓
Capability
 ↓
Sandbox
 ↓
Execution


---

10. Phase 7 — Diagnostics & Observability

Priorität

P1 / P2 / P3

Komponenten

src/logging.atc
src/errors.atc
src/diagnostics.atc
src/telemetry.atc

Logging

Unterstützte Levels:

DEBUG
INFO
WARNING
ERROR
FATAL
SECURITY
AUDIT

Diagnostics

Geplante Funktionen:

Runtime Diagnostics

Process Diagnostics

Memory Diagnostics

IPC Diagnostics

Plugin Diagnostics

Security Diagnostics

Performance Diagnostics


Telemetry

Optional und policy-gesteuert:

CPU

Memory

Execution Time

Process Metrics

Command Metrics

Error Metrics



---

11. Testing Roadmap

Testing ist Bestandteil jeder Entwicklungsphase.

Unit Tests

tests/parser_tests.atc
tests/execution_tests.atc
tests/io_tests.atc

Security Tests

tests/security_tests.atc

Integration Tests

tests/integration_tests.atc

Test Pipeline

Source
 ↓
Parser Test
 ↓
Unit Test
 ↓
Component Test
 ↓
Integration Test
 ↓
Security Test
 ↓
Performance Test
 ↓
Release


---

12. ATCLang Compatibility

Die Shell wird vollständig auf:

ATCLang v0.3

ausgerichtet.

Jede öffentliche Komponente muss:

gültige ATCLang-Syntax verwenden

Type-Signatures besitzen

definierte APIs besitzen

Dependencies deklarieren

Fehlerzustände behandeln

testbar sein



---

13. ATC License Integration

Die Shell muss mit der ATC License Architecture kompatibel bleiben.

Lizenzmodell:

License Identity
       ↓
Permission Layer
       ↓
Validation Layer
       ↓
Enforcement Layer

Beispiel:

ATC-LIC-000001

Mögliche Lizenztypen:

PER_CALL
SUBSCRIPTION
PERPETUAL
REVENUE_SHARE
FREEMIUM
DAO_GOVERNED

Lizenzprüfung darf nicht als Ersatz für OS Security dienen.


---

14. Performance Roadmap

Nach Abschluss der funktionalen Architektur folgt Performance Optimization.

Messgrößen:

Shell Startup Time
Tokenizer Latency
Parser Latency
Command Dispatch Latency
IPC Latency
Process Spawn Time
Memory Usage
CPU Usage

Ziel:

Input
 ↓
Tokenizer
 ↓
Parser
 ↓
Executor

mit minimalem Overhead.

Optimierung darf nicht auf Kosten von:

Security

deterministischem Verhalten

Fehlerbehandlung

API-Stabilität


erfolgen.


---

15. Production Hardening

Ziel

Production-Ready Release.

Bereiche

Reliability

Crash Handling

Recovery

State Integrity

Process Recovery


Security

Security Audit

Sandbox Audit

Capability Audit

Permission Audit

Dependency Audit


Performance

Load Testing

Stress Testing

IPC Benchmarking

Memory Testing


Compatibility

ATCLang Compatibility

Globus OS Compatibility

Plugin Compatibility

API Compatibility



---

16. Release Strategy

Development

0.x

Experimentelle Entwicklung.

Architecture Release

2.0

Stabile Architekturdefinition.

Alpha

2.x-alpha

Core Runtime verfügbar.

Beta

2.x-beta

Globus OS Integration und Security verfügbar.

Release Candidate

2.x-rc

Production Hardening.

Stable

3.0.0

Production-Ready Shell.


---

17. Milestone Overview

Milestone	Ziel	Priorität	Status

M0	Architecture Foundation	P0	🧪
M1	Shell Foundation	P0	📋
M2	Command System	P1	📋
M3	Globus OS Integration	P1	📋
M4	Security	P1	📋
M5	Extension Platform	P2	📋
M6	AI / Agents	P3	📋
M7	Diagnostics	P1-P3	📋
M8	Production Hardening	P0-P3	📋
M9	Stable Release	P0	📋



---

18. Definition of Done

Eine Roadmap-Phase gilt erst als abgeschlossen, wenn:

[ ] Komponenten implementiert
[ ] ATCLang Parser erfolgreich
[ ] APIs definiert
[ ] Dependencies definiert
[ ] Fehlerbehandlung vorhanden
[ ] Security geprüft
[ ] Unit Tests erfolgreich
[ ] Integration Tests erfolgreich
[ ] Dokumentation aktualisiert
[ ] FILE_REGISTER.md aktualisiert
[ ] CHANGELOG.md aktualisiert
[ ] STATUS.md aktualisiert


---

19. Current Status

Project:
ATC Globus Shell

Version:
2.0

Architecture:
Defined

Component Plan:
Defined

File Register:
Defined

ATC License Architecture:
Defined

Implementation:
Pending

Testing:
Pending

Production:
Pending

Aktueller Projektstatus:

🏗️ ARCHITECTURE EXPANSION


---

20. Target Architecture

Die langfristige Zielarchitektur:

┌──────────────────────┐
                         │        USER          │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │      TERMINAL        │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │      CLI INPUT       │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │      TOKENIZER       │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │       PARSER         │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │     COMMAND AST      │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │    SHELL RUNTIME     │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │  COMMAND EXECUTOR    │
                         └──────────┬───────────┘
                                    │
                         ┌──────────┴───────────┐
                         │                      │
                         ▼                      ▼
                  ┌──────────────┐       ┌──────────────┐
                  │   SECURITY   │       │   BUILTINS   │
                  └──────┬───────┘       └──────┬───────┘
                         │                      │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │      I/O / IPC       │
                         └──────────┬───────────┘
                                    │
                         ┌──────────┴───────────┐
                         │                      │
                         ▼                      ▼
                  ┌──────────────┐       ┌──────────────┐
                  │ GLOBUS API   │       │   SERVICES   │
                  └──────┬───────┘       └──────┬───────┘
                         │                      │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │      GLOBUS OS       │
                         └──────────────────────┘


---

21. Roadmap Governance

Die Roadmap wird gemeinsam mit folgenden Dokumenten gepflegt:

ARCHITECTURE.md
COMPONENT_PLAN.md
FILE_REGISTER.md
STATUS.md
CHANGELOG.md
LICENSE

Änderungsfluss:

Architecture Decision
        ↓
Roadmap
        ↓
Component Plan
        ↓
File Register
        ↓
Implementation
        ↓
Testing
        ↓
Status
        ↓
Changelog


---

22. Long-Term Vision

Die ATC Globus Shell soll langfristig mehr als eine klassische CLI sein.

Sie soll die standardisierte Steuerungsschicht zwischen:

User
Applications
Processes
Services
AI Agents
Plugins
ATC Systems
Globus OS

bilden.

Langfristiges Ziel:

ATC Globus Shell
        │
        ├── Human Interface
        ├── Process Interface
        ├── Service Interface
        ├── System Interface
        ├── Security Interface
        ├── Plugin Interface
        ├── AI Interface
        └── ATC Interface

Damit wird atc-globus-shell zu einer zentralen Systemkomponente innerhalb der Globus-OS-Architektur.


---

ATC Globus Shell · Roadmap v2.0 · 2026-08-23

**Wichtige Korrektur:** In deinem bisherigen Plan steht einmal `src/stderr.atc`; das sollte konsistent **`src/stderr.atc`** heißen. Die Roadmap oben verwendet diese Schreibweise.0