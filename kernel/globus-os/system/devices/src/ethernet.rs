//! Ethernet NIC DMA rings with explicit ownership transitions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RingError {
    Empty,
    Full,
    TooLarge,
    InvalidBuffer,
    DeviceOwnsBuffer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmaBuffer {
    pub address: u64,
    pub length: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TxDescriptor {
    pub buffer: DmaBuffer,
    pub length: u16,
    pub owned_by_device: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RxDescriptor {
    pub buffer: DmaBuffer,
    pub capacity: u16,
    pub owned_by_device: bool,
}

pub const MAX_FRAME_SIZE: u16 = 1518;

pub fn validate_tx(desc: TxDescriptor) -> Result<(), RingError> {
    if desc.buffer.length == 0
        || desc.length == 0
        || desc.length as u32 > desc.buffer.length
        || desc.length > MAX_FRAME_SIZE
    {
        return Err(RingError::InvalidBuffer);
    }
    if desc.owned_by_device {
        return Err(RingError::DeviceOwnsBuffer);
    }
    Ok(())
}
pub fn validate_rx(desc: RxDescriptor) -> Result<(), RingError> {
    if desc.buffer.length == 0
        || desc.capacity == 0
        || desc.capacity as u32 > desc.buffer.length
        || desc.capacity > MAX_FRAME_SIZE
    {
        return Err(RingError::InvalidBuffer);
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxRing {
    entries: Vec<Option<TxDescriptor>>,
    producer: usize,
    consumer: usize,
    count: usize,
}
impl TxRing {
    pub fn new(depth: usize) -> Result<Self, RingError> {
        if depth == 0 {
            Err(RingError::InvalidBuffer)
        } else {
            Ok(Self {
                entries: vec![None; depth],
                producer: 0,
                consumer: 0,
                count: 0,
            })
        }
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn capacity(&self) -> usize {
        self.entries.len()
    }
    pub fn submit(&mut self, mut d: TxDescriptor) -> Result<(), RingError> {
        validate_tx(d)?;
        if self.count == self.entries.len() {
            return Err(RingError::Full);
        }
        d.owned_by_device = true;
        self.entries[self.producer] = Some(d);
        self.producer = (self.producer + 1) % self.entries.len();
        self.count += 1;
        Ok(())
    }
    pub fn device_complete(&mut self) -> Result<TxDescriptor, RingError> {
        if self.count == 0 {
            return Err(RingError::Empty);
        }
        let mut d = self.entries[self.consumer].take().ok_or(RingError::Empty)?;
        d.owned_by_device = false;
        self.consumer = (self.consumer + 1) % self.entries.len();
        self.count -= 1;
        Ok(d)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RxRing {
    entries: Vec<Option<RxDescriptor>>,
    producer: usize,
    consumer: usize,
    count: usize,
}
impl RxRing {
    pub fn new(depth: usize) -> Result<Self, RingError> {
        if depth == 0 {
            Err(RingError::InvalidBuffer)
        } else {
            Ok(Self {
                entries: vec![None; depth],
                producer: 0,
                consumer: 0,
                count: 0,
            })
        }
    }
    pub fn available(&self) -> usize {
        self.count
    }
    pub fn post(&mut self, mut d: RxDescriptor) -> Result<(), RingError> {
        validate_rx(d)?;
        if self.count == self.entries.len() {
            return Err(RingError::Full);
        }
        d.owned_by_device = true;
        self.entries[self.producer] = Some(d);
        self.producer = (self.producer + 1) % self.entries.len();
        self.count += 1;
        Ok(())
    }
    pub fn device_receive(&mut self, length: u16) -> Result<RxDescriptor, RingError> {
        if self.count == 0 {
            return Err(RingError::Empty);
        }
        let mut d = self.entries[self.consumer].take().ok_or(RingError::Empty)?;
        if length == 0 || length > d.capacity {
            return Err(RingError::TooLarge);
        }
        d.capacity = length;
        d.owned_by_device = false;
        self.consumer = (self.consumer + 1) % self.entries.len();
        self.count -= 1;
        Ok(d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn b() -> DmaBuffer {
        DmaBuffer {
            address: 0x1000,
            length: 1500,
        }
    }
    #[test]
    fn tx_ownership_roundtrip() {
        let mut r = TxRing::new(2).unwrap();
        r.submit(TxDescriptor {
            buffer: b(),
            length: 1500,
            owned_by_device: false,
        })
        .unwrap();
        assert_eq!(r.len(), 1);
        assert!(!r.device_complete().unwrap().owned_by_device);
        assert_eq!(r.len(), 0);
    }
    #[test]
    fn rx_ring_full_is_rejected() {
        let mut r = RxRing::new(1).unwrap();
        r.post(RxDescriptor {
            buffer: b(),
            capacity: 1500,
            owned_by_device: false,
        })
        .unwrap();
        assert_eq!(
            r.post(RxDescriptor {
                buffer: b(),
                capacity: 1500,
                owned_by_device: false
            }),
            Err(RingError::Full)
        );
    }
}
