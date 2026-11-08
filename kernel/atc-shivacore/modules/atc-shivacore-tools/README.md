# ATC ShivaCore Tools

## Status: Sprint 0 -- Grundgeruest (08.07.2026)

Linux-gehostetes **Software-Tooling** rund um den ShivaCore-Kernel. Dies ist
**KEIN Kernel-Code** und **KEINE Architektur-Aenderung** -- der Kernel selbst
bleibt ausschliesslich bare-metal Rust no_std in
[atc-shivacore](../a-townchain-os/src/modules/atc-shivacore).

## Wichtige Abgrenzung (Standing Instruction)

GlobusOS/ShivaCore bleibt der alleinige, bare-metal OS-Standard des Oekosystems.
Linux-basierte Ansaetze fuer den OS-Kernel selbst sind explizit abgelehnt. Dieses
Repo veraendert daran NICHTS -- es enthaelt ausschliesslich Linux-seitige
Hilfssoftware, die den Kernel-Entwicklungsprozess unterstuetzt (Build, Test,
Deployment), aber selbst nicht Teil des Kernels ist und nicht auf dem Zielsystem
laeuft.

## Geplanter Scope

- Build-Skripte (Cross-Compile-Wrapper fuer `atc-shivacore`, Bootimage-Erstellung)
- QEMU-Testharness (automatisiertes Booten + Assertions gegen Serial-Output)
- Flash/Deploy-Tools (Image auf USB/Ziel-Hardware schreiben)
- CI-Helper (fuer GitHub Actions in atc-shivacore)

Sprache: Rust (std) oder Shell-Skripte, je nach Tool -- noch nicht final
entschieden pro Komponente.

## Naechster Schritt

Erstes Tool waehlen (Kandidat: QEMU-Testharness, da direkt fuer K-Sprint 3
Multitasking-Tests nuetzlich) und implementieren.

---
*Angelegt: 08.07.2026 -- reines Tooling-Repo, kein Kernel-Code.*

## ShivaCore Kernel Status (03.08.2026)

Der ShivaCore-Kernel (`atc-shivacore`) hat K-Sprint 0-16 abgeschlossen:
24 Rust-Module, 302/302 Tests grün.

Subsysteme: Boot, GDT/IDT/PIC, Paging/Heap, Capabilities, Prozesse,
DA-HEFT Scheduler, IPC, DID/RCT, Ed25519, Knowledge Graph, VFS,
Syscalls (ATC-96), Timer/Clock, Block-Device, Netzwerk (Ethernet/ARP),
TCP/IP (IPv4/UDP/TCP/Sockets), P2P-Consensus, Security Layer,
Konsens (DAG + PoH + Validator + Voting).

Dieses Tools-Repo enthält Hilfswerkzeuge und Utilities fuer die
Kernel-Entwicklung (z.B. QEMU-Scripts, Build-Hilfen, Test-Runner).

Kanonische Statusquelle: `REALITY_STATUS.md` im Root von `a-townchain-os`.

## Kernel Status (03.08.2026)

ShivaCore K0-K21: 29 Module, 441 Tests grün.
Inkl. AI-Kernel (Aurora AI): Tensoren, NN, Vektor-Gedächtnis, LLM-Router.
Kanonische Quelle: REALITY_STATUS.md in a-townchain-os.
