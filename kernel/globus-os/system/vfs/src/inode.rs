//! Stable on-disk inode representation and deterministic inode allocation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InodeDiskError {
    Buffer,
    Invalid,
    Overflow,
}

pub const INODE_SIZE: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiskInode {
    pub id: u64,
    pub mode: u16,
    pub kind: u8,
    pub links: u32,
    pub size: u64,
    pub generation: u64,
    pub data_start: u64,
    pub data_blocks: u64,
}

fn checksum(bytes: &[u8]) -> u32 {
    let mut hash = 0x811c9dc5u32;
    for byte in bytes {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x01000193);
    }
    hash
}

impl DiskInode {
    pub fn encode(&self, out: &mut [u8]) -> Result<(), InodeDiskError> {
        if out.len() < INODE_SIZE {
            return Err(InodeDiskError::Buffer);
        }
        if self.id == 0 || self.links == 0 {
            return Err(InodeDiskError::Invalid);
        }
        out[..INODE_SIZE].fill(0);
        out[..8].copy_from_slice(&self.id.to_le_bytes());
        out[8..10].copy_from_slice(&self.mode.to_le_bytes());
        out[10] = self.kind;
        out[12..16].copy_from_slice(&self.links.to_le_bytes());
        out[16..24].copy_from_slice(&self.size.to_le_bytes());
        out[24..32].copy_from_slice(&self.generation.to_le_bytes());
        out[32..40].copy_from_slice(&self.data_start.to_le_bytes());
        out[40..48].copy_from_slice(&self.data_blocks.to_le_bytes());
        let digest = checksum(&out[..48]);
        out[48..52].copy_from_slice(&digest.to_le_bytes());
        Ok(())
    }

    pub fn decode(input: &[u8]) -> Result<Self, InodeDiskError> {
        if input.len() < INODE_SIZE {
            return Err(InodeDiskError::Buffer);
        }
        let expected = u32::from_le_bytes(
            input[48..52]
                .try_into()
                .map_err(|_| InodeDiskError::Buffer)?,
        );
        if checksum(&input[..48]) != expected
            || input[11] != 0
            || input[52..INODE_SIZE].iter().any(|b| *b != 0)
        {
            return Err(InodeDiskError::Invalid);
        }
        let inode = Self {
            id: u64::from_le_bytes(input[..8].try_into().map_err(|_| InodeDiskError::Buffer)?),
            mode: u16::from_le_bytes(
                input[8..10]
                    .try_into()
                    .map_err(|_| InodeDiskError::Buffer)?,
            ),
            kind: input[10],
            links: u32::from_le_bytes(
                input[12..16]
                    .try_into()
                    .map_err(|_| InodeDiskError::Buffer)?,
            ),
            size: u64::from_le_bytes(
                input[16..24]
                    .try_into()
                    .map_err(|_| InodeDiskError::Buffer)?,
            ),
            generation: u64::from_le_bytes(
                input[24..32]
                    .try_into()
                    .map_err(|_| InodeDiskError::Buffer)?,
            ),
            data_start: u64::from_le_bytes(
                input[32..40]
                    .try_into()
                    .map_err(|_| InodeDiskError::Buffer)?,
            ),
            data_blocks: u64::from_le_bytes(
                input[40..48]
                    .try_into()
                    .map_err(|_| InodeDiskError::Buffer)?,
            ),
        };
        if inode.id == 0 || inode.links == 0 {
            return Err(InodeDiskError::Invalid);
        }
        inode
            .data_start
            .checked_add(inode.data_blocks)
            .ok_or(InodeDiskError::Overflow)?;
        Ok(inode)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InodeAllocator {
    next: u64,
}

impl InodeAllocator {
    pub fn new(first: u64) -> Result<Self, InodeDiskError> {
        if first == 0 {
            return Err(InodeDiskError::Invalid);
        }
        Ok(Self { next: first })
    }

    pub fn allocate(&mut self) -> Result<u64, InodeDiskError> {
        let id = self.next;
        self.next = self.next.checked_add(1).ok_or(InodeDiskError::Overflow)?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inode_round_trip() {
        let inode = DiskInode {
            id: 2,
            mode: 0o644,
            kind: 1,
            links: 1,
            size: 123,
            generation: 4,
            data_start: 100,
            data_blocks: 3,
        };
        let mut bytes = [0u8; INODE_SIZE];
        inode.encode(&mut bytes).unwrap();
        assert_eq!(DiskInode::decode(&bytes).unwrap(), inode);
    }

    #[test]
    fn corruption_is_rejected() {
        let inode = DiskInode {
            id: 2,
            mode: 0o644,
            kind: 1,
            links: 1,
            size: 1,
            generation: 1,
            data_start: 2,
            data_blocks: 1,
        };
        let mut bytes = [0u8; INODE_SIZE];
        inode.encode(&mut bytes).unwrap();
        bytes[24] ^= 1;
        assert_eq!(DiskInode::decode(&bytes), Err(InodeDiskError::Invalid));
    }

    #[test]
    fn allocator_is_monotonic() {
        let mut allocator = InodeAllocator::new(2).unwrap();
        assert_eq!(allocator.allocate().unwrap(), 2);
        assert_eq!(allocator.allocate().unwrap(), 3);
    }
}
