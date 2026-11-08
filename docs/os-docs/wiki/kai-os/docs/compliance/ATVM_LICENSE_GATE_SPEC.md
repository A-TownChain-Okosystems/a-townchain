# ATVM License Gate — Technische Spezifikation
## A-Town Virtual Machine License Enforcement

> **Version:** 1.0.0 | **Datum:** 06.07.2026
> **Autor:** Michael Wroblewski / ShivaCore / A-TownChain-Okosystems
> **Status:** TECHNICAL SPEC — Implementation geplant
> **Referenz-Standard:** ATC-LIC v1.0

---

## 1. Architektur

```
ATVM ARCHITECTURE
├── Pre-Execution Layer
│   ├── Code Hasher (SHA-256)
│   ├── License Registry Client
│   ├── Royalty Calculator
│   └── Balance Verifier
├── Execution Layer
│   ├── ATCLang Bytecode Interpreter
│   ├── Sandbox Manager
│   ├── Resource Monitor (ATC-37)
│   └── Isolation Container
├── Post-Execution Layer
│   ├── DAG Logger (ATC-04)
│   ├── Registry Updater
│   └── Event Emitter
└── Security Layer
    ├── ECDSA Signature Verifier (ATC-03)
    ├── Tamper Detection (ATC-LIC)
    └── Secure Boot Integration (ATS-1000+)
```

## 2. License Gate State Machine

```
         ┌─────────────┐
         │  IDLE       │
         └──────┬──────┘
                │ execute_request(code_hash, caller_did, input)
                v
         ┌─────────────┐
         │  HASHING    │  code_hash = SHA-256(code)
         └──────┬──────┘
                v
         ┌─────────────┐
         │  REGISTRY   │  entry = Registry.get(code_hash)
         │  LOOKUP     │
         └──────┬──────┘
                │
           entry == NULL?
           ├── YES -> PUBLIC DOMAIN -> EXECUTE FREE
           └── NO  -> continue
                v
         ┌─────────────┐
         │  LICENSE    │  licensed, royalty = check(entry, caller)
         │  CHECK      │
         └──────┬──────┘
                │
           licensed == false?
           ├── YES -> LOG BLOCKED -> RETURN DENIED
           └── NO  -> continue
                v
         ┌─────────────┐
         │  BALANCE    │  caller_balance >= royalty?
         │  CHECK      │
         └──────┬──────┘
                │
           insufficient?
           ├── YES -> LOG INSUFFICIENT -> RETURN DENIED
           └── NO  -> continue
                v
         ┌─────────────┐
         │  ATOMIC     │  BEGIN TRANSACTION
         │  EXECUTE    │  ├── Transfer royalty
         │             │  ├── Execute code (sandboxed)
         │             │  ├── Log to DAG
         │             │  └── Update registry
         └──────┬──────┘
                v
         ┌─────────────┐
         │  COMMIT     │  COMMIT TRANSACTION
         │  or ROLLBACK│  (all or nothing)
         └──────┬──────┘
                v
         ┌─────────────┐
         │  RESULT     │  return execution result
         │  RETURN     │
         └─────────────┘
```

## 3. API Specification

### 3.1 ATVM Execute API

```atc
// ATCLang ATVM API
namespace ATC::ATVM {

  /// Hauptfunktion: Code mit License-Gate ausfuehren
  fn execute(
    code: Bytecode,
    caller_did: DID,
    input: Data,
    options: ExecOptions
  ) -> ExecResult {
    // 1. Hash
    let code_hash = crypto::sha256(code);

    // 2. License Check
    let (licensed, royalty) = License::Registry::check_license(code_hash, caller_did);

    if !licensed {
      return ExecResult::Denied(LicenseError::NotLicensed);
    }

    // 3. Balance Check
    if royalty > 0 && !Token::ATC11::has_balance(caller_did, royalty) {
      return ExecResult::Denied(LicenseError::InsufficientBalance);
    }

    // 4. Atomic Execute
    atomic {
      if royalty > 0 {
        Token::ATC11::transfer(caller_did,
          License::Registry::get_developer(code_hash), royalty);
      }

      let result = self.execute_sandboxed(code, input, options);

      License::Registry::record_call(code_hash, caller_did, royalty);
      DAG::log("ATVM_EXEC", {
        code_hash, caller: caller_did,
        royalty, timestamp: block::time(),
        result_hash: crypto::sha256(result)
      });

      return ExecResult::Ok(result);
    }
  }

  /// Sandboxed execution with resource limits
  fn execute_sandboxed(
    code: Bytecode,
    input: Data,
    options: ExecOptions
  ) -> Data {
    let sandbox = Sandbox::new()
      .memory_limit(options.max_memory)
      .time_limit(options.max_time)
      .network_policy(options.network)
      .fs_namespace(options.fs_ns);

    sandbox.run(code, input)
  }
}
```

### 3.2 ExecOptions

```atc
struct ExecOptions {
  max_memory: u64,        // Bytes (default: 256MB)
  max_time: u64,          // Milliseconds (default: 5000)
  max_cpu: u8,            // Percent (default: 50)
  network: NetworkPolicy, // None / Allowlisted / Full
  fs_ns: String,          // ATCFS namespace
  priority: Priority,     // Low / Normal / High (ATC-37)
}
```

### 3.3 ExecResult

```atc
enum ExecResult {
  Ok(Data),                    // Successful execution
  Denied(LicenseError),        // License gate blocked
  RuntimeError(RuntimeErr),    // Code execution failed
  Timeout,                     // Time limit exceeded
  ResourceLimitExceeded,       // Memory/CPU limit hit
}

enum LicenseError {
  NotLicensed,                 // License check failed
  InsufficientBalance,         // Caller cannot pay royalty
  LicenseRevoked,              // Developer revoked license
  LicenseExpired,              // Subscription ended
}
```

## 4. Performance Characteristics

| Operation | Latency | Gas/Cost |
|-----------|---------|----------|
| Registry Lookup | ~5ms | 1 ATC-11 unit |
| Royalty Calculation | ~2ms | 1 ATC-11 unit |
| Balance Check | ~3ms | 1 ATC-11 unit |
| Token Transfer | ~50ms | 10 ATC-11 units |
| Sandboxed Execution | variable | based on code complexity |
| DAG Logging | ~100ms | 5 ATC-11 units |
| **Total Overhead** | **~160ms + execution** | **~17 ATC-11 units** |

## 5. Security Model

### 5.1 Threat Analysis

| Threat | Mitigation |
|--------|-----------|
| Bypass License Gate | ATVM is in kernel (ATS-1000+), cannot be bypassed |
| Forge Code Hash | SHA-256 collision-resistant |
| Manipulate Registry | Smart Contract on-chain (ATC-01) |
| Replay Execution | DAG ensures immutability |
| Double-spend Royalty | Atomic transaction prevents |
| Sandbox Escape | Memory/CPU/Network isolation |
| Tamper with ATVM | Secure Boot (ATC-LIC) |
| Unauthorized Node | TPM-Attestation (ATC-LIC) |

### 5.2 Key Management

- **Developer Private Key** — Signs code registration (never leaves device)
- **Caller Private Key** — Signs execution requests (ATC-03 DID)
- **ATVM Internal Key** — Generated by TPM, never exported
- **DAO Multi-Sig Key** — For governance actions (ATC-17)

## 6. Integration Points

| System | Interface | Direction |
|--------|-----------|-----------|
| License Registry | Smart Contract calls | ATVM -> Registry |
| ATC-11 Token | Transfer calls | ATVM -> Token Contract |
| ATC-04 DAG | Log entries | ATVM -> DAG |
| ATC-03 Identity | DID verification | ATVM -> Identity Contract |
| ATC-17 DAO | Governance queries | ATVM -> DAO Contract |
| ATC-37 Resource Alloc | Quota checks | ATVM -> Scheduler |
| ATC-LIC TPM | Hardware attestation | ATVM -> Kernel -> TPM |
| GlobusOS Dashboard | Read-only queries | Dashboard -> Registry |

---

*Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. Apache-2.0 lizenziert — siehe LICENSE*
*Letzte Aktualisierung: 06.07.2026 | Aurora (Superagent)*
