# ATC_OS_ARCHITECTURE_REFERENCE_MODEL — OS-Referenzmodell des ATC-Stacks (v0.1.0, Draft)

<!-- document_id: ATC-DOC-OSARCH-001 -->
<!-- status: draft -->
<!-- standard: ATC-STD-MD-001 -->
<!-- date: 2026-10-06 -->

> **Zweck:** Einheitliches OS-Architektur-Referenzmodell für den ATC-Stack
> (Kernel bis Userspace), analog zum ATC_ARCHITECTURE_REFERENCE_MODEL.md
> (Blockchain-Stack). Verbindlich: vierstufige Evidence-Klassifikation.
>
> **Wichtigste Erkenntnis (Repo-Scan 06.10.2026):** Der aktive Kernel-Source-of-Truth
> liegt in **`globus-os`** (Modul `atc-shivacore`). Das Repository `atc-shivacore`
> ist auf unterstützendes Material reduziert (7 Rust-Dateien, 10 Tests).
> Alle älteren Kernel-Claims aus dem atc-shivacore-Repo (K-Sprints K1-K50,
> „60 Kernel-Module", „2146 Tests") sind dadurch **LEGACY/UNVERIFIED** —
> sie müssen gegen globus-os re-verifiziert werden.

## 1. Klassifikationsmodell

Identisch mit ATC_ARCHITECTURE_REFERENCE_MODEL.md §1:
**NORMATIVE → IMPLEMENTED (nur Tests + Exact-SHA-Evidence) → RESEARCH → LEGACY/UNVERIFIED.**
Existenz einer Spezifikation, Datei oder alter Doku-Status ist nicht gleich VERIFIED.

## 2. OS-Schichtenmodell mit ATC-Zuordnung

```text
+---------------------------------------------+
| Anwendungen / Dienste / Shell               |  <- KAI-OS Userspace (Produkt)
+---------------------------------------------+
| System Call Interface                       |  <- UNVERIFIED (Evidence ausstehend)
+---------------------------------------------+
| Kernel (Prozesse, Speicher, I/O, IPC)       |  <- globus-os/modules/atc-shivacore
+---------------------------------------------+
| HAL / Treiber                              |  <- globus-os/modules/atc-drivers
+---------------------------------------------+
| Hardware                                   |  <- Zielplattform (Rust no_std)
+---------------------------------------------+
```

### 2.1 Statusblock

| Bereich | Status | Evidence / Bemerkung |
|---|---|---|
| Kernel (globus-os/modules/atc-shivacore) | **UNVERIFIED — CONFIRMED** | Repo-Scan 06.10.2026 (211 .rs-Dateien, 2283 Test-Funktionen); **Zusatzbefund:** main kompiliert nicht am Head e28a055 (Rust-CI Check failure, 33 Fehler — Issue globus-os#44) → Kernel-Tests laufen aktuell gar nicht |
| HAL/Treiber (globus-os/modules/atc-drivers) | **UNVERIFIED** | Existenz verifiziert (Repo-Scan); Funktions-Evidence fehlt |
| Syscall-Interface | **UNVERIFIED** | K-Sprint-Doku behauptet Implementierung; keine SHA-Evidence im aktiven SSOT |
| Userspace/Services | **UNVERIFIED** | dito |
| Kernel-Architektur: Mikrokernel | **RESEARCH** (maturity: DEVELOPMENT) | Dokumentiertes Designziel: „capability-basierter Rust/no_std-Microkernel" (atc-shivacore README); kein Freeze, kein Evidence-Run |
| atc-shivacore-Repo (Alt-Kernel) | **LEGACY/UNVERIFIED** | Demotion dokumentiert: SSOT liegt inzwischen in globus-os; alte K-Sprint-Claims dort re-verifizieren |
| KAI-OS (Produktname) | **RESEARCH** | Produktbezeichnung; Phase: Prototyp (Roadmap Notion) |

Hebung auf IMPLEMENTED erst nach Evidence-Run (Tests grün + Commit-SHA je Bereich).

## 3. Kernel-Architektur im Vergleich

| Typ | Prinzip | ATC-Relevanz |
|---|---|---|
| Monolithisch | Alles im Kernel | — |
| **Mikrokernel** | Minimaldienste im Kernel, Rest als User-Prozesse | **Dokumentiertes ATC-Ziel** (capability-based, no_std Rust, referenziert seL4/QNX-Klasse) |
| Hybrid | Kompromiss | — (keine dokumentierte ATC-Position) |
| Exokernel | Nur Ressourcenschutz | — |
| Unikernel | OS+App als Einzelimage | — (keine dokumentierte ATC-Position) |

Klassifikation der Wahl: RESEARCH/DEVELOPMENT — die Entscheidung „Mikrokernel" ist
Designziel, bis Freeze + Evidence-Run vorliegen. Keine Aussage als „final eingefroren".

## 4. Kernkomponenten (ATC-Mapping, alle UNVERIFIED bis Evidence-Run)

- **Prozess-/Thread-Management** → K-Sprint-Bereich (globus-os): UNVERIFIED
- **Scheduler** → K-Sprint-Bereich: UNVERIFIED
- **Speicherverwaltung/Paging** → K-Sprint-Bereich (u. a. page_fault, elf_loader): UNVERIFIED
- **Dateisystem/Journaling** → fs_journal (K50-Claim, Alt-Repo): LEGACY/UNVERIFIED, in globus-os re-verifizieren
- **IPC** → K-Sprint-Bereich: UNVERIFIED
- **Synchronisation** → UNVERIFIED
- **Sicherheit** → atc-security-Modul (in atc-shivacore-Restrepo: 6 Dateien/10 Tests): UNVERIFIED (Restrepo, Funktionsstand unklar)
- **Netzwerkstack** → UNVERIFIED

## 5. User Mode vs. Kernel Mode

Konzept gilt unverändert (Ringe/MMU/Syscall-Übergang). Konkrete ATC-Implementierung
(Treiber-Wechsel, Capability-Durchsetzung): UNVERIFIED bis Evidence-Run im aktiven SSOT.

## 6. Virtualisierung/Container

Keine dokumentierte ATC-Position (Stand 06.10.2026). Bei Bedarf als eigener
RESEARCH-Eintrag führen — nicht implizit voraussetzen.

## 7. Evidence-Anforderungen (Hebungskriterien je Bereich)

| Bereich | Erforderlich für IMPLEMENTED |
|---|---|
| Kernel | `cargo test` im globus-os grün + Commit-SHA + Testzahl |
| HAL/Treiber | Test bzw. minimaler Boot-/Treiber-Nachweis + SHA |
| Syscall | Interface-Test-Suite (IFC-Pfad, ATC-STD-204) + SHA |
| Userspace | Beispiel-Applikation läuft + SHA |
| Architektur (Mikrokernel) | Freeze-Dokument + Owner-Freigabe |

## 8. Offene Punkte

| Punkt | Zuständigkeit | Status |
|---|---|---|
| Evidence-Run `cargo test` globus-os (Kernel-Hebung) | Agent | OFFEN — Todo Task-DB |
| Alt-Kernel atc-shivacore: K-Sprint-Claims gegen globus-os re-verifizieren | Agent | OFFEN |
| Mikrokernel-Freeze als formale Entscheidung (analog Consensus-Freeze) | Owner | OFFEN — Vorschlag |
| KAI-OS-Phasen-Roadmap mit Statusblock abgleichen (Notion) | Doku | OFFEN |

## 9. Querverweise

- `ATC_ARCHITECTURE_REFERENCE_MODEL.md` (Blockchain-Stack, Klassifikationsquelle)
- `globus-os/README.md`, `globus-os/modules/atc-shivacore/` (aktiver Kernel-SSOT)
- `atc-shivacore/README.md` (Demotion-Erklärung)
- KAI-OS-Produktbezeichnung: „A-TownChain OS (technical) / KAI-OS (product name)"
- `ATC_VM_ARCHITECTURE_REFERENCE_MODEL.md` (VM-Referenzmodell; ShivaVM = K19-Kernel-Modul ist Contract-VM, keine System-VM)
- Roadmap: Notion Master Roadmap (KAI-OS Phasen 1-4, Kernel Sprints)

## 10. Versionierung

| Version | Datum | Änderung |
|---|---|---|
| v0.1.0 | 2026-10-06 | Initiale Fassung (Draft): Schichtenmodell, Statusblock mit Kernel-Migration (atc-shivacore→globus-os) als LEGACY/UNVERIFIED dokumentiert, Evidence-Anforderungen definiert |

Status-Änderungen (draft → proposed → active) erfolgen per Owner-Review gemäß ATC-STD-MD-001.
