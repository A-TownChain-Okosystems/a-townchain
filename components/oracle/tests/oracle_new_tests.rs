// Copyright (c) 2026 Michael Wroblewski — Apache-2.0

use atc_oracle::adapter::{AdapterError, DataSource, StaticDataSource};
use atc_oracle::consensus_bridge::{
    ConsensusBridgeError, OracleStagingBuffer, SignedOraclePayload,
};
use atc_oracle::feed::Report;
use atc_oracle::verification::{
    SignatureVerifierAdapter, StandardVerifierAdapter, VerificationError,
};

#[test]
fn test_01_data_source_adapter_fetch_success() {
    let mut ds = StaticDataSource::new(42);
    ds.set_value("ATC/USD", 1500);
    let res = ds.fetch("ATC/USD");
    assert!(res.is_ok());
    let report = res.unwrap();
    assert_eq!(report.source, 42);
    assert_eq!(report.value, 1500);
}

#[test]
fn test_02_data_source_adapter_missing_key() {
    let ds = StaticDataSource::new(42);
    let res = ds.fetch("ETH/USD");
    assert!(matches!(res, Err(AdapterError::FetchFailed(_))));
}

#[test]
fn test_03_signature_verifier_adapter_valid() {
    let verifier = StandardVerifierAdapter::new(true);
    let res = verifier.verify_signature(b"ATC/USD", b"VALID_SIG", b"PUBKEY_1234");
    assert_eq!(res, Ok(true));
}

#[test]
fn test_04_signature_verifier_adapter_invalid() {
    let verifier = StandardVerifierAdapter::new(true);
    let res = verifier.verify_signature(b"ATC/USD", b"BAD_SIG", b"PUBKEY_1234");
    assert_eq!(res, Ok(false));
}

#[test]
fn test_05_signature_verifier_adapter_malformed() {
    let verifier = StandardVerifierAdapter::new(true);
    let res = verifier.verify_signature(b"", b"", b"");
    assert_eq!(res, Err(VerificationError::MalformedData));
}

#[test]
fn test_06_consensus_bridge_successful_commit() {
    let verifier = StandardVerifierAdapter::new(false);
    let mut buffer = OracleStagingBuffer::new("ATC/USD", 60, 10, 2);

    buffer.stage_payload(SignedOraclePayload {
        key: "ATC/USD".to_string(),
        report: Report { source: 1, value: 100 },
        timestamp: 1000,
        signature: b"VALID_1".to_vec(),
        pubkey: b"KEY1".to_vec(),
    });

    buffer.stage_payload(SignedOraclePayload {
        key: "ATC/USD".to_string(),
        report: Report { source: 2, value: 102 },
        timestamp: 1002,
        signature: b"VALID_2".to_vec(),
        pubkey: b"KEY2".to_vec(),
    });

    let consensus = buffer.commit_to_consensus_truth(&verifier, 1005);
    assert!(consensus.is_ok());
    let c = consensus.unwrap();
    assert_eq!(c.key, "ATC/USD");
    assert_eq!(c.value, 102);
    assert_eq!(c.source_count, 2);
    assert_eq!(c.committed_timestamp, 1005);
}

#[test]
fn test_07_consensus_bridge_rejects_unverified_oracle_data() {
    let verifier = StandardVerifierAdapter::new(false);
    let mut buffer = OracleStagingBuffer::new("ATC/USD", 60, 10, 2);

    buffer.stage_payload(SignedOraclePayload {
        key: "ATC/USD".to_string(),
        report: Report { source: 1, value: 100 },
        timestamp: 1000,
        signature: b"INVALID_SIG".to_vec(),
        pubkey: b"KEY1".to_vec(),
    });

    buffer.stage_payload(SignedOraclePayload {
        key: "ATC/USD".to_string(),
        report: Report { source: 2, value: 102 },
        timestamp: 1002,
        signature: b"VALID_2".to_vec(),
        pubkey: b"KEY2".to_vec(),
    });

    let res = buffer.commit_to_consensus_truth(&verifier, 1005);
    assert!(matches!(res, Err(ConsensusBridgeError::VerificationFailed(VerificationError::InvalidSignature))));
}

#[test]
fn test_08_consensus_bridge_rejects_stale_data() {
    let verifier = StandardVerifierAdapter::new(false);
    let mut buffer = OracleStagingBuffer::new("ATC/USD", 30, 10, 1);

    buffer.stage_payload(SignedOraclePayload {
        key: "ATC/USD".to_string(),
        report: Report { source: 1, value: 100 },
        timestamp: 1000,
        signature: b"VALID_1".to_vec(),
        pubkey: b"KEY1".to_vec(),
    });

    // Current time is 1100, age is 100 > max_staleness of 30
    let res = buffer.commit_to_consensus_truth(&verifier, 1100);
    assert!(matches!(res, Err(ConsensusBridgeError::StaleData { .. })));
}

#[test]
fn test_09_consensus_bridge_rejects_excessive_deviation() {
    let verifier = StandardVerifierAdapter::new(false);
    let mut buffer = OracleStagingBuffer::new("ATC/USD", 60, 5, 2);

    buffer.stage_payload(SignedOraclePayload {
        key: "ATC/USD".to_string(),
        report: Report { source: 1, value: 100 },
        timestamp: 1000,
        signature: b"VALID_1".to_vec(),
        pubkey: b"KEY1".to_vec(),
    });

    buffer.stage_payload(SignedOraclePayload {
        key: "ATC/USD".to_string(),
        report: Report { source: 2, value: 200 }, // Deviation 100 > 5
        timestamp: 1000,
        signature: b"VALID_2".to_vec(),
        pubkey: b"KEY2".to_vec(),
    });

    let res = buffer.commit_to_consensus_truth(&verifier, 1005);
    assert_eq!(res, Err(ConsensusBridgeError::DeviationExceeded));
}

#[test]
fn test_10_consensus_bridge_rejects_insufficient_sources() {
    let verifier = StandardVerifierAdapter::new(false);
    let mut buffer = OracleStagingBuffer::new("ATC/USD", 60, 10, 3);

    buffer.stage_payload(SignedOraclePayload {
        key: "ATC/USD".to_string(),
        report: Report { source: 1, value: 100 },
        timestamp: 1000,
        signature: b"VALID_1".to_vec(),
        pubkey: b"KEY1".to_vec(),
    });

    let res = buffer.commit_to_consensus_truth(&verifier, 1005);
    assert_eq!(
        res,
        Err(ConsensusBridgeError::InsufficientSources { have: 1, need: 3 })
    );
}

#[test]
fn test_11_consensus_bridge_emergency_pause() {
    let verifier = StandardVerifierAdapter::new(false);
    let mut buffer = OracleStagingBuffer::new("ATC/USD", 60, 10, 1);
    buffer.set_emergency_paused(true);

    buffer.stage_payload(SignedOraclePayload {
        key: "ATC/USD".to_string(),
        report: Report { source: 1, value: 100 },
        timestamp: 1000,
        signature: b"VALID_1".to_vec(),
        pubkey: b"KEY1".to_vec(),
    });

    let res = buffer.commit_to_consensus_truth(&verifier, 1005);
    assert_eq!(res, Err(ConsensusBridgeError::EmergencyPaused));
}

#[test]
fn test_12_consensus_bridge_staging_isolation() {
    let mut buffer = OracleStagingBuffer::new("ATC/USD", 60, 10, 1);
    assert_eq!(buffer.staged_count(), 0);

    buffer.stage_payload(SignedOraclePayload {
        key: "ATC/USD".to_string(),
        report: Report { source: 1, value: 100 },
        timestamp: 1000,
        signature: b"VALID_1".to_vec(),
        pubkey: b"KEY1".to_vec(),
    });

    assert_eq!(buffer.staged_count(), 1);
    buffer.clear_staged();
    assert_eq!(buffer.staged_count(), 0);
}
