---
document_id: ATC-DOC-ARC-001
title: Repository Architecture Specification
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-07
standard: ATC-STD-MD-001
---

# Architecture Specification — a-townchain-os

## Übersicht

`a-townchain-os` bildet die Integrationsschicht Layer L7 (Domain Integration) des A-TownChain-Ökosystems. Es führt die Cargo-Workspace-Crates, Docker Launch-Services und CI/CD-Pipelines zusammen.

## Komponenten

- **Cargo Workspace:** Multi-Crate Workspace mit 19+ Rust-Crates.
- **Sync Engine:** `scripts/sync_modules.py` importiert Modul-Stände aus den Produkt-Repositories.
- **Launch Stack:** Docker Compose Umgebung für Systemdienste (Monitoring, Services).
- **CI/CD Integration:** Workflows für CodeQL, Dependency Auditing und Governance-Gates.

## Data Flow & Modul-Synchronisation

Produkt-Repositories (atc-shivacore, atc-vm, etc.) → Sync Engine → Monorepo Workspace → Test Suites → Release.


## 1.1 Daten- & Integrationsfluss (Dependency Direction)

Das Ökosystem folgt einer **gerichteten Abhängigkeitsarchitektur**. Komponenten dürfen nur in Richtung ihrer definierten Verantwortungs- und Abstraktionsebene abhängen. Rückwärtsabhängigkeiten, zyklische Abhängigkeiten und eine Verlagerung von Kernfunktionalität in die Integrationsschicht sind ausgeschlossen.

### Architekturfluss

```text
                         ┌───────────────────────────────┐
                         │        a-townchain-os         │
                         │  Orchestration / Integration  │
                         │  Compliance / Evidence       │
                         └───────────────┬───────────────┘
                                         │
                    ┌────────────────────┼────────────────────┐
                    │                    │                    │
                    ▼                    ▼                    ▼
             ┌────────────┐       ┌────────────┐       ┌────────────┐
             │ GlobusOS   │       │ Aurora AI  │       │  Genesis   │
             │ OS Layer   │       │ AI Plane   │       │   Engine   │
             └─────┬──────┘       └─────┬──────┘       └─────┬──────┘
                   │                    │                    │
                   ▼                    ▼                    │
             ┌────────────┐       ┌────────────┐             │
             │ ShivaCore  │       │ ATC AI     │             │
             │ Kernel/TCB │       │ Runtime    │             │
             └────────────┘       └────────────┘             │
                                                            │
                                                            ▼
                                                   Game / ECS / Content
```

Die Blockchain-Seite bleibt davon logisch getrennt:

```text
                    ┌────────────────────────────┐
                    │      A-TownChain L1        │
                    │    Blockchain Core (L2)    │
                    └──────────────┬─────────────┘
                                   │
                                   ▼
                    ┌────────────────────────────┐
                    │          ATC-VM (L3)       │
                    │ deterministic execution    │
                    └──────────────┬─────────────┘
                                   │
                                   ▼
                    ┌────────────────────────────┐
                    │       ATCLang / ABI        │
                    │ contracts / program model  │
                    └────────────────────────────┘
```

### Verbindliche Dependency-Regeln

| Regel | Architekturvorgabe | Prüfstatus |
|---|---|---|
| D-001 | Core-Repositories bleiben standalone-fähig | NORMATIVE |
| D-002 | `a-townchain-os` enthält keine Core-Implementierung | NORMATIVE |
| D-003 | Ecosystem darf Core-Komponenten integrieren, aber nicht deren Funktionalität duplizieren | NORMATIVE |
| D-004 | Abhängigkeiten müssen azyklisch sein | NORMATIVE |
| D-005 | Blockchain Core ist von Aurora/GlobusOS/Genesis Engine unabhängig | NORMATIVE |
| D-006 | ATC-VM ist Ausführungsschicht und kein Ersatz für Blockchain Core | NORMATIVE |
| D-007 | ShivaCore ist Kernel/TCB und keine Blockchain- oder VM-Implementierung | NORMATIVE |
| D-008 | Aurora ist Control/Intelligence Plane und kein Kernel | NORMATIVE |
| D-009 | Genesis Engine ist Game/ECS-Schicht und kein Blockchain-Core oder ATC-VM-Ersatz | NORMATIVE |
| D-010 | Cross-repository Integrationen werden über definierte Interfaces/ABIs/Contracts realisiert | NORMATIVE |
| D-011 | Cross-layer Rückwärtsabhängigkeiten benötigen explizite Architekturfreigabe | CONTROLLED |
| D-012 | Jede behauptete Implementierung muss durch Exact-SHA-Evidence verifizierbar sein | GOVERNANCE |

### 1.1.1 Kanonische Richtung

Die zentrale Regel lautet:

**Implementation → Contract → Integration → Evidence**

und nicht:

**Integration → Core-Implementation**

Damit bleibt insbesondere `a-townchain-ecosystem` bzw. `a-townchain-os` eine Integrations- und Kontrollschicht, während die fachliche Implementierung im jeweils zuständigen Standalone-Repository verbleibt.

Für die Auditierung müssen daher mindestens folgende Beziehungen separat geprüft werden:

```text
Repository
    │
    ├── owns → Source
    ├── owns → Tests
    ├── owns → Build
    ├── owns → API / ABI
    └── owns → Release Evidence
             │
             ▼
       Ecosystem Layer
             │
             ├── Integration
             ├── Compliance
             ├── Cross-System Tests
             └── Evidence Aggregation
```

**Audit-Hinweis:** Die vorstehende Darstellung ist eine Architekturdefinition. Sie behauptet nicht, dass jede Beziehung im aktuellen GitHub-Stand bereits vollständig implementiert und VERIFIED ist. Die tatsächliche Konformität ist pro Repository und Exact-SHA nachzuweisen.

### 1.1.2 Prüfbare Dependency-Invarianten

Für die maschinen- und auditierbare Prüfung gelten zusätzlich:

- **I-001 — Ownership:** Source, Tests, Build, API/ABI und Release-Evidence gehören zum jeweils zuständigen Standalone-Repository.
- **I-002 — No Duplication:** Integrations-Repositories dürfen keine konkurrierende Implementierung eines kanonischen Core-Vertrags enthalten.
- **I-003 — Acyclicity:** Die deklarierte Dependency-Graph-Struktur muss azyklisch bleiben.
- **I-004 — Contract Boundary:** Cross-repository Kommunikation erfolgt ausschließlich über den jeweils kanonischen Contract, ABI oder explizit freigegebenes Interface.
- **I-005 — Evidence Separation:** Integration/Evidence kann Implementierungsnachweise aggregieren, ersetzt aber weder Source-of-Truth noch die CI-Evidence des zuständigen Repositories.
- **I-006 — Exact-SHA:** Ein VERIFIED-Status darf ausschließlich aus Evidence für den konkret geprüften Commit abgeleitet werden.
- **I-007 — Architecture Exceptions:** Abweichungen von D-001 bis D-012 benötigen eine explizite, versionierte Architekturentscheidung und dürfen nicht stillschweigend durch Integration entstehen.

Diese Invarianten sind normative Architekturregeln; ihr tatsächlicher Erfüllungsgrad bleibt ein separater Auditbefund.

## KAI-OS — Architekturbegriff (ATC-ARCH-001)

> **KAI-OS = Cryptographic AI Operating System Architecture** — der übergeordnete
> Architekturbegriff für den gesamten kryptografisch abgesicherten AI-OS-Stack.
> KAI-OS ist KEIN separates Betriebssystem und KEIN Repository.
>
> Die sechs Kernkomponenten: ATCLang (Sprache) · ShivaCore (Kernel) · ATC-VM
> (Execution) · A-TownChain (Trust/Consensus) · Aurora OS (AI) · **GlobusOS
> (Betriebssystem)**. Dieses Repo (a-townchain-os) ist die Integrations-/Orchestrierungsschicht (L7).
>
> Standard: [ATC-ARCH-001](https://github.com/A-TownChain-Okosystems/atc-standards/blob/main/standards/architecture/ATC-ARCH-001.md)
