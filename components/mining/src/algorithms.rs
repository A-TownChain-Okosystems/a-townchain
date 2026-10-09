// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Hashing algorithms module for atc-mining.

use crate::miner::{hash_candidate, MiningError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorithmKind {
    Sha3Atc,
    ProvisionalPoW,
}

pub fn compute_hash(
    kind: AlgorithmKind,
    header: &[u8],
    nonce: u64,
) -> Result<[u8; 32], MiningError> {
    match kind {
        AlgorithmKind::Sha3Atc | AlgorithmKind::ProvisionalPoW => {
            hash_candidate(header, nonce)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_hash_computation() {
        let h1 = compute_hash(AlgorithmKind::Sha3Atc, b"header", 123).unwrap();
        let h2 = compute_hash(AlgorithmKind::ProvisionalPoW, b"header", 123).unwrap();
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_algorithm_empty_header() {
        assert_eq!(
            compute_hash(AlgorithmKind::Sha3Atc, b"", 123),
            Err(MiningError::EmptyHeader)
        );
    }
}
