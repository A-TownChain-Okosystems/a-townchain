---
document_id: SHIVA-KERNEL-REUSE-001
title: ShivaCore Reusable Kernel Contract
version: 1.0.0
status: active
---

# ShivaCore Reusable Kernel Contract

## Purpose

ShivaCore is intended to be usable as the kernel foundation of more than one operating system. The reusable boundary is the kernel contract, not the GlobusOS application stack.

## Layering

```text
Firmware / Bootloader
        |
        v
Architecture HAL
        |
        v
ShivaCore Microkernel / TCB
  - capabilities
  - address spaces / memory
  - scheduling
  - IPC
  - timers
  - minimal interrupt primitives
        |
        v
Kernel-facing OS services
        |
        v
OS userspace
```

An OS using ShivaCore MUST NOT require A-TownChain, ATCLang, Aurora AI, or GameFi semantics in the kernel contract.

## Kernel responsibilities

The kernel contract covers:

1. boot hand-off and kernel entry;
2. CPU/architecture initialization through a HAL;
3. physical/virtual memory primitives;
4. address-space isolation;
5. capability/object authorization;
6. threads, scheduling and context switching;
7. interrupt/timer primitives;
8. IPC and endpoint semantics;
9. kernel panic/fault behavior;
10. controlled transition to userspace.

## Non-kernel responsibilities

Filesystems, network protocols, blockchain state, smart contracts, AI inference, package managers, device policy, graphical UI and OS policy belong above the microkernel unless a future specification explicitly promotes a primitive into the TCB.

## Reuse rule

A new OS may implement its own userspace and services while reusing ShivaCore unchanged. Platform-specific code MUST be isolated behind the HAL/boot boundary.

## Evidence requirement

A target is considered reusable only when CI demonstrates a successful build of the kernel for that target and the target-specific boot contract is documented. README claims alone are insufficient evidence.
