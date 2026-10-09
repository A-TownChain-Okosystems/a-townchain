# Testplan — atc-contracts

**Status:** Native ATCLang/ATVM migration in progress (`atc-contracts#5`).

## Native test layers

### 1. ATCLang consensus conformance

`tests/run_atclang_conformance.sh` runs the canonical ATCLang toolchain against every `.atc` source with the `consensus` profile.

The gate covers:

- parsing
- semantic compilation
- SecurityGate/static contract checks
- consensus-profile restrictions

### 2. Native EXEC-CHAIN / ATVM execution

`exec_chain/tests/run_tests.sh` runs deterministic contract vectors through:

```text
.atc contract
  ↓
native Rust assembler
  ↓
.ops bytecode
  ↓
native ATVM runner
  ↓
expected vector
  ↓
PASS / FAIL
```

The suite includes positive and negative execution vectors for ATCoin, ATC-8300, governance, bridge, authorization, balance/supply invariants and storage transitions.

### 3. Determinism

`.github/workflows/determinism-gate.yml` runs both native gates twice and compares their textual evidence output byte-for-byte.

### 4. CI evidence

`.github/workflows/test-suite.yml` requires:

- ATCLang consensus conformance
- native ATVM EXEC-CHAIN execution
- repository JavaScript tests/builds

Only when all required jobs pass is the immutable evidence artifact produced.

## Removed legacy layer

The former pytest suite executed Python contract implementations. Those production implementations and their Python test fixtures have been removed. They are no longer part of the consensus contract path.

## Closure gate

`atc-contracts#5` remains open until CI demonstrates the complete native pipeline green and the ATCLang/ATVM coverage is accepted as the replacement for the former Python test coverage.
