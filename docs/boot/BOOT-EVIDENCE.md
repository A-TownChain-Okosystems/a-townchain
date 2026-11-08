---
title: ShivaCore-Kernel — Boot-Evidence
summary: Belege, dass der Bare-Metal-Kernel unter QEMU sowohl über UEFI als auch über BIOS startet, samt Scheduler-Aktivitaet.
---

# ShivaCore-Kernel — Boot-Evidence

**Datum:** 2026-11-08 · **Host:** QEMU 7.2 (Debian), OVMF, TCG-Emulation (kein KVM) · **Gast:** q35, 512 MB

## Gegenstand

`kernel/globus-os/target/x86_64-unknown-none/release/shivacore` — ELF64, x86-64,
Einstiegspunkt `_start` bei `0x7030`, 123 016 Bytes. Gebaut mit
`cargo +nightly build --release -p shivacore --features x86-boot
--target x86_64-unknown-none -Z build-std=core,alloc`.

Boot-Images erzeugt `modules/atc-shivacore/boot` (Builder auf `bootloader 0.11`).

## Ergebnis UEFI

Bootloader lädt das ELF nach `0x1e030000`, baut eine neue Level-4-Tabelle
(`0x101000`) und mappt die drei `PT_LOAD`-Segmente. Danach greift der Kernel:

```
INFO : Create Memory Map
INFO : Create bootinfo
INFO : Jumping to kernel entry point at VirtAddr(0x8000007030)
ShivaCore: Kernel-Einstiegspunkt erreicht.
ShivaCore: CPU vendor=AMD family=15 model=107 stepping=1 apic=true x2apic=false nx=true invariant_tsc=false
ShivaCore: CPU HAL compatibility check OK.
ShivaCore: ACPI RSDP/root/table checksums validated; tables=7
EXCEPTION: BREAKPOINT
InterruptStackFrame {
instruction_pointer: VirtAddr(
0x800000575e,
),
```

Weiter im Protokoll: CPU-Erkennung (AMD, family 15, APIC, NX), ACPI-Prüfung
über 7 Tabellen, ein bewusst ausgelöster `BREAKPOINT` mit sauberem
Interrupt-Rahmen (Ring 0) und anschließend laufender Scheduler.

| Messgröße (25 s Laufzeit) | Wert |
| --- | --- |
| Kernel-Einstiegspunkt erreicht | 1× |
| Kontextwechsel zwischen PID 1000 und 1001 | 366 |
| SCHED-001 Register-Checkpoints | 3662 |
| Ausnahmen | 1× `BREAKPOINT` (erwartet) |

## Ergebnis BIOS

`shivacore-bios.img`: Stage 4 lädt das ELF nach `0x1000000`, der Kernel läuft
mit denselben Checkpoints. Beide Wege sind über
`./tools/ecosystem.sh boot` reproduzierbar.

## Grenzen

Der Kernel schreibt nach dem Boot auf die serielle Schnittstelle, nicht in den
Framebuffer — ein Screenshot zeigt nur den aufgeräumten Hintergrund
(`#001400`). Textausgabe über den Bildschirm ist nicht nachgewiesen. Ein
echter Hardware-Boot wurde nicht geprüft, nur QEMU mit TCG.
