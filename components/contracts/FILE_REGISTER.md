# 📋 File Register — atc-contracts

> Canonical inventory for the ATCLang contract repository.
> Updated after the native ATCLang/ATVM migration for `atc-contracts#5`.

## Production-language policy

| Metric | Current state |
|---|---|
| Production `.py` contract implementations | **0** |
| Production `.atc` contract sources | **Canonical** |
| Native Rust EXEC-GATE assembler | `atc-vm/src/assembler.rs` |
| Python contract runtime dependency | **Removed** |
| Python contract test suite | **Removed** |

## Canonical contract sources

- `atc8300/atc8300.atc`
- `atc8300/atc8300_token.atc`
- `atcoin/atcoin.atc`
- `base/base_contract.atc`
- `bridge/bridge_contract.atc`
- `contracts/atc001/genesis_token.atc`
- `contracts/revenue.atc`
- `contracts/token.atc`
- `governance/governance_contract.atc`
- `smart_contract_registry.atc`
- `smart_contracts.atc`
- `standards/*.atc`
- `modules/atc-contracts/**` — synchronized ATCLang module mirror
- `modules/atc-standards-refs/**` — synchronized standards references

## Native verification

- `tests/run_atclang_conformance.sh` performs ATCLang consensus SecurityGate + semantic compilation for all `.atc` sources.
- `exec_chain/tests/run_tests.sh` performs native Rust assembly and ATVM execution against deterministic vectors.
- `.github/workflows/test-suite.yml` requires both native conformance and native ATVM execution.
- `.github/workflows/determinism-gate.yml` executes the native gates twice and byte-compares their evidence output.

## Migration rule

On-chain contract logic must be authored and maintained in ATCLang. Python is not a production contract implementation language for this repository.
