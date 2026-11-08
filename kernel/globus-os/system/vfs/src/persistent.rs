//! Persistent filesystem metadata and deterministic free-space allocation.

use crate::{DiskInode, FreeSpaceBitmap, INODE_SIZE, MetadataLayout};
use globus_devices::block::{BlockDevice, BlockRequest};

const MAGIC: &[u8; 8] = b"GLOBFS01";
const HEADER_SIZE: usize = 40;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Superblock {
    pub block_size: u32,
    pub total_blocks: u64,
    pub metadata_start: u64,
    pub metadata_blocks: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {
    Io,
    InvalidGeometry,
    InvalidSuperblock,
    Buffer,
    Overflow,
    Bitmap,
    Inode,
}
fn checksum(bytes: &[u8]) -> u32 {
    let mut h = 0x811c9dc5u32;
    for b in bytes {
        h ^= u32::from(*b);
        h = h.wrapping_mul(0x01000193);
    }
    h
}
impl Superblock {
    pub fn encode(&self, out: &mut [u8]) -> Result<(), FsError> {
        if out.len() < HEADER_SIZE {
            return Err(FsError::Buffer);
        }
        out[..HEADER_SIZE].fill(0);
        out[..8].copy_from_slice(MAGIC);
        out[8..12].copy_from_slice(&self.block_size.to_le_bytes());
        out[12..20].copy_from_slice(&self.total_blocks.to_le_bytes());
        out[20..28].copy_from_slice(&self.metadata_start.to_le_bytes());
        out[28..36].copy_from_slice(&self.metadata_blocks.to_le_bytes());
        let digest = checksum(&out[..36]);
        out[36..40].copy_from_slice(&digest.to_le_bytes());
        Ok(())
    }
    pub fn decode(input: &[u8]) -> Result<Self, FsError> {
        if input.len() < HEADER_SIZE || &input[..8] != MAGIC {
            return Err(FsError::InvalidSuperblock);
        }
        let expected = u32::from_le_bytes(input[36..40].try_into().map_err(|_| FsError::Buffer)?);
        if checksum(&input[..36]) != expected {
            return Err(FsError::InvalidSuperblock);
        }
        let block_size = u32::from_le_bytes(input[8..12].try_into().map_err(|_| FsError::Buffer)?);
        let total_blocks =
            u64::from_le_bytes(input[12..20].try_into().map_err(|_| FsError::Buffer)?);
        let metadata_start =
            u64::from_le_bytes(input[20..28].try_into().map_err(|_| FsError::Buffer)?);
        let metadata_blocks =
            u64::from_le_bytes(input[28..36].try_into().map_err(|_| FsError::Buffer)?);
        let end = metadata_start
            .checked_add(metadata_blocks)
            .ok_or(FsError::Overflow)?;
        if !block_size.is_power_of_two()
            || !(512..=65536).contains(&block_size)
            || total_blocks == 0
            || metadata_start == 0
            || metadata_blocks == 0
            || end > total_blocks
        {
            return Err(FsError::InvalidSuperblock);
        }
        Ok(Self {
            block_size,
            total_blocks,
            metadata_start,
            metadata_blocks,
        })
    }
}

pub struct PersistentFs<D> {
    device: D,
    superblock: Superblock,
    layout: MetadataLayout,
    bitmap: FreeSpaceBitmap,
}
impl<D: BlockDevice> PersistentFs<D> {
    fn calculate_layout(sb: Superblock) -> Result<MetadataLayout, FsError> {
        let inode_count = u64::from(sb.block_size) / INODE_SIZE as u64;
        MetadataLayout::calculate(
            sb.total_blocks,
            u64::from(sb.block_size),
            sb.metadata_blocks,
            inode_count,
            1,
        )
        .map_err(|_| FsError::InvalidGeometry)
    }
    fn read_block(device: &mut D, lba: u64, size: usize) -> Result<Vec<u8>, FsError> {
        let mut b = vec![0u8; size];
        device
            .read(
                BlockRequest {
                    lba,
                    blocks: 1,
                    write: false,
                },
                &mut b,
            )
            .map_err(|_| FsError::Io)?;
        Ok(b)
    }
    fn write_block(device: &mut D, lba: u64, b: &[u8]) -> Result<(), FsError> {
        device
            .write(
                BlockRequest {
                    lba,
                    blocks: 1,
                    write: true,
                },
                b,
            )
            .map_err(|_| FsError::Io)
    }
    fn persist_bitmap(&mut self) -> Result<(), FsError> {
        let size = self.superblock.block_size as usize;
        let mut b = vec![0u8; size];
        self.bitmap.encode(&mut b).map_err(|_| FsError::Bitmap)?;
        Self::write_block(&mut self.device, self.layout.bitmap_start, &b)
    }
    fn root_inode() -> DiskInode {
        DiskInode {
            id: 1,
            mode: 0o755,
            kind: 2,
            links: 1,
            size: 0,
            generation: 1,
            data_start: 0,
            data_blocks: 0,
        }
    }
    fn persist_root_inode(
        device: &mut D,
        layout: MetadataLayout,
        size: usize,
    ) -> Result<(), FsError> {
        let mut b = vec![0u8; size];
        Self::root_inode()
            .encode(&mut b)
            .map_err(|_| FsError::Inode)?;
        Self::write_block(device, layout.inode_start, &b)
    }
    fn validate_root_inode(
        device: &mut D,
        layout: MetadataLayout,
        size: usize,
    ) -> Result<(), FsError> {
        let b = Self::read_block(device, layout.inode_start, size)?;
        let inode = DiskInode::decode(&b).map_err(|_| FsError::Inode)?;
        if inode.id != 1 || inode.kind != 2 || inode.data_blocks != 0 {
            return Err(FsError::Inode);
        }
        Ok(())
    }
    pub fn format(mut device: D, metadata_blocks: u64) -> Result<Self, FsError> {
        let g = device.geometry();
        if !g.valid() || metadata_blocks == 0 || metadata_blocks >= g.block_count {
            return Err(FsError::InvalidGeometry);
        }
        let sb = Superblock {
            block_size: g.block_size,
            total_blocks: g.block_count,
            metadata_start: 1,
            metadata_blocks,
        };
        let layout = Self::calculate_layout(sb)?;
        let mut bitmap = FreeSpaceBitmap::new(g.block_count).map_err(|_| FsError::Bitmap)?;
        bitmap
            .reserve(0, layout.data_start)
            .map_err(|_| FsError::Bitmap)?;
        let mut block = vec![0u8; g.block_size as usize];
        sb.encode(&mut block)?;
        Self::write_block(&mut device, 0, &block)?;
        let mut bitmap_block = vec![0u8; g.block_size as usize];
        bitmap
            .encode(&mut bitmap_block)
            .map_err(|_| FsError::Bitmap)?;
        Self::write_block(&mut device, layout.bitmap_start, &bitmap_block)?;
        Self::persist_root_inode(&mut device, layout, g.block_size as usize)?;
        Ok(Self {
            device,
            superblock: sb,
            layout,
            bitmap,
        })
    }
    pub fn mount(mut device: D) -> Result<Self, FsError> {
        let g = device.geometry();
        if !g.valid() {
            return Err(FsError::InvalidGeometry);
        }
        let block = Self::read_block(&mut device, 0, g.block_size as usize)?;
        let sb = Superblock::decode(&block)?;
        if sb.block_size != g.block_size || sb.total_blocks != g.block_count {
            return Err(FsError::InvalidSuperblock);
        }
        let layout = Self::calculate_layout(sb)?;
        let bitmap_block =
            Self::read_block(&mut device, layout.bitmap_start, g.block_size as usize)?;
        let bitmap =
            FreeSpaceBitmap::decode(g.block_count, &bitmap_block).map_err(|_| FsError::Bitmap)?;
        for b in 0..layout.data_start {
            if bitmap.is_free(b).map_err(|_| FsError::Bitmap)? {
                return Err(FsError::InvalidSuperblock);
            }
        }
        Self::validate_root_inode(&mut device, layout, g.block_size as usize)?;
        Ok(Self {
            device,
            superblock: sb,
            layout,
            bitmap,
        })
    }
    pub fn superblock(&self) -> Superblock {
        self.superblock
    }
    pub fn layout(&self) -> MetadataLayout {
        self.layout
    }
    pub fn device_geometry(&self) -> globus_devices::block::BlockGeometry {
        self.device.geometry()
    }
    pub fn allocate_blocks(&mut self, count: u64) -> Result<u64, FsError> {
        let start = self.bitmap.allocate(count).map_err(|_| FsError::Bitmap)?;
        self.persist_bitmap()?;
        Ok(start)
    }
    pub fn release_blocks(&mut self, start: u64, count: u64) -> Result<(), FsError> {
        if start < self.layout.data_start {
            return Err(FsError::InvalidSuperblock);
        }
        self.bitmap
            .release(start, count)
            .map_err(|_| FsError::Bitmap)?;
        self.persist_bitmap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use globus_devices::block::{BlockGeometry, MemoryBlockDevice};
    #[test]
    fn format_mount_and_persist_allocation() {
        let d = MemoryBlockDevice::new(BlockGeometry {
            block_size: 512,
            block_count: 64,
        })
        .unwrap();
        let mut fs = PersistentFs::format(d, 8).unwrap();
        let start = fs.allocate_blocks(2).unwrap();
        let d = fs.device;
        let mut mounted = PersistentFs::mount(d).unwrap();
        assert_eq!(mounted.allocate_blocks(1).unwrap(), start + 2)
    }
    #[test]
    fn root_inode_survives_mount() {
        let d = MemoryBlockDevice::new(BlockGeometry {
            block_size: 512,
            block_count: 64,
        })
        .unwrap();
        let fs = PersistentFs::format(d, 8).unwrap();
        let d = fs.device;
        let mounted = PersistentFs::mount(d).unwrap();
        assert_eq!(mounted.layout().inode_blocks, 1)
    }
    #[test]
    fn rejects_checksum_corruption() {
        let sb = Superblock {
            block_size: 512,
            total_blocks: 32,
            metadata_start: 1,
            metadata_blocks: 4,
        };
        let mut b = [0u8; 512];
        sb.encode(&mut b).unwrap();
        b[28] ^= 1;
        assert_eq!(Superblock::decode(&b), Err(FsError::InvalidSuperblock))
    }
    #[test]
    fn rejects_metadata_overflow() {
        let sb = Superblock {
            block_size: 512,
            total_blocks: 32,
            metadata_start: 31,
            metadata_blocks: 2,
        };
        let mut b = [0u8; 512];
        sb.encode(&mut b).unwrap();
        assert_eq!(Superblock::decode(&b), Err(FsError::InvalidSuperblock))
    }
}
