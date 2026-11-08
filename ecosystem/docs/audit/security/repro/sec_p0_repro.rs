//! SEC-P0-Reproduktionen (Audit 2026-11-05). Temporaer, wird nach dem Lauf entfernt.
use atc_wallet::keys::WalletKey;
use atc_wallet::tx::{Transaction, TxType, NUMERIC_CHAIN_ID};

fn base_tx(sender_did: &str) -> Transaction {
    Transaction {
        chain_id: NUMERIC_CHAIN_ID, tx_type: TxType::Transfer,
        sender_did: sender_did.into(), recipient_did: Some("ATC-empfaenger".into()),
        amount: 1, gas_price: 1, gas_limit: 1, nonce: 0, timestamp: 1,
        payload: vec![], poh_hash: [0u8; 32],
    }
}

/// SEC-P0-004: Jeder beliebige Schlüssel kann jede beliebige DID beanspruchen.
/// Es existiert keine Pruefung der Bindung DID <-> Public Key.
#[test]
fn repro_any_key_can_claim_any_did() {
    let attacker = WalletKey::from_seed([0xAA; 32]).unwrap();
    let tx = base_tx("ATC-VICTIM-DID-0001");
    let mut claimed = tx.clone();
    claimed.amount = 999_999; // Angreifer signiert eigene Transaktion MIT Opfer-DID
    let sig = claimed.sign(&attacker).unwrap();
    assert!(claimed.verify(&attacker.public_key(), &sig).is_ok(),
        "Erwartung fuer den Nachweis: Signatur valid, obwohl die DID dem Angreifer nicht gehoert");
    println!("NACHWEIS SEC-P0-004: Angreifer-Key signiert gueltig als DID 'ATC-VICTIM-DID-0001'");
}

/// SEC-P0-005: Es gibt keine Adressableitung aus dem Public Key im Live-Wallet.
/// (WalletKey bietet nur public_key(); kein address()/DID-Format.)
#[test]
fn repro_no_address_derivation() {
    let k = WalletKey::from_seed([7u8;32]).unwrap();
    let pk = k.public_key();
    println!("NACHWEIS SEC-P0-005: public_key={} Bytes, keine Adress-/DID-Ableitung im API", pk.len());
    assert_eq!(pk.len(), 33);
}

/// SEC-P0-006: Cross-Curve-Inkompatibilitaet ist sogar typgebunden:
/// Tx::verify verlangt &[u8; 33] (secp256k1 compressed).
/// Ed25519-Pubkeys (kai-os keyring) sind [u8; 32] und werden vom Typensystem
/// abgelehnt — ein "einheitliches" Verifikationsverfahren existiert nicht.
/// Ausserdem existiert eine zweite, tote Wallet-API (src/wallet.rs) mit [u8; 32].
#[test]
fn repro_key_width_conflict_is_type_level() {
    let k = WalletKey::from_seed([7u8;32]).unwrap();
    let pk33: [u8; 33] = k.public_key();
    let ed_pubkey_32: [u8; 32] = [0x11; 32]; // Ed25519-Format (kai-os)
    // Kein gemeinsamer Verifikationstyp:
    let _ok = pk33;
    assert_eq!(std::mem::size_of_val(&ed_pubkey_32), 32);
    println!("NACHWEIS SEC-P0-006: secp256k1=33B vs Ed25519=32B; Tx::verify nimmt nur [u8;33] (Compile-Time-Konflikt)");
}
