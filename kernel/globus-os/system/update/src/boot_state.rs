//! Persistent, checksummed A/B boot-selection state.

use crate::Slot;

const MAGIC: &[u8; 8] = b"GBOOT001";
const SIZE: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootStateError {
    Buffer,
    Invalid,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootState {
    pub slot: Slot,
    pub attempts: u32,
    pub confirmed: bool,
    pub generation: u64,
}

fn checksum(bytes: &[u8]) -> u32 {
    let mut hash = 0x811c9dc5u32;
    for byte in bytes {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x01000193);
    }
    hash
}

impl BootState {
    pub fn encode(&self, out: &mut [u8]) -> Result<(), BootStateError> {
        if out.len() < SIZE {
            return Err(BootStateError::Buffer);
        }
        out[..SIZE].fill(0);
        out[..8].copy_from_slice(MAGIC);
        out[8] = match self.slot {
            Slot::A => 0,
            Slot::B => 1,
        };
        out[9] = u8::from(self.confirmed);
        out[12..16].copy_from_slice(&self.attempts.to_le_bytes());
        out[16..24].copy_from_slice(&self.generation.to_le_bytes());
        let digest = checksum(&out[..24]);
        out[24..28].copy_from_slice(&digest.to_le_bytes());
        Ok(())
    }

    pub fn decode(input: &[u8]) -> Result<Self, BootStateError> {
        if input.len() < SIZE || &input[..8] != MAGIC {
            return Err(BootStateError::Invalid);
        }
        let expected = u32::from_le_bytes(
            input[24..28]
                .try_into()
                .map_err(|_| BootStateError::Buffer)?,
        );
        if checksum(&input[..24]) != expected
            || input[10] != 0
            || input[11] != 0
            || input[28..].iter().any(|b| *b != 0)
        {
            return Err(BootStateError::Invalid);
        }
        let slot = match input[8] {
            0 => Slot::A,
            1 => Slot::B,
            _ => return Err(BootStateError::Invalid),
        };
        let attempts = u32::from_le_bytes(
            input[12..16]
                .try_into()
                .map_err(|_| BootStateError::Buffer)?,
        );
        let generation = u64::from_le_bytes(
            input[16..24]
                .try_into()
                .map_err(|_| BootStateError::Buffer)?,
        );
        Ok(Self {
            slot,
            attempts,
            confirmed: input[9] != 0,
            generation,
        })
    }

    pub fn record_attempt(&mut self) -> Result<(), BootStateError> {
        self.attempts = self
            .attempts
            .checked_add(1)
            .ok_or(BootStateError::Overflow)?;
        self.confirmed = false;
        Ok(())
    }

    pub fn confirm(&mut self) {
        self.confirmed = true;
    }

    pub fn switch_slot(&mut self) -> Result<(), BootStateError> {
        self.slot = match self.slot {
            Slot::A => Slot::B,
            Slot::B => Slot::A,
        };
        self.attempts = 0;
        self.confirmed = false;
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or(BootStateError::Overflow)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let state = BootState {
            slot: Slot::B,
            attempts: 2,
            confirmed: true,
            generation: 7,
        };
        let mut bytes = [0u8; SIZE];
        state.encode(&mut bytes).unwrap();
        assert_eq!(BootState::decode(&bytes).unwrap(), state);
    }

    #[test]
    fn corruption_is_rejected() {
        let state = BootState {
            slot: Slot::A,
            attempts: 0,
            confirmed: false,
            generation: 1,
        };
        let mut bytes = [0u8; SIZE];
        state.encode(&mut bytes).unwrap();
        bytes[16] ^= 1;
        assert_eq!(BootState::decode(&bytes), Err(BootStateError::Invalid));
    }

    #[test]
    fn switching_resets_attempts() {
        let mut state = BootState {
            slot: Slot::A,
            attempts: 3,
            confirmed: true,
            generation: 1,
        };
        state.switch_slot().unwrap();
        assert_eq!(state.slot, Slot::B);
        assert_eq!(state.attempts, 0);
        assert!(!state.confirmed);
        assert_eq!(state.generation, 2);
    }
}
