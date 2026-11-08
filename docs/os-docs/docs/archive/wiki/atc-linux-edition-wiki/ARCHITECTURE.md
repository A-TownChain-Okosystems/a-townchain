# ARCHITECTURE.md — atc-linux-edition
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
- **src/main.rs**: Native Linux executable entry point handling process signals and initializing GUI runtime.
- **src/gui/**: Native graphical user interface constructed using `egui` / `eframe`.
- **src/app.rs** & **src/node.rs**: Desktop application process manager, RPC sync loop, and Linux daemon supervisor.
- **src/wallet.rs** & **src/settings.rs**: Integrated desktop wallet manager and Linux system configuration interface.
- **Cargo.toml**: Rust package manifest configured for Linux GNU x86_64 targets.

## Build System
Cargo build system targetting `x86_64-unknown-linux-gnu`. GCC / Clang toolchain and pkg-config required for Linux system library resolution.

## Dependencies
Rust std (1.75+), `egui` (0.24+), `eframe`, `tokio`, `serde`, `wayland-client` / `x11-dl`, `alsa` / `dbus` libraries.
