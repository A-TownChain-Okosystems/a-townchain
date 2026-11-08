# AURORA-AI-COMPUTE-PLATFORM-001 — GlobusOS Integration

**Status:** ARCHITECTURE_ONLY  
**Role:** GlobusOS integration contract  
**Normative authority:** `atc-standards`

## Boundary

GlobusOS provides system services between Aurora and ShivaCore.

```
Aurora
  |
  v
Aurora Runtime
  |
  v
GlobusOS AI Services
  |
  v
ShivaCore IPC / Capabilities
  |
  v
ATC Hardware HAL
```

## AI Services

The GlobusOS layer may provide AI IPC, model service, memory service, RAG service, tool runtime, storage integration, network integration, device/service discovery, and audit/event integration. Each service must have an explicit capability and IPC contract.

## Hardware Separation

GlobusOS does not expose raw hardware access to Aurora.

```
Aurora
  -> GlobusOS Service
  -> ShivaCore IPC
  -> HAL
  -> Device Service / Driver
  -> Hardware
```

## Compute Backends

```
ATC AI Runtime
      |
      +---- CPU
      +---- GPU
      +---- NPU
```

The operating-system layer owns lifecycle, isolation, resource accounting, IPC and device-service boundaries. AI semantics remain in Aurora/ATC AI Runtime.

## Security

Required architectural boundaries include secure boot, measured boot, hardware-backed key services, TEE integration where the target platform provides a verifiable TEE, capability-controlled device access, auditable AI service operations, and fail-closed authorization for privileged operations.

A target platform is not considered secure merely because the corresponding hardware feature exists.

## Source of Truth

This repository is the GlobusOS integration SSOT. The active ShivaCore kernel implementation remains in:

```
globus-os/modules/atc-shivacore/kernel/
```

This document does not move or duplicate the kernel source.
