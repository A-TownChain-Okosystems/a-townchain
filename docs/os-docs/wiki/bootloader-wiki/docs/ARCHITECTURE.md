# 🏗️ Bootloader Architektur — atc-bootloader

---

## 1. Boot Sequence Overview

Der Bootloader schlägt die Brücke zwischen der Bare-Metal Firmware und dem ShivaOS Microkernel:

1. **Stage 1**: MBR / UEFI EntryPoint -> Setup der 64-Bit GDT/IDT und Long-Mode Enablement.
2. **Stage 2**: Mounting des ATCFS Volumes, Laden des Kernel Binary (`kernel.elf`) und kryptografische Validierung via Ed25519 Signature.
3. **Stage 3**: Übergabe von Control Registers, Physical Memory Map & Device Tree an den Kernel Entrypoint (`_start`).

---

## 2. Memory Layout Map

| Memory Range | Zweck |
|--------------|-------|
| `0x0000_0000 - 0x0000_7C00` | Real Mode Stack & BIOS Data Area |
| `0x0000_7C00 - 0x0000_7E00` | Stage 1 MBR Bootsector (512 Bytes) |
| `0x0001_0000 - 0x0009_FFFF` | Stage 2 Executable & ATCFS Buffer |
| `0x0010_0000 +`             | Kernel Image Payload (ShivaOS Core) |
