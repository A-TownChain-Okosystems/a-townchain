---
document_id: ATC-DOC-SHIVACORE-ARCH-001
title: Microkernel Architecture Specification - atc-shivacore
version: 2.0.0
status: active
standard: ATC-STD-MD-001
created: 2026-09-08
updated: 2026-09-14
---

# Architecture — atc-shivacore

## System role

`atc-shivacore` is a reusable capability-based Rust/no_std microkernel. It is the kernel/TCB foundation for GlobusOS, but its kernel contract is OS-neutral.

The kernel MUST NOT depend on A-TownChain, ATCLang, Aurora AI or GameFi semantics for its core operation.

## Layer model

```text
Firmware / Bootloader
        |
        v
Architecture HAL
        |
        v
ShivaCore Microkernel / TCB
  +-- capability / CSpace
  +-- memory / address spaces
  +-- scheduling / threads
  +-- IPC / endpoints
  +-- timers / traps
        |
        v
Kernel-facing OS services
        |
        v
OS userspace / applications
```

## TCB boundary

The minimum TCB consists of the mechanisms required to enforce isolation and controlled privilege:

- capability validation;
- address-space and memory protection;
- scheduling and context switching;
- IPC endpoint authorization;
- interrupt/trap handling;
- timer primitives;
- architecture-specific mechanisms required by those functions.

Filesystem policy, network protocol stacks, blockchain execution, AI services, package management and application policy SHOULD remain outside the TCB.

## Core components

### Capability / CSpace

Capabilities are the authorization primitive for kernel objects. Access MUST be explicit; the kernel MUST NOT grant ambient authority.

### Memory

Memory management provides allocator and address-space primitives while preserving kernel/userspace isolation.

### Scheduler

The scheduler manages kernel execution contexts. Scheduling algorithms are implementation details and MUST NOT compromise isolation or starvation guarantees.

### IPC

IPC is the primary service boundary. Message delivery MUST be checked against the sender's and receiver's authorized capabilities.

### HAL

Architecture-specific CPU, memory, interrupt and timer operations are isolated behind the HAL contract defined in `docs/specs/SHIVA-HAL-001.md`.

### Boot

Boot is a separate image/entry mechanism. The current repository provides a BIOS/UEFI image-builder; the kernel contract is defined independently in `docs/specs/SHIVA-BOOT-001.md`.

## Current repository implementation boundary

The repository contains historical or experimental kernel modules beyond the minimal TCB. Their presence does not automatically make them part of the normative kernel contract. New TCB APIs MUST be introduced through the documented boundary rather than by adding OS/application policy to the kernel.

## OS reuse

A different operating system can consume ShivaCore as:

```text
OS-A userspace ----+
                   |
OS-B userspace ----+--> ShivaCore --> Hardware
                   |
GlobusOS userspace-+
```

OS-specific services remain above the kernel boundary.

## Blockchain boundary

```text
ATCLang -> ATC-VM -> A-TownChain
```

This execution path is separate from the ShivaCore kernel contract.

## Verification requirements

A target is not considered supported merely because source code exists. CI/release evidence must demonstrate:

1. kernel compilation for the target;
2. target-specific HAL coverage;
3. boot/image generation where boot support is claimed;
4. kernel smoke or integration testing;
5. ABI/IPC tests;
6. security/audit evidence for privileged `unsafe` code.
