# GlobusOS Boot → Login → Register → Wallet Flow

## State machine

```text
BOOT
  -> SHIVACORE_READY
  -> SERVICES_READY
  -> LOGIN

LOGIN
  -> AUTHENTICATED
  -> REGISTER
  -> RECOVERY

REGISTER
  -> PROFILE_CREATED
  -> WALLET_PROVISIONING
  -> RECOVERY_PHRASE_VERIFICATION
  -> IDENTITY_BOUND
  -> AUTHENTICATED

RECOVERY
  -> MNEMONIC_VALIDATED
  -> WALLET_RECONSTRUCTED
  -> IDENTITY_RESTORED
  -> AUTHENTICATED
```

## Registration invariant

A registration is successful only when all of the following are true:

```text
User ID exists
AND
wallet public identity exists
AND
identity binding verifies
AND
recovery backup was verified
AND
wallet state is securely persisted
```

Otherwise the operation is aborted and no partially initialized identity is presented as ready.

## Wallet/account relationship

```text
GlobusOS User Profile
        |
        +-- User ID
        +-- authentication credential
        +-- profile metadata
        |
        +-- Wallet Identity
              +-- address (public)
              +-- public key (public)
              +-- private key (protected)
              +-- recovery phrase (user-held recovery secret)
```

The address is an identity reference; possession of the address alone never authorizes a transaction.

## Signing policy

```text
App
 -> request operation
 -> capability/policy evaluation
 -> Wallet Service
 -> user confirmation when policy requires
 -> cryptographic signature
 -> return signature
```

Raw private-key export is not part of the normal application API.
