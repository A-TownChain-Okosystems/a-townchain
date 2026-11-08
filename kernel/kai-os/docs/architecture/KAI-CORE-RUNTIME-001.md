---
document_id: ATC-DOC-CRT-001
title: KAI-OS v2.0.0 Core Runtime Specification
version: 1.0.0
status: active
owner: Michael Wroblewski
created: 2026-09-11
updated: 2026-09-11
standard: ATC-STD-MD-001
related: [ATC-STD-ENG-001, ATC-AI-GOV-001, ATC-DOC-ARC-001]
---

# KAI-OS v2.0.0 Core Runtime Specification

> **Implementierungsreihenfolge nach Systemkritikalität und Abhängigkeiten — nicht nach Convenience.**
> Owner-Direktive Michael Wroblewski, 11.09.2026.

## 1. Priorisierungsmatrix

| Priorität | Bereich | Ziel |
|---|---|---|
| **P0** | Node Runtime / Daemon | Ein stabiler, überwachbarer KAI-OS-Prozess |
| **P0** | Sandbox / Resource Isolation | KI und Contracts dürfen das System nicht destabilisieren |
| **P0** | P2P + State Sync | Nodes müssen zuverlässig miteinander kommunizieren und synchronisieren |
| **P0** | Key Management | Private Keys dürfen niemals direkt aus Applikationen erreichbar sein |
| **P1** | AI-Agent Runtime + IPC Bus | Sichere Ausführung und Kommunikation mehrerer Agenten |
| **P1** | Immutable Audit/Logging | Nachvollziehbarkeit von System-, Security- und AI-Aktionen |
| **P1** | Model Repository / Cache | Verifizierte und reproduzierbare AI-Modelle |
| **P1** | CLI | Einheitliche Operations-/Developer-Schnittstelle |
| **P2** | ATC-Paketmanager | Modulare Erweiterbarkeit des gesamten Systems |

## 2. Zielarchitektur kai-os-node

```
kai-os
│
├── node/          # Daemon, Supervisor, Lifecycle, Health, Config, Shutdown
├── runtime/       # Sandbox, Resources, Scheduler
├── network/       # P2P, Discovery, Transport, Sync
├── security/      # Keyring, Permissions, Audit
├── ai/            # Agent-Runtime, IPC, Model-Registry
└── cli/           # Client der Control-API (nicht eigene Logik)
```

**Der Node Daemon ist Orchestrierungsebene, nicht Implementierungsort.**

```
KAI-OS Node
     │
     ├── Node Supervisor
     │      ├── Consensus
     │      ├── P2P
     │      ├── State Sync
     │      ├── AI Runtime
     │      ├── Storage
     │      └── Security
     │
     ├── Resource Manager
     ├── Health Monitor
     ├── Audit Logger
     └── IPC Bus
```

## 3. Ausführungskette (verbindlich)

> **Policy → Sandbox → Capability → Execution**

Der Daemon startet KEINE beliebigen Prozesse mit Root-/Host-Rechten.

> **KI erkennt → Policy entscheidet → System führt aus.**

## 4. Sandbox / Capability Model

Ein AI-Agent erhält niemals unbegrenzte Ressourcen. Stattdessen:

```
Agent
 │
 ▼
Capability Policy
 │
 ├── CPU quota
 ├── Memory quota
 ├── Storage quota
 ├── Network policy
 ├── syscall policy
 └── cryptographic capability
 │
 ▼
Sandbox
 │
 ▼
Execution
```

Langfristig: **WASM/Capability-basierte Isolation** als primäre portable Ausführungsschicht; native Isolation wo für Node-/Systemkomponenten erforderlich.

## 5. P2P + State Sync (Security Boundary)

```
Peer Discovery → Secure Transport → Peer Authentication → Consensus Networking
     → Block/State Sync → Snapshot → Verification
```

**Snapshot-Prinzip:** Ein Snapshot ist niemals „Ich vertraue dem Snapshot", sondern
„Ich verifiziere den Snapshot gegen einen kryptographisch definierten Chain-/State-Root."

State Sync und Snapshot Verification sind Security Boundaries.

## 6. AI Runtime + IPC Bus

```
AI Agent Runtime
       │
       ├── Agent Identity
       ├── Capability Model
       ├── Lifecycle
       ├── Resource Limits
       ├── IPC
       ├── Model Loading
       ├── Model Verification
       └── Audit
```

**IPC-Bus ist KEIN Message-Queue-System**, sondern eine standardisierte KAI-OS-Schnittstelle:

```
Agent A
   │  signed message
   ▼
KAI IPC Bus
   ├── identity verification
   ├── authorization
   ├── schema validation
   ├── replay protection
   └── audit
   │
   ▼
Agent B
```

Blockchain-, Wallet-, Storage- und OS-Komponenten kommunizieren über dieselbe Capability-/IPC-Architektur.

## 7. Keyring (Key-Isolation)

```
Application
    │  sign(request)
    ▼
KAI Keyring Daemon
    │
    ├── Policy
    ├── Authentication
    ├── Key isolation
    ├── Hardware-backed keys
    ├── Multi-signature
    └── Audit
    │
    ▼
Cryptographic Provider
```

**Private Keys verlassen diese Boundary niemals.**

Die Anwendung bekommt: `signature = keyring.sign(hash)`
Die Anwendung bekommt NIEMALS: `private_key = keyring.get_private_key()`

## 8. Audit-System

```
KAI Event → Structured Event → Audit Pipeline → Integrity Hash
    → Append-only Storage → Optional ATC anchoring
```

Getrennte Ereignisklassen: Debug-Logs · System-Logs · Security Events ·
AI Decision Records · Consensus Events · Governance Events.

**Nicht alles geht auf die Blockchain. Integritätsverankerung statt vollständiger On-Chain-Protokollierung.**

## 9. CLI als Client

```
CLI → KAI-OS Control API → Services → Kernel/Runtime/ATC
```

Die CLI ist austauschbar, da sie nur Client der APIs ist.

## 10. ATC-Paketmanager (signiertes Modulmodell)

```
ATC Module
├── Module ID          ├── Code hash
├── Version            ├── Metadata
├── Dependencies       ├── Signature
├── Capability reqs    └── Compatibility declaration
├── ABI/API
```

Installationskette: Signature Verification → Hash Verification → Dependency Resolution
→ Compatibility Check → Capability Analysis → Policy Approval → Install.

KEIN npm/cargo-Klon — signierte ATC-Module passend zur Governance-Architektur.

## 11. Core Runtime Track (S01–S26)

| Sprint | Block |
|---|---|
| S01–S03 | Node Daemon + Supervisor + Lifecycle |
| S04–S06 | Resource Manager + Sandbox |
| S07–S09 | P2P Transport + Discovery + Peer Security |
| S10–S12 | State Sync + Snapshots + Verification |
| S13–S15 | AI Agent Runtime + IPC |
| S16–S18 | Keyring + Capability Security |
| S19–S20 | Immutable Audit / Event System |
| S21–S22 | Model Registry + Verified Cache |
| S23–S24 | kai-os CLI + Control API |
| S25 | ATC Module/Package Manager |
| S26 | Production Readiness Gate |

## 12. GATE-KAI-001 — Production Readiness Gate

Nicht „Build funktioniert", sondern:

```
GATE-KAI-001
├── Reproducible Build
├── Security Audit
├── Sandbox Escape Tests
├── Resource Exhaustion Tests
├── P2P Fault Tests
├── State-Sync Recovery Tests
├── Key Isolation Tests
├── Agent Isolation Tests
├── Crash Recovery Tests
├── Upgrade/Rollback Tests
├── Snapshot Verification Tests
└── Disaster Recovery Test
```

## 14. Language & Execution Boundary (AD-008)

Verbindliche Sprach- und Ausführungsgrenze: **[AD-008](AD-008.md)** — Language & Execution
Architecture Decision (Owner-freigegeben 12.09.2026, ersetzt Rule 0 „Alles ist ATCLang").

> **ATCLang definiert deterministische, konsensrelevante Ausführung.
> Rust implementiert die vertrauenswürdige Trägerschicht.
> Der ATVM ist die kontrollierte Ausführungsgrenze.**

Kernprinzipien (AD-008.1–008.5): On-chain Canonicality · Native Infrastructure ·
VM Boundary · No Native Consensus Bypass · Deterministic Verification.

AI-Regel: **AI may propose. ATCLang specifies. ATVM executes. ATC commits.**

Durchsetzung: **GATE-008 — Language Boundary Compliance** (8 automatisierbare Checks, s. AD-008 §8).

## 13. Kernentscheidung

> Node Runtime + Sandbox + P2P/State-Sync werden als zusammenhängender
> **Core-Runtime-Block** spezifiziert und implementiert. Nicht zuerst CLI,
> nicht zuerst Package Manager, nicht zuerst ein großes AI-Agent-Framework.

Sind diese drei Fundamentbereiche stabil, setzen AI-Agenten, Wallet, Storage,
Smart Contracts, CLI und Module als Services darauf auf. KAI-OS v2.0.0 wird damit
zu einem Runtime-Stack — nicht zu einem Monorepo, das Komponenten bündelt.

---
*Verbindliche Direktive des Owners. Änderungen nur via SCR (atc-standards).*
