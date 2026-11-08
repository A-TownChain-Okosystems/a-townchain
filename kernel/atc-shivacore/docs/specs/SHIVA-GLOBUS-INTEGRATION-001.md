# SHIVA-GLOBUS-INTEGRATION-001 — GlobusOS Integration Contract

**Status:** DRAFT
**Scope:** ShivaCore ↔ GlobusOS userspace boundary

## Purpose

Define the kernel boundary required by GlobusOS without making GlobusOS semantics part of the ShivaCore TCB.

## Kernel responsibilities

ShivaCore owns:

- address spaces and memory protection
- threads and scheduling
- IPC primitives
- interrupts and timers
- capability creation/transfer/revocation
- low-level resource accounting
- architecture-specific traps and context switching

## GlobusOS responsibilities

GlobusOS owns:

- process/service policy
- VFS and storage services
- network services
- device/driver services
- graphics/audio
- package/update lifecycle
- identity and policy services above kernel capabilities
- Aurora integration

## Forbidden kernel dependencies

ShivaCore MUST NOT depend on:

- Aurora or LLM/model runtimes
- A-TownChain state or consensus
- ATCLang syntax or contracts
- desktop/window-manager policy
- package repositories
- network protocol policy

## Capability handoff

The boot/runtime integration shall provide GlobusOS only the capabilities explicitly assigned by the kernel boot contract. Missing or invalid capabilities fail closed.

## Readiness

This document is an integration contract, not evidence that the full GlobusOS boot path is implemented. Implementation and conformance evidence must be recorded separately.
