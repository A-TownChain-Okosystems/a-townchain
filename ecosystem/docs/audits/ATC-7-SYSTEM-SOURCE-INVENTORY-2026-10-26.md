# ATC-7-SYSTEM-SOURCE-INVENTORY-001

**Status:** `RESIDUAL` — catalog bindings extracted; live-head reconciliation and functional verification remain open.
**Snapshot date:** 2026-10-26
**Catalog source:** `docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json` on `main`; catalog blob SHA `f6c84201cbea3d8b0ea1510e0cab52574c3faaea`.

This inventory records the seven requested architecture domains from the existing component/function catalog. Source commit and blob SHAs below are the values recorded in that catalog; they are **not asserted to be current HEAD/blob values** until a same-run live census re-fetches each repository. No row is functionally verified merely because a source binding exists.

## Summary

| Component | Catalog ID | Catalog function contracts | Functionally verified in baseline |
|---|---|---:|---:|
| ATCLang | `COMP-LANG` | 10 | 0 claimed |
| ATC-VM | `COMP-VM` | 29 | 0 claimed |
| ShivaCore Kernel | `COMP-SHIVA` | 3 | 0 claimed |
| A-TownChain Blockchain Core | `COMP-CHAIN` | 3 | 0 claimed |
| GlobusOS | `COMP-GLOBUS` | 3 | 0 claimed |
| Aurora AI | `COMP-AURORA` | 4 | 0 claimed |
| Genesis Engine | `COMP-GENESIS` | 2 | 0 claimed |

## Function and source-binding inventory

Each row preserves the catalog function ID, contract intent, source path, symbol/entrypoint, recorded source commit, recorded blob SHA, and recorded binding status. Missing values are shown as `UNBOUND` rather than inferred.

| Component | Function ID | Contract / responsibility | Source path | Symbol / entrypoint | Recorded source commit | Recorded blob SHA | Recorded binding status |
|---|---|---|---|---|---|---|---|
| ATCLang | `FN-LANG-001` | Source parsing and diagnostics | `crates/atc-core/src/parser.rs` | `parse_program` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | `993df7a9a6626d869ef3e22f02a006fa13466586` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATCLang | `FN-LANG-002` | AST and directive validation | `crates/atc-core/src/native.rs` | `parse_native_contract` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | `43fdf67e3b9b57f38aed5c008ae6e8da3cc9e477` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATCLang | `FN-LANG-003` | AST-to-bytecode lowering | `crates/atc-core/src/lower.rs` | `lower_program` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | `ab3c28accab95339594fb5d797d83bc855ef0d0f` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATCLang | `FN-LANG-004` | Canonical bytecode encoding and verification | `crates/atc-core/src/bytecode.rs` | `verify` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | `d63ba13087a71d6f06ec44c0dc2496644906eeea` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATCLang | `FN-LANG-005` | Artifact envelope with hash binding | `crates/atc-core/src/artifact.rs` | `decode_and_verify` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | `1b98def2f848a51c147eaad13a39fd8154fa2b06` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATCLang | `FN-LANG-006` | ABI value and protocol type boundary | `crates/atc-core/src/value.rs` | `AbiType` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | `cb720e71cbdc26f01cae36388a99d6f2c1a5d8a0` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATCLang | `FN-LANG-007` | Deterministic reference execution (reference only) | `crates/atc-core/src/vm.rs` | `execute_verified` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | `009bdec170527898957b9622cb29ec406f77ed81` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATCLang | `FN-LANG-008` | Differential parser and corpus conformance | `crates/atc-core/tests/differential.rs` | `differential_gemeinsamer_subset` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | `5f588cb83291ea1be90ef455a15f9d6bce8bd193` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATCLang | `FN-LANG-009` | Standard library reference conformance | `crates/atc-core/tests/stdlib_reference_conformance.rs` | `encode_be` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | `a7ada9f5e3c8597ca647cec206201e1ef0ede68c` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATCLang | `FN-LANG-010` | Determinism gate tooling | `tools/determinism_check.py` | `main` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | `1d360ba239362a3b7cabfa99e17f2299786ae0a1` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-001` | Parse textual .ops instruction stream | `components/vm/src/ops.rs` | `parse_ops` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `2b5efdff367b72e0025a6bfe7c05e89d23cf0a43` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-002` | Deterministic bounded execution | `components/vm/src/vm.rs` | `execute_state_transition_with_gas` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `57a919d911ae7900f740bab739bb20fabf0d4313` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-003` | VM construction | `components/vm/src/vm.rs` | `new` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `57a919d911ae7900f740bab739bb20fabf0d4313` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-004` | VM construction with caller and state | `components/vm/src/vm.rs` | `with_context` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `57a919d911ae7900f740bab739bb20fabf0d4313` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-005` | Read caller identity | `components/vm/src/vm.rs` | `caller` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `57a919d911ae7900f740bab739bb20fabf0d4313` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-006` | Read VM storage state | `components/vm/src/vm.rs` | `state` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `57a919d911ae7900f740bab739bb20fabf0d4313` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-007` | Fail-closed legacy state-transition entrypoint | `components/vm/src/vm.rs` | `execute_state_transition` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `57a919d911ae7900f740bab739bb20fabf0d4313` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-008` | Structural program verifier | `components/vm/src/vm.rs` | `verify` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `57a919d911ae7900f740bab739bb20fabf0d4313` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-009` | Gas-bounded transactional interpreter | `components/vm/src/vm.rs` | `run_with_gas` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `57a919d911ae7900f740bab739bb20fabf0d4313` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-010` | Raw local interpreter | `components/vm/src/vm.rs` | `run` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `57a919d911ae7900f740bab739bb20fabf0d4313` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-011` | Validate chain execution context | `components/vm/src/context.rs` | `validate` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `ef0a0cee2ff6a73adc19870a75fc1ed5e00ac7bc` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-012` | Verify computed genesis identity | `components/vm/src/context.rs` | `verify_genesis_id` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `ef0a0cee2ff6a73adc19870a75fc1ed5e00ac7bc` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-013` | Fail-closed execution context gate | `components/vm/src/context.rs` | `execution_gate` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `ef0a0cee2ff6a73adc19870a75fc1ed5e00ac7bc` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-014` | Assemble EXEC-GATE source to .ops | `components/vm/src/assembler.rs` | `assemble` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `816f3ebb3ae62e4e6e054c0c992a9fd60dd3b7c4` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ATC-VM | `FN-VM-015` | Initialize expression parser | `components/vm/src/assembler.rs` | `new` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `816f3ebb3ae62e4e6e054c0c992a9fd60dd3b7c4` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-016` | Inspect next expression token | `components/vm/src/assembler.rs` | `peek` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `816f3ebb3ae62e4e6e054c0c992a9fd60dd3b7c4` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-017` | Consume next expression token | `components/vm/src/assembler.rs` | `take` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `816f3ebb3ae62e4e6e054c0c992a9fd60dd3b7c4` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-018` | Parse additive expression | `components/vm/src/assembler.rs` | `expr` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `816f3ebb3ae62e4e6e054c0c992a9fd60dd3b7c4` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-019` | Parse multiplicative term | `components/vm/src/assembler.rs` | `term` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `816f3ebb3ae62e4e6e054c0c992a9fd60dd3b7c4` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-020` | Parse expression factor | `components/vm/src/assembler.rs` | `factor` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `816f3ebb3ae62e4e6e054c0c992a9fd60dd3b7c4` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-021` | Tokenize restricted expression | `components/vm/src/assembler.rs` | `tokenize` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `816f3ebb3ae62e4e6e054c0c992a9fd60dd3b7c4` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-022` | Extract function parameters and body | `components/vm/src/assembler.rs` | `find_function` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `816f3ebb3ae62e4e6e054c0c992a9fd60dd3b7c4` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-023` | Read numeric u64 from JSON-like vector | `components/vm/src/assembler.rs` | `json_u64_for_key` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `816f3ebb3ae62e4e6e054c0c992a9fd60dd3b7c4` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-024` | Simulate generated arithmetic instructions | `components/vm/src/assembler.rs` | `simulate` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `816f3ebb3ae62e4e6e054c0c992a9fd60dd3b7c4` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-025` | Parse CLI slot/value pair | `components/vm/src/main.rs` | `parse_slot_pair` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `ae010fc16ac310e8a98316f63c89e7d9b96cbca8` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-026` | Parse runner CLI arguments | `components/vm/src/main.rs` | `parse_args` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `ae010fc16ac310e8a98316f63c89e7d9b96cbca8` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-027` | Execute a .ops file through runner | `components/vm/src/main.rs` | `run_ops_file` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `ae010fc16ac310e8a98316f63c89e7d9b96cbca8` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-028` | Dispatch VM runner and assembler CLI | `components/vm/src/main.rs` | `main` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `ae010fc16ac310e8a98316f63c89e7d9b96cbca8` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ATC-VM | `FN-VM-029` | Print runner usage and exit | `components/vm/src/main.rs` | `usage` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `ae010fc16ac310e8a98316f63c89e7d9b96cbca8` | `PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS` |
| ShivaCore Kernel | `FN-SHIVA-001` | Capability authorization | `modules/atc-shivacore/kernel/src/capability.rs` | `check` | `c7337ef4fcff447ad0ee31c54e8adfee7f895bf6` | `dd5d4f7a0330f91badf1a956a210f4f745104e66` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ShivaCore Kernel | `FN-SHIVA-002` | IPC and process isolation | `modules/atc-shivacore/kernel/src/ipc.rs` | `create_channel` | `c7337ef4fcff447ad0ee31c54e8adfee7f895bf6` | `d95aebd8e48c47346ca282c79bef35d19bdbc6b8` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| ShivaCore Kernel | `FN-SHIVA-003` | Virtual-memory mapping lifecycle | `modules/atc-shivacore/kernel/src/vmm.rs` | `map_page` | `c7337ef4fcff447ad0ee31c54e8adfee7f895bf6` | `a3ad9e0d727d5644ae500ff0bb80f10606121f3f` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| A-TownChain Blockchain Core | `FN-CHAIN-001` | Transaction validation and signing contract | `components/node/src/validator.rs` | `verify` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `ee5d3817e2c67c4ef82a595a47711588cf967c64` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| A-TownChain Blockchain Core | `FN-CHAIN-002` | Canonical amount and state encoding | `components/sdk/src/lib.rs` | `validate_amount` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `998154660957916ad2d6d4e8b9390d62dd59ac0c` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| A-TownChain Blockchain Core | `FN-CHAIN-003` | Block/state transition validation | `components/vm/src/vm.rs` | `execute_state_transition` | `a70b16f2d23c6e47ea1cd8691fed8a6d86f0efc9` | `57a919d911ae7900f740bab739bb20fabf0d4313` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| GlobusOS | `FN-GLOBUS-001` | Service lifecycle and isolation | `modules/atc-shivacore/kernel/src/process.rs` | `spawn` | `c7337ef4fcff447ad0ee31c54e8adfee7f895bf6` | `960c80753ca3a7c9ff54887ca00501e150127554` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| GlobusOS | `FN-GLOBUS-002` | Boot and platform discovery | `modules/atc-bootloader/src/boot.rs` | `next_stage` | `c7337ef4fcff447ad0ee31c54e8adfee7f895bf6` | `7147afd7964dbc5c31a5272e4075faa35f098089` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| GlobusOS | `FN-GLOBUS-003` | OS service boundary and platform integration | `UNMAPPED` | `PROPOSED_CONTRACT_ONLY` | `c7337ef4fcff447ad0ee31c54e8adfee7f895bf6` | `UNBOUND` | `UNBOUND_RESIDUAL` |
| Aurora AI | `FN-AURORA-001` | Intent-to-proposal planning | `modules/atc-aurora-core/src/aurora_core.rs` | `process_request` | `1b5ff1824bb032958b9a2ae4e53365d7af01c3af` | `cc649331479373d6dd2918441b8477dd1528e3ef` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| Aurora AI | `FN-AURORA-002` | Policy/capability-gated tool invocation | `modules/atc-aurora-agents/src/agent_pool.rs` | `execute` | `1b5ff1824bb032958b9a2ae4e53365d7af01c3af` | `34f672015b0945e6f4a511cc66b788f3930ba404` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| Aurora AI | `FN-AURORA-003` | Swarm coordination and result validation | `modules/atc-aurora-core/src/agent_registry.rs` | `register` | `1b5ff1824bb032958b9a2ae4e53365d7af01c3af` | `ad2d7826e66ef3c20924ae10e16b6bb5abd1c32a` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| Aurora AI | `FN-AURORA-004` | Policy-gated Neural AI orchestration | `UNMAPPED` | `PROPOSED_CONTRACT_ONLY` | `1b5ff1824bb032958b9a2ae4e53365d7af01c3af` | `UNBOUND` | `UNBOUND_RESIDUAL` |
| Genesis Engine | `FN-GENESIS-001` | Genesis configuration validation | `modules/atc-genesis-build/src/lib.rs` | `validate` | `8326eb53e3e10cf08518e6769b44d4f034de1144` | `c8ca893727f292f0ae397dcb35549dabdb68e710` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |
| Genesis Engine | `FN-GENESIS-002` | Environment bootstrap and compatibility | `modules/atc-genesis-ecs/src/lib.rs` | `WorldEcsBridge` | `8326eb53e3e10cf08518e6769b44d4f034de1144` | `e5227050fb472819fd670e4eb44a333d07a3479f` | `SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED` |

## Required live-census reconciliation

1. Pin exact `main` HEAD SHA and source tree SHA for each canonical repository.
2. Enumerate Rust files; resolve external `mod` declarations, inline modules, `#[cfg(test)]`, target/feature gates and macro-generated/public exports separately.
3. Enumerate Python, TypeScript and other public exports in multi-language repositories; do not limit Aurora AI or Genesis Engine to Rust.
4. Recompute each source file Git blob SHA and verify catalog path/symbol against the pinned commit.
5. Reconcile contract-vs-symbol cardinality; distinguish public APIs, internal helpers, test surfaces, data artifacts and specification documents.
6. Run component-specific checks and cross-component integration tests. Record Actions Run → Job → Step, exit code, log/artifact URL and artifact SHA-256.
7. Keep missing, stale, ambiguous or untested entries `RESIDUAL` or `BLOCKED`; never promote a binding to `VERIFIED` solely because a symbol or path exists.

## Initial priority gaps

- `COMP-GLOBUS / FN-GLOBUS-003` and `COMP-AURORA / FN-AURORA-004` are unbound architecture contracts; reconcile source ownership before implementation claims.
- `COMP-VM` has 29 catalog contracts, including runner/internal/test surfaces; reconcile public API scope before treating all 29 as public functions.
- Separate `atc-vm` is mapped as specification support; executable VM SSOT remains `a-townchain/components/vm`.
- `a-townchain-ecosystem` is the control plane and must not become a duplicate implementation repository.
- Workspace/package counts are separate metrics from source files, public symbols, function contracts and test-only symbols.

## Gate decision

`RESIDUAL / BLOCKED`. This is a catalog-derived inventory snapshot, not proof of a complete source census, tests, end-to-end integration, security review or production readiness.