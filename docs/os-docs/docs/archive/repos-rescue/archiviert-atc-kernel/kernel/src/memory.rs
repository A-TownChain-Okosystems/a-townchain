// ShivaCore — Speicherverwaltung (Paging).
// K-Sprint 2: Liest die vom Bootloader bereits aktive Page-Table aus (CR3)
// und stellt einen `OffsetPageTable`-Mapper bereit, ueber den der Heap-
// Allokator neue Seiten einrichten kann. Kein Fremd-Kernel-Code -- nur die
// `x86_64`-Crate fuer die Register-/Tabellen-Strukturen (kein OS-Layer).

use x86_64::{
    structures::paging::{OffsetPageTable, PageTable},
    PhysAddr, VirtAddr,
};

/// Initialisiert einen `OffsetPageTable`-Mapper.
///
/// # Safety
/// Der Aufrufer muss garantieren, dass das *gesamte* physische RAM ab
/// `physical_memory_offset` linear ins virtuelle Adressvolumen gemappt ist
/// (das erledigt der Bootloader fuer uns, wenn `physical_memory` in der
/// `BootloaderConfig` auf `Mapping::Dynamic` gesetzt ist). Darf nur einmal
/// aufgerufen werden, um `&mut` Aliasing der Page-Tables zu vermeiden.
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

/// Simpler Frame-Allocator, der die vom Bootloader gemeldete Speicherkarte
/// (`MemoryRegions`) linear nach freien Frames durchsucht.
pub struct BootInfoFrameAllocator {
    memory_map: &'static bootloader_api::info::MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// # Safety
    /// `memory_map` muss valide sein; als `USABLE` markierte Regionen
    /// duerfen tatsaechlich frei/unbenutzt sein.
    pub unsafe fn init(memory_map: &'static bootloader_api::info::MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = x86_64::structures::paging::PhysFrame> {
        use bootloader_api::info::MemoryRegionKind;
        use x86_64::structures::paging::PhysFrame;

        let regions = self.memory_map.iter();
        let usable_regions =
            regions.filter(|r| r.kind == MemoryRegionKind::Usable);
        let addr_ranges = usable_regions.map(|r| r.start..r.end);
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl x86_64::structures::paging::FrameAllocator<x86_64::structures::paging::Size4KiB>
    for BootInfoFrameAllocator
{
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
