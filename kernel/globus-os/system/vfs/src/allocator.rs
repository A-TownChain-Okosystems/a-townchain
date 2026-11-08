//! Persistent block allocation policy for the filesystem.
//! Rustfmt is enforced by the repository auto-format gate.

use crate::bitmap::{BitmapError, FreeSpaceBitmap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationError {
    Bitmap(BitmapError),
    InvalidRange,
    Overflow,
    OutOfSpace,
}

impl From<BitmapError> for AllocationError {
    fn from(value: BitmapError) -> Self {
        Self::Bitmap(value)
    }
}

/// Allocates data blocks while keeping the reserved metadata range unavailable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockAllocator {
    bitmap: FreeSpaceBitmap,
    reserved_start: u64,
    reserved_blocks: u64,
}

impl BlockAllocator {
    pub fn new(
        total_blocks: u64,
        reserved_start: u64,
        reserved_blocks: u64,
    ) -> Result<Self, AllocationError> {
        if total_blocks == 0 {
            return Err(AllocationError::InvalidRange);
        }
        let end = reserved_start
            .checked_add(reserved_blocks)
            .ok_or(AllocationError::Overflow)?;
        if reserved_start >= total_blocks || end > total_blocks || reserved_blocks == 0 {
            return Err(AllocationError::InvalidRange);
        }
        let mut bitmap = FreeSpaceBitmap::new(total_blocks).map_err(AllocationError::Bitmap)?;
        for block in reserved_start..end {
            bitmap
                .set_free(block, false)
                .map_err(AllocationError::Bitmap)?;
        }
        // Block zero contains the superblock and is never data-allocatable.
        if reserved_start != 0 {
            bitmap.set_free(0, false).map_err(AllocationError::Bitmap)?;
        }
        Ok(Self {
            bitmap,
            reserved_start,
            reserved_blocks,
        })
    }

    pub fn allocate(&mut self, blocks: u64) -> Result<u64, AllocationError> {
        if blocks == 0 {
            return Err(AllocationError::InvalidRange);
        }
        let count = blocks;
        let total = self.bitmap.len();
        if blocks > total {
            return Err(AllocationError::OutOfSpace);
        }
        for start in 0..=total - count {
            let mut free = true;
            for i in 0..count {
                if !self
                    .bitmap
                    .is_free(start + i)
                    .map_err(AllocationError::Bitmap)?
                {
                    free = false;
                    break;
                }
            }
            if free {
                for i in 0..count {
                    self.bitmap
                        .set_free(start + i, false)
                        .map_err(AllocationError::Bitmap)?;
                }
                return Ok(start);
            }
        }
        Err(AllocationError::OutOfSpace)
    }

    pub fn release(&mut self, start: u64, blocks: u64) -> Result<(), AllocationError> {
        let end = start.checked_add(blocks).ok_or(AllocationError::Overflow)?;
        let total = self.bitmap.len();
        if blocks == 0 || end > total {
            return Err(AllocationError::InvalidRange);
        }
        let reserved_end = self
            .reserved_start
            .checked_add(self.reserved_blocks)
            .ok_or(AllocationError::Overflow)?;
        if start < reserved_end && end > self.reserved_start {
            return Err(AllocationError::InvalidRange);
        }
        if start == 0 {
            return Err(AllocationError::InvalidRange);
        }
        for block in start..end {
            self.bitmap
                .set_free(block, true)
                .map_err(AllocationError::Bitmap)?;
        }
        Ok(())
    }

    pub fn is_free(&self, block: u64) -> Result<bool, AllocationError> {
        self.bitmap.is_free(block).map_err(AllocationError::Bitmap)
    }

    pub fn free_blocks(&self) -> u64 {
        (0..self.bitmap.len())
            .filter(|&i| self.bitmap.is_free(i).unwrap_or(false))
            .count() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserves_metadata_and_allocates() {
        let mut a = BlockAllocator::new(32, 1, 4).unwrap();
        assert!(!a.is_free(0).unwrap());
        assert!(!a.is_free(2).unwrap());
        let first = a.allocate(2).unwrap();
        assert_eq!(first, 5);
        assert!(!a.is_free(first).unwrap());
        a.release(first, 2).unwrap();
        assert!(a.is_free(first).unwrap());
    }

    #[test]
    fn cannot_release_reserved_range() {
        let mut a = BlockAllocator::new(16, 1, 4).unwrap();
        assert_eq!(a.release(2, 1), Err(AllocationError::InvalidRange));
    }

    #[test]
    fn rejects_zero_allocation() {
        let mut a = BlockAllocator::new(16, 1, 4).unwrap();
        assert_eq!(a.allocate(0), Err(AllocationError::InvalidRange));
    }
}
