# ARCHITECTURE.md — atc-bootloader

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-bootloader/
├── Cargo.toml — UEFI application build manifest targeting x86_64-unknown-uefi
├── .gitignore — Git ignore settings for target outputs and build artifacts
└── src/
    ├── main.rs — Bootloader entry point (efi_main), system table init, and boot orchestrator
    ├── gpt.rs — GUID Partition Table parser and partition validation
    ├── secure_boot.rs — Secure Boot cryptographic verification of kernel binary integrity
    ├── kernel_handoff.rs — UEFI boot service termination and kernel execution handoff
    └── framebuffer.rs — Graphics Output Protocol (GOP) setup and early framebuffer console
```

## Module Descriptions
- src/main.rs — Main UEFI application entry point managing boot sequence, memory map acquisition, and error handling.
- src/gpt.rs — Parses GPT headers and partition entries to locate ShivaCore kernel partitions on storage devices.
- src/secure_boot.rs — Verifies cryptographic Ed25519 signatures of the kernel image before execution to enforce trust.
- src/kernel_handoff.rs — Exits UEFI boot services, prepares boot info structures, and jumps to kernel entry point `_start`.
- src/framebuffer.rs — Queries UEFI Graphics Output Protocol (GOP) to initialize visual video modes and debug output.

## Build System
- Cargo.toml — Configured for `#![no_std]` targeting `x86_64-unknown-uefi`.
- Generates PE/COFF executable (`atc-bootloader.efi`) for native UEFI firmware boot.

## Dependencies
- uefi / uefi-services — Safe Rust abstractions for UEFI firmware system tables, protocols, and boot services.
- ed25519-dalek — High-speed cryptographic signature verification for secure bootloader image validation.
