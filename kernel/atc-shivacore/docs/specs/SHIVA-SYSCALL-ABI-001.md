---
document_id: SHIVA-SYSCALL-ABI-001
title: ShivaCore Syscall ABI v1
version: 1.0.0
status: active
---

# ShivaCore Syscall ABI v1

The canonical active kernel implementation is in globus-os. This specification defines the reusable ABI contract.

## Invariants
- Architecture-neutral syscall IDs.
- Versioned ABI with major/minor compatibility.
- Unknown IDs fail closed.
- Capability and rights validation precede privileged work.
- User memory lengths and pointers are validated before dereference.
- No ambient authority.
- Errors are stable and explicit.

## Current IDs
| ID | Name | Authority |
|---:|---|---|
| 0x0001 | YIELD | none |
| 0x0010 | IPC_SEND | IPC + WRITE |
| 0x0011 | IPC_RECEIVE | IPC + READ |
| 0x0020 | CAPABILITY_QUERY | INSPECT |
| 0x0030 | HANDLE_CLOSE | explicit handle authority |
| 0x0040 | MONOTONIC_TIME | none |

## Reserved domains
Scheduler/process, IPC, capabilities, handles/objects, time/randomness, memory, process/thread, filesystem, networking, devices, containers, runtime/VM and audit/telemetry each receive non-overlapping ID ranges.

## Required verification
Implementations must test malformed input, invalid handles, owner mismatch, rights mismatch, revocation, namespace mismatch, quota exhaustion, concurrency, cancellation, audit and recovery.
