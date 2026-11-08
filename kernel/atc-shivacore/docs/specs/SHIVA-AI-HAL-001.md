# SHIVA-AI-HAL-001 — ShivaCore AI Hardware Boundary

**Status:** ARCHITECTURE_ONLY  
**Role:** ShivaCore TCB/HAL boundary specification  
**Normative authority:** `atc-standards`

## Purpose

Define the security boundary through which GlobusOS and Aurora may access CPU, GPU and NPU resources.

## Principle

ShivaCore does not implement AI semantics. It provides the isolation, IPC, capability and hardware boundary required by AI services.

```
Aurora
  |
  v
GlobusOS AI Service
  |
  v
ShivaCore Capability / IPC
  |
  v
ATC Hardware HAL
  |
  +---- CPU HAL
  +---- GPU HAL
  +---- NPU HAL
  +---- Security HAL
```

## NPU Boundary

The NPU is treated as an untrusted compute accelerator unless a separate security contract establishes stronger guarantees.

Aurora must never receive unrestricted MMIO access, physical-address access, DMA control, device-register access, or firmware replacement authority.

## Security HAL

The security boundary may expose controlled services for measurement, attestation, key operations, sealing/unsealing, hardware randomness, and secure-execution integration.

TPM, secure processor and TEE are separate capabilities and must not be conflated.

## Active Kernel SSOT

The canonical active ShivaCore kernel source is:

```
globus-os/modules/atc-shivacore/kernel/
```

This repository contains reusable contracts, specifications and supporting material; it is not a second kernel SSOT.

## Evidence

Implementation and verification status must be established by the active source tree, tests and exact-source CI evidence. Documentation alone remains ARCHITECTURE_ONLY.
