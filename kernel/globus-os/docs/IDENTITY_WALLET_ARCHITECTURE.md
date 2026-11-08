# GlobusOS Identity & Wallet Architecture v1.0.0

**Status:** DRAFT / implementation baseline  
**Scope:** Boot, registration, local authentication, wallet provisioning, recovery, identity binding  
**Security principle:** No secret material leaves the trusted local wallet boundary.

## 1. Objective

GlobusOS shall provide a native identity flow in which a newly created user profile receives an A-TownChain-compatible wallet identity. The public wallet address is bound to the GlobusOS user identity; private key material and the 24-word recovery phrase remain local and are never sent to servers, GitHub, Aurora AI, telemetry, or logs.

The wallet is a cryptographic identity/control mechanism. It is **not** the daily login password.

## 2. Boot-to-desktop flow

```text
UEFI / Secure Boot
  -> GlobusOS bootloader
  -> ShivaCore
  -> capability + IPC initialization
  -> GlobusOS service manager
  -> Identity Service
  -> Login / Register UI
  -> local session
  -> Wallet Service (only when authorized)
  -> Aurora / Desktop / Applications
```

## 3. Registration flow

```text
Register
  -> create local GlobusOS User ID
  -> initialize profile
  -> obtain CSPRNG entropy
  -> generate 24-word recovery mnemonic
  -> derive wallet seed/key hierarchy
  -> derive public key
  -> derive A-TownChain wallet address
  -> create signed identity binding
  -> require recovery-phrase verification
  -> persist encrypted wallet state
  -> establish authenticated session
```

Registration MUST fail closed if cryptographic randomness, key derivation, encrypted persistence, or identity binding cannot be completed.

## 4. Secret-material rules

### Recovery phrase

The 24 words are generated from a cryptographically secure entropy source and are shown only during provisioning/recovery. The UI MUST warn the user that anyone possessing the phrase can recover the wallet.

The phrase MUST NOT be:

- uploaded to a server;
- stored in plaintext on disk;
- included in telemetry or crash reports;
- exposed to Aurora AI or ordinary applications;
- written to GitHub, source control, debug output, or logs;
- used as the normal login credential.

### Private key

Private keys MUST remain behind the Wallet Service capability boundary. Applications receive signatures or public information, not raw private keys.

## 5. Identity binding

A GlobusOS profile and wallet address are related through an explicit binding record:

```text
binding_version
user_id
wallet_address
public_key
chain_id
network
created_at
key_version
signature
```

The binding MUST be cryptographically verifiable. The implementation MUST NOT treat a locally editable `wallet_address` field as proof of wallet ownership.

## 6. Login

Daily login uses a local authentication credential such as a password/PIN or hardware-backed credential. Wallet authorization is requested separately when an operation requires a cryptographic signature.

```text
Login credential
  -> authenticated local session

Application operation
  -> capability check
  -> Wallet Service request
  -> user approval where required
  -> signature
```

The recovery phrase MUST NOT be requested for ordinary login.

## 7. Recovery

```text
Restore Identity
  -> 24-word recovery phrase
  -> validate mnemonic
  -> derive wallet keys
  -> reconstruct public identity
  -> verify address / binding where applicable
  -> create new local authentication credential
  -> encrypt and persist local wallet state
```

A recovery flow MUST NOT require the original device to be online.

## 8. Trust boundaries

```text
                 UNTRUSTED / APPLICATION SPACE
 Aurora / Apps / Desktop
          |
          | IPC + explicit capability
          v
     Identity Service
          |
          | wallet capability
          v
      Wallet Service
          |
          | protected signing
          v
   ShivaCore capabilities
```

Aurora AI and blockchain logic remain outside the ShivaCore TCB. Wallet signing is a privileged operation and MUST be mediated by explicit capabilities and policy.

## 9. Data classification

| Data | Classification | Storage | Application access |
|---|---|---|---|
| User ID | public/local | local profile | identity service |
| Profile metadata | private | encrypted/local | authorized services |
| Wallet address | public | local profile / chain | permitted apps |
| Public key | public | local profile | permitted apps |
| Private key | secret | protected encrypted wallet store | wallet service only |
| 24-word phrase | critical secret | memory only during provisioning/recovery | identity UI only |
| Identity binding | integrity-sensitive | authenticated local store | identity service |
| Signature | operation evidence | optional audit store | requester / audit service |

## 10. Implementation boundary

The initial implementation belongs in the GlobusOS userspace service layer:

```text
system/identity/   Identity/profile lifecycle
system/security/   authentication + capability policy
system/           Wallet service integration boundary
```

The wallet cryptography itself should be provided by the dedicated `atc-wallet` component rather than duplicated in the OS. GlobusOS owns the OS integration, authorization, secure storage boundary, UI lifecycle, and IPC contract.

## 11. Acceptance criteria

- [ ] Registration creates a unique local User ID.
- [ ] Registration provisions exactly one initial wallet identity per profile unless an explicit multi-wallet feature is enabled.
- [ ] Wallet address is derived from cryptographic key material, not generated as an arbitrary identifier.
- [ ] 24-word recovery material is generated using a CSPRNG-backed implementation.
- [ ] Recovery phrase is never persisted in plaintext.
- [ ] Recovery phrase never enters logs, telemetry, IPC messages to untrusted applications, or AI context.
- [ ] User must verify recovery material before provisioning completes.
- [ ] Private keys are inaccessible to ordinary applications.
- [ ] Signing requires an explicit capability-mediated request.
- [ ] Identity binding is cryptographically verifiable.
- [ ] Recovery works without the original device.
- [ ] Login does not require the recovery phrase.
- [ ] Failure of secure storage or cryptographic initialization causes fail-closed behavior.

## 12. Non-goals

This specification does not define the A-TownChain address encoding, consensus rules, transaction format, mnemonic standard implementation, hardware-wallet protocol, or chain account model. Those remain owned by their respective ATC components and standards.
