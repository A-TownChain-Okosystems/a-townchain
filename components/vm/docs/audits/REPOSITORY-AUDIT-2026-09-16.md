# ATC-VM Repository Audit — 2026-09-16

Status: **IN PROGRESS — R1 development/skeleton; production readiness not established**

Audit mode: CI-independent source/static audit because GitHub Actions evidence is currently unavailable/incomplete.

## Scope

Checked individually:

- repository structure and governance metadata
- README ↔ implementation consistency
- Rust source, context gate, interpreter and assembler surface
- tests and negative-path coverage
- gas/sandbox/verifier claims
- deterministic execution and chain identity boundary
- TODO/FIXME/stub/error-pattern scan
- file format and programming-language suitability

## Confirmed architecture strengths

`atc-vm` is correctly Rust-first and is explicitly positioned as the boundary between ATCLang bytecode and Rust chain infrastructure. The state-transition entrypoint calls a fail-closed chain/protocol/VM/genesis gate before state mutation. The interpreter also checks stack underflow, division by zero and jump bounds.

## Findings

### F-20260916-ATCVM-001

- Class: **P1**
- Category: **Completeness / Security / Resource control**
- Family: **ATC-VM / Gas Accounting / Execution Limits**
- Tags: `P1 vm gas gas-metering resource-exhaustion completeness`
- Evidence: `README.md` claims deterministic, gas-limited execution and a gas-cost model, but `src/vm.rs` contains no gas budget, gas meter, per-op gas schedule, gas exhaustion error or gas accounting in `run()`.
- Impact: a loop or large program can execute without a protocol-level gas budget. The implementation therefore does not yet enforce the documented resource-control guarantee.
- Required remediation: define the canonical gas schedule from the applicable ATC standard, add a gas meter to execution state, charge every opcode before effect, fail closed on exhaustion, and expose consumed/remaining gas deterministically. Add tests for exact accounting, exhaustion, zero gas and state non-mutation on pre-effect exhaustion.
- Closure: source re-read + deterministic unit/integration tests + conformance evidence against the normative gas schedule + CI/runtime evidence when available.

### F-20260916-ATCVM-002

- Class: **P1**
- Category: **Completeness / Security / Verification**
- Family: **ATC-VM / Bytecode Verifier / Sandbox**
- Tags: `P1 vm verifier sandbox bytecode-validation completeness`
- Evidence: repository documentation promises a Bytecode Verifier that rejects unverified bytecode, but the current `src/` inventory contains assembler, context, ops, VM and CLI files without a dedicated verifier implementation or visible verifier gate in `Vm::run()`.
- Impact: the documented security boundary is not demonstrated by the current source. Raw `run()` accepts a program directly.
- Required remediation: implement a canonical verifier and make state-transition execution require successful verification before interpretation. Define forbidden opcodes, structural constraints, jump targets, stack effects, resource limits and canonical encoding rules. Add negative tests for malformed/unverified bytecode.
- Closure: source re-read + negative verifier tests + integration proof that unverified programs cannot execute through the state-transition path.

### F-20260916-ATCVM-003

- Class: **P2**
- Category: **Logic / Arithmetic semantics**
- Family: **ATC-VM / Integer Semantics / Consensus Determinism**
- Tags: `P2 vm arithmetic wrapping overflow consensus`
- Evidence: `Add`, `Sub` and `Mul` use `wrapping_*` semantics for `u64`.
- Impact: overflow behavior is consensus-critical. If wrapping arithmetic is not explicitly normative for these operations, divergent expectations between compiler/reference/runtime can produce incompatible state transitions.
- Required remediation: bind arithmetic semantics to the normative ATCLang/ATVM specification and add conformance vectors. If wrapping is normative, encode and test it explicitly; otherwise return a deterministic overflow error.

### F-20260916-ATCVM-004

- Class: **P2**
- Category: **API / Safety boundary**
- Family: **ATC-VM / Raw Interpreter / State Transition API**
- Tags: `P2 vm api raw-run state-transition boundary`
- Evidence: `Vm::run()` is public while documentation says state transitions MUST use `execute_state_transition()`.
- Impact: callers can bypass the identity/protocol/genesis gate if they have access to the VM object.
- Required remediation: narrow visibility of raw execution where architecture permits, or require an explicit non-consensus/reference mode type. Make the safe state-transition API the only path available to chain execution integration.

## Documentation consistency

The README is correctly conservative about R1/development status, but the statements that ATVM provides gas-limited execution and a verifier should be treated as target capabilities until the implementation is present and tested. The audit record intentionally does not mark these claims as implemented merely because they are documented.

## File format / language decision

- Rust is the correct canonical language for the VM, verifier, gas engine and host boundary because these are deterministic, security-critical runtime components.
- Markdown is appropriate for architecture/audit/roadmap documentation.
- TOML is appropriate for Cargo manifests.
- YAML is appropriate only for standardized machine-readable governance/CI metadata.
- No language migration is justified by the current evidence.

## Security / malware / virus posture

Static review did not establish immunity from malware or all exploit classes. The meaningful protection currently demonstrated is fail-closed identity gating, bounds checks, stack-underflow handling and explicit division-by-zero rejection. Gas/verifier gaps remain open security/resource-control findings.

A repository cannot be proven "virus-safe" from source inspection alone. The defensible evidence model is reproducible builds, dependency/SBOM review, signed artifacts, provenance, sandboxed execution, malware scanning and CI/runtime tests. Those claims require evidence and are therefore not marked proven here.

## Verification state

- Static source findings: verified against current main source.
- Runtime tests: **not claimed** because GitHub Actions evidence is unavailable/incomplete.
- Findings remain open until source changes are re-read and tests/conformance evidence are available.
- Release posture: **NOT PRODUCTION READY**.
