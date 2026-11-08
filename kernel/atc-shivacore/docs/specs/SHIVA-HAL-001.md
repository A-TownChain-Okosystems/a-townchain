---
document_id: SHIVA-HAL-001
title: ShivaCore Hardware Abstraction Layer Contract
version: 1.0.0
status: active
---

# ShivaCore HAL Contract

## Goal

The Hardware Abstraction Layer (HAL) isolates architecture and board-specific mechanisms from the reusable microkernel core.

## HAL boundary

```text
ShivaCore core
    |
    +-- cpu
    +-- memory
    +-- interrupt
    +-- timer
    +-- serial/debug (optional)
    |
    v
Target HAL
    |
    +-- x86_64
    +-- aarch64
    +-- future targets
```

## Requirements

Each target implementation MUST provide, directly or through an equivalent typed interface:

- CPU initialization and context representation;
- page/table primitives required by the memory subsystem;
- interrupt/trap entry and dispatch hooks;
- monotonic timer source;
- architecture-safe synchronization primitives where required;
- a boot information hand-off structure;
- explicit target capabilities and unsupported features.

## TCB rule

The HAL is part of the trusted computing base only for mechanisms that are necessary to enforce isolation, memory safety, scheduling, IPC or controlled hardware access. Device drivers and policy SHOULD remain outside the TCB.

## Portability

Architecture-specific `unsafe` code MUST be isolated, reviewed, and covered by target-specific tests. Portable kernel code MUST NOT depend directly on x86_64 or aarch64 implementation details.
