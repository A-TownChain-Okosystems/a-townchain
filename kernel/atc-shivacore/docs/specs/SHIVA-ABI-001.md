---
document_id: SHIVA-ABI-001
title: ShivaCore Kernel Userspace ABI Boundary
version: 1.0.0
status: active
---

# ShivaCore Kernel / Userspace ABI Boundary

## Principle

The microkernel MUST expose a small, stable kernel-facing contract. OS services and applications MUST communicate through explicitly authorized capabilities and IPC rather than direct kernel-global state.

## Required object classes

The ABI model reserves explicit handles/capabilities for:

- address spaces;
- threads;
- IPC endpoints;
- notifications/events;
- memory objects;
- interrupt/timer resources where exposed;
- controlled device resources.

## Syscall model

The concrete syscall numbers are architecture-neutral identifiers. Architecture-specific entry instructions are implementation details of the target port.

A syscall MUST:

1. validate the caller context;
2. validate the capability/handle;
3. validate user pointers and lengths before dereference;
4. perform the minimum privileged operation;
5. return an explicit success/error result;
6. never grant ambient authority implicitly.

## ABI stability

Breaking changes require a versioned ABI revision and compatibility evidence. Internal kernel structs MUST NOT be treated as a userspace ABI.

## OS independence

GlobusOS is one consumer of this ABI. Another OS MAY provide a different service model while using the same kernel contract.
