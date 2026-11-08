// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Regressionstests SEC-P0-004: DID-Aufloesung, Rotation, strikte Policies.

use atc_node::did::DidRegistry;
use atc_node::mempool::{IdentityPolicy, Mempool, MempoolError, TxEnvelope, TxLedger};
use atc_wallet::keys::WalletKey;
use atc_wallet::tx::{Transaction, TxType, NUMERIC_CHAIN_ID};
use sha2::{Digest, Sha256};

fn key(seed: u8) -> WalletKey {
    WalletKey::from_seed([seed; 32]).unwrap()
}

fn envelope(k: &WalletKey, sender: &str, nonce: u64) -> TxEnvelope {
    let tx = Transaction {
        chain_id: NUMERIC_CHAIN_ID,
        tx_type: TxType::Transfer,
        sender_did: sender.to_string(),
        recipient_did: Some("ATC-empfaenger".into()),
        amount: 5,
        gas_price: 1,
        gas_limit: 21000,
        nonce,
        timestamp: 1_700_000_000,
        payload: Vec::new(),
        poh_hash: [3u8; 32],
    };
    let signature = tx.sign(k).unwrap();
    TxEnvelope { tx, public_key: k.public_key(), signature }
}

fn sign_raw(k: &WalletKey, message: &[u8]) -> [u8; 64] {
    let digest: [u8; 32] = Sha256::digest(message).into();
    k.sign_digest(digest).serialize_compact()
}

#[test]
fn strikter_mempool_lehnt_unbekannte_did_ab() {
    let k = key(31);
    let policy = IdentityPolicy::strict(DidRegistry::new());
    let mut pool = Mempool::with_policy(10, policy);
    let did = k.address();
    assert_eq!(pool.admit(envelope(&k, &did, 0)), Err(MempoolError::UnknownDid));
}

#[test]
fn strikter_mempool_akzeptiert_registrierte_did() {
    let k = key(32);
    let mut reg = DidRegistry::new();
    let did = reg.register(k.public_key()).unwrap();
    let mut pool = Mempool::with_policy(10, IdentityPolicy::strict(reg));
    assert!(pool.admit(envelope(&k, &did, 0)).is_ok());
}

#[test]
fn nach_rotation_gilt_nur_noch_der_neue_schluessel() {
    let alt = key(33);
    let neu = key(34);
    let mut reg = DidRegistry::new();
    let did = reg.register(alt.public_key()).unwrap();

    // Rotation: aktueller Schluessel signiert die kanonische Nachricht
    let msg = DidRegistry::rotation_message(&did, &neu.public_key(), 2, reg.epoch());
    let proof = sign_raw(&alt, &msg);
    reg.rotate(&did, neu.public_key(), &proof, 2).unwrap();

    let mut pool = Mempool::with_policy(10, IdentityPolicy::strict(reg));

    // Alter Schluessel: Signatur technisch gueltig, aber nicht mehr autoritativ
    let alt_env = envelope(&alt, &did, 0);
    assert_eq!(pool.admit(alt_env), Err(MempoolError::KeyMismatch));

    // Neuer Schluessel: akzeptiert
    let neu_env = envelope(&neu, &did, 0);
    assert!(pool.admit(neu_env).is_ok(), "neuer Schluessel muss gelten");
}

#[test]
fn strikter_ledger_prueft_dids_im_block() {
    let reg_unbekannt = DidRegistry::new();
    let ledger = TxLedger::with_policy(IdentityPolicy::strict(reg_unbekannt));
    let k = key(35);
    let did = k.address();
    assert!(ledger.check_block(&[envelope(&k, &did, 0)]).is_err());

    let mut reg = DidRegistry::new();
    let did = reg.register(k.public_key()).unwrap();
    let ledger = TxLedger::with_policy(IdentityPolicy::strict(reg));
    assert!(ledger.check_block(&[envelope(&k, &did, 0)]).is_ok());
}
