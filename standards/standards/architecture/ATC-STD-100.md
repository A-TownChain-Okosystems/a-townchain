---
standard:
  id: ATC-STD-100
  title: "Language & Technology Stack Standard"
  version: "2.0.0"
  status: approved
  category: architecture
  authority: "A-TownChain Ecosystems"
  owner: "Michael Wroblewski (Owner-Entwurf) / Standards Governance"
  created: "2026-09-07"
  updated: "2026-09-11"
  normative: true
  applies_to: "Alle ATC-Repositories"
  mandate: "Owner-Entwurf 11.09.2026 (Language Architecture, SCR-0093)"
----

# ATC-STD-100 — Language & Technology Stack Standard (v2.0.0, APPROVED)

> **Status:** APPROVED (v2.0.0) — Owner-Entwurf Language Architecture 11.09.2026
> (SCR-0093): Hierarchische Language Architecture L0-L8 mit Entscheidungskaskade
> und Repo-Matrix; ersetzt das 5-Layer-Modell von v1.0.0 (Mapping §7). par.9-freigegeben 11.09.2026 (Owner-Direktive, SCR-0102) — v2.0.0 APPROVED als MAJOR (Migration par.7 dokumentiert, COMPAT-001). **Scope:** ATC-STD-100 · Sprache- und
> Technologieauswahl aller Repositories · **Governance:** ATC-STD-000

## Abstract

ATC-STD-100 definiert die verbindliche Language Architecture des
A-TownChain-Ökosystems: keine zufällige Polyglot-Architektur, sondern eine
hierarchisch definierte Sprach- und Technologieauswahl je Layer und Repository.
Kern: Rust als Systemsprache (Security-/Consensus-/Execution-TCB), ATCLang als
native Anwendungssprache der Plattform (ATC-99), TypeScript als UI-/Application-
Sprache, Python als AI-/Research-Sprache (mit TCB-Verbot), C/Assembly als
Hardware-Grenzsprachen, WASM als portables Execution Target, SQL als Query-/
Persistence-Layer (nicht Consensus Truth).

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC 2119 gemäß ATC-STD-000 §10.

## 1. Language Architecture L0-L8 (REQ-STD-105: MUST)

Die Ökosystem-Architektur MUSS dem folgenden Layer-Modell folgen; jede
Komponente MUSS ihrem Layer zugeordnet sein:

| Layer | Bereich | Sprachen |
|---|---|---|
| L0 — Hardware | Firmware, Boot, CPU-Schnittstellen | Assembly + C |
| L1 — Kernel | ShivaCore-Kernel, Memory, Capabilities | Rust |
| L2 — Runtime | ATC Runtime, ATC-VM, Deterministic Execution | Rust |
| L3 — Blockchain | Node, Consensus, Networking, Kryptografie, ZKP, Storage | Rust |
| L4 — Native ATC Programming | Smart Contracts, On-chain Logic, ATC-Programme | ATCLang |
| L5 — AI | Modelle, Research, Agents | Python + Rust |
| L6 — SDK | Client-Libraries, Bindings | Rust + TypeScript (+ Python SDK) |
| L7 — Applications | Wallet, Explorer, Marketplace, OS-UI | TypeScript + ATCLang |
| L8 — Data | Indexer, Analytics, Off-chain State | SQL + Rust/Python |

Kette: ShivaCore → Rust → ATC Runtime → ATC-VM → A-TownChain Node → Consensus.

## 2. Sprachbindungen (REQ-STD-106: MUST)

- **Rust** MUSS die zentrale Systemsprache sein: Kernel, Security, Consensus,
  VM, Kryptografie, ZKP, Storage, Node. Alles innerhalb des Security-/
  Consensus-/Execution-Trusted-Computing-Bereichs MUSS standardmäßig Rust sein
  (Memory Safety, kein GC, no_std, deterministische Systeme).
- **ATCLang** MUSS die native Anwendungssprache der Plattform sein (ATC-99
  ATCLang First). Trennung: Rust = Implementierung der Plattform, ATCLang =
  Programmierung für die Plattform (Application → ATCLang → ATC Compiler →
  ATC Bytecode → ATC-VM → Rust → A-TownChain). ATCLang MUSS NICHT Rust
  ersetzen; der ATCLang-Compiler/Toolchain selbst SOLLTE Rust-first sein (AD-021).
- **C** DARF nur an Hardwaregrenzen eingesetzt werden: Firmware, Bootloader-
  Komponenten, Hardware Interfaces, Vendor SDKs, C-ABI/FFI. Begründung MUSS
  technisch sein ("die Hardware-/ABI-Schnittstelle erfordert C"), NICHT "C ist
  schnell".
- **Assembly** DARF nur für die letzten 1 % verwendet werden: Boot Entry,
  CPU-Initialization, Interrupt/Trap Entry, Context Switching, Register-
  operationen, Architektur-Optimierungen (x86_64, AArch64). Assembly MUSS
  NICHT in normale Business Logic einfließen.
- **C++** DARF nur bei zwingend benötigten existierenden High-Performance-/
  GPU-/Krypto-/ZKP-Libraries und externen Engines eingesetzt werden; für neue
  ATC-Kernkomponenten MUSS Rust gewählt werden.
- **Python** MUSS für AI/ML, Research, Prototyping, Testautomation, Tooling,
  Build-Skripte, Repository-Audits, CI-Utilities eingesetzt werden. Python
  MUSS NICHT Konsens oder Kernel-Sicherheit kontrollieren: Python darf AI
  entwickeln und orchestrieren, aber nicht automatisch den TCB (L1-L3).
- **TypeScript** MUSS die Standard-Frontend-/UI-Sprache sein: Wallet, Explorer,
  Marketplace, Launchpad, GlobusOS-UI, Admin-Interfaces, Developer Portals.
  Kette: React → TypeScript → ATC SDK → Rust/WASM → ATC Node.
- **JavaScript** SOLLTE vermieden werden (nur Legacy-/Runtime-Zwang).
- **WASM** IST Deployment-/Execution Target (keine Primärsprache): Browser,
  portable Tools, Plugins, Sandbox, SDK-Komponenten. ATC Bytecode und WASM
  MÜSSEN NICHT ungeprüft gleichgesetzt werden — das ATC-Execution-Modell
  behält seine eigenen Semantics.
- **SQL** DARF für Query-/Index-/Persistence-Layer eingesetzt werden (Indexer,
  Explorer, Analytics); der kanonische Blockchain-State MUSS NICHT mit SQL
  gleichgesetzt werden — SQL ist nicht Consensus Truth.
- **Go** DARF nur bei konkret nachgewiesenem Vorteil für Infrastruktur-
  Services/Networking eingesetzt werden; eine zusätzliche Sprache ohne
  technischen Grund MUSS NICHT eingeführt werden.

## 3. Auswahl-Entscheidungskaskade (REQ-STD-107: MUST)

Jede neue Komponente MUSS der Kaskade folgen (erste JA-Antwort gewinnt):

1. Security/Kernel/Consensus/Runtime? → JA → **Rust**
2. Hardware/CPU/ABI-Boundary? → JA → **C / Assembly**
3. On-chain Application? → JA → **ATCLang**
4. AI/ML/Research? → JA → **Python**
5. Web/UI? → JA → **TypeScript**
6. Database-Query? → JA → **SQL**
7. Kein Treffer → Entscheidung via SCR + ATC-STD-TUD-001 (Technology-Registry)

Abweichungen von der Kaskade MÜssen als Finding (ATC-STD-BUG-001) oder
Exception (ATC-STD-EXC-Familie) dokumentiert sein.

## 4. Repository-Matrix (REQ-STD-108: MUST)

| Repository | Primär | Sekundär |
|---|---|---|
| atc-shivacore | Rust | Assembly, C (Boot/HAL) |
| atclang | Rust (Compiler/Toolchain) | ATCLang (Zielsprache) |
| atc-vm | Rust | Python (nur Referenz, nie TCB) |
| a-townchain | Rust | Rust/Python (Tests/Tooling) |
| atc-node | Rust | — |
| atc-zkp | Rust | Python (Research) |
| atc-algorithm | Rust | Python (Benchmark/Research) |
| atc-storage | Rust | SQL (Query) |
| atc-compute | Rust | Python (Workloads/Research) |
| atc-oracle | Rust | Adapter je externem System |
| atc-interop | Rust | Adapter je Protokoll |
| atc-wallet | Rust (Core) | TypeScript (UI), WASM (Browser) |
| atc-indexer | Rust (Core) | SQL (Database) |
| atc-explorer | TypeScript (Frontend) | Rust oder TypeScript (Backend) |
| atc-marketplace | TypeScript (Frontend) | ATCLang (On-chain Logic) |
| atc-launchpad | TypeScript + ATCLang | Rust/TypeScript (Backend) |
| atc-contracts | ATCLang | — |
| aurora-ai | Python (Research/AI) | Rust (Production Runtime), TypeScript (UI) |
| globus-os | Rust (System) | TypeScript (UI); Kernel: ShivaCore; AI: Aurora |
| genesis-engine | Rust/C++ je Engine-Entscheidung | ATCLang (Game Logic, langfristig), TypeScript (Tools/UI) |

Repository-Rollen (Scope/Write) regelt ATC-STD-202; die Matrix hier ist
sprachseitig verbindlich.

## 5. Governance-Kopplungen (REQ-STD-109: MUST)

- Neue Technologie-Entscheidungen MÜSSEN durch ATC-STD-TUD-001
  (Technology Uniqueness & Differentiation) laufen: keine Sprache/Framework
  ohne Technology-Registry-Eintrag und Klassifikation.
- Sprach-Policy-Drift (Repo weicht von Matrix ab) MUSS vom Repository-Audit
  (ATC-STD-REPO-AUDIT-003) als Finding gemeldet werden; CI-Sprach-Gate als
  Validator-Erweiterung SOLLTE folgen.
- v1.0.0-Regeln bleiben wirksam, soweit nicht durch §1-§4 ersetzt (§7).

## 6. Migration v1 → v2 (Mapping, REQ-STD-110)

v1.0.0-Layer → v2.0.0-Layer: L1 Protocol/Core Rust → L1-L3; L2 Application
TypeScript → L7; L3 AI Rust+Python → L5; L4 OS Rust → L0-L1; L5 Spec/Governance
Markdown → ATC-STD-000 (nicht mehr Sprach-Layer, sondern Dokumentationsform).
Neu in v2.0.0: L0 Hardware (Assembly/C), L4 ATCLang als eigene Stufe, L6 SDK,
L8 Data. Das Ökosystem-Layer-Modell (AD-026, L0-L12) bleibt davon unberührt —
Sprach-Layer ≠ Ökosystem-Layer.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-105 — Language Architecture L0-L8: Layer-Zuordnung jeder Komponente MUSS vollständig sein.
- id: REQ-STD-106 — Sprachbindungen: MUST/VERBOT-Regeln je Sprache inkl. TCB-Regel (Rust), ATCLang-Trennung (ATC-99), Python-TCB-Verbot, JS-Vermeidung, WASM-Semantik-Trennung, SQL-nicht-Consensus-Truth.
- id: REQ-STD-107 — Entscheidungskaskade: Auswahl MUSS der Kaskade folgen; Abweichungen MÜSSEN dokumentiert sein.
- id: REQ-STD-108 — Repository-Matrix: Sprachzuordnung je Repository MUSS der Matrix entsprechen; Drift MUSS als Finding gemeldet werden.
- id: REQ-STD-109 — Governance-Kopplung: Neue Sprachen/Technologien MÜSSEN via TUD-001 laufen; REPO-AUDIT-003 meldet Drift.
- id: REQ-STD-110 — Migration: v1-Layer-Mapping MUSS dokumentiert bleiben; Ökosystem-Layer (AD-026) MUSS NICHT umbenannt werden.

## Compliance

Prüfung im Governance-Validator: S-06/S-07 (Struktur), MD-Gates (Frontmatter),
REQ-IDs deklariert und eindeutig. Repository-Audit (ATC-STD-REPO-AUDIT-003)
prüft Repository-Sprachprofil gegen die Matrix (REQ-STD-108) und meldet
Abweichungen als Finding.

## Security Considerations

- TCB-Schutz: Nur Rust in L1-L3 (Memory Safety, no_std, deterministische
  Ausführung); Python/AI MUSS NICHT Konsens oder Kernel kontrollieren —
  KI-Governance per ATC/ATS-Trennung (ATC-STD-TUD-001 §ATC-TECH-004).
- Supply Chain: Neue Sprachen/Frameworks = neue Angriffsfläche → TUD-001-
  Klassifikation + ATC-STD-019 (Supply Chain Security) vor Einführung.
- WASM-Sandbox: WASM-Komponenten gelten als untrusted code; ATC Bytecode
  behält eigene Semantics (keine ungeprüfte Gleichsetzung).
- FFI/C-Grenze: Jede C-ABI/FFI-Stelle MUSS technisch begründet und im
  Repository-Audit sichtbar sein.

## Implementierungsstatus

**Status: SPECIFIED** — Normativ spezifiziert, Wartet auf §9-Freigabe;
Implementierungs-Evidence entsteht über Technology Profile Audits je Repository
(S. ATC-STD-REPO-AUDIT-003): aktuelle R1-Skelette sind Soll-Zustand; bekannte
Policy-Drifts (atclang nur Python, a-townchain ohne Rust-Chain-Code) sind
als Findings zu führen und über SCR zu beheben. Kein Status ohne Evidence
(ATC-STD-000 §14). SSOT für Implementierungsstand: registry/standard-
implementation.yaml.

## Changelog

- v2.0.0 (2026-09-11): Owner-Entwurf Language Architecture — L0-L8-Layer-
  Modell, Entscheidungskaskade, Repository-Matrix, Governance-Kopplung an
  TUD-001/REPO-AUDIT-003, Migration von v1.0.0 (5-Layer) dokumentiert.
  Status CANDIDATE (SCR-0093).
- v1.0.0 (2026-09-07): 5-Layer-Modell (L1 Rust Core, L2 TypeScript App,
  L3 AI, L4 OS Rust, L5 Markdown), Sprachbindungen, Ausnahmen, Technology
  Profile Audit. Status APPROVED.

## References

- ATC-STD-000 — Governance Root (§9 Lifecycle, §10 RFC 2119, §14 Evidence)
- ATC-99 / ATCLang First Policy — native Programmierung
- ATC-STD-202 — Repository Naming & Classification (Rollen/Scope)
- ATC-STD-TUD-001 — Technology Uniqueness & Differentiation (Technologieauswahl)
- ATC-STD-REPO-AUDIT-003 — Repository-Audit (Sprachprofil-Drift)
- AD-021/AD-022 — ATCLang Rust-first, 21-Crate-Layout
- AD-026 — Ökosystem-Layer-Modell (unberührt, Mapping §6)
- audits/PRIOR-ART-2026-09-11.md — Technologie-Klassifikationen des Stacks
