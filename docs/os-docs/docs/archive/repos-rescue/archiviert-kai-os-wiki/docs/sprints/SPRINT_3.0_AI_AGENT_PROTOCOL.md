# Sprint 3.0 — AI Agent Protocol (ATC-97)

> **Issue:** #80 | **Priorität:** HIGH | **Meilenstein:** MK7
> **Status:** 🔵 IN ARBEIT | **Ziel:** Q3 2026

## Overview

Implementierung des ATC-97 (Agent Interaction Protocol) als zentrales Kommunikationsprotokoll für KI-Agenten im A-TownChain OS. Standard: ATC-97. Architektur-Decision: AD-005.

## Todos

### Phase 1: Spezifikation (Woche 1-2)
- [ ] **AD-005** Architektur-Dokument erstellen
- [ ] **ATC-97** Standard Entwurf: Agent Interaction Protocol
- [ ] Kommunikationsmatrix definieren (Agent ↔ Agent, Agent ↔ Kernel, Agent ↔ Bus)
- [ ] Message-Format Spezifikation (JSON-RPC 2.0 + ATC-Erweiterungen)
- [ ] Agent Lifecycle State Machine (INIT → REGISTER → ACTIVE → IDLE → TERMINATE)
- [ ] Capability Negotiation Protocol (Agent-Skills deklarieren)
- [ ] Error Handling & Fallback Strategie

### Phase 2: Core Implementation (Woche 3-5)
- [ ] `modules/ai/agent_registry.py` — AgentRegistry Module
- [ ] AgentDiscovery Protocol (UDP Broadcast + DHT Lookup)
- [ ] TaskDelegation Protocol (Request → Accept/Reject → Execute → Complete/Fail)
- [ ] ResultCollection Aggregator (Map-Reduce Pattern)
- [ ] Multi-Agent Orchestration Engine (DAG-basierte Task-Graph-Ausführung)
- [ ] Agent Context Manager (Session State, Memory Isolation)
- [ ] Heartbeat & Health-Check Protocol

### Phase 3: AIBus Integration (Woche 6-7)
- [ ] AIBus (AD-13) ↔ AgentRegistry Bridge
- [ ] Agent Message Routing über AIBus
- [ ] NPC Agent Integration (Behavior Trees → ATC-97 Wrapper)
- [ ] Pathfinding Agent (A* → ATC-97 Delegation Pattern)
- [ ] Dialog Agent (LLM Router → ATC-97 Task Routing)
- [ ] TelemetryBus Integration (Agent Metrics Stream)

### Phase 4: Safety & Limits (Woche 8-9)
- [ ] Agent Sandbox (CPU/Memory/Time Limits per Agent)
- [ ] Agent Permission System (Read/Write/Execute Token-basiert)
- [ ] Kill-Switch Integration (Constitutional AI → Agent Termination)
- [ ] Audit-Log für alle Agent-Interaktionen (immutable, chain-verifiable)
- [ ] Rate-Limiting pro Agent (Requests/Second, Tokens/Minute)
- [ ] Recursive Delegation Prevention (Max Depth: 3)

### Phase 5: Tests & Dokumentation (Woche 10)
- [ ] Unit-Tests für AgentRegistry (≥20 Tests)
- [ ] Integration-Tests für TaskDelegation (≥10 Tests)
- [ ] E2E Test: 3-Agent Orchestrierung Szenario (NPC + Pathfinding + Dialog)
- [ ] Wiki-Kapitel 75: "ATC-97 Agent Protocol"
- [ ] ATC-97 Standard-Dokumentation im Wiki
- [ ] Developer Guide: "How to Write an A-TownChain Agent"
- [ ] API-Referenz: AgentRegistry, TaskDelegation, Orchestration

## Voraussetzungen
- ✅ AIBus (AD-13) — bereits implementiert in Sprint 2.4
- ✅ ATCLang Spec v1.0 — Sprint 2.1 abgeschlossen
- ✅ Constitutional AI Framework — im AI Kernel (modules/kernel/ai_kernel/)

## Deliverables
1. AD-005 Architektur-Dokument
2. ATC-97 Standard Spezifikation
3. `modules/ai/agent_registry.py` (Core Module)
4. `modules/ai/orchestrator.py` (Orchestration Engine)
5. 30+ Unit- und Integration-Tests
6. Wiki-Kapitel 75 + Developer Guide

## Querverweise
- [Issue #80](../issues/ISSUE_80_SPRINT_3.0_ATC-97_AGENT_INT.md)
- [Roadmap MK7](../ROADMAP.md#mk7--ai-agent-protocol--in-arbeit)
- [Sprint Roadmap](../../SPRINT_ROADMAP.md#-sprint-30--ai-agent-protocol-in-arbeit)
- [GCL v2.0 Architektur](../wiki/)

---

*Aurora Agent · 05.07.2026*
