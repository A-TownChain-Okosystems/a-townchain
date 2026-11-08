# 📁 File Register — atc-globus-shell

> **Version:** 2.0
> **Stand:** 2026-08-23
> **Projekt:** ATC Globus Shell
> **Plattform:** Globus OS
> **Sprache:** ATCLang v0.3
> **Status:** Architecture Expansion

---

## 1. Documentation

| Datei | Typ | Status |
|---|---|---|
| README.md | Documentation | ✅ |
| ARCHITECTURE.md | Architecture | ✅ |
| COMPONENT_PLAN.md | Architecture | ✅ |
| ROADMAP.md | Planning | ✅ |
| STATUS.md | Status | ✅ |
| CHANGELOG.md | Changelog | ✅ |
| FILE_REGISTER.md | Registry | ✅ |

---

## 2. Core / Runtime

| Datei | Typ | Priorität | Status |
|---|---|---:|---|
| src/shell.atc | ATCLang | P0 | 📋 |
| src/shell_runtime.atc | ATCLang | P0 | 📋 |
| src/shell_session.atc | ATCLang | P0 | 📋 |
| src/shell_state.atc | ATCLang | P0 | 📋 |
| src/environment.atc | ATCLang | P0 | 📋 |

---

## 3. CLI / Input

| Datei | Typ | Priorität | Status |
|---|---|---:|---|
| src/cli_commands.atc | ATCLang | P0 | 📋 |
| src/cli_tokenizer.atc | ATCLang | P0 | 📋 |
| src/cli_parser.atc | ATCLang | P0 | 📋 |
| src/shell_history.atc | ATCLang | P1 | 📋 |
| src/shell_completion.atc | ATCLang | P1 | 📋 |

---

## 4. Command Execution

| Datei | Typ | Priorität | Status |
|---|---|---:|---|
| src/command_executor.atc | ATCLang | P0 | 📋 |
| src/process_manager.atc | ATCLang | P1 | 📋 |
| src/job_control.atc | ATCLang | P1 | 📋 |
| src/exit_codes.atc | ATCLang | P1 | 📋 |

---

## 5. I/O

| Datei | Typ | Priorität | Status |
|---|---|---:|---|
| src/pipe_system.atc | ATCLang | P0 | 📋 |
| src/stream.atc | ATCLang | P0 | 📋 |
| src/stdin.atc | ATCLang | P0 | 📋 |
| src/stdout.atc | ATCLang | P0 | 📋 |
| src/stderr.atc | ATCLang | P0 | 📋 |
| src/redirection.atc | ATCLang | P1 | 📋 |

---

## 6. Built-in Commands

| Datei | Typ | Priorität | Status |
|---|---|---:|---|
| src/builtin.atc | ATCLang | P1 | 📋 |
| src/filesystem_commands.atc | ATCLang | P1 | 📋 |
| src/process_commands.atc | ATCLang | P1 | 📋 |
| src/system_commands.atc | ATCLang | P1 | 📋 |
| src/shell_commands.atc | ATCLang | P1 | 📋 |

---

## 7. Globus OS Integration

| Datei | Typ | Priorität | Status |
|---|---|---:|---|
| src/globus_api.atc | ATCLang | P1 | 📋 |
| src/ipc.atc | ATCLang | P1 | 📋 |
| src/service_manager.atc | ATCLang | P2 | 📋 |
| src/device_manager.atc | ATCLang | P2 | 📋 |
| src/session_manager.atc | ATCLang | P2 | 📋 |

---

## 8. Security

| Datei | Typ | Priorität | Status |
|---|---|---:|---|
| src/permissions.atc | ATCLang | P1 | 📋 |
| src/capability.atc | ATCLang | P1 | 📋 |
| src/sandbox.atc | ATCLang | P2 | 📋 |
| src/authentication.atc | ATCLang | P2 | 📋 |
| src/audit.atc | ATCLang | P2 | 📋 |

---

## 9. Configuration

| Datei | Typ | Priorität | Status |
|---|---|---:|---|
| src/shell_config.atc | ATCLang | P1 | 📋 |
| src/profiles.atc | ATCLang | P1 | 📋 |
| src/aliases.atc | ATCLang | P1 | 📋 |
| src/variables.atc | ATCLang | P1 | 📋 |
| src/themes.atc | ATCLang | P3 | 📋 |

---

## 10. Plugin / Extension System

| Datei | Typ | Priorität | Status |
|---|---|---:|---|
| src/plugin_loader.atc | ATCLang | P2 | 📋 |
| src/plugin_registry.atc | ATCLang | P2 | 📋 |
| src/command_registry.atc | ATCLang | P2 | 📋 |
| src/shell_hooks.atc | ATCLang | P3 | 📋 |

---

## 11. AI / Agent Integration

| Datei | Typ | Priorität | Status |
|---|---|---:|---|
| src/agent_bridge.atc | ATCLang | P3 | 📋 |
| src/ai_command.atc | ATCLang | P3 | 📋 |
| src/context.atc | ATCLang | P3 | 📋 |
| src/tool_bridge.atc | ATCLang | P3 | 📋 |

---

## 12. Diagnostics

| Datei | Typ | Priorität | Status |
|---|---|---:|---|
| src/logging.atc | ATCLang | P1 | 📋 |
| src/errors.atc | ATCLang | P0 | 📋 |
| src/diagnostics.atc | ATCLang | P2 | 📋 |
| src/telemetry.atc | ATCLang | P3 | 📋 |

---

## 13. Tests

| Datei | Typ | Priorität | Status |
|---|---|---:|---|
| tests/parser_tests.atc | Test | P0 | 📋 |
| tests/execution_tests.atc | Test | P0 | 📋 |
| tests/io_tests.atc | Test | P1 | 📋 |
| tests/security_tests.atc | Test | P1 | 📋 |
| tests/integration_tests.atc | Test | P1 | 📋 |

---

## 14. Status Definitions

| Symbol | Status |
|---|---|
| 📋 | GEPLANT |
| 🔨 | IN ENTWICKLUNG |
| 🧪 | TESTING |
| ✅ | IMPLEMENTIERT |
| 🔒 | STABIL |
| ❌ | BLOCKIERT |

---

## 15. Registration Rules

Jede neue Projektdatei muss vor der Implementierung registriert werden.

Eine Datei gilt erst als `IMPLEMENTIERT`, wenn:

- ATCLang v0.3 Parser erfolgreich
- Type-Signatures vorhanden
- API definiert
- Dependencies definiert
- Fehlerbehandlung implementiert
- Security geprüft
- Tests vorhanden
- Tests erfolgreich
- Dokumentation aktualisiert

---

## 16. Architecture Synchronization

Diese Datei muss synchron gehalten werden mit:

- COMPONENT_PLAN.md
- ARCHITECTURE.md
- STATUS.md
- CHANGELOG.md
- README.md

Änderungsfluss:

Architecture
↓
Component Plan
↓
File Register
↓
Implementation
↓
Tests
↓
Status

---

*ATC Globus Shell · File Register v2.0 · 2026-08-23*