// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Encryption utilities
pub struct EncryptionUtil;

impl EncryptionUtil {
    pub fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
        data.iter().enumerate()
            .map(|(i, &b)| b ^ key[i % key.len()])
            .collect()
    }

    pub fn xor_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
        Self::xor_encrypt(data, key)
    }

    pub fn generate_nonce(len: usize) -> Vec<u8> {
        (0..len).map(|i| ((i * 37 + 11) % 256) as u8).collect()
    }

    pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() { return false; }
        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        result == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_roundtrip() {
        let data = b"Hello, ATC!";
        let key = b"secret";
        let encrypted = EncryptionUtil::xor_encrypt(data, key);
        let decrypted = EncryptionUtil::xor_decrypt(&encrypted, key);
        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(EncryptionUtil::constant_time_eq(b"abc", b"abc"));
        assert!(!EncryptionUtil::constant_time_eq(b"abc", b"abd"));
    }
}
