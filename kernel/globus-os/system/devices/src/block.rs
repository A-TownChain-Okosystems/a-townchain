//! Generic block-device contract used by NVMe and persistent storage.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockGeometry {
    pub block_size: u32,
    pub block_count: u64,
}

impl BlockGeometry {
    pub fn capacity_bytes(&self) -> Option<u64> {
        u64::from(self.block_size).checked_mul(self.block_count)
    }
    pub fn valid(&self) -> bool {
        self.block_size.is_power_of_two()
            && (512..=65536).contains(&self.block_size)
            && self.block_count > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockRequest {
    pub lba: u64,
    pub blocks: u32,
    pub write: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockError {
    InvalidGeometry,
    OutOfRange,
    ZeroLength,
    Overflow,
    InvalidBuffer,
    NotReady,
    Io,
}

pub fn validate_request(g: BlockGeometry, r: BlockRequest) -> Result<(), BlockError> {
    if !g.valid() {
        return Err(BlockError::InvalidGeometry);
    }
    if r.blocks == 0 {
        return Err(BlockError::ZeroLength);
    }
    let end = r
        .lba
        .checked_add(u64::from(r.blocks))
        .ok_or(BlockError::Overflow)?;
    if end > g.block_count {
        return Err(BlockError::OutOfRange);
    }
    Ok(())
}

pub trait BlockDevice {
    fn geometry(&self) -> BlockGeometry;
    fn read(&mut self, request: BlockRequest, destination: &mut [u8]) -> Result<(), BlockError>;
    fn write(&mut self, request: BlockRequest, source: &[u8]) -> Result<(), BlockError>;
}

/// Deterministic in-memory block device used for tests and as a HAL adapter target.
/// Hardware drivers implement `BlockDevice` against their own DMA/MMIO queues.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryBlockDevice {
    geometry: BlockGeometry,
    storage: Vec<u8>,
}

impl MemoryBlockDevice {
    pub fn new(geometry: BlockGeometry) -> Result<Self, BlockError> {
        let bytes = geometry.capacity_bytes().ok_or(BlockError::Overflow)?;
        if !geometry.valid() || bytes > usize::MAX as u64 {
            return Err(BlockError::InvalidGeometry);
        }
        Ok(Self {
            geometry,
            storage: vec![0; bytes as usize],
        })
    }

    fn byte_range(
        &self,
        request: BlockRequest,
        buffer_len: usize,
    ) -> Result<std::ops::Range<usize>, BlockError> {
        validate_request(self.geometry, request)?;
        let block_size = self.geometry.block_size as usize;
        let bytes = (request.blocks as usize)
            .checked_mul(block_size)
            .ok_or(BlockError::Overflow)?;
        if buffer_len != bytes {
            return Err(BlockError::InvalidBuffer);
        }
        let start = (request.lba as usize)
            .checked_mul(block_size)
            .ok_or(BlockError::Overflow)?;
        let end = start.checked_add(bytes).ok_or(BlockError::Overflow)?;
        Ok(start..end)
    }
}

impl BlockDevice for MemoryBlockDevice {
    fn geometry(&self) -> BlockGeometry {
        self.geometry
    }
    fn read(&mut self, request: BlockRequest, destination: &mut [u8]) -> Result<(), BlockError> {
        let range = self.byte_range(request, destination.len())?;
        destination.copy_from_slice(&self.storage[range]);
        Ok(())
    }
    fn write(&mut self, request: BlockRequest, source: &[u8]) -> Result<(), BlockError> {
        let range = self.byte_range(
            BlockRequest {
                write: true,
                ..request
            },
            source.len(),
        )?;
        self.storage[range].copy_from_slice(source);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_past_end() {
        let g = BlockGeometry {
            block_size: 4096,
            block_count: 10,
        };
        assert_eq!(
            validate_request(
                g,
                BlockRequest {
                    lba: 9,
                    blocks: 2,
                    write: false
                }
            ),
            Err(BlockError::OutOfRange)
        );
    }
    #[test]
    fn memory_device_round_trip() {
        let mut d = MemoryBlockDevice::new(BlockGeometry {
            block_size: 512,
            block_count: 4,
        })
        .unwrap();
        let source = vec![0x5a; 512];
        let mut destination = vec![0; 512];
        d.write(
            BlockRequest {
                lba: 2,
                blocks: 1,
                write: true,
            },
            &source,
        )
        .unwrap();
        d.read(
            BlockRequest {
                lba: 2,
                blocks: 1,
                write: false,
            },
            &mut destination,
        )
        .unwrap();
        assert_eq!(source, destination);
    }
    #[test]
    fn buffer_size_is_exact() {
        let mut d = MemoryBlockDevice::new(BlockGeometry {
            block_size: 512,
            block_count: 1,
        })
        .unwrap();
        assert_eq!(
            d.read(
                BlockRequest {
                    lba: 0,
                    blocks: 1,
                    write: false
                },
                &mut [0; 1]
            ),
            Err(BlockError::InvalidBuffer)
        );
    }
}
