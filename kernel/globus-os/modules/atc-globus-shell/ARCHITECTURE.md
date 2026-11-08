🏗️ ARCHITECTURE.md — ATC Globus Shell

> Version: 2.0
Datum: 2026-08-23
Projekt: ATC Globus Shell
Repository: atc-globus-shell
Plattform: Globus OS
Sprache: ATCLang v0.3
Status: Architecture Planning




---

1. Architekturübersicht

atc-globus-shell ist die zentrale Command-Line- und Steuerungsschicht von Globus OS.

Die Shell bildet die technische Verbindung zwischen:

Benutzer

Terminal

CLI

ATCLang Runtime

Command Parser

Command Executor

Built-in Commands

externen Prozessen

I/O-System

IPC

Globus OS Services

Security

Plugins

AI Agents


Die Architektur ist modular aufgebaut und trennt Parsing, Execution, I/O, Systemzugriff und Security.

┌─────────────────────────────────────────────────────────────┐
│                       USER / TERMINAL                       │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                       SHELL INPUT                           │
│                                                             │
│  History │ Completion │ Tokenizer │ CLI Parser              │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                         COMMAND AST                         │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                       SHELL RUNTIME                         │
│                                                             │
│ Session │ State │ Environment │ Configuration │ Errors      │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     COMMAND EXECUTOR                        │
└───────────────┬──────────────┬──────────────┬───────────────┘
                │              │              │
                ▼              ▼              ▼
        ┌────────────┐ ┌──────────────┐ ┌──────────────┐
        │  Builtins  │ │  Processes   │ │   Pipelines  │
        └────────────┘ └──────────────┘ └──────────────┘
                │              │              │
                └──────────────┼──────────────┘
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                         SECURITY                            │
│ Permissions │ Capabilities │ Sandbox │ Authentication       │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                         I/O LAYER                           │
│ Streams │ stdin │ stdout │ stderr │ Pipes │ Redirects      │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    GLOBUS OS INTERFACE                      │
│ API │ IPC │ Services │ Devices │ Sessions                   │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                         GLOBUS OS                           │
└─────────────────────────────────────────────────────────────┘


---

2. Architekturprinzipien

2.1 Separation of Concerns

Jede Komponente besitzt eine klar definierte technische Verantwortung.

Beispiel:

Tokenizer
    ↓
Parser
    ↓
AST
    ↓
Executor

Der Parser führt keine Commands aus.

Der Executor interpretiert keinen Rohtext.


---

2.2 Modularität

Die Shell besteht aus unabhängig testbaren Modulen.

Module dürfen nur über definierte APIs miteinander kommunizieren.

Module
   │
   ├── Public API
   │
   └── Internal Implementation


---

2.3 Security by Design

Security ist Bestandteil des Execution Flows und keine optionale Zusatzschicht.

Command
   ↓
Executor
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

Kein Command darf den Security Layer umgehen.


---

2.4 API First

Öffentliche Funktionen müssen klar definierte Type-Signatures besitzen.

Jede öffentliche API muss dokumentieren:

Input

Output

Fehler

Ownership

Side Effects

Permissions



---

2.5 ATCLang Native

Die Shell wird für ATCLang v0.3 entwickelt.

Alle .atc-Module müssen:

syntaktisch kompatibel sein

Type-Signatures verwenden

definierte Module Interfaces besitzen

mit dem ATCLang Compiler kompatibel sein



---

3. Layer Architecture

Die Shell wird in zwölf Hauptbereiche gegliedert.

L0  Terminal / User Interface
L1  Input / CLI
L2  Parsing / AST
L3  Shell Runtime
L4  Command Execution
L5  Builtins / Processes
L6  I/O
L7  Security
L8  Globus OS Integration
L9  Configuration / Extensions
L10 AI / Agents
L11 Diagnostics / Testing


---

4. Layer 0 — Terminal

Der Terminal Layer ist die Benutzeroberfläche der Shell.

Verantwortung:

Tastatureingabe

Terminal Output

Prompt

Cursor

Interactive Input

Terminal State


Der Terminal Layer darf keine direkte Systemausführung durchführen.


---

5. Layer 1 — Input / CLI

Komponenten:

cli_commands.atc
cli_tokenizer.atc
shell_history.atc
shell_completion.atc

Input Flow

Raw Input
   ↓
Tokenizer
   ↓
Tokens
   ↓
Parser

Der Tokenizer ist ausschließlich für lexikalische Analyse verantwortlich.


---

6. Layer 2 — Parser / AST

Der Parser erzeugt aus Tokens einen strukturierten Command AST.

Beispiel:

cat file.txt | grep test > result.txt

wird konzeptionell:

Pipeline
├── Command
│   ├── name: cat
│   └── argument: file.txt
│
└── Command
    ├── name: grep
    ├── argument: test
    └── redirect:
        └── stdout → result.txt

Der AST dient als standardisierte Übergabeschnittstelle zwischen Parser und Executor.


---

7. Layer 3 — Shell Runtime

Komponenten:

shell.atc
shell_runtime.atc
shell_session.atc
shell_state.atc
environment.atc
errors.atc

Die Runtime verwaltet:

Lifecycle

Session

State

Environment

Command Context

Fehler

Execution Context



---

8. Layer 4 — Command Execution

Zentrale Komponente:

command_executor.atc

Der Executor entscheidet anhand des AST, wie ein Command ausgeführt wird.

AST
 │
 ▼
Command Resolver
 │
 ├── Builtin
 ├── Plugin
 ├── External Process
 ├── Service
 └── AI Agent
 │
 ▼
Security
 │
 ▼
Execution


---

9. Layer 5 — Builtins und Prozesse

Builtins

builtin.atc
filesystem_commands.atc
process_commands.atc
system_commands.atc
shell_commands.atc

Builtins werden direkt innerhalb der Shell Runtime ausgeführt.

Beispiele:

cd
pwd
history
alias
set
unset
exit
help


---

External Processes

Externe Programme werden über den Process Manager gestartet.

process_manager.atc
job_control.atc
exit_codes.atc


---

10. Process Architecture

Process Manager
      │
      ├── Process Start
      ├── Process Stop
      ├── Process Kill
      ├── Process Suspend
      ├── Process Resume
      └── Process Status

Job Control verwaltet:

Foreground
Background
Job ID
Job State
Job Switching


---

11. Layer 6 — I/O Architecture

Komponenten:

stream.atc
stdin.atc
stdout.atc
stderr.atc
pipe_system.atc
redirection.atc

Grundmodell:

Process A
   │
 stdout
   │
   ▼
 Pipe
   │
 stdin
   │
   ▼
Process B


---

11.1 Standard Streams

Jeder ausführbare Command besitzt konzeptionell:

stdin
stdout
stderr


---

11.2 Pipes

Beispiel:

command1 | command2 | command3

Pipeline:

command1
   │
   ▼
pipe
   │
   ▼
command2
   │
   ▼
pipe
   │
   ▼
command3


---

11.3 Redirects

Unterstützte Grundoperationen:

command > file
command >> file
command < file


---

12. Layer 7 — Security Architecture

Security-Komponenten:

permissions.atc
capability.atc
sandbox.atc
authentication.atc
audit.atc

Security Flow:

User
 ↓
Session
 ↓
Command
 ↓
Executor
 ↓
Authentication
 ↓
Permission
 ↓
Capability
 ↓
Policy
 ↓
Sandbox
 ↓
Execution


---

12.1 Permission Model

Ein Command muss vor der Ausführung über eine Berechtigung verfügen.

Beispiel:

filesystem.read
filesystem.write
process.execute
process.kill
service.control
device.access
system.admin


---

12.2 Capability Model

Capabilities erlauben eine feinere Kontrolle als reine Benutzerrechte.

Capability
    ↓
Resource
    ↓
Operation

Beispiel:

Capability:
filesystem.read

Resource:
/system/config

Operation:
READ


---

13. Layer 8 — Globus OS Integration

Komponenten:

globus_api.atc
ipc.atc
service_manager.atc
device_manager.atc
session_manager.atc

Die Shell kommuniziert nicht direkt mit Kernel-internen Implementierungsdetails.

Stattdessen:

Shell
 ↓
Globus API
 ↓
IPC
 ↓
Globus OS Service
 ↓
Kernel / System


---

14. IPC Architecture

IPC stellt die Kommunikation zwischen Shell, Prozessen und Services bereit.

Shell
 │
 ├──────────────┐
 ▼              ▼
Process       Service
 │              │
 └──────┬───────┘
        ▼
       IPC
        │
        ▼
   Globus OS

IPC muss mindestens folgende Konzepte unterstützen:

Message

Request

Response

Channel

Endpoint

Service Identity

Error

Timeout



---

15. Service Manager

Der Service Manager kontrolliert Globus-OS-Services.

service start <name>
service stop <name>
service restart <name>
service status <name>
service enable <name>
service disable <name>

Architektur:

Shell Command
      ↓
System Commands
      ↓
Service Manager
      ↓
IPC
      ↓
Globus Service


---

16. Device Manager

Der Device Manager abstrahiert Hardwarezugriffe.

Mögliche Geräteklassen:

CPU
GPU
Memory
Storage
Network
USB
Display
Input
Audio

Hardwarezugriff muss über Security Policies kontrolliert werden.


---

17. Session Architecture

Es existieren zwei unterschiedliche Ebenen:

Shell Session
      │
      ▼
Globus OS Session

Die Shell Session enthält:

Session ID

User

Working Directory

Environment

Permissions

Capabilities

Shell State



---

18. Layer 9 — Configuration

Komponenten:

shell_config.atc
profiles.atc
aliases.atc
variables.atc
themes.atc

Konfigurationshierarchie:

System Defaults
      ↓
System Configuration
      ↓
User Profile
      ↓
Shell Profile
      ↓
Session Configuration
      ↓
Command Context

Spätere Ebenen können frühere Werte überschreiben, sofern Security Policies dies erlauben.


---

19. Plugin Architecture

Komponenten:

plugin_loader.atc
plugin_registry.atc
command_registry.atc
shell_hooks.atc

Plugin Flow:

Plugin
  ↓
Loader
  ↓
Validation
  ↓
Permission Check
  ↓
Registry
  ↓
Command / Hook

Plugins dürfen niemals ungeprüft geladen werden.


---

20. Command Registry

Die Command Registry stellt eine zentrale Discovery-Schicht bereit.

Command Registry
│
├── Builtins
├── System Commands
├── Plugin Commands
├── AI Commands
└── Future Extensions

Jeder Command sollte Metadaten besitzen:

name
description
version
arguments
options
permissions
capabilities
handler


---

21. Shell Hooks

Lifecycle Events:

on_start
on_command
on_before_execute
on_after_execute
on_error
on_exit

Hooks dürfen die Security Architecture nicht umgehen.


---

22. Layer 10 — AI / Agent Architecture

AI ist als Erweiterung der Shell konzipiert und nicht Bestandteil des minimalen Shell-Kerns.

Komponenten:

agent_bridge.atc
ai_command.atc
context.atc
tool_bridge.atc

Architektur:

Shell
  │
  ▼
AI Command
  │
  ▼
Agent Bridge
  │
  ▼
Context
  │
  ▼
Agent
  │
  ▼
Tool Bridge
  │
  ▼
Globus OS

Beispiele:

ai generate
ai analyze
ai explain
ai execute

ai execute muss dieselben Permission-, Capability- und Sandbox-Regeln verwenden wie normale Commands.


---

23. Layer 11 — Diagnostics

Komponenten:

logging.atc
errors.atc
diagnostics.atc
telemetry.atc

Diagnostics müssen zwischen folgenden Kategorien unterscheiden:

INFO
WARNING
ERROR
FATAL
DEBUG
SECURITY
AUDIT


---

24. Error Architecture

Alle Komponenten verwenden ein einheitliches Error-Modell.

Konzeptionell:

Error
├── Code
├── Category
├── Message
├── Component
├── Context
└── Recovery

Beispiel:

ERROR
Code: SHELL_COMMAND_NOT_FOUND
Component: command_executor
Recovery: command discovery


---

25. Telemetry

Telemetry ist optional und muss konfigurierbar sein.

Mögliche Daten:

Performance

Command Execution Time

Process Statistics

Resource Usage

Error Statistics


Security-relevante Daten müssen getrennt vom normalen Performance Monitoring behandelt werden.


---

26. Repository Architecture

Empfohlene Struktur:

atc-globus-shell/
│
├── src/
│   ├── shell.atc
│   ├── shell_runtime.atc
│   ├── shell_session.atc
│   ├── shell_state.atc
│   │
│   ├── cli_commands.atc
│   ├── cli_tokenizer.atc
│   ├── cli_parser.atc
│   ├── shell_history.atc
│   ├── shell_completion.atc
│   │
│   ├── command_executor.atc
│   ├── process_manager.atc
│   ├── job_control.atc
│   ├── exit_codes.atc
│   ├── environment.atc
│   │
│   ├── pipe_system.atc
│   ├── stream.atc
│   ├── stdin.atc
│   ├── stdout.atc
│   ├── stderr.atc
│   ├── redirection.atc
│   │
│   ├── builtin.atc
│   ├── filesystem_commands.atc
│   ├── process_commands.atc
│   ├── system_commands.atc
│   ├── shell_commands.atc
│   │
│   ├── globus_api.atc
│   ├── ipc.atc
│   ├── service_manager.atc
│   ├── device_manager.atc
│   ├── session_manager.atc
│   │
│   ├── permissions.atc
│   ├── capability.atc
│   ├── sandbox.atc
│   ├── authentication.atc
│   ├── audit.atc
│   │
│   ├── shell_config.atc
│   ├── profiles.atc
│   ├── aliases.atc
│   ├── variables.atc
│   ├── themes.atc
│   │
│   ├── plugin_loader.atc
│   ├── plugin_registry.atc
│   ├── command_registry.atc
│   ├── shell_hooks.atc
│   │
│   ├── agent_bridge.atc
│   ├── ai_command.atc
│   ├── context.atc
│   ├── tool_bridge.atc
│   │
│   ├── logging.atc
│   ├── errors.atc
│   ├── diagnostics.atc
│   └── telemetry.atc
│
├── tests/
│   ├── parser_tests.atc
│   ├── execution_tests.atc
│   ├── io_tests.atc
│   ├── security_tests.atc
│   └── integration_tests.atc
│
├── COMPONENT_PLAN.md
├── ARCHITECTURE.md
├── FILE_REGISTER.md
└── README.md


---

27. Dependency Architecture

Grundregel:

Terminal
   ↓
CLI
   ↓
Tokenizer
   ↓
Parser
   ↓
AST
   ↓
Runtime
   ↓
Executor
   ↓
Security
   ↓
I/O
   ↓
Globus API
   ↓
IPC
   ↓
Globus OS

Dependency Restrictions

Nicht erlaubt:

Parser → Process Manager
Parser → Globus OS
Tokenizer → Security
Terminal → Kernel
AI Agent → direkte Kernel API
Plugin → Security Bypass

Erlaubt:

Executor → Process Manager
Executor → Builtins
Executor → Security
Executor → I/O
Globus API → IPC
AI → Tool Bridge
Plugin → Command Registry


---

28. Security Boundary

Die wichtigste Trust Boundary befindet sich zwischen Shell Runtime und Systemzugriff.

┌──────────────────────── TRUSTED SHELL ───────────────────────┐
│                                                             │
│ Parser → Runtime → Executor                                │
│                                                             │
└───────────────────────────┬─────────────────────────────────┘
                            │
                       SECURITY GATE
                            │
┌───────────────────────────▼─────────────────────────────────┐
│                  PROTECTED SYSTEM                           │
│                                                             │
│ Filesystem │ Processes │ Devices │ Services │ IPC           │
│                                                             │
└─────────────────────────────────────────────────────────────┘


---

29. Execution Pipeline

Vollständiger Command Lifecycle:

1. User Input
       ↓
2. Tokenization
       ↓
3. Parsing
       ↓
4. AST Validation
       ↓
5. Command Resolution
       ↓
6. Context Construction
       ↓
7. Authentication
       ↓
8. Permission Check
       ↓
9. Capability Check
       ↓
10. Policy Check
       ↓
11. Sandbox Setup
       ↓
12. Execution
       ↓
13. I/O Processing
       ↓
14. Exit Code
       ↓
15. Logging / Audit
       ↓
16. Result


---

30. Pipeline Execution

Beispiel:

find /data | grep ".atc" | sort

Execution:

┌───────────┐
             │   find    │
             └─────┬─────┘
                   │ stdout
                   ▼
                ┌───────┐
                │ Pipe  │
                └───┬───┘
                    │ stdin
                    ▼
             ┌───────────┐
             │   grep    │
             └─────┬─────┘
                   │ stdout
                   ▼
                ┌───────┐
                │ Pipe  │
                └───┬───┘
                    │ stdin
                    ▼
             ┌───────────┐
             │   sort    │
             └───────────┘

Jeder Prozess besitzt einen eigenen Execution Context.


---

31. Configuration Flow

Default Configuration
        ↓
System Configuration
        ↓
User Profile
        ↓
Shell Profile
        ↓
Environment
        ↓
Session State
        ↓
Command Context


---

32. Plugin Security Flow

Plugin File
    ↓
Load
    ↓
Signature / Metadata Validation
    ↓
Compatibility Check
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


---

33. AI Security Flow

AI-generierte Commands werden niemals automatisch als vertrauenswürdig behandelt.

AI Request
    ↓
Agent
    ↓
Generated Action
    ↓
Command Parser
    ↓
Command Executor
    ↓
Security Gate
    ↓
Permission / Capability
    ↓
Sandbox
    ↓
Execution

Damit unterliegt AI denselben Sicherheitsregeln wie ein menschlicher Benutzer.


---

34. Testing Architecture

Tests werden auf mehreren Ebenen durchgeführt.

Unit Tests
    ↓
Component Tests
    ↓
Integration Tests
    ↓
Security Tests
    ↓
End-to-End Tests

Testbereiche

Parser

Tokenisierung

Quotes

Escaping

Operators

Pipes

Redirects

Syntax Errors


Execution

Builtins

External Processes

Exit Codes

Jobs

Pipelines


I/O

stdin

stdout

stderr

Pipes

Redirects


Security

Permissions

Capabilities

Sandbox

Authentication

Unauthorized Commands


Integration

Shell
 ↕
Globus API
 ↕
IPC
 ↕
Globus OS


---

35. Entwicklungsphasen

Phase 1 — Shell Foundation

Priorität: P0

Shell
Runtime
Session
Tokenizer
Parser
Executor
Environment
Errors
Streams
stdin
stdout
stderr

Ziel: Minimal funktionsfähige Shell.


---

Phase 2 — Command System

Priorität: P1

Builtins
Filesystem Commands
Process Manager
Job Control
History
Completion
Pipes
Redirects
Exit Codes

Ziel: Vollwertige interaktive CLI.


---

Phase 3 — Globus OS Integration

Priorität: P1/P2

Globus API
IPC
Service Manager
Device Manager
Session Manager

Ziel: Steuerung von Globus OS.


---

Phase 4 — Security

Priorität: P1/P2

Permissions
Capabilities
Sandbox
Authentication
Audit

Ziel: Production-taugliche Security Architecture.


---

Phase 5 — Extension Platform

Priorität: P2/P3

Plugin Loader
Plugin Registry
Command Registry
Hooks
Themes

Ziel: Erweiterbare Shell-Plattform.


---

Phase 6 — AI / Agents

Priorität: P3

Agent Bridge
AI Commands
Context
Tool Bridge

Ziel: AI-native Globus Shell.


---

Phase 7 — Production Hardening

Priorität: P0–P3

Testing
Security Testing
Diagnostics
Performance
Documentation
Telemetry

Ziel: Production-Ready Release.


---

36. Definition of Done

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

37. Statusmodell

📋 GEPLANT
    ↓
🔨 IN ENTWICKLUNG
    ↓
🧪 TESTING
    ↓
✅ IMPLEMENTIERT
    ↓
🔒 STABIL

Fehler-/Blockierungsstatus:

❌ BLOCKIERT


---

38. Architecture ↔ Component Plan

ARCHITECTURE.md und COMPONENT_PLAN.md müssen synchron gehalten werden.

COMPONENT_PLAN.md
        │
        │ definiert
        ▼
Komponenten
        │
        ▼
ARCHITECTURE.md
        │
        │ definiert
        ▼
Abhängigkeiten / Layer / Datenfluss
        │
        ▼
Implementation

Regel

Keine neue Core-Komponente darf implementiert werden, ohne:

1. Eintrag im COMPONENT_PLAN.md


2. Architekturzuordnung


3. definierte Abhängigkeiten


4. API Definition


5. Teststrategie




---

39. Architektur-Governance

Änderungen an der Architektur müssen nachvollziehbar sein.

Bei strukturellen Änderungen:

Architecture Change
       ↓
COMPONENT_PLAN Update
       ↓
ARCHITECTURE Update
       ↓
FILE_REGISTER Update
       ↓
Implementation
       ↓
Tests


---

40. Gesamtziel

Die Zielarchitektur von atc-globus-shell ist:

ATC GLOBUS SHELL
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
       CLI               Runtime            AI
        │                  │                  │
        └──────────────────┼──────────────────┘
                           │
                      EXECUTION
                           │
             ┌─────────────┼─────────────┐
             │             │             │
          Builtins      Processes      Plugins
             │             │             │
             └─────────────┼─────────────┘
                           │
                        SECURITY
                           │
                         I/O
                           │
                        IPC/API
                           │
                      GLOBUS OS

Architekturziel: Eine modulare, sichere, erweiterbare und ATCLang-native Shell Runtime, die als zentrale CLI-Steuerungsschicht von Globus OS fungiert.


---

ATC Globus Shell · Architecture v2.0 · 2026-08-23