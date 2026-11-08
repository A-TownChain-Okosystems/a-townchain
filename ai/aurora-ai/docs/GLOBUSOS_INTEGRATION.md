# Aurora ↔ GlobusOS Integration Contract

## Boundary

Aurora is a userspace AI platform. It runs above GlobusOS and never receives implicit kernel authority.

```text
Aurora AI
  ↓ explicit IPC/API
GlobusOS service layer
  ↓ capabilities
ShivaCore
```

## Required controls

- AI tool calls MUST be mediated by an OS permission/capability broker.
- AI agents MUST NOT receive ambient filesystem, network, device or process authority.
- Kernel capabilities MUST NOT be directly exposed as model/tool objects.
- Sensitive operations require an explicit policy decision and, where configured, human approval.
- AI output MUST NOT override deterministic kernel, security, chain or governance authority.

## Integration surfaces

1. IPC request/response transport.
2. System event subscription with least-privilege filters.
3. Capability-backed tool broker.
4. Model/runtime resource broker for CPU/GPU/NPU quotas.
5. Audit/evidence events for security-sensitive actions.

## Readiness

This document defines the integration contract. It does not claim that the complete GlobusOS bridge or production AI sandbox is implemented.
