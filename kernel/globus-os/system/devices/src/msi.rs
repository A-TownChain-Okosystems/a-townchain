//! PCI MSI/MSI-X capability and interrupt-vector runtime.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MsiMessage {
    pub address: u64,
    pub data: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MsixTableEntry {
    pub message: MsiMessage,
    pub masked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptMode {
    Legacy,
    Msi { vectors: u16 },
    Msix { vectors: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsiError {
    InvalidVectorCount,
    NoVectors,
    VectorUnavailable,
    InvalidVector,
}

pub fn valid_vector_count(count: u16) -> bool {
    count > 0 && count <= 2048
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorAllocator {
    base: u16,
    allocated: Vec<u16>,
    limit: u16,
}

impl VectorAllocator {
    pub fn new(base: u16, count: u16) -> Result<Self, MsiError> {
        if !valid_vector_count(count) || base.checked_add(count).is_none() {
            return Err(MsiError::InvalidVectorCount);
        }
        Ok(Self {
            base,
            allocated: Vec::new(),
            limit: count,
        })
    }
    pub fn allocate(&mut self) -> Result<u16, MsiError> {
        for offset in 0..self.limit {
            let vector = self.base + offset;
            if !self.allocated.contains(&vector) {
                self.allocated.push(vector);
                self.allocated.sort();
                return Ok(vector);
            }
        }
        Err(MsiError::NoVectors)
    }
    pub fn release(&mut self, vector: u16) -> Result<(), MsiError> {
        let before = self.allocated.len();
        self.allocated.retain(|v| *v != vector);
        if before == self.allocated.len() {
            Err(MsiError::InvalidVector)
        } else {
            Ok(())
        }
    }
    pub fn is_allocated(&self, vector: u16) -> bool {
        self.allocated.contains(&vector)
    }
    pub fn allocated(&self) -> &[u16] {
        &self.allocated
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsixTable {
    entries: Vec<MsixTableEntry>,
}
impl MsixTable {
    pub fn new(count: u16) -> Result<Self, MsiError> {
        if !valid_vector_count(count) {
            return Err(MsiError::InvalidVectorCount);
        }
        Ok(Self {
            entries: vec![
                MsixTableEntry {
                    message: MsiMessage {
                        address: 0,
                        data: 0
                    },
                    masked: true
                };
                count as usize
            ],
        })
    }
    pub fn set(&mut self, index: u16, message: MsiMessage) -> Result<(), MsiError> {
        let e = self
            .entries
            .get_mut(index as usize)
            .ok_or(MsiError::InvalidVector)?;
        e.message = message;
        Ok(())
    }
    pub fn mask(&mut self, index: u16, masked: bool) -> Result<(), MsiError> {
        let e = self
            .entries
            .get_mut(index as usize)
            .ok_or(MsiError::InvalidVector)?;
        e.masked = masked;
        Ok(())
    }
    pub fn get(&self, index: u16) -> Result<MsixTableEntry, MsiError> {
        self.entries
            .get(index as usize)
            .copied()
            .ok_or(MsiError::InvalidVector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allocator_is_deterministic() {
        let mut a = VectorAllocator::new(32, 2).unwrap();
        assert_eq!(a.allocate().unwrap(), 32);
        assert_eq!(a.allocate().unwrap(), 33);
        assert_eq!(a.allocate(), Err(MsiError::NoVectors));
        a.release(32).unwrap();
        assert_eq!(a.allocate().unwrap(), 32);
    }
    #[test]
    fn msix_starts_masked() {
        let t = MsixTable::new(2).unwrap();
        assert!(t.get(0).unwrap().masked);
    }
}
