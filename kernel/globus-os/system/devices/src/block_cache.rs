//! Write-through block cache boundary. The cache never acknowledges data that was not written to the device.

use super::block::{BlockDevice, BlockError, BlockGeometry, BlockRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheError {
    Block(BlockError),
    InvalidBlock,
}

pub struct BlockCache<D> {
    device: D,
    geometry: BlockGeometry,
    cached_lba: Option<u64>,
    data: Vec<u8>,
}

impl<D: BlockDevice> BlockCache<D> {
    pub fn new(device: D) -> Self {
        let geometry = device.geometry();
        Self {
            device,
            geometry,
            cached_lba: None,
            data: vec![0; geometry.block_size as usize],
        }
    }

    pub fn geometry(&self) -> BlockGeometry {
        self.geometry
    }

    pub fn read_block(&mut self, lba: u64, out: &mut [u8]) -> Result<(), CacheError> {
        if out.len() != self.geometry.block_size as usize {
            return Err(CacheError::InvalidBlock);
        }
        if self.cached_lba != Some(lba) {
            self.device
                .read(
                    BlockRequest {
                        lba,
                        blocks: 1,
                        write: false,
                    },
                    &mut self.data,
                )
                .map_err(CacheError::Block)?;
            self.cached_lba = Some(lba);
        }
        out.copy_from_slice(&self.data);
        Ok(())
    }

    pub fn write_block(&mut self, lba: u64, input: &[u8]) -> Result<(), CacheError> {
        if input.len() != self.geometry.block_size as usize {
            return Err(CacheError::InvalidBlock);
        }
        self.device
            .write(
                BlockRequest {
                    lba,
                    blocks: 1,
                    write: true,
                },
                input,
            )
            .map_err(CacheError::Block)?;
        self.data.copy_from_slice(input);
        self.cached_lba = Some(lba);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::MemoryBlockDevice;
    #[test]
    fn write_through_round_trip() {
        let device = MemoryBlockDevice::new(BlockGeometry {
            block_size: 512,
            block_count: 2,
        })
        .unwrap();
        let mut cache = BlockCache::new(device);
        let input = [0xA5; 512];
        let mut out = [0; 512];
        cache.write_block(1, &input).unwrap();
        cache.read_block(1, &mut out).unwrap();
        assert_eq!(out, input);
    }
}
