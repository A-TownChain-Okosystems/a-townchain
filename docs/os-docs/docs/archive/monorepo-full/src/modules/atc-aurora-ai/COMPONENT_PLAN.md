# 📋 Komponenten-Plan — atc-aurora-ai

> **Erstellt:** 2026-08-06 | **Agent:** Aurora (MasterBrain · Base44)

## Übersicht

**Repo:** atc-aurora-ai
**Name:** Aurora AI — KI-Kernschicht
**Beschreibung:** Aurora AI Core — die zentrale KI-Schicht des A-TownChain Ökosystems. Agent-Runtime, LLM-Orchestrierung, Multi-Agent-Koordination, Memory, Tool-Use, Model-Hub. Bündelt die verstreuten AI-Module (core/ai/, ai_kernel, kai_routes) in einem dedizierten Repo. Implementiert die 12 Agenten-Rollen aus Regel 11.
**Layer:** L6 — AI Layer
**Sprint:** 3.2
**ATC-Standards:** ATC-97, ATC-24, ATC-17, ATC-45

---

## Komponenten (17 total)

### 1. `core/aurora_core.atc`

**Beschreibung:** Zentrale AI-Initialisierung

**Funktionen:** AuroraConfig, init, status, shutdown

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 2. `core/agent_registry.atc`

**Beschreibung:** Agent-Typ-Registrierung (12 Rollen)

**Funktionen:** register, get, list, 12 roles

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 3. `core/model_hub.atc`

**Beschreibung:** LLM-Modell-Registrierung und -Routing

**Funktionen:** register, list, route, compare

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 4. `runtime/llm_router.atc`

**Beschreibung:** LLM-Request-Routing

**Funktionen:** Route, Fallback, Load-Balancing

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 5. `runtime/agent_runtime.atc`

**Beschreibung:** Agenten-Ausführungsumgebung

**Funktionen:** Session, Task, Lifecycle

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 6. `runtime/tool_executor.atc`

**Beschreibung:** Sichere Tool-Ausführung

**Funktionen:** register, execute, validate, history

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 7. `agents/knowledge_agent.atc`

**Beschreibung:** Knowledge Agent (Wissen)

**Funktionen:** ingest, query, nodes, conflicts

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 8. `agents/architect_agent.atc`

**Beschreibung:** Architect Agent (Architektur)

**Funktionen:** analyze, dependencies, violations

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 9. `agents/coding_agent.atc`

**Beschreibung:** Coding Agent (ATCLang)

**Funktionen:** generate, migrate, review, fix

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 10. `agents/research_agent.atc`

**Beschreibung:** Research Agent (Recherche)

**Funktionen:** search, summarize, compare, recommend

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 11. `agents/security_agent.atc`

**Beschreibung:** Security Agent (Audit)

**Funktionen:** audit, crypto, threats, compliance

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 12. `agents/orchestrator_agent.atc`

**Beschreibung:** Orchestrator Agent (Koordination)

**Funktionen:** distribute, collect, deadlock

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 13. `memory/agent_memory.atc`

**Beschreibung:** Agenten-Gedächtnis (On-chain)

**Funktionen:** store, recall, forget, consolidate

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 14. `memory/context_window.atc`

**Beschreibung:** Kontext-Fenster-Manager

**Funktionen:** add, compress, summarize, get

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 15. `orchestration/workflow_engine.atc`

**Beschreibung:** Workflow-Engine

**Funktionen:** create, execute, pause, status

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 16. `orchestration/sync_orchestrator.atc`

**Beschreibung:** Daily Sync (16 Dienste)

**Funktionen:** daily, github, notion, google, report

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

### 17. `orchestration/decision_handler.atc`

**Beschreibung:** Decision-Verwaltung (AD-001-007)

**Funktionen:** list, propose, escalate, apply

**Status:** 🔄 STUB (ATCLang v0.3 Syntax, parst bereit)

**Akzeptanzkriterien:**
1. Datei existiert und parst mit ATCLang v0.3 Parser
2. Alle öffentlichen Funktionen haben Type-Signatures (struct-basiert)
3. Modul ist im FILE_REGISTER.md eingetragen
4. ATC-Standard-Referenz im Header

---

## Implementierungs-Reihenfolge

1. `core/aurora_core.atc` — Zentrale AI-Initialisierung
2. `core/agent_registry.atc` — Agent-Typ-Registrierung (12 Rollen)
3. `core/model_hub.atc` — LLM-Modell-Registrierung und -Routing
4. `runtime/llm_router.atc` — LLM-Request-Routing
5. `runtime/agent_runtime.atc` — Agenten-Ausführungsumgebung
6. `runtime/tool_executor.atc` — Sichere Tool-Ausführung
7. `agents/knowledge_agent.atc` — Knowledge Agent (Wissen)
8. `agents/architect_agent.atc` — Architect Agent (Architektur)
9. `agents/coding_agent.atc` — Coding Agent (ATCLang)
10. `agents/research_agent.atc` — Research Agent (Recherche)
11. `agents/security_agent.atc` — Security Agent (Audit)
12. `agents/orchestrator_agent.atc` — Orchestrator Agent (Koordination)
13. `memory/agent_memory.atc` — Agenten-Gedächtnis (On-chain)
14. `memory/context_window.atc` — Kontext-Fenster-Manager
15. `orchestration/workflow_engine.atc` — Workflow-Engine
16. `orchestration/sync_orchestrator.atc` — Daily Sync (16 Dienste)
17. `orchestration/decision_handler.atc` — Decision-Verwaltung (AD-001-007)

## Test-Strategie

1. Parse-Test: Jede .atc Datei muss mit ATCLang v0.3 Parser parsen
2. Unit-Tests: Mindestens 3 Tests pro Komponente
3. Integration-Test: Komponenten interagieren korrekt
4. Coverage-Ziel: >80%

---
*Auto-generiert 2026-08-06 · Aurora (MasterBrain · Base44)*
