// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
use atc_wallet::{balance::Balance, keys::WalletKey};

fn main() {
    let key = WalletKey::from_seed([0u8; 32]);
    let public_key = key.public_key();
    let balance = Balance::default();
    println!("atc-wallet — A-TownChain-Okosystems");
    println!("public_key={}", hex_encode(&public_key));
    println!("available={}", balance.available());
    println!("locked={}", balance.locked());
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}
