//! Deterministic virtual-page allocator contract.

use crate::VirtualAddress;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRange {
    pub start: VirtualAddress,
    pub pages: u64,
}

#[derive(Debug)]
pub struct PageAllocator {
    next: u64,
    page_size: u64,
}

impl PageAllocator {
    pub fn new(start: VirtualAddress, page_size: u64) -> Option<Self> {
        if page_size == 0 || !page_size.is_power_of_two() {
            return None;
        }
        Some(Self {
            next: start.0,
            page_size,
        })
    }

    pub fn allocate(&mut self, pages: u64) -> Option<PageRange> {
        if pages == 0 {
            return None;
        }
        let bytes = pages.checked_mul(self.page_size)?;
        let start = self.next;
        self.next = self.next.checked_add(bytes)?;
        Some(PageRange {
            start: VirtualAddress(start),
            pages,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allocates_contiguous_pages() {
        let mut a = PageAllocator::new(VirtualAddress(0x1000), 0x1000).unwrap();
        assert_eq!(a.allocate(2).unwrap().start, VirtualAddress(0x1000));
        assert_eq!(a.allocate(1).unwrap().start, VirtualAddress(0x3000));
    }
}
