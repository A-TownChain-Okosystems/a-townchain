# A-TownChain Ecosystem — UI Menu and Settings Catalog

- **Catalog ID:** `ATC-ECOSYSTEM-UI-SETTINGS-001`
- **Version:** `0.1.0-draft`
- **Status:** `PROPOSED / SOURCE-RECONCILIATION-INCOMPLETE`
- **Review date:** 2026-10-15
- **Scope:** Cross-ecosystem navigation, user-visible actions, and settings ownership.
- **Authority:** This file is an integration catalog, not an implementation or a claim of runtime availability.

## Purpose and ownership

This catalog defines the shared navigation vocabulary and the cross-repository contract for settings. It does not move or duplicate implementation ownership.

| Area | Canonical owner | Responsibility in this catalog |
|---|---|---|
| Ecosystem integration, cross-repository contracts and evidence | `a-townchain-ecosystem` | Index, reconcile ownership, record evidence and residuals; no duplicate feature implementation |
| Desktop, Control Center, system settings and OS APIs | `globus-os` | Canonical OS/UI implementation; see [PR #27](https://github.com/A-TownChain-Okosystems/globus-os/pull/27), whose catalog remains a draft and requires source reconciliation |
| ShivaCore kernel | `globus-os/modules/atc-shivacore/kernel/` | Kernel/TCB authority and privileged service boundaries |
| Aurora AI | `aurora-ai` | Agent/model/tool orchestration and AI policy integration; model output never grants authority |
| Blockchain, wallet, node and L1 transaction behavior | `a-townchain` | Canonical L1 and wallet/node behavior; UI only calls supported authoritative APIs |
| ATC-VM | `a-townchain/components/vm` | Canonical VM; ecosystem UI must not implement a second VM |
| ATCLang | canonical `atclang` repository | Language/toolchain ownership; UI surfaces only supported interfaces |
| Normative cross-system requirements | `atc-standards` | Standards and normative contract registry |

Imported `components/*` directories in the ecosystem repository are integration snapshots, not alternate SSOTs. Their freshness and exact source SHAs must be reconciled before making source-backed claims.

## Navigation catalog

All entries below are **proposed navigation contracts**, not proof that a screen, route, action, API, or implementation currently exists.

| ID | Menu | Included surface | Primary owner | Initial status |
|---|---|---|---|---|
| `NAV-OVERVIEW` | Übersicht | System status, alerts, resources, recent events, evidence summary | `globus-os` | `UNASSESSED` |
| `NAV-SYSTEM` | System | Kernel status, services, hardware, network, storage, updates | `globus-os` | `UNASSESSED` |
| `NAV-PERSONAL` | Persönlich | Profile, language, appearance, accessibility, notifications | `globus-os` | `UNASSESSED` |
| `NAV-SECURITY` | Sicherheit | Identity, keys, capabilities, policies, Secure Boot, TPM/TEE status, audit | `globus-os` + authoritative identity/security services | `UNASSESSED` |
| `NAV-ADMIN` | Verwaltung | Users, devices, services, organization policies and administration | `globus-os` | `UNASSESSED` |
| `NAV-AURORA` | Aurora AI | Models, agents, tools, memory, permissions, approval queue, AI audit | `aurora-ai` + `globus-os` authorization boundary | `UNASSESSED` |
| `NAV-ATC` | A-TownChain | Network profile, wallet connection, transactions, node/chain status | `a-townchain` | `UNASSESSED` |
| `NAV-DEV` | Entwicklung | ATCLang, canonical ATC-VM, SDKs, APIs, diagnostics, logs, test environments | Each canonical developer-tool repository | `UNASSESSED` |
| `NAV-TRUST` | Trust Center | Identity posture, capability grants, policy decisions, security events and evidence | `globus-os` + canonical security services | `UNASSESSED` |

A menu entry may be split into separate screens when the canonical API and authorization boundaries differ. Menu presence must not be used as evidence that the underlying capability is implemented.

## Settings catalog

Every item is an intended setting domain. Individual keys, schemas, defaults, APIs, persistence semantics, owners and tests still require source-level inventory. Do not infer that all listed controls exist.

| ID | Domain | Candidate settings / controls | Authoritative owner | Initial status |
|---|---|---|---|---|
| `SET-SYSTEM` | System | Startup behavior, service lifecycle, resource limits, update channel and update state | `globus-os` | `UNASSESSED` |
| `SET-HARDWARE` | Hardware | CPU/GPU/NPU visibility, driver status, device permissions, accelerator selection | `globus-os` + device services | `UNASSESSED` |
| `SET-NETWORK` | Netzwerk | P2P connectivity, connection profiles, DNS, proxy and network policy | `globus-os` + network services | `UNASSESSED` |
| `SET-IDENTITY` | Identität | User/device identity, key-provider references, session and identity lifecycle | Canonical identity/security service | `UNASSESSED` |
| `SET-SECURITY` | Sicherheit | Secure Boot status/configuration, TPM/TEE integration, capability grants, policy rules | `globus-os` + ShivaCore/security services | `UNASSESSED` |
| `SET-PRIVACY` | Datenschutz | Telemetry, data sharing, retention and export controls | Service that owns each data store; UI in `globus-os` | `UNASSESSED` |
| `SET-AURORA` | Aurora AI | Model/provider selection, agent enablement, tool scopes, memory/data access, approval requirements | `aurora-ai` + `globus-os` authorization boundary | `UNASSESSED` |
| `SET-BLOCKCHAIN` | Blockchain | Network profile, RPC endpoint, wallet attachment, transaction limits and confirmation policy | `a-townchain` / canonical wallet and node APIs | `UNASSESSED` |
| `SET-DEVELOPER` | Entwickler | SDK/toolchain selection, API endpoints, diagnostics, logging and test environments | Respective canonical developer-tool repository | `UNASSESSED` |
| `SET-APPEARANCE` | Oberfläche | Theme, layout, language, accessibility, notifications | `globus-os` | `UNASSESSED` |

## Mandatory setting contract

Each concrete setting must have a stable record that resolves this chain:

`SETTING_ID → SPEC → DOC → OWNER → SCHEMA → READ/WRITE API → AUTHORIZATION → IMPLEMENTATION SHA → TEST → EXACT-SHA CI → AUDIT`

Minimum per-setting fields:

- Stable ID, display label, purpose, category and sensitivity classification.
- Canonical owner repository, exact source HEAD, source path, symbol/API and blob SHA.
- Typed schema, allowed values, default, validation and migration semantics.
- Read/write API contract, error behavior, persistence owner and concurrency semantics.
- Required identity, capability, policy decision, scope and approval requirements.
- Tests, exact-head CI run/job/step/exit code/log or artifact, audit event and residuals.
- Explicit state: `NOT_COLLECTED`, `PENDING`, `FAILED`, `BLOCKED`, `VERIFIED`, or `RESIDUAL`.

Missing evidence must remain missing; it must not be silently upgraded to `VERIFIED`.

## Security and authority invariants

1. A UI toggle is a request, not authority. Sensitive writes must pass through the authoritative API and policy boundary.
2. Required sequence: `UI → API → Identity → Capability → Policy → Validation → Authoritative State → Audit`.
3. Unknown identity, capability, policy, schema, or approval context fails closed.
4. Aurora follows `Model → Agent → Capability → Policy → Approval (when required) → Tool → authorized service`. Model output does not grant privilege.
5. Wallet signing, transaction submission, consensus, VM execution, kernel/TCB changes, TPM/TEE operations and security-policy changes must never be implemented as UI-side bypasses.
6. Read-only status and verified evidence must be distinguishable from desired configuration, requested state and unverified telemetry.
7. No UI control may claim that Secure Boot, TPM/TEE, production readiness, blockchain finality or an AI action is verified without authoritative evidence.

## Source reconciliation and completion gates

The initial catalog is a domain-level baseline only. Completion requires a bidirectional, exact-source-HEAD reconciliation in each canonical repository:

1. Enumerate actual UI routes, screens, commands, actions, settings keys, schemas, read/write APIs, IPC/RPC surfaces and service symbols.
2. Map every discovered runtime/UI symbol to a catalog ID, or record a reproducible exclusion reason (test-only, generated, documentation, build/CI, internal helper, deprecated, unreachable, duplicate, or non-user-facing).
3. Map every catalog item to a real source symbol/API or mark it `BLOCKED` / `RESIDUAL` with an explicit reason; do not count design mocks as implementation.
4. Detect duplicates and overlapping functions; keep one canonical owner and link aliases rather than duplicating implementations.
5. Validate schema and catalog invariants, then run strict bidirectional reconciliation with completeness required.
6. Record exact HEAD SHA and full Run → Job → Step → exit code/log or artifact evidence. A structural validator pass alone is not functional verification.
7. Keep the PR Draft and release state blocked until all mandatory gates pass; do not merge, weaken CI, bypass authorization, or claim end-to-end verification based on catalog presence.

## Known residuals at draft creation

- `globus-os` PR #27 already contains a detailed OS/UI/settings catalog baseline. This cross-ecosystem file references it; it does not replace or duplicate it.
- Cross-repository source heads and UI/API contracts have not been reconciled by this document.
- No menu or setting in this catalog is marked implemented or verified solely because it appears here.
- Concrete setting IDs beyond the domain-level IDs above, schemas, owner symbols, API contracts, tests and exact-SHA evidence remain to be collected.

## Related records

- [GlobusOS OS/UI/settings catalog PR #27](https://github.com/A-TownChain-Okosystems/globus-os/pull/27)
- [Ecosystem architecture and SSOT ownership](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/blob/main/README.md)
- [Ecosystem source-reconciliation PR #115](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/pull/115)
