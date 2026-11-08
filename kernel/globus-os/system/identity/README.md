# GlobusOS Identity Service

Identity is the userspace authority for GlobusOS profiles, authentication sessions, wallet identity binding, and recovery orchestration.

## Implemented

The `globus-identity` crate now provides:

- validated `UserId` and non-secret `AccountProfile` types;
- explicit `LoginState` and expiring `IdentitySession` state;
- local 24-word BIP39 wallet generation using 256-bit entropy;
- deterministic recovery from the 24-word phrase;
- ATC address derivation matching the documented ecosystem format;
- `IdentityBinding` linking user ID, wallet address, chain ID, network, and key version;
- zeroizing recovery-material containers;
- tests for generation, recovery, binding, session expiry, and weak seed rejection.

The documented ATC wallet flow is 256-bit entropy → 24 words → PBKDF2-HMAC-SHA512 seed → HMAC-SHA256 private key → SHA-256 public key → ATC address. fileciteturn20file0L2-L2

## Account lifecycle primitives (Issue #67)

The first implementation slice exports `AccountUsername` and `RecoveryConfirmation` from the identity crate:

- Usernames are canonicalized to lowercase ASCII and constrained to 3–32 characters from `[a-z0-9_.-]`, with alphanumeric endpoints. This prevents case-collisions and visually ambiguous Unicode usernames.
- Registration can request three randomly selected recovery-word positions and must not activate the account until the answers verify.
- The short-lived confirmation object zeroizes its expected words when dropped. The caller remains responsible for a trusted local display/input path and must never log or transmit the phrase.

These primitives do **not** yet provide a persistent multi-account username directory, atomic uniqueness enforcement, login throttling, recovery session revocation, production UI/IPC, or the complete account-wallet-Aurora binding coordinator. They are not a production-complete account lifecycle. Wallet derivation must also be reconciled with canonical `atc-wallet` before production signing or address compatibility is claimed.

## Security boundary

The Identity Service does **not** expose private keys to applications. Applications request high-level operations; the Wallet Service performs protected signing after capability and policy checks.

```text
Application
   |
   | identity/sign request
   v
Identity Service
   |
   | capability check
   v
Wallet Service
   |
   | protected key operation
   v
ShivaCore capability boundary
```

## Registration

```text
User registration
    -> User ID / profile
    -> local 24-word recovery phrase
    -> wallet key derivation
    -> ATC wallet address
    -> IdentityBinding
    -> authenticated session
```

The recovery phrase is exposed only through an explicit trusted UI boundary. It must never be logged, uploaded, sent to a server, included in telemetry, or passed into Aurora/AI context. The private key is not returned by the wallet-creation API.

## Recovery

Recovery accepts a user-supplied 24-word phrase, reconstructs the same ATC wallet address, and produces a new local `IdentityBinding`. A platform credential provider must establish the new login credential; the identity crate deliberately does not store passwords.

## Production gates still open

- encrypted persistent keystore with authenticated encryption;
- hardware-backed key protection where available (TPM/TEE/platform keystore);
- capability-mediated Wallet Service implementation;
- signing API and transaction authorization policy;
- integration with the canonical `atc-wallet` signing implementation;
- end-to-end registration/login UI and IPC integration;
- CI build/test execution on supported targets.

The current implementation is therefore a functional identity/wallet derivation baseline, not yet a production-ready key custody system.
