//! Persistent file data storage over block extents.

use crate::{ExtentError, ExtentMap};
use globus_devices::block::{BlockDevice, BlockRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStoreError {
    Io,
    Buffer,
    Overflow,
    Extent(ExtentError),
    OutOfRange,
}
impl From<ExtentError> for FileStoreError {
    fn from(value: ExtentError) -> Self {
        Self::Extent(value)
    }
}

pub struct FileStore<D> {
    device: D,
    block_size: u64,
}

impl<D: BlockDevice> FileStore<D> {
    pub fn new(device: D) -> Result<Self, FileStoreError> {
        let g = device.geometry();
        if !g.valid() || g.block_size == 0 {
            return Err(FileStoreError::Buffer);
        }
        Ok(Self {
            device,
            block_size: u64::from(g.block_size),
        })
    }
    pub fn block_size(&self) -> u64 {
        self.block_size
    }
    pub fn into_device(self) -> D {
        self.device
    }
    pub fn read_at(
        &mut self,
        extents: &ExtentMap,
        offset: u64,
        out: &mut [u8],
    ) -> Result<usize, FileStoreError> {
        if out.is_empty() {
            return Ok(0);
        }
        let end = offset
            .checked_add(out.len() as u64)
            .ok_or(FileStoreError::Overflow)?;
        let first = offset / self.block_size;
        let last = (end - 1) / self.block_size;
        let mut copied = 0usize;
        for logical in first..=last {
            let physical = extents.physical_for(logical)?;
            let mut block = vec![0u8; self.block_size as usize];
            self.device
                .read(
                    BlockRequest {
                        lba: physical,
                        blocks: 1,
                        write: false,
                    },
                    &mut block,
                )
                .map_err(|_| FileStoreError::Io)?;
            let block_start = logical * self.block_size;
            let from = offset.max(block_start) - block_start;
            let to = end.min(block_start + self.block_size) - block_start;
            let len = (to - from) as usize;
            out[copied..copied + len].copy_from_slice(&block[from as usize..to as usize]);
            copied += len;
        }
        Ok(copied)
    }
    pub fn write_at(
        &mut self,
        extents: &ExtentMap,
        offset: u64,
        data: &[u8],
    ) -> Result<usize, FileStoreError> {
        if data.is_empty() {
            return Ok(0);
        }
        let end = offset
            .checked_add(data.len() as u64)
            .ok_or(FileStoreError::Overflow)?;
        let first = offset / self.block_size;
        let last = (end - 1) / self.block_size;
        let mut consumed = 0usize;
        for logical in first..=last {
            let physical = extents.physical_for(logical)?;
            let block_start = logical * self.block_size;
            let from = offset.max(block_start) - block_start;
            let to = end.min(block_start + self.block_size) - block_start;
            let len = (to - from) as usize;
            let mut block = vec![0u8; self.block_size as usize];
            if from != 0 || to != self.block_size {
                self.device
                    .read(
                        BlockRequest {
                            lba: physical,
                            blocks: 1,
                            write: false,
                        },
                        &mut block,
                    )
                    .map_err(|_| FileStoreError::Io)?;
            }
            block[from as usize..to as usize].copy_from_slice(&data[consumed..consumed + len]);
            self.device
                .write(
                    BlockRequest {
                        lba: physical,
                        blocks: 1,
                        write: true,
                    },
                    &block,
                )
                .map_err(|_| FileStoreError::Io)?;
            consumed += len;
        }
        Ok(consumed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Extent;
    use globus_devices::block::{BlockGeometry, MemoryBlockDevice};
    #[test]
    fn partial_block_round_trip() {
        let d = MemoryBlockDevice::new(BlockGeometry {
            block_size: 512,
            block_count: 16,
        })
        .unwrap();
        let mut s = FileStore::new(d).unwrap();
        let mut m = ExtentMap::new();
        m.insert(Extent {
            logical: 0,
            physical: 4,
            blocks: 2,
        })
        .unwrap();
        let data = b"globus-persistent-data";
        assert_eq!(s.write_at(&m, 3, data).unwrap(), data.len());
        let mut out = vec![0u8; data.len()];
        assert_eq!(s.read_at(&m, 3, &mut out).unwrap(), data.len());
        assert_eq!(&out, data);
    }
}
