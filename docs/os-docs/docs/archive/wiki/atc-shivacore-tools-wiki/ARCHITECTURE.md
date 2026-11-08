# ARCHITECTURE.md — atc-shivacore-tools
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
├── .gitignore
├── CHANGELOG.md
├── COMPONENT_PLAN.md
├── FILE_REGISTER.md
├── LICENSE
├── Makefile
├── README.md
├── ROADMAP.md
├── STATUS.md
├── build.atc
├── debug.atc
├── flash.atc
├── profiler.atc
├── qemu.atc
├── scripts/
└── tools/
    └── manager.atc
```

## Module Descriptions
- **scripts/**: Shell automation scripts for toolchain setup, cross-compilation target initialization, and binary verification.
- **Makefile**: GNU Makefile orchestrating targets for build, debug, flash, profiler, and QEMU emulation environments.
- **README.md**: Toolchain installation instructions and shell command usage documentation.
- **tools/**: Embedded binary flashing wrappers, hardware debugging drivers, and memory profiling helpers.

## Build System
GNU Make build automation combined with POSIX Shell / Bash execution scripts.

## Dependencies
Bash 4.0+, GNU Make, QEMU (`qemu-system-x86_64` / `qemu-system-arm`), GCC / Clang cross-compiler, `esptool` / `openocd`.
