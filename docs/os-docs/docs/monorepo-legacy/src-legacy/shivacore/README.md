# ShivaCore — Bare-Metal Kernel (K-Sprint 0 ✅)

Eigener Kernel von Grund auf in Rust (`no_std`, `x86_64-unknown-none`) — kein
Linux-Unterbau, kein Fremdcode ausser dem minimalen Boot-Protokoll
(`bootloader` 0.11 Crate). Teil des GlobusOS-Betriebssystems
(GlobusOS = OS gesamt, ShivaCore = nur der Kernel darin).

## Status: K-Sprint 0 abgeschlossen (07.07.2026)

Verifiziert per QEMU-Boot-Test (BIOS-Image):
- ✅ Kernel-ELF wird vom Bootloader korrekt geladen und Entry-Point erreicht
- ✅ Serielle Debug-Konsole funktioniert (`serial_println!` via UART 16550)
- ✅ Framebuffer-Textausgabe funktioniert (Pixel-Modus, nicht klassisches VGA-Text)
- ✅ Kein Hang, sauberer Übergang in Idle-Loop (`hlt`)

**Vorheriges Problem (05.-06.07.) war ein Diagnose-Irrtum, kein echter Bug:**
Frühere Sessions vermuteten einen Hang zwischen Bootloader und Kernel wegen
leerem Serial-Log. Root-Cause-Analyse (07.07.) mit einem minimalen
Raw-Serial-Diagnose-Kernel (direkter Port-Write vor jeglicher Initialisierung)
zeigte: der Kernel wurde immer schon erreicht und konnte schreiben. Der
vollständige Kernel (inkl. `lazy_static`-Serial-Init und Framebuffer-Init)
bootet bei erneutem Test einwandfrei durch — vermutlich war das leere Log in
früheren Sessions ein Artefakt eines fehlgeschlagenen/falsch konfigurierten
Testlaufs, kein Kernel-Bug.

## Bauen

```bash
cd kernel && cargo build --release
cd ../boot && cargo run --release -- \
  ../kernel/target/x86_64-unknown-none/release/shivacore \
  ../images
```

Erzeugt `images/shivacore-bios.img` und `images/shivacore-uefi.img`.

## Testen (QEMU)

```bash
qemu-system-x86_64 -drive format=raw,file=images/shivacore-bios.img \
  -serial stdio -display none -no-reboot
```

Erwartete Ausgabe:
```
ShivaCore: Kernel-Einstiegspunkt erreicht.
ShivaCore: Framebuffer-Ausgabe erfolgreich.
ShivaCore: Boot vollstaendig. Uebergabe an Idle-Loop.
```

## Nächster Schritt: K-Sprint 1

CPU-Grundlagen — GDT, IDT, Interrupt-Handler (Breakpoint, Double-Fault, Page-Fault),
PIC-Remapping. Baut direkt auf diesem Kernel auf (`kernel/src/main.rs`).

## Struktur

- `kernel/` — der eigentliche bare-metal Kernel-Crate (kompiliert zu ELF, läuft ring 0)
- `boot/` — Host-Tool, das aus dem Kernel-ELF bootfähige BIOS/UEFI-Images baut
  (nutzt `bootloader::BiosBoot`/`UefiBoot`, läuft NICHT im Kernel-Kontext)


## Status: K-Sprint 1 abgeschlossen (07.07.2026)

GDT + TSS (dedizierter Double-Fault-Stack via IST), IDT mit Breakpoint-/
Double-Fault-/Page-Fault-Handlern, PIC-Remapping (8259 von 0x08-0x0F auf
0x20-0x2F), Timer- + Keyboard-Interrupts aktiv. QEMU-verifiziert: Breakpoint
(`int3`) kehrt sauber zurueck, kein Crash, Idle-Loop laeuft weiter.

**Gefixter Bug:** Erster Testlauf loeste einen Double Fault direkt nach dem
Breakpoint-Handler aus. Ursache: nach dem Laden des eigenen (minimalen) GDT
zeigte der alte Stack-Segment-Selektor (SS, vom Bootloader-GDT) ins Leere.
Bei der IRETQ-Rueckkehr aus dem Interrupt wird SS zwingend neu geladen und
validiert -> #GP waehrend IRETQ -> vom Prozessor als Double Fault eskaliert.
Fix: SS nach dem GDT-Laden explizit auf den Null-Selektor setzen (in
Long-Mode bei CPL0 fuer das Stack-Segment zulaessig, da Flat-Memory-Modell).

## Nächster Schritt: K-Sprint 2

Speicherverwaltung — Paging (aktuelle Page-Tables auslesen/verstehen), Heap-
Allokator (`#[global_allocator]`), damit `alloc`/`Box`/`Vec` nutzbar werden.


## Status: K-Sprint 2 abgeschlossen (07.07.2026)

Paging-Mapper (`OffsetPageTable` über das vom Bootloader linear gemappte
physische RAM), einfacher `BootInfoFrameAllocator` (iteriert die vom
Bootloader gemeldete `MemoryRegions`-Karte nach freien 4-KiB-Frames), Heap
(100 KiB, `linked_list_allocator`). `alloc` (Box/Vec/String) ist jetzt im
Kernel nutzbar. QEMU-verifiziert: `Box::new(41)` und `Vec` mit Summe 0..10=45
funktionieren fehlerfrei, kein Crash.

Voraussetzung war eine `BootloaderConfig` mit `mappings.physical_memory =
Some(Mapping::Dynamic)`, eingebettet via `entry_point!(kernel_main, config =
&BOOTLOADER_CONFIG)` in `main.rs`.

## Nächster Schritt: K-Sprint 3

Multitasking — Prozess-/Task-Struktur, einfacher Scheduler, Context-Switch.


## Ehrlicher Pitch — was ShivaCore ist (und was nicht)

**Nicht:** "Schon jetzt besser als Linux/seL4/Redox OS." ShivaCore ist Stand
07.07.2026 drei K-Sprints alt (Boot, GDT/IDT/PIC, Paging/Heap) — kein
fairer Vergleich zu Kerneln mit jahrzehntelanger Härtung.

**Was heute real ist:** Rust `no_std` statt C ist der einzige bereits
beweisbare Vorteil — ganze Bug-Klassen (Buffer-Overflow, Use-after-free,
Data Races), die bei Linux-Kernel-Treibern die häufigste CVE-Quelle sind,
sind durch den Rust-Borrow-Checker strukturell ausgeschlossen.

**Was der Plan ist (ATC-1 bis ATC-40, noch nicht implementiert):** Die
meisten "Blockchain-OS"-Projekte sind in Wahrheit normales Linux mit einem
Blockchain-Node als Userspace-App obendrauf — architektonisch nichts Neues.
Der Anspruch hinter GlobusOS/ShivaCore ist, Dinge wie P2P-Konsens,
DID-/Agent-Identität und Capability-basierte Security näher an den Kernel
zu bringen statt als nachträglich aufgesetzten Daemon.

**Was heute fehlt:** Dateisystem, Netzwerk-Treiber, Multitasking (in
Arbeit, K-Sprint 3), Hardware-Breite, Jahre an Battle-Testing.

**Kurz:** kleine, auditierbare, speichersichere Codebasis mit
blockchain-nativer Architektur als Ziel — nicht "schon jetzt überlegen".
Für alle Agenten, die an diesem Projekt arbeiten: bitte diese Erwartungshaltung
in Doku/Kommunikation nach außen konsistent halten, keine Überverkaufs-Sprache.


---

## ⚠️ UMZUG (07.07.2026): ShivaCore hat jetzt ein eigenes Repository

Der komplette ShivaCore-Kernel-Code (kernel/, boot/) wurde in ein eigenes
Repository ausgelagert: **https://github.com/A-TownChain-Okosystems/atc-shivacore**

**Kanonische Quelle ab sofort:** `atc-shivacore` (eigenes Repo).
Dieser Pfad hier (`a-townchain-os/shivacore/`) bleibt vorerst als Kopie/
Referenz stehen, wird aber NICHT mehr aktiv weiterentwickelt. Alle
zukünftigen K-Sprints (K3+) werden im neuen Repo committet.

Grund: eigenständige Versionierung/CI für den Kernel, unabhängig vom
Monorepo-Rhythmus des restlichen A-TownChain-Ökosystems.
