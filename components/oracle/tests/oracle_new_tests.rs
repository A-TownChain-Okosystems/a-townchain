// Copyright (c) 2026 Michael Wroblewski — Apache-2.0

use atc_oracle::adapter::{AdapterError, DataSource, StaticDataSource};
use atc_oracle::consensus_bridge::{
    ConsensusBridgeError, OracleStagingBuffer, SignedOraclePayload,
};
use atc_oracle::feed::Report;
use atc_oracle::verification::{
    Ed25519VerifierAdapter, Secp256k1VerifierAdapter, SignatureVerifierAdapter, VerificationError,
};

/// Test-Double NUR fuer Konsens-Bridge-Tests. Es prueft bewusst keine Krypto;
/// Produktionscode darf diesen Typ nicht verwenden (siehe SEC-P0-002).
struct TestVerifier {
    require_matching_key: bool,
}

impl TestVerifier {
    fn new(require_matching_key: bool) -> Self {
        Self { require_matching_key }
    }
}

impl SignatureVerifierAdapter for TestVerifier {
    fn verify_signature(&self, message: &[u8], signature: &[u8], pubkey: &[u8]) -> Result<bool, VerificationError> {
        if message.is_empty() || signature.is_empty() || pubkey.is_empty() {
            return Err(VerificationError::MalformedData);
        }
        if self.require_matching_key && pubkey.len() < 4 {
            return Err(VerificationError::PublicKeyMismatch);
        }
        Ok(signature.starts_with(b"VALID"))
    }
}

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
fn test_03_ed25519_verifier_accepts_real_signature() {
    use ed25519_dalek::{Signer, SigningKey};
    let sk = SigningKey::from_bytes(&[7u8; 32]);
    let pk = sk.verifying_key().to_bytes();
    let msg = b"ATC/USD:1500";
    let sig = sk.sign(msg);
    let verifier = Ed25519VerifierAdapter;
    assert_eq!(verifier.verify_signature(msg, &sig.to_bytes(), &pk), Ok(true));
    // Manipulierte Nachricht muss scheitern
    assert_eq!(verifier.verify_signature(b"ATC/USD:9999", &sig.to_bytes(), &pk), Ok(false));
    // Fremder Schluessel muss scheitern
    let other = SigningKey::from_bytes(&[8u8; 32]).verifying_key().to_bytes();
    assert_eq!(verifier.verify_signature(msg, &sig.to_bytes(), &other), Ok(false));
}

#[test]
fn test_04_secp256k1_verifier_accepts_real_signature() {
    use secp256k1::{Message, Secp256k1, SecretKey};
    use sha2::{Digest, Sha256};
    let secp = Secp256k1::new();
    let sk = SecretKey::from_byte_array([7u8; 32]).unwrap();
    let pk = secp256k1::PublicKey::from_secret_key(&secp, &sk).serialize();
    let msg = b"ATC-TX-DOMAIN-V2|transfer|1";
    let digest: [u8; 32] = Sha256::digest(msg).into();
    let sig = secp.sign_ecdsa(Message::from_digest(digest), &sk).serialize_compact();
    let verifier = Secp256k1VerifierAdapter;
    assert_eq!(verifier.verify_signature(msg, &sig, &pk), Ok(true));
    assert_eq!(verifier.verify_signature(b"ATC-TX-DOMAIN-V2|transfer|2", &sig, &pk), Ok(false));
}

#[test]
fn test_05_verifier_rejects_malformed_input() {
    let verifier = Secp256k1VerifierAdapter;
    assert_eq!(verifier.verify_signature(b"", b"", b""), Err(VerificationError::MalformedData));
    // Falsche Laengen duerfen nie als gueltig durchgehen
    let ed = Ed25519VerifierAdapter;
    assert_eq!(ed.verify_signature(b"x", &[0u8; 12], &[0u8; 32]), Err(VerificationError::MalformedData));
    assert_eq!(ed.verify_signature(b"x", &[0u8; 64], &[0u8; 3]), Err(VerificationError::PublicKeyMismatch));
}

#[test]
fn test_06_consensus_bridge_successful_commit() {
    let verifier = TestVerifier::new(false);
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
    let verifier = TestVerifier::new(false);
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
    let verifier = TestVerifier::new(false);
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
    let verifier = TestVerifier::new(false);
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
    let verifier = TestVerifier::new(false);
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
    let verifier = TestVerifier::new(false);
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
