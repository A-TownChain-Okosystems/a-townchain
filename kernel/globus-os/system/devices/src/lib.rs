//! Isolated device-service registry and hardware contracts.

pub mod acpi;
pub mod apic;
pub mod block;
pub mod block_cache;
pub mod block_manager;
pub mod boot;
pub mod ethernet;
pub mod interrupt;
pub mod iommu;
pub mod msi;
pub mod nvme;
pub mod nvme_controller;
pub mod pci;
pub mod registry;
pub mod smp;
pub mod timer;

pub use block_cache::{BlockCache, CacheError};
pub use block_manager::{BlockDeviceId, BlockDeviceRegistry};
pub use registry::DeviceRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeviceId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    Storage,
    Network,
    Display,
    Input,
    Audio,
    Usb,
    Gpu,
    Other,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub id: DeviceId,
    pub class: DeviceClass,
    pub driver: String,
}
