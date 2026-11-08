//! SEC-P0-007: Blockhashes sind 256 Bit breit und deckungsgleich gebunden.
use atc_node::bootstrap::Genesis;
use atc_node::chain::{hash_hex, parse_hash, Block, Chain};

#[test]
fn blockhash_ist_256_bit_breit() {
    let mut c = Chain::from_genesis(&Genesis::devnet());
    let h = c.produce("block-1").unwrap();
    assert_eq!(h.len(), 32);
    assert_eq!(hash_hex(&h).len(), 64);
}

#[test]
fn genesis_block_bindet_vollen_boot_hash() {
    let g = Genesis::devnet();
    let b = Block::genesis_block(&g);
    assert_eq!(b.prev_hash, g.boot_hash256());
    assert_eq!(hash_hex(&b.prev_hash).len(), 64);
}

#[test]
fn hex_roundtrip_ist_verlustfrei() {
    let h = [0xab_u8; 32];
    assert_eq!(parse_hash(&hash_hex(&h)).unwrap(), h);
    assert!(parse_hash("abc").is_err());
    assert!(parse_hash(&"z".repeat(64)).is_err());
}

#[test]
fn jede_payload_aenderung_kippt_den_hash() {
    let mut a = Chain::from_genesis(&Genesis::devnet());
    let mut b = Chain::from_genesis(&Genesis::devnet());
    a.produce("payload-a").unwrap();
    b.produce("payload-b").unwrap();
    assert_ne!(a.best_hash(), b.best_hash());
}

#[test]
fn hoehen_ergeben_verschiedene_hashes() {
    let mut c = Chain::from_genesis(&Genesis::devnet());
    let h1 = c.produce("gleich").unwrap();
    let h2 = c.produce("gleich").unwrap();
    assert_ne!(h1, h2, "gleicher Payload auf anderer Hoehe muss anderen Hash ergeben");
}
