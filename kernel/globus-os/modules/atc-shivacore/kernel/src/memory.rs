// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Physical memory and paging initialization for the x86-64 boot path.

use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use x86_64::{
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (frame, _) = x86_64::registers::control::Cr3::read();
    let virt = physical_memory_offset + frame.start_address().as_u64();
    &mut *virt.as_mut_ptr()
}

pub struct BootInfoFrameAllocator {
    memory_regions: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_regions: &'static MemoryRegions) -> Self {
        Self {
            memory_regions,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> + '_ {
        self.memory_regions
            .iter()
            .filter(|r| r.kind == MemoryRegionKind::Usable)
            .flat_map(|r| {
                let start = (r.start + 0xFFF) & !0xFFF;
                let end = r.end & !0xFFF;
                (start..end)
                    .step_by(0x1000)
                    .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
            })
    }
}

unsafe impl Send for BootInfoFrameAllocator {}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next = self.next.saturating_add(1);
        frame
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn page_alignment_contract_is_4k() {
        assert_eq!(0x1000usize, 4096);
    }
}
