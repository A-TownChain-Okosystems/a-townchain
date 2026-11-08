//! Boot hand-off contracts for UEFI/firmware memory discovery.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryKind {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    Mmio,
    MmioPort,
    BootServices,
    RuntimeServices,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryRegion {
    pub physical_start: u64,
    pub page_count: u64,
    pub kind: MemoryKind,
    pub cacheable: bool,
}

impl MemoryRegion {
    pub fn end_exclusive(&self) -> Option<u64> {
        self.page_count
            .checked_mul(4096)?
            .checked_add(self.physical_start)
    }
    pub fn is_dma_safe(&self) -> bool {
        matches!(self.kind, MemoryKind::Usable) && self.cacheable
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Framebuffer {
    pub base: u64,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub bytes_per_pixel: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootInfo<'a> {
    pub memory: &'a [MemoryRegion],
    pub framebuffer: Option<Framebuffer>,
    pub rsdp: Option<u64>,
}

pub fn validate_framebuffer(fb: Framebuffer) -> bool {
    fb.width != 0
        && fb.height != 0
        && fb.bytes_per_pixel >= 1
        && fb.bytes_per_pixel <= 8
        && fb.stride >= fb.width.saturating_mul(u32::from(fb.bytes_per_pixel))
        && fb.size >= u64::from(fb.stride).saturating_mul(u64::from(fb.height))
}
