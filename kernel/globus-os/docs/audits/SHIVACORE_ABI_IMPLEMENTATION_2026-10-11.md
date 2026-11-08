# ShivaCore ABI Boundary, Phase 1

- **Commit scope:** first ABI contract implementation
- **Base:** `fef3af85ff9ecab90c25bc179e2a3284fa17f633`
- **Status:** `IMPLEMENTED / TESTED`, not final ABI freeze

## Implemented

The existing `system/shivacore-api` crate now exposes the first stable primitives needed by Service-Space:

- `Hash32`
- deterministic compatibility `simple_hash`
- public `CapabilityRights` bit representation
- public `KernelBootPhase` values

The hash-dependent Service-Space modules now use `libshivacore::simple_hash` instead of importing the kernel's private `security` implementation. The kernel crate remains the implementation owner; the wire-compatible contract is defined outside the kernel.

## Verification

```text
cargo test -p libshivacore --lib       PASS, 5/5
cargo test -p shivacore --lib          PASS, 546/546
cargo test -p shivacore-service-space --lib  PASS, 548/548
```

## Explicit remaining work

This phase does not claim that all Service-Space coupling is removed. The following remain for later ABI phases:

- `capability` types and table access
- `ats1000::NetworkStack`
- `security::TokenBucket`
- `kernel_init::KernelState::boot`

Those migrations require explicit contract semantics and focused tests. They must not be replaced by aliases that leak kernel internals back into the public boundary.
