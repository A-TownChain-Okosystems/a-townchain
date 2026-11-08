# ARCHITECTURE.md — atc-drivers

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-drivers/
├── Cargo.toml — Hardware driver crate manifest for no_std kernel space
├── .gitignore — Git ignore settings
└── src/
    ├── lib.rs — Crate root and hardware driver manager registry
    ├── usb.rs — xHCI/uHCI USB host controller driver and USB transfer engine
    ├── gpu.rs — Display controller drivers, framebuffer acceleration, and mode setting
    ├── audio.rs — HD Audio / AC97 sound controller driver and DMA audio stream management
    ├── storage.rs — NVMe and AHCI SATA storage controller drivers and command queues
    ├── hid.rs — Human Interface Device drivers (USB Keyboard, Mouse, Touchpad)
    ├── net.rs — Network Interface Card drivers (Intel e1000, Realtek RTL8139, VirtIO-net)
    └── driver_framework.rs — Unified driver lifecycle management, probe, and driver traits
```

## Module Descriptions
- src/lib.rs — Driver subsystem entry point and dynamic hardware driver registry.
- src/usb.rs — Manages USB host controllers, pipe transfers, and USB device enumeration.
- src/gpu.rs — Configures display controller hardware, framebuffer blitting, and resolution modes.
- src/audio.rs — Drives audio controller hardware with DMA ring buffers for audio processing.
- src/storage.rs — Low-latency driver for NVMe SSD queues and SATA storage controllers.
- src/hid.rs — Parses raw scancodes and mouse motion events into structured HID inputs.
- src/net.rs — Driver layer for physical ethernet hardware and VirtIO network adapters.
- src/driver_framework.rs — Standardized `Driver`, `Device`, and `Bus` traits for system drivers.

## Build System
- Cargo.toml — `#![no_std]` Rust library targetable by ShivaCore kernel driver subsystem.

## Dependencies
- spin — Bare-metal spinlocks for interrupt and hardware register locking.
- bitflags — Safe hardware control register bitfield definitions.
