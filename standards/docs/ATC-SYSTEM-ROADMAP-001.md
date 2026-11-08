---
document:
  id: ATC-SYSTEM-ROADMAP-001
  title: "System-Meilensteinmodell M0-M14 — Gesamt-Roadmap A-TownChain (Entwurf 06.10.2026)"
  version: "0.1.0-DRAFT"
  status: draft
  scr: SCR-0136
  chain_id: 658467
  authority: A-TownChain-Okosystems
  owner: "Michael Wroblewski (Owner-Entwurf) / Standards Governance"
  created: "2026-10-06"
  updated: "2026-10-06"
  classification: PUBLIC
  language: de-DE
  bezug:
    - "ATC-STD-MILESTONE-001 (SCR-0018) — Meilenstein-Governance, Registry registry/milestones.yaml bleibt SSOT"
    - "ATC-STD-UPDATE-001 UPD-G04 (MAJOR) + COMPAT-001 — Revalidationsform"
    - "AD-027 (Lauffehigkeits-Roadmap M1-M8, laeuft als ATC-M-001..008, Migrationsklausel par.18)"
    - "OEP-0007-MERGE-GATE-GOVERNANCE (PR 93, SCR-0135) — M1.3/M1.5/M3.7"
    - "SPEC-ATC-GATE-METHODOLOGY-001 (PR 92, SCR-0132) — VERIFIED-Schema M1.3"
  note: 'Dokument-ID, kein Registry-Standard. Ein gemeinsames System-Meilensteinmodell, KEINE getrennten konkurrierenden Meilensteinregister je Repository; jedes Projekt bildet seine Etappen als Beitraege zu den System-Meilensteinen ab. Grundlage der MAJOR-Revalidation: ATC-M-001..008 bleiben erhalten, M0-M14 ausschliesslich als Alt-Neu-Abbildung innerhalb derselben Registry (Anhang A).'
---

# System-Meilensteinmodell M0–M14

Grundsatz: Keine getrennten, konkurrierenden Meilensteinregister pro Repository.
Stattdessen ein gemeinsames System-Meilensteinmodell, in dem jedes Projekt seine
Etappen als Beiträge zu den System-Meilensteinen abbildet.

## Gesamt-Roadmap

| M | System-Meilenstein | Ergebnis |
|---|---|---|
| M0 | Architektur & SSOT | Zuständigkeiten, SoT und Abhängigkeiten eindeutig |
| M1 | Governance Foundation | Standards, Evidence, Merge-/Gate-Modell belastbar |
| M2 | Engineering Foundation | atc-engineering als verbindliche Engineering-/Evidence-Schicht |
| M3 | Protocol Foundation | atc-standards + atc-algorithm normative Basis |
| M4 | Language Foundation | atc-lang spezifiziert und deterministisch |
| M5 | VM Foundation | atc-vm als kanonische L3-Ausführungsschicht |
| M6 | Blockchain Core | A-TownChain als deterministischer L2-Core |
| M7 | ShivaCore TCB | bootfähiger, capability-basierter Kernel |
| M8 | GlobusOS Base | Betriebssystemdienste auf ShivaCore |
| M9 | ATC Toolchain | vollständiger Build → Test → Verify → Package-Pfad |
| M10 | Aurora Runtime | Aurora AI als kontrollierte Authority-/Runtime-Schicht |
| M11 | System Integration | Blockchain + VM + OS + AI + Toolchain integriert |
| M12 | Genesis Engine | Engine auf stabiler ATC-/OS-Basis |
| M13 | System RC | vollständiger Release Candidate |
| M14 | Production Release | reproduzierbares, verifiziertes Gesamtsystem |

---

## M0 — Architektur & SSOT

Ziel: Keine konkurrierenden Wahrheiten.

Etappe M0.1 — Repository Ownership: jedes Repo bekommt eindeutige Verantwortung;
keine Funktionalität im Ecosystem-Repo, die eigentlich in ein Standalone-Repo gehört.

Etappe M0.2 — Layer Mapping: L0–L7 Architektur, Registry-L0–L7, explizite Mapping-Tabelle.

Etappe M0.3 — Dependency Graph: atc-standards, atc-algorithm, a-townchain, atc-vm,
atc-lang, ShivaCore, GlobusOS, Aurora, Toolchain, Genesis.

Etappe M0.4 — SSOT Audit: jede normative Spezifikation genau einem SoT zugeordnet;
Dubletten entweder entfernt oder als Referenz markiert.

Exit: Architekturgraph ohne Ownership-Ambiguität.

## M1 — Governance Foundation

Ziel: „Green CI" wird nicht mit „VERIFIED" verwechselt.

- M1.1 Evidence Model: UNANALYZED / ANALYZED / FIXED / RERUNNING / VERIFIED / RESIDUAL
- M1.2 SHA Binding: Run → Job → Step → Exit → Log; Exact-SHA-Bindung
- M1.3 VERIFIED Schema: Result, Freshness, SHA-Binding, Effectiveness, Scope
- M1.4 Merge/Gate Governance: MERGE_ALLOWED, Tier-Bindung, keine Agenten als Approver
- M1.5 Enforcement: Schutzänderung → Enforcement-Test, periodisches Negativ-Sampling,
  F-160 bis Wirksamkeitsnachweis OPEN

Exit: Evidence kann maschinenlesbar zwischen TESTED und VERIFIED unterscheiden.

## M2 — Engineering Foundation (atc-engineering)

Ziel: Zentrale Engineering-Infrastruktur.

- M2.1 Fleet Inventory: vollständiger Repository-Bestand, aktuell 33 Repositories
- M2.2 CI Governance: zentrale Workflows, SHA-Pinning, reproduzierbare Gates
- M2.3 Audit Framework: Engineering Audit, Dependency-/Workflow-/Naming-Audit, Exact-SHA Auditor
- M2.4 Fleet Enforcement: Wave 1, Wave 2, D3-Sampling
- M2.5 Engineering Baseline: alle verpflichtenden Repositories klassifiziert, BLOCKED/RESIDUAL explizit

Exit: atc-engineering kann den Zustand der gesamten Flotte reproduzierbar feststellen.

## M3 — Protocol Foundation (atc-standards)

Ziel: Normative Protokollfamilie.

- M3.1 OEP-1 Account-Modell
- M3.2 OEP-2 WASM Policy Carrier, deterministische Sandbox
- M3.3 OEP-3 MPT / State Commitment
- M3.4 OEP-4 Finality-Modell
- M3.5 OEP-5 Validator Set / Epoch
- M3.6 OEP-6 BFT-/Consensus-Familie; PoH als Clock nur sofern final entschieden
- M3.7 OEP-7 Merge-/Gate-Governance, VERIFIED-Schema

Exit: Protocol Family 0.1.0 ist normativ geschlossen.

## M3b — atc-algorithm (parallel zu M3)

1. Algorithm Ownership · 2. Consensus Interface · 3. Validator Selection ·
4. Epoch Model · 5. Finality · 6. PoH/Clock · 7. Determinismus ·
8. Slashing/Reward Interfaces · 9. Golden Vectors · 10. Cross-implementation Tests

Exit: Algorithmik ist keine implizite Blockchain-Implementierungslogik mehr,
sondern eindeutig spezifiziert.

## M4 — Language Foundation (atc-lang)

Ziel: Eine deterministische Contract-/Programmiersprache.

- M4.1 Language Specification: Syntax, Semantik, Typmodell
- M4.2 Canonical Core: Rust Core
- M4.3 Bytecode: ATCB, Version, Instruction Count, deterministische Encoding-Regeln
- M4.4 VM ABI: Opcode ABI, Host Interface, Memory/Stack Contract
- M4.5 Golden Vectors: Compiler → Bytecode, Parser, Encoder, Decoder
- M4.6 Negative Corpus: invalid programs, malformed bytecode, boundary cases
- M4.7 Freeze Gates: keine semantischen Unklarheiten, keine divergierenden Implementierungen

Exit: ATCLang Freeze-ready.

## M5 — VM Foundation (atc-vm)

Ziel: kanonische L3-Ausführungsumgebung. SoT: a-townchain/components/vm
(Owner-Entscheidung 06.10.: Monorepo kanonisch, kein konkurrierendes atc-vm-Repo-SSOT).

- M5.1 VM Core: Instruction Dispatch, Stack, Memory, Storage
- M5.2 Gas: Gas Budget, Gas Meter, Opcode Schedule, Exhaustion Enforcement
- M5.3 Determinismus: keine Host-Abhängigkeiten, definierte Integer-/Memory-Semantik
- M5.4 Security: Bounds Checks, Resource Limits, Fault Handling
- M5.5 ATCLang Integration: Bytecode Loader, ABI, execution contract
- M5.6 Golden Vectors: same input → same state/output
- M5.7 VM Freeze

Exit: L3 kann deterministisch Programme ausführen.

## M6 — Blockchain Core (A-TownChain)

Ziel: stabiler L2 Blockchain Core.

- M6.1 Ledger Core: Block, Transaction, State, Receipt
- M6.2 TX V2: secp256k1, RFC6979, SHA-256, low-S, compressed SEC1, 64-byte compact signature
- M6.3 Economics: Chain ID 658467, MAX_SUPPLY 360,000,000 ATC, 18 decimals,
  u128 Monetary Values, u64 Counters
- M6.4 Consensus Integration: atc-algorithm, Validator/Epoch/Finality
- M6.5 VM Integration: L2 → L3 boundary
- M6.6 State/Storage: canonical state commitment, MPT sofern OEP-3 final
- M6.7 Golden Vectors: TX, Block, State, Receipt
- M6.8 Network/Node: deterministic node behavior, synchronization, validation

Exit: deterministischer Blockchain-Core mit reproduzierbarer State Transition.

## M7 — ShivaCore Kernel

Ziel: minimaler kryptografischer TCB.

- M7.1 Boot: Secure Boot, Kernel Entry, Hardware Discovery
- M7.2 Memory: Page Management, Address Spaces, Isolation
- M7.3 Scheduler: Tasks, Context Switching, Priorities
- M7.4 Capability System: Creation, Delegation, Revocation, no ambient authority
- M7.5 IPC: IPCBus, Message Passing, Capability Transfer
- M7.6 Security: TPM/TEE, IdentityKeyProvider, Secure State
- M7.7 Audit: security events, immutable/auditable records
- M7.8 Hardware Matrix: AMD, Intel, QEMU
- M7.9 TCB Freeze

Exit: bootfähiger und sicherheitstechnisch begrenzter ShivaCore-TCB.

## M8 — GlobusOS

Ziel: echtes Betriebssystem oberhalb des Kernels.

- M8.1 Service Architecture: User/Service Space, Kernel/User Boundary
- M8.2 Core Services: Process/Task Management, VFS, Networking, IPC, EventBus
- M8.3 Device Layer: Drivers, Hardware Abstraction
- M8.4 System Management: Settings, Updates, Recovery, Diagnostics
- M8.5 Security: Authentication, Authorization, Capability Policies
- M8.6 ATC Integration: Blockchain services, VM service, wallet/key services
- M8.7 OS Image: reproducible image, boot verification

Exit: GlobusOS bootet und betreibt Systemdienste auf ShivaCore.

## M9 — ATC Toolchain

Ziel: vollständiger Entwickler-/Build-/Release-Kette.

Pfad: Source → ATCLang → Compiler → ATCB → ATC-VM → A-TownChain → Test →
Evidence → Verified Artifact → Release.

Etappe: 1. Compiler Tooling · 2. Formatter/Linter · 3. Bytecode Tooling ·
4. VM Runner · 5. Test Harness · 6. Golden Vector Runner · 7. Cross-language Tests ·
8. Evidence Generator · 9. Artifact Signing · 10. Reproducible Build ·
11. Release Packaging

Exit: ein Commit kann reproduzierbar vom Source bis zum verifizierten Artifact
verfolgt werden.

## M10 — Aurora AI

Ziel: AI ist Control Plane, nicht Kernel-Authority.

- M10.1 Aurora Runtime · M10.2 Aurora Authority Plane · M10.3 Model ABI (ATC Model ABI) ·
- M10.4 Compute HAL: CPU, GPU, NPU, Security HAL
- M10.5 Capability Interface: Aurora AI → Intent → Policy → Capability → Approval → Tool → Execution
- M10.6 Audit: every action, provenance, policy decision, evidence
- M10.7 No Direct Kernel Authority: Aurora darf nicht direkt Kernel-Rechte erhalten.

Exit: Aurora kann Systemaktionen kontrolliert auslösen, ohne den TCB zu umgehen.

## M11 — System Integration

Verbund der zuvor stabilisierten Teile:

Aurora AI → Authority Plane → Policy/Capability → GlobusOS → ShivaCore →
{A-TownChain → ATC-VM → ATCLang | Services}

Etappe: 1. OS ↔ Blockchain · 2. OS ↔ VM · 3. VM ↔ ATCLang · 4. Aurora ↔ OS ·
5. Aurora ↔ Blockchain · 6. Toolchain ↔ alle Artefakte · 7. Evidence ↔ alle Releases ·
8. Security Boundary Tests · 9. End-to-End Tests

Exit: System funktioniert als zusammenhängende Plattform.

## M12 — Genesis Engine

Ziel: Genesis Engine wird auf die stabile Plattform gesetzt.

- M12.1 Engine Core: Rust, ECS, Scheduler, Resource System
- M12.2 Layer Registry: eindeutige Abgrenzung Architecture L7 / Registry L6
- M12.3 Rendering: GPU abstraction
- M12.4 Simulation: deterministic simulation
- M12.5 ATC Integration: Blockchain, Assets, Contracts
- M12.6 Marketplace/NFT: nur nach stabiler Protokollbasis
- M12.7 Multiplayer/Network
- M12.8 Genesis Chronicles Runtime

Exit: Genesis Engine ist ein echter Consumer der Plattform, kein paralleler
Technologie-Stack.

## M13 — System Release Candidate

Hier gilt erstmals: Kein „fast fertig". Jedes Subsystem bekommt exakt einen
Zustand: VERIFIED / BLOCKED / RESIDUAL. BLOCKED darf kein Release-Blocker
bleiben. RESIDUAL darf nur existieren, wenn explizit als akzeptiertes
RC-Residual klassifiziert.

RC-Gates (je VERIFIED): Architecture, Governance, Engineering, Standards,
Algorithm, ATCLang, ATC-VM, Blockchain, ShivaCore, GlobusOS, Toolchain,
Aurora, Genesis.

## M14 — Production Release

Der finale Zustand ist nicht „alles grün", sondern: ein reproduzierbar gebautes,
SHA-gebundenes, sicherheitsgeprüftes und evidenzbasiertes verifiziertes
Gesamtsystem.

Release-Gates: 1. Source Freeze · 2. Specification Freeze · 3. ABI Freeze ·
4. Consensus Freeze · 5. VM Freeze · 6. Kernel Freeze · 7. OS Image Freeze ·
8. Toolchain Freeze · 9. Aurora Runtime Freeze · 10. Genesis Engine
Compatibility Freeze · 11. Reproducible Build · 12. Golden Vector Suite ·
13. Security Test Suite · 14. Negative Test Suite · 15. Evidence Audit ·
16. Exact-SHA Release · 17. Signed Artifacts.

---

## Die 11 Projekte im Verhältnis

atc-engineering (Governance/Evidence) → atc-standards (Normative Specs) →
{atc-algorithm (Consensus), atc-lang (Language) → atc-vm (L3)} → A-TownChain (L2) →
GlobusOS/ShivaCore → Aurora AI (Authority/Runtime) → Genesis Engine.
ATC Toolchain verbindet Build/Test/Verify/Release über alles.

Wichtigste Reihenfolge — nicht alle elf Projekte gleichzeitig auf „Feature
Complete" bringen. Die kritische Kette:

Governance → Standards → Algorithm/Language → VM → Blockchain → Kernel → OS →
Aurora → Integration → Genesis → Release.

Toolchain und atc-engineering laufen quer darüber (Nachweis-, Build- und
Verifikationsschicht).

Ergebnis: 15 klare System-Meilensteine mit überprüfbaren Etappen statt elf
voneinander unabhängiger „Roadmaps".

---

## Anhang A — Alt→Neu-Abbildung ATC-M-001..008 (Grundlage der MAJOR-Revalidation)

Registry `registry/milestones.yaml` bleibt SSOT (ATC-STD-MILESTONE-001 §17/§18,
Migrationsklausel §18). Statusse bleiben erhalten, keine semantische Rücksetzung.
M0–M14 werden ausschließlich als Alt→Neu-Abbildung innerhalb derselben Registry
geführt (UPD-G04/COMPAT-001).

| Alt (ATC-M) | Status | Inhalt | Abbildung (neu) |
|---|---|---|---|
| ATC-M-001 | ACCEPTED | M1 Sprache läuft (L0 atclang) | M4 Language Foundation — G1/G2-Evidence (Python-Referenz, 20/20) zählt auf M4.1; Rust Canonical Core separat M4.2 (F-108) |
| ATC-M-002 | ACCEPTED | M2 Kernel läuft (L1 atc-shivacore) | M7 ShivaCore TCB — 674/674 + Boot L0–L10 zählt auf M7.1/M7.2 |
| ATC-M-003 | IN_PROGRESS | M3 KI läuft (L2 aurora-ai via Kernel-Event-Bridge) | M10 Aurora Runtime |
| ATC-M-004 | PLANNED | M4 Blockchain läuft (L3 a-townchain) | M6 Blockchain Core |
| ATC-M-005 | PLANNED | M5 Betriebssystem läuft (L4 globus-os) | M8 GlobusOS Base |
| ATC-M-006 | PLANNED | M6 Dienste laufen (L5 13 Services) | M8.2 Core Services + M11 System Integration |
| ATC-M-007 | PLANNED | M7 Spiel läuft (L6 genesis-engine + Chronicles, NFT auf Chain) | M12 Genesis Engine |
| ATC-M-008 | PLANNED | M8 Ökosystem läuft (L7 a-townchain-os Launch-Stack) | M11 + M13 + M14 (Integration/RC/Production) |

Schema-Invarianten (Owner 06.10. 18:55, besttigt): status_alt wird als
eingefrorenes Feld gefhrt; status_neu startet separat bei null und wird
ausschliesslich aus neuen Nachweisen gesetzt (kein impliziter bertrag
ACCEPTED/IN_PROGRESS/PLANNED); mapping_alt ist ein Set (1 zu n), Referenz,
kein Evidence-Feld, nicht als solches typisiert. Umgesetzt in SCR-0137
(PR registry/system-milestones-major-001).

Eintragungsform im Registry-Upgrade: die bestehenden acht Einträge bleiben
unverändert erhalten; die System-Meilensteine M0–M14 werden als übergeordnete
System-Ebene mit Verweis auf die Alt-IDs ergänzt (mapping_alt-Feld je
System-Meilenstein). Registry-Write erfolgt erst nach Owner-Freigabe dieser
Abbildung (Human-Gate-Regel).

## Anhang B — Deltas zur Skizze von 17:51 (Renumbering, protokolliert)

| Thema | 17:51-Skizze | System-Modell 0.1.0 |
|---|---|---|
| Fleet Audit | M2 | M2.4 Fleet Enforcement (in Engineering Foundation) |
| OEP-7 VERIFIED-Schema | M4 | M1.3 + M3.7 |
| Enforcement/F-160 | M5 | M1.5 |
| ATC-VM | M7 | M5 |
| ATCLang Freeze | M8 | M4.7 (Golden Vectors M4.5, vor Freeze-Gates vorgezogen) |
| ShivaCore | M9 | M7 |
| GlobusOS | M10 | M8 |
| Aurora | M11 | M10 |
| Toolchain | (verteilt) | M9 (eigener System-Meilenstein) |
| NEU | — | M2 Engineering Foundation, M3b atc-algorithm als Paralleletappe |

Heutige Arbeitspakete zählen damit auf: Gate-Methodik (PR 92) + OEP-7 (PR 93) → M1;
W2-Welle → M1.5/M2.4/M4; Protocol Family (PR 90) → M3; MECH-04 Wallet-SSOT → M0.

## Anhang C — Benennungshinweis (offen, nicht blockierend)

Der Roadmap-Text verwendet „atc-lang"; das kanonische Repository heißt aktuell
`atclang`. Wortgetreue Übernahme des Owner-Texts; die endgültige Benennung ist über
das Namens-Audit (AD-050-Kontext) zu klären und bei Registry-Eintragung zu
normalisieren.
