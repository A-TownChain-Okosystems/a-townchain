// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Canonical ATC-VM typed integer values.
//!
//! This module establishes the VM type contract independently from the legacy
//! u64 stack representation. Consensus-facing integrations MUST use these
//! canonical encodings rather than platform-dependent integer serialization.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    U256([u64; 4]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueError {
    Overflow,
}

impl UInt {
    /// Canonical unsigned big-endian encoding.
    pub fn to_be_bytes(self) -> Vec<u8> {
        match self {
            Self::U8(v) => vec![v],
            Self::U16(v) => v.to_be_bytes().to_vec(),
            Self::U32(v) => v.to_be_bytes().to_vec(),
            Self::U64(v) => v.to_be_bytes().to_vec(),
            Self::U128(v) => v.to_be_bytes().to_vec(),
            Self::U256(words) => {
                let mut out = Vec::with_capacity(32);
                for word in words {
                    out.extend_from_slice(&word.to_be_bytes());
                }
                out
            }
        }
    }

    pub fn width_bits(self) -> usize {
        match self {
            Self::U8(_) => 8,
            Self::U16(_) => 16,
            Self::U32(_) => 32,
            Self::U64(_) => 64,
            Self::U128(_) => 128,
            Self::U256(_) => 256,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_widths_are_fixed() {
        assert_eq!(UInt::U8(1).to_be_bytes().len(), 1);
        assert_eq!(UInt::U16(1).to_be_bytes().len(), 2);
        assert_eq!(UInt::U32(1).to_be_bytes().len(), 4);
        assert_eq!(UInt::U64(1).to_be_bytes().len(), 8);
        assert_eq!(UInt::U128(1).to_be_bytes().len(), 16);
        assert_eq!(UInt::U256([0, 0, 0, 1]).to_be_bytes().len(), 32);
    }

    #[test]
    fn u128_is_big_endian_and_fixed_width() {
        assert_eq!(
            UInt::U128(1).to_be_bytes(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn u256_is_big_endian_and_fixed_width() {
        assert_eq!(
            UInt::U256([0, 0, 0, 1]).to_be_bytes(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );
    }
}
