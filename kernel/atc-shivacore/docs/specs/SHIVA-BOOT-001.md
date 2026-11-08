---
document_id: SHIVA-BOOT-001
title: ShivaCore Boot Contract
version: 1.0.0
status: active
---

# ShivaCore Boot Contract

## Scope

The boot layer loads the kernel and supplies platform information. It is not an OS service and MUST NOT embed GlobusOS, blockchain, AI or application policy.

## Boot hand-off

The boot environment MUST provide, as applicable:

- kernel entry address;
- memory map;
- usable physical memory ranges;
- firmware/ACPI or device-tree information;
- framebuffer information only when explicitly enabled;
- CPU topology;
- command-line/boot configuration as an immutable input;
- bootloader/protocol version.

## Entry invariant

At kernel entry, the kernel MUST establish its own page tables, allocator state, interrupt configuration and scheduler state before exposing user/kernel services.

## BIOS / UEFI

The current repository contains a `boot` image-builder using the `bootloader` crate. This is an implementation path, not the definition of the kernel ABI. Future bootloaders MUST conform to the documented hand-off contract.

## Verification

Boot support is only considered implemented when CI can build the kernel and image for the declared target and a boot/smoke test validates the entry path. A successful Rust compilation alone is not boot evidence.
