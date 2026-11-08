//! Deterministic virtual-to-physical page mapping contract.

use crate::{AddressSpace, PageFlags, VirtualAddress};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhysicalAddress(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mapping {
    pub virtual_address: VirtualAddress,
    pub physical_address: PhysicalAddress,
    pub flags: PageFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapError {
    Unaligned,
    AlreadyMapped,
    NotMapped,
    AddressSpaceMismatch,
}

#[derive(Debug, Default)]
pub struct PageTable {
    mappings: std::collections::BTreeMap<(AddressSpace, VirtualAddress), Mapping>,
}

impl PageTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn map(&mut self, space: AddressSpace, mapping: Mapping) -> Result<(), MapError> {
        if !mapping.virtual_address.0.is_multiple_of(4096)
            || !mapping.physical_address.0.is_multiple_of(4096)
        {
            return Err(MapError::Unaligned);
        }
        let key = (space, mapping.virtual_address);
        if self.mappings.contains_key(&key) {
            return Err(MapError::AlreadyMapped);
        }
        self.mappings.insert(key, mapping);
        Ok(())
    }

    pub fn unmap(
        &mut self,
        space: AddressSpace,
        virtual_address: VirtualAddress,
    ) -> Result<Mapping, MapError> {
        self.mappings
            .remove(&(space, virtual_address))
            .ok_or(MapError::NotMapped)
    }

    pub fn lookup(&self, space: AddressSpace, virtual_address: VirtualAddress) -> Option<Mapping> {
        self.mappings.get(&(space, virtual_address)).copied()
    }

    pub fn mapped_pages(&self, space: AddressSpace) -> usize {
        self.mappings
            .range((space, VirtualAddress(0))..=(space, VirtualAddress(u64::MAX)))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maps_and_unmaps_page() {
        let space = AddressSpace(1);
        let mut table = PageTable::new();
        let mapping = Mapping {
            virtual_address: VirtualAddress(0x4000),
            physical_address: PhysicalAddress(0x8000),
            flags: PageFlags::user_read_only(),
        };
        table.map(space, mapping).unwrap();
        assert_eq!(table.lookup(space, VirtualAddress(0x4000)), Some(mapping));
        assert_eq!(table.unmap(space, VirtualAddress(0x4000)), Ok(mapping));
    }

    #[test]
    fn rejects_duplicate_and_unaligned_pages() {
        let space = AddressSpace(2);
        let mut table = PageTable::new();
        let mapping = Mapping {
            virtual_address: VirtualAddress(0x1000),
            physical_address: PhysicalAddress(0x2000),
            flags: PageFlags::user_read_only(),
        };
        table.map(space, mapping).unwrap();
        assert_eq!(table.map(space, mapping), Err(MapError::AlreadyMapped));
        assert_eq!(
            table.map(
                space,
                Mapping {
                    virtual_address: VirtualAddress(0x1001),
                    ..mapping
                }
            ),
            Err(MapError::Unaligned)
        );
    }
}
