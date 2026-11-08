# GlobusOS Account ↔ Wallet ↔ Aurora Node Binding v1.0.0

**Document ID:** GOS-IDENTITY-AURORA-BINDING-001  
**Status:** SPECIFIED / implementation and verification pending  
**Owner:** GlobusOS Identity Service  
**Canonical layer:** GlobusOS userspace; wallet cryptography remains owned by `atc-wallet`; kernel enforcement remains owned by ShivaCore.

## 1. Purpose

Define a verifiable relationship between three distinct identities:

1. **GlobusOS account identity** — local account/profile and authenticated session.
2. **A-TownChain wallet identity** — chain address and public key controlled through the Wallet Service.
3. **Aurora AI node identity** — a node-specific public identity used for peer authentication and swarm membership.

A binding links these identities; it does not collapse them into one credential or transfer authority between them.

## 2. Version 1 cardinality

- Each active GlobusOS account MUST have exactly one **primary wallet binding** before it is marked `ACTIVE`.
- Each account MAY have one registered Aurora node identity per device installation. Additional devices are separate nodes, not cloned node identities.
- Multi-wallet support is out of scope for v1. Future multi-wallet support requires an explicit versioned contract and migration; it must not silently replace the primary wallet.
- A user may keep the Aurora node offline or disable voluntary peer/compute participation without losing the account or wallet binding.

## 3. Identity separation and authority

| Identity | Proof / credential | Authority granted | Must not imply |
|---|---|---|---|
| GlobusOS account | Local authentication credential and session | Access to account-scoped OS services | Wallet signing or kernel privilege |
| Wallet | Wallet-key signature over a fresh challenge | Proof of control of the specified address/key; individual signing operations subject to policy | GlobusOS login, node ownership by itself, or OS/agent privilege |
| Aurora node | Node-key signature and authenticated peer handshake | Peer authentication for the registered node | Wallet custody, user login, or permission to execute peer-supplied code |
| ShivaCore capability | Kernel-mediated capability/IPC policy | Only the specific kernel/service operation named by the capability | Any authority merely because a wallet or node is bound |

The wallet private key MUST remain behind the dedicated Wallet Service boundary. The Aurora node key MUST be distinct from the wallet key and protected separately. GlobusOS login credentials, wallet keys and node keys MUST NOT be reused as each other.

## 4. Binding record

The canonical local record SHOULD contain:

- `binding_version`
- `binding_id` — random, non-secret identifier
- `account_subject` — opaque account identifier; never a password or recovery secret
- `wallet_address`
- `wallet_public_key`
- `chain_id` and `network_id`
- `aurora_node_id`
- `node_public_key`
- `created_at`, `updated_at`, `key_version`
- `challenge_id`, `challenge_nonce`, `challenge_expires_at`
- `wallet_signature` and `node_signature`
- `status` (`PENDING`, `ACTIVE`, `ROTATION_PENDING`, `REVOKED`, `RECOVERY_REQUIRED`)

Do not publish a raw local account ID if a pairwise pseudonymous `account_subject` is sufficient. Do not include private keys, seed phrases, login credentials, or recovery phrases in the binding record, logs, telemetry, peer messages or AI context.

The exact serialization and cryptographic algorithms MUST be selected from the canonical ATC wallet/identity standards before production implementation. Do not invent a parallel wallet address or signature format in GlobusOS.

## 5. Enrollment protocol

1. Create the local account in `PENDING` state and authenticate the user locally.
2. Provision or restore the primary wallet through `atc-wallet`; verify recovery backup according to the wallet contract.
3. Generate a separate node keypair using the platform CSPRNG or an approved protected key provider.
4. Create a fresh, unpredictable, single-use challenge bound to the protocol domain, binding ID, account subject, wallet address/public key, chain/network, node ID/public key, challenge nonce and expiry.
5. Ask the Wallet Service to sign the canonical challenge. The wallet service MUST display/confirm the binding intent as required by policy; it MUST NOT export the private key.
6. Sign the same canonical binding payload with the node key to prove node-key possession.
7. Verify both signatures, key/address consistency, domain, nonce, expiry and one-time challenge status. Reject stale, replayed, malformed or cross-network payloads.
8. Persist the record atomically. Mark the account `ACTIVE` only after the binding verifies and protected persistence succeeds.
9. Emit a redacted audit event containing result/reason and non-secret identifiers only.

Any failure MUST leave the account non-active or restore the prior valid binding. Partial enrollment MUST NOT be presented as successful.

## 6. Wallet or node key rotation

- Require fresh local re-authentication and a new challenge.
- Verify control of the existing wallet/node key where available; otherwise use the separately specified recovery process.
- Bind the replacement key/address with new signatures and an explicit rotation record.
- Do not delete the previous binding until the replacement is verified and atomically committed.
- Revoke the old node identity from peer discovery/session admission after successful rotation.
- Recovery, loss of the old key, account migration and suspected compromise require distinct audited paths; recovery must not silently bypass ownership verification.
- A wallet rotation MUST NOT automatically grant new Aurora capabilities or alter OS/kernel permissions.

## 7. Aurora peer participation and privacy

A registered node may advertise only the minimum public node metadata required for peer discovery. It MUST NOT broadcast the local account profile, wallet recovery material, login identity, private wallet key or unrelated wallet activity.

Peer authentication proves control of a node key, not that remote tasks are safe. Incoming tasks remain subject to local policy, resource quotas, sandboxing and explicit opt-in. No node is required to share CPU/GPU/NPU, storage, model weights or bandwidth by default.

Wallet ownership MUST NOT be used as a blanket reputation score or as authorization to execute arbitrary peer code. Any future payments, staking or on-chain reputation need separate versioned protocols and explicit user consent.

## 8. Security boundaries

```text
GlobusOS Account / Identity Service
    | authenticated session; local account policy
    v
Binding Coordinator
    | challenge + verified signatures; no raw secrets
    +---------------------> atc-wallet / Wallet Service
    |                       signs through protected boundary
    +---------------------> Aurora Node Identity
    |                       separate node key / peer handshake
    v
GlobusOS IPC + capability checks
    v
ShivaCore enforcement boundary
```

Aurora orchestration and wallet ownership are outside the ShivaCore TCB. The OS and kernel must independently enforce service capabilities. A successful binding is an identity assertion, not an authorization bypass.

## 9. Required tests and evidence gates

- Valid wallet and node signatures activate the binding.
- Wrong wallet key/address, wrong node key, wrong domain, wrong chain/network or mismatched payload is rejected.
- Expired, reused and replayed challenges are rejected.
- Tampered local binding records are detected.
- Storage/crypto failure never produces an `ACTIVE` account.
- Wallet/node rotation is atomic; failure preserves the last valid binding.
- Account recovery does not require the original device but cannot bypass the defined recovery proof.
- No private key, seed phrase or recovery phrase appears in logs, telemetry, IPC to untrusted services, peer messages or Aurora context.
- Disabling node participation does not disable account login or wallet access.
- Wallet binding alone never grants OS, kernel, agent, signing or peer-task execution authority.
- Tests run against the exact source SHA; evidence records Run → Job → Step → exit/log. Documentation or a passing unit test alone is not end-to-end verification.

## 10. Ownership and residuals

- **GlobusOS:** account lifecycle, binding coordinator, local persistence, UI confirmation, audit and IPC/service authorization.
- **atc-wallet:** wallet derivation, address/key semantics, protected signing and recovery primitives.
- **Aurora AI:** node lifecycle, node identity use, peer protocol and opt-in resource participation.
- **ATC Standards:** canonical payload encoding, domain separation, challenge rules, node identity format and protocol versioning.
- **ShivaCore:** kernel capability and isolation enforcement; no wallet-specific policy in the TCB.

Current status remains `SPECIFIED`. Production implementation, canonical signature/serialization selection, secure key storage, peer protocol integration, tests and exact-SHA verification remain residuals until separately evidenced.
