# ARCHITECTURE.md — atc-stdlib

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-stdlib/
├── Cargo.toml — no_std standard library alternative crate manifest
├── .gitignore — Git ignore configuration
└── src/
    ├── lib.rs — Crate entry point, re-exports standard structures and core error definitions
    ├── string.rs — Heap-allocated string types (AtcString) designed for no_std environments
    ├── collections.rs — Dynamic collection structures (Vec, HashMap, BTreeMap) for custom allocators
    ├── io.rs — Core I/O traits (Read, Write, Seek) and buffer management for embedded code
    ├── alloc.rs — Custom global memory allocator binding and raw memory allocation helpers
    ├── errno.rs — System error codes, OS error definitions, and status code conversions
    ├── syscall.rs — Low-level assembly system call wrappers for ShivaCore kernel ABI
    ├── math.rs — Fixed-point arithmetic, cryptographic math routines, and fast utility math
    ├── time.rs — Monotonic timers, system uptime counters, and timestamp utilities
    └── fs.rs — File operations, file handle abstractions, and VFS path manipulation
```

## Module Descriptions
- src/lib.rs — Crate root exposing standard core types, macros, and basic data types.
- src/string.rs — Implements safe, owned, and borrowed string buffers without relying on std::string.
- src/collections.rs — Provides heap-allocated collection types tailored for no_std applications and smart contracts.
- src/io.rs — Defines fundamental I/O traits for stream processing and byte buffer manipulation.
- src/alloc.rs — Interface connecting system memory allocators with runtime data structures.
- src/errno.rs — Standardized POSIX and ShivaCore error code definitions and error mapping traits.
- src/syscall.rs — Low-level inline assembly bindings for issuing syscalls to the kernel.
- src/math.rs — Overflow-safe arithmetic, fixed-point calculation, and fast mathematical helpers.
- src/time.rs — Access interfaces for system clocks, monotonic timers, and tick counters.
- src/fs.rs — File system interaction primitives bridging user applications to the virtual file system.

## Build System
- Cargo.toml — Configured with `#![no_std]` for cross-platform utility across kernel, user binaries, and ShivaVM.

## Dependencies
- core / alloc — Standard Rust core primitives and dynamic allocation traits.
