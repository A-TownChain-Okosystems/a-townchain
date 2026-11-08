//! PCI/PCIe discovery and MMIO contracts.
//!
//! Raw config-space access and BAR mapping remain privileged HAL operations.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PciAddress {
    pub segment: u16,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}
impl PciAddress {
    pub const fn new(segment: u16, bus: u8, device: u8, function: u8) -> Self {
        Self {
            segment,
            bus,
            device,
            function,
        }
    }
    pub const fn valid(&self) -> bool {
        self.device < 32 && self.function < 8
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciId {
    pub vendor: u16,
    pub device: u16,
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
}
impl PciId {
    pub const fn is_nvme(&self) -> bool {
        self.class == 0x01 && self.subclass == 0x08
    }
    pub const fn is_network(&self) -> bool {
        self.class == 0x02
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarKind {
    Memory32,
    Memory64,
    Io,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciBar {
    pub index: u8,
    pub kind: BarKind,
    pub base: u64,
    pub size: u64,
    pub prefetchable: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciError {
    InvalidAddress,
    ConfigAccessFailed,
    InvalidBar,
    MappingDenied,
}
pub trait PciConfigAccess {
    fn read_u32(&self, address: PciAddress, offset: u16) -> Result<u32, PciError>;
    fn write_u32(&self, address: PciAddress, offset: u16, value: u32) -> Result<(), PciError>;
}
pub trait MmioMapper {
    type Mapping;
    fn map_device_bar(&self, bar: PciBar) -> Result<Self::Mapping, PciError>;
}

pub fn decode_id(class: u8, subclass: u8, prog_if: u8, vendor: u16, device: u16) -> PciId {
    PciId {
        vendor,
        device,
        class,
        subclass,
        prog_if,
    }
}
pub fn validate_bar(bar: PciBar) -> Result<(), PciError> {
    if bar.index >= 6
        || bar.size == 0
        || !bar.size.is_power_of_two()
        || bar.base.checked_add(bar.size).is_none()
    {
        Err(PciError::InvalidBar)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn supports_64bit_bar_addresses() {
        let bar = PciBar {
            index: 0,
            kind: BarKind::Memory64,
            base: 0x1_0000_0000,
            size: 0x1000,
            prefetchable: false,
        };
        assert!(bar.base > u32::MAX as u64);
        assert!(validate_bar(bar).is_ok());
    }
    #[test]
    fn identifies_nvme() {
        assert!(decode_id(1, 8, 2, 0, 0).is_nvme());
    }
}
