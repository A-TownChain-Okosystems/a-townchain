# ARCHITECTURE.md — atc-windows-edition
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
├── .gitignore
├── CHANGELOG.md
├── COMPONENT_PLAN.md
├── Cargo.toml
├── FILE_REGISTER.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── STATUS.md
└── src/
    ├── app.rs
    ├── explorer.rs
    ├── gui/
    ├── main.rs
    ├── node.rs
    ├── settings.rs
    └── wallet.rs
```

## Module Descriptions
- **src/main.rs**: Native Windows executable entry point initializing process runtime and GUI event loops.
- **src/gui/**: Desktop graphical user interface built with `egui` and `eframe`.
- **src/app.rs** & **src/node.rs**: Core desktop app context, async task management, and local node supervisor.
- **src/wallet.rs** & **src/explorer.rs**: Integrated Windows desktop wallet view and embedded block explorer tab.
- **Cargo.toml**: Rust package manifest specifying crate metadata and MSVC target dependencies.

## Build System
Cargo build system targetting `x86_64-pc-windows-msvc`. MSVC toolchain required for native C/C++ linking on Windows.

## Dependencies
Rust std (1.75+), `egui` (0.24+), `eframe`, `tokio` (async runtime), `serde` / `serde_json`, `windows-sys` / `winapi` bindings.
