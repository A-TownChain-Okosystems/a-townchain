// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Memory and Address Space management (SC-001 / SC-007).

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PagePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub user: bool,
}

impl PagePermissions {
    pub fn rx() -> Self {
        Self { read: true, write: false, execute: true, user: true }
    }

    pub fn rw() -> Self {
        Self { read: true, write: true, execute: false, user: true }
    }

    pub fn rwx() -> Self {
        Self { read: true, write: true, execute: true, user: true }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryRegion {
    pub phys_base: u64,
    pub virt_base: u64,
    pub size: usize,
    pub permissions: PagePermissions,
    pub is_mapped: bool,
}

impl MemoryRegion {
    pub fn contains_virt(&self, addr: u64) -> bool {
        addr >= self.virt_base && addr < self.virt_base + self.size as u64
    }

    pub fn translate(&self, virt_addr: u64) -> Option<u64> {
        if self.contains_virt(virt_addr) {
            let offset = virt_addr - self.virt_base;
            Some(self.phys_base + offset)
        } else {
            None
        }
    }
}

pub struct AddressSpace {
    regions: Vec<MemoryRegion>,
}

impl AddressSpace {
    pub fn new() -> Self {
        Self { regions: Vec::new() }
    }

    pub fn map(&mut self, phys_base: u64, virt_base: u64, size: usize, permissions: PagePermissions) -> Result<(), String> {
        // Check for overlap
        for r in &self.regions {
            let virt_end = virt_base + size as u64;
            let r_end = r.virt_base + r.size as u64;
            if virt_base < r_end && virt_end > r.virt_base {
                return Err(String::from("Address space region overlap"));
            }
        }

        self.regions.push(MemoryRegion {
            phys_base,
            virt_base,
            size,
            permissions,
            is_mapped: true,
        });
        Ok(())
    }

    pub fn translate(&self, virt_addr: u64) -> Option<(u64, PagePermissions)> {
        for r in &self.regions {
            if let Some(phys) = r.translate(virt_addr) {
                return Some((phys, r.permissions));
            }
        }
        None
    }

    pub fn regions(&self) -> &[MemoryRegion] {
        &self.regions
    }
}

impl Default for AddressSpace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_space_mapping_and_translation() {
        let mut aspace = AddressSpace::new();
        aspace.map(0x1000_0000, 0x4000_0000, 0x1000, PagePermissions::rw()).unwrap();

        let (phys, perms) = aspace.translate(0x4000_0080).expect("Translation should succeed");
        assert_eq!(phys, 0x1000_0080);
        assert!(perms.read && perms.write && !perms.execute);

        // Overlapping map must fail
        assert!(aspace.map(0x2000_0000, 0x4000_0500, 0x1000, PagePermissions::rx()).is_err());
    }
}
