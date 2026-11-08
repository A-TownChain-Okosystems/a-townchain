# ARCHITECTURE.md — atc-vm

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-vm/
├── Cargo.toml — ShivaVM core execution engine manifest (no_std)
├── .gitignore — Git ignore rules
└── src/
    ├── lib.rs — ShivaVM entry point, execution environment, and state machine runner
    ├── opcodes.rs — Instruction set architecture opcode definitions and instruction decoder
    ├── stack.rs — High-performance operand stack and call stack implementation
    ├── gas.rs — Deterministic opcode gas metering and resource limit enforcement
    └── storage.rs — Persistent state trie storage interface and state rollback support
```

## Module Descriptions
- src/lib.rs — Main virtual machine execution engine, bytecode runner loop, and environment host bindings.
- src/opcodes.rs — Complete specification and decoding matrix for ShivaVM instruction opcodes.
- src/stack.rs — Efficient fixed-capacity stack operations with strict push/pop boundary checks.
- src/gas.rs — Tracks instruction gas costs to guarantee deterministic execution and prevent infinite loops.
- src/storage.rs — Key-value state storage interface backing virtual machine execution state.

## Build System
- Cargo.toml — `#![no_std]` crate usable in on-chain smart contracts or embedded node runtimes.

## Dependencies
- byteorder — Fast endianness conversion for bytecode parsing without standard library.
- digest — Cryptographic hash abstractions for state root calculation.
