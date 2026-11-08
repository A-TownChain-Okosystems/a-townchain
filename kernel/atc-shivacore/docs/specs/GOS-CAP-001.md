---
document_id: GOS-CAP-001
title: Capability Security Model
version: 1.0.0
status: active
---

# Capability Security Model

Authority is represented explicitly as capability + rights + namespace + resource.

## Required checks
1. caller identity
2. handle validity
3. object lifecycle
4. namespace compatibility
5. resource type
6. requested rights
7. revocation state
8. quota/policy
9. audit

Delegation is explicit. Capability transfer must distinguish copy and move semantics. Revocation must be lineage-aware.

The kernel is the security enforcement point; userspace services are policy consumers and cannot bypass the kernel boundary.
