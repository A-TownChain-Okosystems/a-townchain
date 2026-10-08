// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Deterministic mining primitives & device classification.

use sha3::{Digest, Sha3_256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MiningJob {
    pub job_id: u64,
    pub header: Vec<u8>,
    pub target: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MiningResult {
    pub job_id: u64,
    pub nonce: u64,
    pub digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiningError {
    EmptyHeader,
    NoSolution,
    InvalidTarget,
    DoubleClaim,
    JobNotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinerDeviceClass {
    Cpu,
    Gpu,
    Mobile,
    Provisional,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceConfig {
    pub class: MinerDeviceClass,
    pub thread_count: usize,
}

pub fn classify_device(hardware_info: &str) -> MinerDeviceClass {
    let lower = hardware_info.to_lowercase();
    if lower.contains("nvidia") || lower.contains("cuda") || lower.contains("radeon") || lower.contains("gpu") {
        MinerDeviceClass::Gpu
    } else if lower.contains("arm") || lower.contains("android") || lower.contains("ios") || lower.contains("mobile") {
        MinerDeviceClass::Mobile
    } else if lower.contains("x86") || lower.contains("intel") || lower.contains("amd") || lower.contains("cpu") {
        MinerDeviceClass::Cpu
    } else {
        MinerDeviceClass::Provisional
    }
}

/// Computes the canonical SHA3-256 digest of `header || nonce_le`.
pub fn hash_candidate(header: &[u8], nonce: u64) -> Result<[u8; 32], MiningError> {
    if header.is_empty() {
        return Err(MiningError::EmptyHeader);
    }
    let mut hasher = Sha3_256::new();
    hasher.update(header);
    hasher.update(nonce.to_le_bytes());
    Ok(hasher.finalize().into())
}

/// Returns true when the digest is numerically <= the supplied big-endian target.
pub fn meets_target(digest: &[u8; 32], target: &[u8; 32]) -> bool {
    digest <= target
}

/// Searches a bounded nonce range. No consensus state is mutated here.
pub fn mine(job: &MiningJob, start_nonce: u64, attempts: u64) -> Result<MiningResult, MiningError> {
    if job.header.is_empty() {
        return Err(MiningError::EmptyHeader);
    }
    if job.target == [0u8; 32] {
        return Err(MiningError::InvalidTarget);
    }

    for offset in 0..attempts {
        let nonce = match start_nonce.checked_add(offset) {
            Some(value) => value,
            None => break,
        };
        let digest = hash_candidate(&job.header, nonce)?;
        if meets_target(&digest, &job.target) {
            return Ok(MiningResult { job_id: job.job_id, nonce, digest });
        }
    }

    Err(MiningError::NoSolution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_deterministic() {
        assert_eq!(hash_candidate(b"header", 7), hash_candidate(b"header", 7));
        assert_ne!(hash_candidate(b"header", 7), hash_candidate(b"header", 8));
    }

    #[test]
    fn target_comparison_is_big_endian() {
        let low = [0x01u8; 32];
        let high = [0xffu8; 32];
        assert!(meets_target(&low, &high));
        assert!(!meets_target(&high, &low));
    }

    #[test]
    fn bounded_mining_finds_easy_target() {
        let job = MiningJob { job_id: 42, header: b"header".to_vec(), target: [0xffu8; 32] };
        let result = mine(&job, 0, 1).unwrap();
        assert_eq!(result.job_id, 42);
        assert_eq!(result.nonce, 0);
    }

    // --- NEW TESTS (atc-mining) ---

    #[test]
    fn test_device_classification() {
        assert_eq!(classify_device("NVIDIA RTX 4090"), MinerDeviceClass::Gpu);
        assert_eq!(classify_device("Apple ARM M2 Mobile"), MinerDeviceClass::Mobile);
        assert_eq!(classify_device("Intel Core i9 x86 CPU"), MinerDeviceClass::Cpu);
        assert_eq!(classify_device("Unknown Device"), MinerDeviceClass::Provisional);
    }

    #[test]
    fn test_mine_empty_header_error() {
        let job = MiningJob { job_id: 1, header: vec![], target: [0xffu8; 32] };
        assert_eq!(mine(&job, 0, 10), Err(MiningError::EmptyHeader));
    }

    #[test]
    fn test_mine_zero_target_error() {
        let job = MiningJob { job_id: 1, header: b"test".to_vec(), target: [0u8; 32] };
        assert_eq!(mine(&job, 0, 10), Err(MiningError::InvalidTarget));
    }
}
