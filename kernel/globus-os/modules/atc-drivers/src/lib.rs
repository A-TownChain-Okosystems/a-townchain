//! Hardware-Treiber — USB, GPU, Audio, SATA/NVMe, HID, Network
//!
//! Part of the A-TownChain-Okosystems ecosystem.
//! Copyright (c) Michael Wroblewski. All Rights Reserved.

#![no_std]

pub mod audio;
pub mod driver_framework;
pub mod gpu;
pub mod hid;
pub mod net;
pub mod storage;
pub mod usb;
