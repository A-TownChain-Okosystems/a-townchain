// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Attestation Handles and Verification.

extern crate alloc;
use alloc::string::String;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AttestationId(pub u64);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttestationRecord {
    pub id: AttestationId,
    pub measurement: [u8; 32],
    pub timestamp: u64,
    pub verified: bool,
}

pub struct AttestationVerifier;

impl AttestationVerifier {
    pub fn verify(record: &mut AttestationRecord, expected_measurement: &[u8; 32]) -> Result<bool, String> {
        if record.measurement == *expected_measurement {
            record.verified = true;
            Ok(true)
        } else {
            record.verified = false;
            Err(String::from("Attestation measurement mismatch"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attestation_verification() {
        let expected = [0xABu8; 32];
        let mut record = AttestationRecord {
            id: AttestationId(1),
            measurement: expected,
            timestamp: 1000,
            verified: false,
        };

        assert!(AttestationVerifier::verify(&mut record, &expected).unwrap());
        assert!(record.verified);

        let wrong = [0x00u8; 32];
        assert!(AttestationVerifier::verify(&mut record, &wrong).is_err());
        assert!(!record.verified);
    }
}
