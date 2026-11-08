// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Memory mapping for kernel image
pub struct MemoryMap {
    pub kernel_start: u64,
    pub kernel_size: u64,
    pub stack_start: u64,
    pub heap_start: u64,
}

impl MemoryMap {
    pub fn new() -> Self {
        Self { kernel_start: 0x100000, kernel_size: 0x200000, stack_start: 0x80000, heap_start: 0x300000 }
    }
    pub fn is_in_kernel(&self, addr: u64) -> bool {
        addr >= self.kernel_start && addr < self.kernel_start + self.kernel_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mmap() {
        let m = MemoryMap::new();
        assert!(m.is_in_kernel(0x100000));
        assert!(!m.is_in_kernel(0x300000));
    }
}
