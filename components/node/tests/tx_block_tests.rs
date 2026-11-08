// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Regressionstests SEC-P0-001: Tx-Pruefung bei Blockproduktion und Block-Import.

use atc_node::bootstrap::{devnet_boot, Genesis};
use atc_node::chain::Chain;
use atc_node::mempool::{LedgerError, TxEnvelope, TxLedger};
use atc_wallet::keys::WalletKey;
use atc_wallet::tx::{Transaction, TxType, NUMERIC_CHAIN_ID};

fn key(seed: u8) -> WalletKey {
    WalletKey::from_seed([seed; 32]).unwrap()
}

fn envelope(k: &WalletKey, nonce: u64) -> TxEnvelope {
    let tx = Transaction {
        chain_id: NUMERIC_CHAIN_ID,
        tx_type: TxType::Transfer,
        sender_did: k.address(),
        recipient_did: Some("ATC-recipient".into()),
        amount: 100,
        gas_price: 1,
        gas_limit: 1000,
        nonce,
        timestamp: 1_700_000_000,
        payload: b"blocktx".to_vec(),
        poh_hash: [9u8; 32],
    };
    let signature = tx.sign(k).unwrap();
    TxEnvelope { tx, public_key: k.public_key(), signature }
}

#[test]
fn ledger_pruefung_ist_bei_fehler_zustandsfrei() {
    let k = key(11);
    let ledger = TxLedger::new();
    let txs = vec![envelope(&k, 0), envelope(&k, 5)]; // Luecke bei Nonce 5
    assert_eq!(
        ledger.check_block(&txs),
        Err(LedgerError::NonceGap { expected: 1 })
    );
    assert_eq!(ledger.expected_nonce(&k.address()), None, "Ledger unveraendert");
}

#[test]
fn doppelte_tx_im_selben_block_wird_abgelehnt() {
    let k = key(12);
    let ledger = TxLedger::new();
    let tx = envelope(&k, 0);
    assert_eq!(
        ledger.check_block(&[tx.clone(), tx]),
        Err(LedgerError::DuplicateTransaction)
    );
}

#[test]
fn falsche_signatur_im_block_wird_abgelehnt() {
    let signer = key(13);
    let fremd = key(14);
    let mut env = envelope(&signer, 0);
    env.public_key = fremd.public_key(); // Signatur passt nicht zum Key
    let ledger = TxLedger::new();
    assert!(matches!(
        ledger.check_block(&[env]),
        Err(LedgerError::InvalidSignature) | Err(LedgerError::SenderNotBound)
    ));
}

#[test]
fn produce_tx_block_haengt_block_an_und_committet_ledger() {
    let g = Genesis::devnet();
    let mut chain = Chain::from_genesis(&g);
    let mut ledger = TxLedger::new();
    let k = key(15);
    let tx = envelope(&k, 0);

    let hash = chain.produce_tx_block(&[tx.clone()], &mut ledger).unwrap();
    assert_eq!(chain.height(), 1);
    assert_eq!(chain.best_hash(), hash);
    assert!(chain.verify().is_ok());
    assert_eq!(ledger.expected_nonce(&k.address()), Some(1));

    let id_hex: String = tx.tx.id().unwrap().iter().map(|b| format!("{:02x}", b)).collect();
    assert_eq!(chain.best().payload, id_hex, "Payload = Tx-ID-hex");
}

#[test]
fn replay_ueber_bloecke_wird_abgelehnt() {
    let g = Genesis::devnet();
    let mut chain = Chain::from_genesis(&g);
    let mut ledger = TxLedger::new();
    let k = key(16);
    let tx = envelope(&k, 0);

    chain.produce_tx_block(&[tx.clone()], &mut ledger).unwrap();
    let err = chain.produce_tx_block(&[tx.clone()], &mut ledger).unwrap_err();
    assert!(err.contains("Duplicate"), "Replay muss scheitern: {}", err);
    assert_eq!(chain.height(), 1, "Kette bleibt unveraendert");
}

#[test]
fn leere_tx_folge_wird_abgelehnt() {
    let g = Genesis::devnet();
    let mut chain = Chain::from_genesis(&g);
    let mut ledger = TxLedger::new();
    assert!(chain.produce_tx_block(&[], &mut ledger).is_err());
    assert_eq!(chain.height(), 0);
}

#[test]
fn import_tx_block_prueft_hoehe_und_transaktionen() {
    let g = Genesis::devnet();
    let mut chain = Chain::from_genesis(&g);
    let mut ledger = TxLedger::new();
    let k = key(17);

    // Falsche Hoehe: abgelehnt, Kette unveraendert
    assert!(chain.import_tx_block(5, &[envelope(&k, 0)], &mut ledger).is_err());
    assert_eq!(chain.height(), 0);

    // Korrekte Hoehe: angehaengt
    let h = chain.import_tx_block(1, &[envelope(&k, 0)], &mut ledger).unwrap();
    assert_eq!(chain.height(), 1);
    assert_eq!(chain.best_hash(), h);
    assert!(chain.verify().is_ok());

    // Nicht-sequenzielle Nonce im Folgeblock: abgelehnt
    let err = chain.import_tx_block(2, &[envelope(&k, 7)], &mut ledger).unwrap_err();
    assert!(err.contains("NonceGap"), "erwartet NonceGap: {}", err);
    assert_eq!(chain.height(), 1);
}

#[test]
fn sequenzielle_nonces_ueber_bloecke_werden_akzeptiert() {
    let g = Genesis::devnet();
    let mut chain = Chain::from_genesis(&g);
    let mut ledger = TxLedger::new();
    let k = key(18);

    chain.produce_tx_block(&[envelope(&k, 0), envelope(&k, 1)], &mut ledger).unwrap();
    assert_eq!(chain.height(), 1);
    assert_eq!(ledger.expected_nonce(&k.address()), Some(2));

    chain.produce_tx_block(&[envelope(&k, 2)], &mut ledger).unwrap();
    assert_eq!(chain.height(), 2);
    assert!(chain.verify().is_ok());
}
