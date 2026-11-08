//! Append-only, checksummed identity transaction journal primitives.
//!
//! This module is a fail-closed journal codec and state-machine layer. It deliberately does not
//! claim that account records and the username directory are atomic until production callers
//! implement a durable `JournalBackend`, serialize writers, and replay/reconcile before exposing
//! accounts. Payloads are opaque metadata; never put credentials, recovery phrases, keys, or other
//! secrets in journal payloads.
//!
//! Record format (big-endian): magic "ATCJ", version u16, state u8, reserved u8 (=0),
//! transaction ID [u8;16], payload length u32, payload, SHA-256(header || payload).
//! The checksum detects accidental corruption; it is not an authenticity mechanism.

use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const MAGIC: &[u8; 4] = b"ATCJ";
const VERSION: u16 = 1;
const HEADER_LEN: usize = 28;
const CHECKSUM_LEN: usize = 32;
const MAX_PAYLOAD_LEN: usize = 64 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TransactionState {
    Prepared = 1,
    Committed = 2,
    Aborted = 3,
}

impl TryFrom<u8> for TransactionState {
    type Error = JournalError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Prepared),
            2 => Ok(Self::Committed),
            3 => Ok(Self::Aborted),
            _ => Err(JournalError::InvalidState),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalRecord {
    pub transaction_id: [u8; 16],
    pub state: TransactionState,
    /// Non-secret operation metadata used by the caller's reconciliation protocol.
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JournalError {
    TruncatedRecord,
    InvalidMagic,
    UnsupportedVersion,
    InvalidState,
    InvalidReservedByte,
    PayloadTooLarge,
    ChecksumMismatch,
    DuplicatePrepare,
    TransitionWithoutPrepare,
    TerminalTransaction,
    ConflictingPayload,
    Backend(String),
}

/// Implementations must make append + flush/sync durable before returning success.
/// Callers must serialize mutations and must not expose accounts until replay/reconciliation ends.
pub trait JournalBackend {
    fn read_all(&self) -> Result<Vec<u8>, JournalError>;
    fn append_sync(&mut self, bytes: &[u8]) -> Result<(), JournalError>;
}

fn encode(record: &JournalRecord) -> Result<Vec<u8>, JournalError> {
    if record.payload.len() > MAX_PAYLOAD_LEN {
        return Err(JournalError::PayloadTooLarge);
    }

    let mut out = Vec::with_capacity(HEADER_LEN + record.payload.len() + CHECKSUM_LEN);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_be_bytes());
    out.push(record.state as u8);
    out.push(0);
    out.extend_from_slice(&record.transaction_id);
    out.extend_from_slice(&(record.payload.len() as u32).to_be_bytes());
    out.extend_from_slice(&record.payload);
    let digest = Sha256::digest(&out);
    out.extend_from_slice(&digest);
    Ok(out)
}

/// Decode every complete record. Any partial tail or invalid record blocks recovery; no
/// automatic truncation is attempted because it could discard a committed transaction.
pub fn decode_all(bytes: &[u8]) -> Result<Vec<JournalRecord>, JournalError> {
    let mut records = Vec::new();
    let mut offset = 0usize;

    while offset < bytes.len() {
        if bytes.len() - offset < HEADER_LEN {
            return Err(JournalError::TruncatedRecord);
        }
        let header = &bytes[offset..offset + HEADER_LEN];
        if &header[..4] != MAGIC {
            return Err(JournalError::InvalidMagic);
        }
        let version = u16::from_be_bytes([header[4], header[5]]);
        if version != VERSION {
            return Err(JournalError::UnsupportedVersion);
        }
        let state = TransactionState::try_from(header[6])?;
        if header[7] != 0 {
            return Err(JournalError::InvalidReservedByte);
        }
        let transaction_id: [u8; 16] = header[8..24]
            .try_into()
            .map_err(|_| JournalError::TruncatedRecord)?;
        let payload_len = u32::from_be_bytes(
            header[24..28]
                .try_into()
                .map_err(|_| JournalError::TruncatedRecord)?,
        ) as usize;
        if payload_len > MAX_PAYLOAD_LEN {
            return Err(JournalError::PayloadTooLarge);
        }
        let record_len = HEADER_LEN + payload_len + CHECKSUM_LEN;
        if bytes.len() - offset < record_len {
            return Err(JournalError::TruncatedRecord);
        }
        let payload_start = offset + HEADER_LEN;
        let payload_end = payload_start + payload_len;
        let checksum_end = offset + record_len;
        let expected = Sha256::digest(&bytes[offset..payload_end]);
        if expected.as_slice() != &bytes[payload_end..checksum_end] {
            return Err(JournalError::ChecksumMismatch);
        }
        records.push(JournalRecord {
            transaction_id,
            state,
            payload: bytes[payload_start..payload_end].to_vec(),
        });
        offset = checksum_end;
    }
    Ok(records)
}

/// Validate the transaction state machine and return each transaction's last durable state.
pub fn replay(bytes: &[u8]) -> Result<BTreeMap<[u8; 16], JournalRecord>, JournalError> {
    let records = decode_all(bytes)?;
    let mut state = BTreeMap::<[u8; 16], JournalRecord>::new();

    for record in records {
        match state.get(&record.transaction_id) {
            None if record.state == TransactionState::Prepared => {
                state.insert(record.transaction_id, record);
            }
            None => return Err(JournalError::TransitionWithoutPrepare),
            Some(previous) if previous.state != TransactionState::Prepared => {
                return Err(JournalError::TerminalTransaction);
            }
            Some(previous) if previous.payload != record.payload => {
                return Err(JournalError::ConflictingPayload);
            }
            Some(_) if record.state == TransactionState::Prepared => {
                return Err(JournalError::DuplicatePrepare);
            }
            Some(_) => {
                state.insert(record.transaction_id, record);
            }
        }
    }
    Ok(state)
}

/// Append a legal state transition. The backend must serialize callers externally and guarantee
/// durable append semantics. Failed writes are surfaced; callers must replay before retrying.
pub fn append_transition<B: JournalBackend>(
    backend: &mut B,
    transaction_id: [u8; 16],
    state: TransactionState,
    payload: Vec<u8>,
) -> Result<(), JournalError> {
    let current = replay(&backend.read_all()?)?;
    match current.get(&transaction_id) {
        None if state != TransactionState::Prepared => {
            return Err(JournalError::TransitionWithoutPrepare);
        }
        None => {}
        Some(previous) if previous.state != TransactionState::Prepared => {
            return Err(JournalError::TerminalTransaction);
        }
        Some(previous) if previous.payload != payload => {
            return Err(JournalError::ConflictingPayload);
        }
        Some(_) if state == TransactionState::Prepared => {
            return Err(JournalError::DuplicatePrepare);
        }
        Some(_) => {}
    }
    let record = JournalRecord {
        transaction_id,
        state,
        payload,
    };
    backend.append_sync(&encode(&record)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MemoryJournal(Vec<u8>);

    impl JournalBackend for MemoryJournal {
        fn read_all(&self) -> Result<Vec<u8>, JournalError> {
            Ok(self.0.clone())
        }

        fn append_sync(&mut self, bytes: &[u8]) -> Result<(), JournalError> {
            self.0.extend_from_slice(bytes);
            Ok(())
        }
    }

    fn tx() -> [u8; 16] {
        [7; 16]
    }

    #[test]
    fn prepared_then_commit_replays_terminal_state() {
        let mut journal = MemoryJournal::default();
        append_transition(
            &mut journal,
            tx(),
            TransactionState::Prepared,
            b"account-a".to_vec(),
        )
        .unwrap();
        append_transition(
            &mut journal,
            tx(),
            TransactionState::Committed,
            b"account-a".to_vec(),
        )
        .unwrap();
        assert_eq!(
            replay(&journal.0).unwrap()[&tx()].state,
            TransactionState::Committed
        );
    }

    #[test]
    fn prepared_can_be_aborted() {
        let mut journal = MemoryJournal::default();
        append_transition(
            &mut journal,
            tx(),
            TransactionState::Prepared,
            b"account-a".to_vec(),
        )
        .unwrap();
        append_transition(
            &mut journal,
            tx(),
            TransactionState::Aborted,
            b"account-a".to_vec(),
        )
        .unwrap();
        assert_eq!(
            replay(&journal.0).unwrap()[&tx()].state,
            TransactionState::Aborted
        );
    }

    #[test]
    fn terminal_transactions_cannot_be_reopened() {
        let mut journal = MemoryJournal::default();
        append_transition(
            &mut journal,
            tx(),
            TransactionState::Prepared,
            b"x".to_vec(),
        )
        .unwrap();
        append_transition(
            &mut journal,
            tx(),
            TransactionState::Committed,
            b"x".to_vec(),
        )
        .unwrap();
        assert_eq!(
            append_transition(
                &mut journal,
                tx(),
                TransactionState::Prepared,
                b"x".to_vec()
            ),
            Err(JournalError::TerminalTransaction)
        );
    }

    #[test]
    fn checksum_corruption_fails_closed() {
        let mut journal = MemoryJournal::default();
        append_transition(
            &mut journal,
            tx(),
            TransactionState::Prepared,
            b"x".to_vec(),
        )
        .unwrap();
        journal.0[HEADER_LEN] ^= 1;
        assert_eq!(replay(&journal.0), Err(JournalError::ChecksumMismatch));
    }

    #[test]
    fn truncated_tail_fails_closed() {
        let mut journal = MemoryJournal::default();
        append_transition(
            &mut journal,
            tx(),
            TransactionState::Prepared,
            b"x".to_vec(),
        )
        .unwrap();
        journal.0.pop();
        assert_eq!(replay(&journal.0), Err(JournalError::TruncatedRecord));
    }

    #[test]
    fn commit_without_prepare_is_rejected() {
        let mut journal = MemoryJournal::default();
        assert_eq!(
            append_transition(
                &mut journal,
                tx(),
                TransactionState::Committed,
                b"x".to_vec()
            ),
            Err(JournalError::TransitionWithoutPrepare)
        );
    }

    #[test]
    fn payload_cannot_change_mid_transaction() {
        let mut journal = MemoryJournal::default();
        append_transition(
            &mut journal,
            tx(),
            TransactionState::Prepared,
            b"account-a".to_vec(),
        )
        .unwrap();
        assert_eq!(
            append_transition(
                &mut journal,
                tx(),
                TransactionState::Committed,
                b"account-b".to_vec()
            ),
            Err(JournalError::ConflictingPayload)
        );
    }
}
