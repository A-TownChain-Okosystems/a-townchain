# ARCHITECTURE.md — atc-cli

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-cli/
├── Cargo.toml — Command Line Interface tool manifest
├── .gitignore — Git ignore settings
└── src/
    ├── main.rs — CLI application entry point and command-line argument parser
    ├── lib.rs — Programmatic CLI engine library for external scripting
    ├── commands.rs — Subcommand handlers (node, wallet, deploy, query, keygen)
    ├── client.rs — Asynchronous HTTP/WebSocket client for node communication
    └── format.rs — Console output formatting (JSON, Table, ANSI colorized terminal)
```

## Module Descriptions
- src/main.rs — Entry point parsing command-line flags and executing matching subcommands.
- src/lib.rs — Core library allowing programmatic execution of CLI commands.
- src/commands.rs — Subcommand implementations for node status, wallet management, contract deployment, and key generation.
- src/client.rs — Handles communication with ShivaCore nodes via API gateway RPC calls.
- src/format.rs — Renders structured output data into human-readable tables, text, or formatted JSON.

## Build System
- Cargo.toml — Standard Rust `std` application built with `clap` and `tokio`.

## Dependencies
- clap — Command line argument parser with derive macro support.
- tokio — Asynchronous event runtime for networking and concurrency.
- reqwest — Asynchronous HTTP client for API requests.
- colored / comfy-table — Terminal output styling and table rendering.
