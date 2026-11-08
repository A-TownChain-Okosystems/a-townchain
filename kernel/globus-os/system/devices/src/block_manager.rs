//! Deterministic block-device registry and I/O boundary.

use super::block::{BlockDevice, BlockError, BlockGeometry, BlockRequest, validate_request};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockDeviceId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockManagerError {
    Duplicate,
    Missing,
    InvalidGeometry,
    Io(BlockError),
}

pub struct BlockDeviceRegistry<D> {
    devices: Vec<(BlockDeviceId, D)>,
}

impl<D: BlockDevice> Default for BlockDeviceRegistry<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: BlockDevice> BlockDeviceRegistry<D> {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn register(&mut self, id: BlockDeviceId, device: D) -> Result<(), BlockManagerError> {
        if self.devices.iter().any(|(existing, _)| *existing == id) {
            return Err(BlockManagerError::Duplicate);
        }
        if !device.geometry().valid() {
            return Err(BlockManagerError::InvalidGeometry);
        }
        self.devices.push((id, device));
        self.devices.sort_by_key(|(id, _)| *id);
        Ok(())
    }

    pub fn geometry(&self, id: BlockDeviceId) -> Result<BlockGeometry, BlockManagerError> {
        self.devices
            .iter()
            .find(|(existing, _)| *existing == id)
            .map(|(_, device)| device.geometry())
            .ok_or(BlockManagerError::Missing)
    }

    pub fn read(
        &mut self,
        id: BlockDeviceId,
        request: BlockRequest,
        destination: &mut [u8],
    ) -> Result<(), BlockManagerError> {
        let (_, device) = self
            .devices
            .iter_mut()
            .find(|(existing, _)| *existing == id)
            .ok_or(BlockManagerError::Missing)?;
        validate_request(device.geometry(), request).map_err(BlockManagerError::Io)?;
        device
            .read(request, destination)
            .map_err(BlockManagerError::Io)
    }

    pub fn write(
        &mut self,
        id: BlockDeviceId,
        request: BlockRequest,
        source: &[u8],
    ) -> Result<(), BlockManagerError> {
        let (_, device) = self
            .devices
            .iter_mut()
            .find(|(existing, _)| *existing == id)
            .ok_or(BlockManagerError::Missing)?;
        validate_request(device.geometry(), request).map_err(BlockManagerError::Io)?;
        device.write(request, source).map_err(BlockManagerError::Io)
    }

    pub fn ids(&self) -> impl Iterator<Item = BlockDeviceId> + '_ {
        self.devices.iter().map(|(id, _)| *id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::MemoryBlockDevice;

    #[test]
    fn registers_and_reads_deterministically() {
        let mut r = BlockDeviceRegistry::new();
        r.register(
            BlockDeviceId(2),
            MemoryBlockDevice::new(BlockGeometry {
                block_size: 512,
                block_count: 2,
            })
            .unwrap(),
        )
        .unwrap();
        r.register(
            BlockDeviceId(1),
            MemoryBlockDevice::new(BlockGeometry {
                block_size: 512,
                block_count: 2,
            })
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            r.ids().collect::<Vec<_>>(),
            vec![BlockDeviceId(1), BlockDeviceId(2)]
        );
    }
}
