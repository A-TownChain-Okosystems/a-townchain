# Multi-account identity transaction model

Status: design contract; not an implementation or production-readiness claim.

Issue: #67  
Scope: GlobusOS `system/identity`

## Problem statement

The current `IdentityService` holds one `profile`, one credential hash, and one session. `FileAccountDirectory` separately persists username-to-`UserId` mappings. Its module documentation explicitly states that it is not a transaction across the directory and account-record stores. Committing these two resources independently can leave an orphaned username or an unreachable account after a crash.

## Required invariant

A registration or account mutation must have one durable commit point. After any process crash, reopening storage must yield either the complete prior state or the complete new state; it must never expose a username index entry without its corresponding valid account record, or an account record whose username is absent or points to another account.

The same invariant applies to account deletion, username changes, password changes, recovery rotation, and session-revocation metadata.

## Proposed storage boundary

Introduce one account-state repository abstraction rather than coordinating two independently committed files:

- `load_state()` returns a validated versioned state containing the username index and account records.
- `commit_state(expected_generation, next_state)` validates all invariants and atomically replaces the complete state.
- `lookup_username(username)` resolves only entries whose account record exists and validates.
- `load_account(user_id)` returns only a validated record; absent and corrupt records fail closed.
- `delete_account(user_id)` removes the account and username mapping in the same transaction.
- A conflict on `expected_generation` returns a conflict and must be retried by the caller; it must not silently overwrite newer state.

The username index must contain identifiers only, never credentials, recovery phrases, wallet private keys, or Aurora node keys. The full state must be protected by the existing encrypted-blob boundary and its `IdentityKeyProvider`; the username index must not become a plaintext sidecar.

## Filesystem commit protocol

If the repository uses the existing atomic encrypted-blob backend, the complete serialized state is one encrypted blob:

1. Acquire the repository's exclusive writer lock.
2. Load and validate the current generation and complete state.
3. Apply the mutation in memory and validate uniqueness and cross-record invariants.
4. Serialize with a versioned, bounded, canonical binary format. Reject unknown versions, duplicate IDs/usernames, truncated fields, oversized counts, trailing bytes, and invalid references.
5. Encrypt/authenticate through `EncryptedBlobStore` and commit the next generation using the atomic file replacement protocol.
6. Publish success to the caller only after the durable commit succeeds; otherwise return an error and leave the previous state authoritative.

The atomic rename is the commit point. A crash before it must preserve the old state; a crash after it must reopen the new complete state. Directory fsync and the encrypted store's generation/rollback contract must be verified on the supported filesystems. Do not silently remove a possibly-live lock. Stale-lock recovery must be an explicit, auditable operation or use a lock primitive with reliable OS ownership semantics.

## Account lifecycle and key boundaries

- Keep account credential hashes, wallet signing keys, and Aurora node keys as distinct types and storage domains.
- Registration must not become active until the recovery phrase has been shown by a trusted local UI and the selected-word challenge has succeeded.
- Recovery phrases and plaintext credentials must never be serialized into account state, logs, telemetry, untrusted IPC, or Aurora requests.
- Password change and recovery must atomically replace credential/recovery metadata and revoke all prior sessions.
- Session state should be ephemeral unless a separately reviewed persistence design explicitly requires otherwise.
- Wallet derivation/signing must cross the canonical wallet/keystore boundary; this repository must not claim compatibility from its current custom derivation code.

## Required tests before acceptance

### Atomicity and crash recovery

- Inject failures before temp creation, during serialization, during write, after file sync, before rename, after rename, and during parent-directory sync.
- Reopen storage after every injected failure and assert that the result is exactly the old or new complete state.
- Verify a username can never resolve to a missing or different account.
- Verify generation rollback and stale-writer conflicts fail closed.

### Multi-account and concurrency

- Register two or more distinct usernames and log each in independently.
- Reject case-folded duplicate usernames and duplicate user IDs.
- Race concurrent registrations for the same username; exactly one transaction may commit.
- Delete or mutate one account without affecting another account or its session.

### Security and lifecycle

- Reject malformed, truncated, oversized, unsupported-version, duplicate, and cross-reference-inconsistent state.
- Verify failed registration leaves no directory or account residue.
- Verify password/recovery changes revoke all prior sessions and old recovery material.
- Verify no secret appears in logs, telemetry, serialized directory metadata, or untrusted IPC.
- Exercise key-provider failure and corrupted/authentication-failed encrypted blobs.

## Evidence and merge gate

For every gate record `PASS`, `FAIL`, or `RESIDUAL` against the exact head SHA. Include GitHub Actions Run → Job → Step → exit/log references. Do not weaken CI, merge on green CI alone, or label this design document as implementation evidence.

The current multi-account integration, cross-store crash consistency, canonical wallet integration, recovery/session revocation, login throttling, and secret-flow tests remain unverified until code and exact-SHA test evidence prove them.
