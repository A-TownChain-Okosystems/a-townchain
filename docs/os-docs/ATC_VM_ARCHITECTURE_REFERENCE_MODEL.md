# ATC_VM_ARCHITECTURE_REFERENCE_MODEL — VM-Referenzmodell des ATC-Stacks (v0.1.0, Draft)

<!-- document_id: ATC-DOC-VMARCH-001 -->
<!-- status: draft -->
<!-- standard: ATC-STD-MD-001 -->
<!-- date: 2026-10-06 -->

> **Zweck:** VM-Referenzmodell analog zu ATC_ARCHITECTURE_REFERENCE_MODEL.md
> (Blockchain) und ATC_OS_ARCHITECTURE_REFERENCE_MODEL.md (OS). Verbindlich:
> vierstufige Evidence-Klassifikation (NORMATIVE → IMPLEMENTED → RESEARCH →
> LEGACY/UNVERIFIED); DEVELOPMENT ist ein Attribut (maturity), keine fünfte Klasse.
>
> **Kernbefund (Repo-Scan 06.10.2026):** „VM" im ATC-Stack ist eine
> **Bytecode-/Contract-VM**, keine System-VM. Es existieren **zwei Instanzen**:
> `ATVM` (Repo atc-vm, ATCLang-Verträge) und `ShivaVM` (globus-os Kernel-Modul
> K19, kernel/src/vm.rs). Hypervisor/System-VM: keine dokumentierte ATC-Position.

## 1. Klassifikationsmodell

Identisch mit ARM §1 / OS-ARM §1. IMPLEMENTED nur mit Tests + Exact-SHA-Evidence.

## 2. Zwei Bedeutungen — ATC-Zuordnung

| Bedeutung | Isolationseinheit | ATC-Instanz |
|---|---|---|
| **System-VM** (Hypervisor) | Ganze Maschine inkl. Kernel | **keine verifiziert** — keine Hypervisor-Komponente im Stack gefunden (Stand 06.10.2026); nicht implizit voraussetzen |
| **Prozess-/Bytecode-VM** | Prozess / Speicher-Sandbox | **ATVM** (atc-vm) und **ShivaVM** (globus-os, K19) — beide Contract-VMs |

ATVM und ShivaVM gehören zur Klasse JVM/CLR/WASM/eBPF: Isolation als Sprach-/
Verifier-Garantie, kein CPU-Ring-Wechsel, keine Hardware-Virtualisierung.

## 3. Statusblock

| Bereich | Status | Evidence / Bemerkung |
|---|---|---|
| ATVM Runtime (atc-vm) | **IMPLEMENTED** (Evidence-Grade) | CI `cargo-tests`: success @ SHA 21132a4 (2026-09-25) + 32 Test-Funktionen (Repo-Scan 06.10.2026). Evidence deckt die existierende Testsuite ab, NICHT die Feature-Vollständigkeit (Repo-Status: development) |
| ShivaVM (globus-os/modules/atc-shivacore/kernel/src/vm.rs, K19) | **UNVERIFIED — BLOCKIERT** | Existenz + Opcode-Satz verifiziert; Test-Run blockiert: globus-os/main kompiliert nicht am Head e28a055 (Rust-CI Check-Step failure, 33 Fehler in 10 Kernel-/System-Dateien, Kernel-Tests skipped — Issue globus-os#44) |
| Architekturwahl „Bytecode-VM" | **RESEARCH** (maturity: DEVELOPMENT) | Beide Instanzen führen ATCLang-/Contract-Bytecode aus; keine Freeze-Dokumentation der Architekturentscheidung |
| System-VM / Hypervisor | — (keine Position) | Keine dokumentierte ATC-Entscheidung; bei Bedarf als RESEARCH-Eintrag führen |
| ATVM ↔ ShivaVM: Verhältnis | **OFFEN (Governance)** | Zwei Contract-VM-Instanzen im Stack: kanonische Instanz, Abgrenzung oder Integration ungeklärt → Owner-Entscheidung |

## 4. Einordnung in die Bytecode-VM-Klasse

| VM | Ausführung | Isolation | ATVM/ShivaVM-Analogie |
|---|---|---|---|
| JVM | Stack-basiert, JIT | Prozessgrenze | Stack-Maschine mit Opcode-Satz (ShivaVM: Push/Pop/…) |
| CLR | RyuJIT | Prozessgrenze | — |
| WASM | Register-nah, AOT/JIT | Lineare Speicher-Sandbox | „technische Grenze zwischen on-chain ATCLang und Rust-Infrastruktur" (ATVM-README) |
| eBPF | Verifier + JIT | In-Kernel-Sandbox | In-Kernel-Ausführung (ShivaVM, K19) — Verifier-Anforderung analog SC-Gates |

## 5. System-VM-Sektion (negativer Befund)

Trap-and-Emulate, EPT/NPT, vCPU-Scheduling, Live Migration, MicroVMs
(Firecracker-Klasse): **keine verifizierten ATC-Komponenten** (Repo-Scans
globus-os, atc-vm, atc-shivacore 06.10.2026). Das OS-ARM §6 (Virtualisierung)
bleibt maßgeblich: keine implizite Annahme.

## 6. Evidence-Anforderungen

| Bereich | Erforderlich für IMPLEMENTED |
|---|---|
| ATVM Feature-Vollständigkeit | CI cargo-tests grün je Milestone + Commit-SHA + Testzahl |
| ShivaVM | modulbezogener Test-Run (cargo test -p) + SHA |
| Architektur-Freeze (Bytecode-VM) | Freeze-Dokument + Owner-Freigabe (analog Consensus-Freeze) |
| ATVM ↔ ShivaVM | Owner-Entscheidung (kanonische Instanz / Abgrenzung) + ggf. SCR |

## 7. Offene Punkte

| Punkt | Zuständigkeit | Status |
|---|---|---|
| ATVM ↔ ShivaVM: kanonische Contract-VM festlegen (Redundanz auflösen) | Owner | OFFEN — Todo Task-DB |
| ShivaVM: modulbezogener Test-Run als Evidence | Agent | **BLOCKIERT** — globus-os/main nicht bauend (Issue #44); zuerst Compile-Fix |
| Entscheidungssequenz ATVM↔ShivaVM (Owner-Review 06.10.2026): (1) ShivaVM-Evidence, (2) ATVM-Semantik gegen ATCLang-SoT abgleichen, (3) ADR erstellen, (4) Rollenentscheidung (Backend / interne System-VM / Adapter / obsolet), (5) VM-Architecture-Freeze vorbereiten | Owner + Agent | LAUFEND |
| Bytecode-VM-Architektur-Freeze (analog Consensus-Freeze) | Owner | OFFEN — Vorschlag |

## 8. Querverweise

- `ATC_ARCHITECTURE_REFERENCE_MODEL.md` (Blockchain-Stack, Contract Layer)
- `ATC_OS_ARCHITECTURE_REFERENCE_MODEL.md` (Kernel-SSOT globus-os, §6 Virtualisierung)
- `atc-vm/README.md` (ATVM, R1-auditiert 2026-09-10, SCR-0075)
- `globus-os/modules/atc-shivacore/kernel/src/vm.rs` (K19 ShivaVM)
- `atc-standards/standards/sc/` (SC-Gates SC-G0..G13, SC-020 AI-Assisted Development)

## 9. Versionierung

| Version | Datum | Änderung |
|---|---|---|
| v0.1.0 | 2026-10-06 | Initiale Fassung (Draft): Zwei-Bedeutungs-Trennung, ATVM mit CI-Evidence IMPLEMENTED (Testsuite), ShivaVM UNVERIFIED, System-VM als negativer Befund, ATVM↔ShivaVM-Governancefrage dokumentiert |
| v0.1.1 | 2026-10-06 | Owner-Review-Ergebnis: ATVM IMPLEMENTED*-Qualifikation bestätigt (Testsuite ≠ Feature-Vollständigkeit); Entscheidungssequenz (Evidence → Rollenklärung → ADR → Entscheidung → Freeze) festgezurrt; ShivaVM-Evidence-Run BLOCKIERT dokumentiert (globus-os#44: main nicht bauend, 33 Fehler); Rollenoptionen A/B/C aufgenommen (B: ShivaVM als konforme Kernel-Runtime, C: getrennte Domänen — B/C plausibler als zwei gleichberechtigte Contract-VMs; TCB-Grenze: Contract-Engine nicht ohne Capability-/Policy-Grenze in den Kernel) |

Status-Änderungen (draft → proposed → active) erfolgen per Owner-Review gemäß ATC-STD-MD-001.
