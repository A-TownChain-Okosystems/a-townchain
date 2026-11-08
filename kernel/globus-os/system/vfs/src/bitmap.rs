//! Deterministic free-space bitmap for persistent filesystem allocation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitmapError {
    InvalidGeometry,
    InvalidBlock,
    AlreadyFree,
    AlreadyAllocated,
    NoSpace,
    Buffer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeSpaceBitmap {
    total_blocks: u64,
    bits: Vec<u8>,
}

impl FreeSpaceBitmap {
    pub fn new(total_blocks: u64) -> Result<Self, BitmapError> {
        if total_blocks == 0 {
            return Err(BitmapError::InvalidGeometry);
        }
        let bytes = total_blocks
            .checked_add(7)
            .ok_or(BitmapError::InvalidGeometry)?
            / 8;
        let bytes = usize::try_from(bytes).map_err(|_| BitmapError::InvalidGeometry)?;
        let mut bits = vec![0xff; bytes];
        // Kanonische Form: ungültige Tail-Bits (jenseits total_blocks) sind
        // immer 0 — identisch zur Maskierung in `decode`. Ohne diese
        // Normalisierung wäre ein encode/decode-Roundtrip nicht gleich.
        if !total_blocks.is_multiple_of(8) {
            let valid = (total_blocks % 8) as u8;
            let last = bytes - 1;
            bits[last] &= (1u8 << valid) - 1;
        }
        Ok(Self { total_blocks, bits })
    }

    pub fn len(&self) -> u64 {
        self.total_blocks
    }

    /// Counterpart to `len`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn total_blocks(&self) -> u64 {
        self.total_blocks
    }

    fn check(&self, block: u64) -> Result<(usize, u8), BitmapError> {
        if block >= self.total_blocks {
            return Err(BitmapError::InvalidBlock);
        }
        let index = usize::try_from(block / 8).map_err(|_| BitmapError::InvalidBlock)?;
        Ok((index, 1u8 << (block % 8)))
    }

    pub(crate) fn set_free(&mut self, block: u64, free: bool) -> Result<(), BitmapError> {
        let (index, mask) = self.check(block)?;
        let allocated = self.bits[index] & mask != 0;
        if free {
            if allocated {
                return Err(BitmapError::AlreadyFree);
            }
            self.bits[index] |= mask;
        } else {
            if !allocated {
                return Err(BitmapError::AlreadyAllocated);
            }
            self.bits[index] &= !mask;
        }
        Ok(())
    }

    pub fn reserve(&mut self, start: u64, count: u64) -> Result<(), BitmapError> {
        let end = start.checked_add(count).ok_or(BitmapError::InvalidBlock)?;
        if count == 0 || end > self.total_blocks {
            return Err(BitmapError::InvalidBlock);
        }
        for block in start..end {
            self.set_free(block, false)?;
        }
        Ok(())
    }

    pub fn release(&mut self, start: u64, count: u64) -> Result<(), BitmapError> {
        let end = start.checked_add(count).ok_or(BitmapError::InvalidBlock)?;
        if count == 0 || end > self.total_blocks {
            return Err(BitmapError::InvalidBlock);
        }
        for block in start..end {
            self.set_free(block, true)?;
        }
        Ok(())
    }

    pub fn is_free(&self, block: u64) -> Result<bool, BitmapError> {
        let (index, mask) = self.check(block)?;
        Ok(self.bits[index] & mask != 0)
    }

    pub fn allocate(&mut self, count: u64) -> Result<u64, BitmapError> {
        if count == 0 {
            return Err(BitmapError::InvalidBlock);
        }
        let mut run_start = 0;
        let mut run = 0;
        for block in 0..self.total_blocks {
            if self.is_free(block)? {
                if run == 0 {
                    run_start = block;
                }
                run += 1;
                if run == count {
                    self.reserve(run_start, count)?;
                    return Ok(run_start);
                }
            } else {
                run = 0;
            }
        }
        Err(BitmapError::NoSpace)
    }

    pub fn encoded_len(&self) -> usize {
        self.bits.len()
    }

    pub fn encode(&self, out: &mut [u8]) -> Result<(), BitmapError> {
        if out.len() < self.bits.len() {
            return Err(BitmapError::Buffer);
        }
        out[..self.bits.len()].copy_from_slice(&self.bits);
        Ok(())
    }

    pub fn decode(total_blocks: u64, input: &[u8]) -> Result<Self, BitmapError> {
        let mut bitmap = Self::new(total_blocks)?;
        if input.len() < bitmap.bits.len() {
            return Err(BitmapError::Buffer);
        }
        let len = bitmap.bits.len();
        bitmap.bits.copy_from_slice(&input[..len]);
        if !total_blocks.is_multiple_of(8) {
            let valid = (total_blocks % 8) as u8;
            let last = bitmap.bits.len() - 1;
            bitmap.bits[last] &= (1u8 << valid) - 1;
        }
        Ok(bitmap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserves_and_allocates_deterministically() {
        let mut b = FreeSpaceBitmap::new(32).unwrap();
        b.reserve(0, 4).unwrap();
        assert_eq!(b.allocate(3).unwrap(), 4);
        assert!(!b.is_free(4).unwrap());
    }

    #[test]
    fn rejects_double_reservation() {
        let mut b = FreeSpaceBitmap::new(8).unwrap();
        b.reserve(2, 1).unwrap();
        assert_eq!(b.reserve(2, 1), Err(BitmapError::AlreadyAllocated));
    }

    #[test]
    fn round_trips() {
        let mut b = FreeSpaceBitmap::new(17).unwrap();
        b.reserve(0, 3).unwrap();
        b.reserve(10, 2).unwrap();
        let mut bytes = vec![0; b.encoded_len()];
        b.encode(&mut bytes).unwrap();
        let restored = FreeSpaceBitmap::decode(17, &bytes).unwrap();
        assert_eq!(restored, b);
    }
}
