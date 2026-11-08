# GlobusOS Settings

Capability-controlled settings service for GlobusOS.

## Scope

`globus-settings` is the policy boundary between Settings UI/API callers and privileged GlobusOS services. It does not access hardware, firmware, the filesystem, TPM, wallet keys, or ShivaCore directly.

## Domains

- system
- security
- privacy
- network
- display
- audio
- storage
- applications
- identity
- ai
- blockchain
- developer
- updates

## Security model

Settings mutations are deny-by-default and must carry a domain-specific capability. A caller authorized for one domain cannot mutate another domain through this crate.

The service is intentionally stateless at this layer; persistence, audit evidence, IPC transport, and hardware-specific operations remain in their respective GlobusOS service boundaries.

## Validation

```bash
cargo check -p globus-settings
cargo test -p globus-settings
cargo clippy -p globus-settings -- -D warnings
```
