---
document_id: ATC-DOC-AI-HW-001
title: ATC AI Compute Backend Contract
version: 0.1.0
status: proposed
owner: A-TownChain-Okosystems
updated: 2026-10-11
standard: atc-standards
---

# ATC AI Compute Backend Contract

## 1. Status and scope

This document specifies the proposed vendor-neutral backend boundary for Aurora AI and the ATC AI Runtime. It is not implementation or hardware compatibility evidence. A backend is supported only for the exact device, driver, firmware, runtime, and model profile that has passed its required tests.

## 2. Ownership

- **Aurora Authority Plane:** policy, planning, model and workload selection. It does not gain device or kernel authority.
- **Aurora Runtime / ATC AI Runtime:** workload lifecycle, backend selection, cancellation, quotas, provenance, and normalized result/error reporting.
- **GlobusOS AI/device services:** device discovery, authorized resource allocation, memory handles, queue/service access, device reset, and revocation.
- **ShivaCore:** capability enforcement, IPC, scheduling and memory protection. Vendor compute runtimes are not part of the kernel TCB.
- **Vendor adapters:** isolated implementations for CPU, GPU, or NPU execution. They must not bypass GlobusOS authorization or access raw device registers/DMA directly from Aurora.

## 3. Logical interface

A backend adapter must implement equivalent semantics for these operations (names are conceptual until a source-level ABI is approved):

- `enumerate_authorized_devices()`: return only devices exposed by the authorized GlobusOS service.
- `query_capabilities(device)`: return explicit supported operations, precision modes, memory limits, and known limitations.
- `allocate_buffer(device, size, policy)`: allocate bounded resources through an authorized service.
- `submit(workload, buffers, limits)`: submit a validated workload with execution and memory budgets.
- `poll_or_wait(job)`: return pending/completed/failed/cancelled state without exposing vendor-specific internals.
- `cancel(job)`: request cancellation and report whether termination is confirmed.
- `release(resource)`: release resources idempotently and invalidate stale handles.
- `recover(device)`: surface device loss/reset; recovery must not silently replay a non-idempotent workload.

The eventual ABI must define handle ownership, version negotiation, integer widths, error codes, alignment, maximum payload sizes, cancellation guarantees, and backward-compatibility rules before implementation.

## 4. Backend families

| Backend class | Candidate integrations | Policy |
|---|---|---|
| CPU | Native CPU execution / portable inference runtime | Baseline fallback where the model and runtime support it |
| AMD GPU | ROCm-capable devices or other supported adapter | Optional; exact model and software versions required |
| Intel GPU | Supported Intel GPU compute stack | Optional; exact model and software versions required |
| NVIDIA GPU | CUDA-capable devices or other supported adapter | Optional; exact model and software versions required |
| NPU | Platform-specific supported runtime | Optional; capability query required; never assume every CPU platform has an NPU |

CUDA and ROCm are optional adapter integrations, not mandatory dependencies of the Aurora core, GlobusOS, or ShivaCore. Vendor names and architecture labels are not sufficient evidence of support.

## 5. Workload and security contract

Every submission must be bounded by:

- maximum input/output sizes and memory budget;
- execution deadline, queue depth, and cancellation policy;
- an explicit model/runtime/backend compatibility decision;
- capability-checked device and buffer handles;
- authorization and approval policy for privileged operations;
- audit/provenance metadata without secrets or private key material.

Models and agents may propose work but cannot directly authorize it. Device handles must be unforgeable at the service boundary, scoped to a caller, revocable, and rejected when stale or unauthorized. No raw physical address, unrestricted DMA mapping, device register access, or kernel-memory access is exposed to Aurora.

## 6. Reproducibility and observability

Each run records, where applicable:

- source commit SHA and runtime/adapter versions;
- device vendor/device/subsystem identifiers and firmware;
- model identifier and cryptographic digest;
- precision/quantization mode and relevant execution options;
- resource limits, normalized result state, and error classification;
- test/evidence references.

Numerical results across different accelerators may vary. Tests must distinguish exact protocol/ABI determinism from numerical tolerance requirements; tolerance must be explicit and justified for each workload.

## 7. Required conformance tests

1. ABI version negotiation and unsupported-version rejection.
2. Unauthorized device and buffer access rejection.
3. Invalid, oversized, misaligned, and stale handle rejection.
4. Memory quota, timeout, queue saturation, and cancellation behavior.
5. Device loss, reset, allocation failure, and partial submission cleanup.
6. Model/backend incompatibility and unsupported precision rejection.
7. Secret-free logs and audit provenance.
8. CPU fallback behavior when an optional accelerator is absent or blocked.
9. Exact-commit CI and integration tests through GlobusOS authorized services.

A passing CPU mock or QEMU test verifies only the tested contract path; it does not certify an AMD, Intel, or NVIDIA physical device.

## 8. Status gates

Use `ARCHITECTURE_ONLY`, `SPECIFIED`, `IMPLEMENTED`, `TESTED`, `CI_VERIFIED`, `INTEGRATED`, and `E2E_VERIFIED`, with `MISSING`, `BLOCKED`, or `RESIDUAL` for exceptions. Advancement requires source-symbol-to-contract-to-test traceability and evidence tied to the exact source SHA. A document, directory, successful compile, or PR merge alone is not implementation evidence.

## 9. Next implementation sequence

1. Inventory existing Aurora runtime/backend source symbols and tests.
2. Reconcile the proposed operations with the existing model/runtime ABI and ATC standards.
3. Approve a versioned source-level ABI and error model.
4. Implement a CPU reference/mock adapter and conformance tests.
5. Add GlobusOS-mediated device discovery and allocation.
6. Add vendor adapters individually, with model-level real-hardware evidence.
7. Enable only backends whose exact-SHA gates pass.
