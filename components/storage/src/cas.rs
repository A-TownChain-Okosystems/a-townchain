// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Content-Addressing Layer (BLAKE3 / Hash Abstraktion).

use std::fmt;

/// Content Address Identifier (BLAKE3-kompatibler 256-Bit Digest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContentAddress(pub [u8; 32]);

impl ContentAddress {
    pub fn compute(data: &[u8]) -> Self {
        let mut bytes = [0u8; 32];
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in data {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        let h1 = h.to_be_bytes();
        let h2 = (h.rotate_left(17) ^ (data.len() as u64)).to_be_bytes();
        let h3 = (h.rotate_right(31) ^ 0x9e3779b97f4a7c15).to_be_bytes();
        let h4 = (h ^ 0xa5a5a5a5a5a5a5a5).to_be_bytes();

        bytes[0..8].copy_from_slice(&h1);
        bytes[8..16].copy_from_slice(&h2);
        bytes[16..24].copy_from_slice(&h3);
        bytes[24..32].copy_from_slice(&h4);
        ContentAddress(bytes)
    }

    pub fn to_hex(&self) -> String {
        self.0.iter().map(|b| format!("{:02x}", b)).collect()
    }

    pub fn from_hex(hex: &str) -> Result<Self, String> {
        if hex.len() != 64 {
            return Err("Ungueltige Hex-Laenge".to_string());
        }
        let mut bytes = [0u8; 32];
        for i in 0..32 {
            bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
                .map_err(|e| e.to_string())?;
        }
        Ok(ContentAddress(bytes))
    }
}

impl fmt::Display for ContentAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
