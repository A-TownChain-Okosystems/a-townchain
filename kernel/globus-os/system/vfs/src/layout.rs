//! Deterministic on-disk metadata layout calculation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutError {
    InvalidGeometry,
    Overflow,
    MetadataTooLarge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetadataLayout {
    pub bitmap_start: u64,
    pub bitmap_blocks: u64,
    pub inode_start: u64,
    pub inode_blocks: u64,
    pub journal_start: u64,
    pub journal_blocks: u64,
    pub data_start: u64,
}

fn ceil_div(value: u64, divisor: u64) -> Result<u64, LayoutError> {
    if divisor == 0 {
        return Err(LayoutError::InvalidGeometry);
    }
    value
        .checked_add(divisor - 1)
        .ok_or(LayoutError::Overflow)
        .map(|v| v / divisor)
}

impl MetadataLayout {
    /// Calculates a stable metadata layout after the superblock at block zero.
    /// Each inode occupies 64 bytes; the journal is caller-sized.
    pub fn calculate(
        total_blocks: u64,
        block_size: u64,
        metadata_blocks: u64,
        inode_count: u64,
        journal_blocks: u64,
    ) -> Result<Self, LayoutError> {
        if total_blocks < 4
            || !block_size.is_power_of_two()
            || block_size < 512
            || metadata_blocks == 0
            || inode_count == 0
            || journal_blocks == 0
        {
            return Err(LayoutError::InvalidGeometry);
        }
        let bitmap_bytes = ceil_div(total_blocks, 8)?;
        let bitmap_blocks = ceil_div(bitmap_bytes, block_size)?;
        let inode_bytes = inode_count.checked_mul(64).ok_or(LayoutError::Overflow)?;
        let inode_blocks = ceil_div(inode_bytes, block_size)?;
        let bitmap_start: u64 = 1;
        let inode_start = bitmap_start
            .checked_add(bitmap_blocks)
            .ok_or(LayoutError::Overflow)?;
        let journal_start = inode_start
            .checked_add(inode_blocks)
            .ok_or(LayoutError::Overflow)?;
        let data_start = journal_start
            .checked_add(journal_blocks)
            .ok_or(LayoutError::Overflow)?;
        let metadata_end = 1u64
            .checked_add(metadata_blocks)
            .ok_or(LayoutError::Overflow)?;
        if data_start > metadata_end || data_start >= total_blocks {
            return Err(LayoutError::MetadataTooLarge);
        }
        Ok(Self {
            bitmap_start,
            bitmap_blocks,
            inode_start,
            inode_blocks,
            journal_start,
            journal_blocks,
            data_start,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_is_deterministic_and_ordered() {
        let l = MetadataLayout::calculate(1024, 512, 32, 64, 8).unwrap();
        assert_eq!(l.bitmap_start, 1);
        assert!(l.bitmap_start < l.inode_start);
        assert!(l.inode_start < l.journal_start);
        assert!(l.journal_start < l.data_start);
    }

    #[test]
    fn rejects_metadata_overflow() {
        assert_eq!(
            MetadataLayout::calculate(32, 512, 2, 64, 8),
            Err(LayoutError::MetadataTooLarge)
        );
    }

    #[test]
    fn rejects_invalid_block_size() {
        assert_eq!(
            MetadataLayout::calculate(128, 1000, 16, 8, 2),
            Err(LayoutError::InvalidGeometry)
        );
    }
}
